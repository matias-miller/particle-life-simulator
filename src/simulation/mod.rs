use crate::particle::Particle;
use crate::particle::ParticleType;
use crate::utils::math::Vec2;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub mod interaction_matrix;
mod quadtree;
pub use interaction_matrix::InteractionMatrix;
pub use self::quadtree::{Bounds, QuadTree};

const INTERACTION_RADIUS: f32 = 100.0;
const INTERACTION_RADIUS_SQUARED: f32 = INTERACTION_RADIUS * INTERACTION_RADIUS;
const COLLISION_DAMPING: f32 = 0.8; // Energy loss during collision

pub struct World {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
    interaction_matrix: InteractionMatrix,
    quad_tree: QuadTree,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            interaction_matrix: InteractionMatrix::default(),
            quad_tree: QuadTree::new(
                Bounds {
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                },
                0,
            ),
        }
    }
    
    pub fn set_interaction_matrix(&mut self, matrix: InteractionMatrix) {
        self.interaction_matrix = matrix;
    }
    
    pub fn get_interaction_matrix(&self) -> InteractionMatrix {
        self.interaction_matrix
    }
    
    pub fn get_interaction_matrix_mut(&mut self) -> &mut InteractionMatrix {
        &mut self.interaction_matrix
    }
    
    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }
    
    pub fn clear(&mut self) {
        self.particles.clear();
    }
    
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
    
    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }
    
    pub fn update(&mut self, dt: f32) {
        // Rebuild quad tree
        self.quad_tree.clear();
        for (i, particle) in self.particles.iter().enumerate() {
            self.quad_tree.insert(i, particle.position);
        }

        // Calculate forces in parallel
        let forces = Arc::new(Mutex::new(vec![Vec2::new(0.0, 0.0); self.particles.len()]));
        let collisions = Arc::new(Mutex::new(Vec::new()));

        // Process particles in parallel chunks
        self.particles.par_iter().enumerate().for_each(|(i, p1)| {
            let mut local_forces = vec![Vec2::new(0.0, 0.0); self.particles.len()];
            
            // Query nearby particles from quad tree
            let mut neighbors = Vec::new();
            let query_bounds = Bounds {
                x: p1.position.x - INTERACTION_RADIUS,
                y: p1.position.y - INTERACTION_RADIUS,
                width: INTERACTION_RADIUS * 2.0,
                height: INTERACTION_RADIUS * 2.0,
            };
            self.quad_tree.query(&query_bounds, &mut neighbors);

            for &j in &neighbors {
                if i == j {
                    continue;
                }
                
                let force = self.calculate_interaction_force(i, j);
                local_forces[i] += force;
                local_forces[j] -= force;
                
                // Record collisions to process later (using squared distance for efficiency)
                let dx = p1.position.x - self.particles[j].position.x;
                let dy = p1.position.y - self.particles[j].position.y;
                let distance_sq = dx * dx + dy * dy;
                let min_distance = p1.radius + self.particles[j].radius;
                let min_distance_sq = min_distance * min_distance;
                
                if distance_sq < min_distance_sq {
                    collisions.lock().unwrap().push((i, j));
                }
            }

            // Merge local forces into global forces
            let mut global_forces = forces.lock().unwrap();
            for (idx, force) in local_forces.into_iter().enumerate() {
                global_forces[idx] += force;
            }
        });

        let forces = Arc::try_unwrap(forces).unwrap().into_inner().unwrap();
        let collisions = Arc::try_unwrap(collisions).unwrap().into_inner().unwrap();

        // Process collisions sequentially
        for (i, j) in collisions {
            self.check_particle_collision(i, j);
        }
        
        // Update particle positions and velocities
        for (i, particle) in self.particles.iter_mut().enumerate() {
            // Apply force
            particle.velocity += forces[i] * dt;
            
            // Update position
            particle.position += particle.velocity * dt;
            
            // Handle boundary collision
            if particle.position.x - particle.radius < 0.0 {
                particle.position.x = particle.radius;
                particle.velocity.x *= -COLLISION_DAMPING;
            } else if particle.position.x + particle.radius > self.width {
                particle.position.x = self.width - particle.radius;
                particle.velocity.x *= -COLLISION_DAMPING;
            }
            
            if particle.position.y - particle.radius < 0.0 {
                particle.position.y = particle.radius;
                particle.velocity.y *= -COLLISION_DAMPING;
            } else if particle.position.y + particle.radius > self.height {
                particle.position.y = self.height - particle.radius;
                particle.velocity.y *= -COLLISION_DAMPING;
            }
            
            // Apply damping
            particle.velocity *= 0.99;
        }
    }
    
    fn check_particle_collision(&mut self, i: usize, j: usize) {
        let dx = self.particles[i].position.x - self.particles[j].position.x;
        let dy = self.particles[i].position.y - self.particles[j].position.y;
        let distance_sq = dx * dx + dy * dy;
        
        let min_distance = self.particles[i].radius + self.particles[j].radius;
        let min_distance_sq = min_distance * min_distance;
        
        if distance_sq < min_distance_sq {
            let distance = distance_sq.sqrt();
            // Collision normal
            let nx = dx / distance;
            let ny = dy / distance;
            
            // Relative velocity
            let dvx = self.particles[i].velocity.x - self.particles[j].velocity.x;
            let dvy = self.particles[i].velocity.y - self.particles[j].velocity.y;
            
            // Impulse scalar
            let impulse_scalar = 2.0 * (dvx * nx + dvy * ny) / 
                (self.particles[i].mass + self.particles[j].mass);
            
            // Update velocities
            let mut p1 = self.particles[i].clone();
            let mut p2 = self.particles[j].clone();
            
            p1.velocity.x -= impulse_scalar * p2.mass * nx * COLLISION_DAMPING;
            p1.velocity.y -= impulse_scalar * p2.mass * ny * COLLISION_DAMPING;
            
            p2.velocity.x += impulse_scalar * p1.mass * nx * COLLISION_DAMPING;
            p2.velocity.y += impulse_scalar * p1.mass * ny * COLLISION_DAMPING;
            
            // Separate particles
            let overlap = min_distance - distance;
            p1.position.x += nx * overlap * 0.5;
            p1.position.y += ny * overlap * 0.5;
            
            p2.position.x -= nx * overlap * 0.5;
            p2.position.y -= ny * overlap * 0.5;
            
            // Update particles
            self.particles[i] = p1;
            self.particles[j] = p2;
        }
    }
    
    fn calculate_interaction_force(&self, i: usize, j: usize) -> Vec2 {
        let p1 = &self.particles[i];
        let p2 = &self.particles[j];
        
        let dx = p2.position.x - p1.position.x;
        let dy = p2.position.y - p1.position.y;
        let distance_sq = dx * dx + dy * dy;
        
        // No interaction if too far (using squared distance for efficiency)
        if distance_sq > INTERACTION_RADIUS_SQUARED {
            return Vec2::new(0.0, 0.0);
        }
        
        // Calculate actual distance only if needed
        // Get force strength from interaction matrix
        let force_strength = self.interaction_matrix.get_force(p1.particle_type, p2.particle_type);
        
        // Skip force calculation if strength is zero
        if force_strength == 0.0 {
            return Vec2::new(0.0, 0.0);
        }
        
        // Calculate actual distance only if needed
        let distance = distance_sq.sqrt();
        
        // Normalized direction
        let nx = dx / distance;
        let ny = dy / distance;
        
        // Scale force by distance (stronger when closer)
        let force_magnitude = force_strength * (1.0 - distance / INTERACTION_RADIUS);
        
        Vec2::new(nx * force_magnitude, ny * force_magnitude)
    }
    
    pub fn load_preset(&mut self, preset: u32) {
        self.clear();
        
        match preset {
            1 => self.create_preset_1(false),
            2 => self.create_preset_2(false),
            3 => self.create_preset_3(false),
            4 => self.create_preset_4(),
            5 => self.create_preset_5(),
            _ => {}
        }
    }
    
    fn create_preset_1(&mut self, include_green: bool) {
        // Red and Blue particles in different regions
        for i in 0..100 {
            let angle = (i as f32) * 0.2;
            
            // Red particles on left side
            let x_red = self.width / 4.0 + angle.cos() * 100.0;
            let y_red = self.height / 2.0 + angle.sin() * 100.0;
            
            // Blue particles on right side
            let x_blue = 3.0 * self.width / 4.0 + angle.cos() * 100.0;
            let y_blue = self.height / 2.0 + angle.sin() * 100.0;
            
            self.add_particle(Particle::new(
                Vec2::new(x_red, y_red),
                Vec2::new(0.0, 0.0),
                ParticleType::Red,
                1.0,
                3.0,
            ));
            
            self.add_particle(Particle::new(
                Vec2::new(x_blue, y_blue),
                Vec2::new(0.0, 0.0),
                ParticleType::Blue,
                1.0,
                3.0,
            ));
            
            // Add green particles in the center if requested
            if include_green {
                let x_green = self.width / 2.0 + (angle * 2.0).cos() * 50.0;
                let y_green = self.height / 2.0 + (angle * 2.0).sin() * 50.0;
                
                self.add_particle(Particle::new(
                    Vec2::new(x_green, y_green),
                    Vec2::new(0.0, 0.0),
                    ParticleType::Green,
                    1.0,
                    2.0,
                ));
            }
        }
    }
    
    fn create_preset_2(&mut self, include_green: bool) {
        // Red and Blue particles in grid pattern
        for x in 0..20 {
            for y in 0..15 {
                let px = (x as f32) * (self.width / 20.0);
                let py = (y as f32) * (self.height / 15.0);
                
                let particle_type = if !include_green {
                    if (x + y) % 2 == 0 {
                        ParticleType::Red
                    } else {
                        ParticleType::Blue
                    }
                } else {
                    match (x + y) % 3 {
                        0 => ParticleType::Red,
                        1 => ParticleType::Blue,
                        _ => ParticleType::Green,
                    }
                };
                
                self.add_particle(Particle::new(
                    Vec2::new(px, py),
                    Vec2::new(0.0, 0.0),
                    particle_type,
                    1.0,
                    2.0,
                ));
            }
        }
    }
    
    fn create_preset_3(&mut self, include_green: bool) {
        let mut rng = rand::thread_rng();
        
        for _ in 0..2000 {
            let x = rng.gen_range(0.0..self.width);
            let y = rng.gen_range(0.0..self.height);
            
            let particle_type = if include_green {
                match rng.gen_range(0..3) {
                    0 => ParticleType::Red,
                    1 => ParticleType::Blue,
                    _ => ParticleType::Green,
                }
            } else {
                if rng.gen_bool(0.5) {
                    ParticleType::Red
                } else {
                    ParticleType::Blue
                }
            };
            
            self.add_particle(Particle::new(
                Vec2::new(x, y),
                Vec2::new(0.0, 0.0),
                particle_type,
                1.0,
                2.0,
            ));
        }
    }
    
    fn create_preset_4(&mut self) {
        // Green particles in the center with red and blue orbiting
        for i in 0..100 {
            let angle = (i as f32) * 0.2;
            
            // Green particles in the center
            let x_green = self.width / 2.0 + angle.cos() * 50.0;
            let y_green = self.height / 2.0 + angle.sin() * 50.0;
            
            // Red particles orbiting clockwise
            let x_red = self.width / 2.0 + (angle * 2.0).cos() * 150.0;
            let y_red = self.height / 2.0 + (angle * 2.0).sin() * 150.0;
            
            // Blue particles orbiting counter-clockwise
            let x_blue = self.width / 2.0 + (angle * 2.0 + std::f32::consts::PI).cos() * 150.0;
            let y_blue = self.height / 2.0 + (angle * 2.0 + std::f32::consts::PI).sin() * 150.0;
            
            // Add green particle
            self.add_particle(Particle::new(
                Vec2::new(x_green, y_green),
                Vec2::new(0.0, 0.0),
                ParticleType::Green,
                1.0,
                3.0,
            ));
            
            // Add red particle
            self.add_particle(Particle::new(
                Vec2::new(x_red, y_red),
                Vec2::new(0.0, 0.0),
                ParticleType::Red,
                1.0,
                3.0,
            ));
            
            // Add blue particle
            self.add_particle(Particle::new(
                Vec2::new(x_blue, y_blue),
                Vec2::new(0.0, 0.0),
                ParticleType::Blue,
                1.0,
                3.0,
            ));
        }
    }
    
    fn create_preset_5(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Create 8,000 particles
        for _ in 1..8000 {
            let x = rng.gen_range(0.0..self.width);
            let y = rng.gen_range(0.0..self.height);
            
            // Weighted distribution: 40% Red, 35% Blue, 25% Green
            let particle_type = match rng.gen_range(0..100) {
                0..40 => ParticleType::Red,
                40..75 => ParticleType::Blue,
                _ => ParticleType::Green,
            };
            
            // Minimal initial velocity for stable formations
            let vel_x = rng.gen_range(-2.0..2.0);
            let vel_y = rng.gen_range(-2.0..2.0);
            
            self.add_particle(Particle::new(
                Vec2::new(x, y),
                Vec2::new(vel_x, vel_y),
                particle_type,
                1.0,
                2.0,
            ));
        }
        
        // "Orbital Clusters" interaction matrix
        // Red particles form cores, Blue orbit around Red, Green creates bridges
        self.interaction_matrix = InteractionMatrix {
            red_red: -0.8,    // Strong repulsion - Red cores spread out
            red_blue: 0.9,    // Strong attraction - Blue orbits Red
            red_green: 0.1,   // Weak attraction - Green slightly drawn to Red
            
            blue_red: 0.3,    // Moderate attraction - Blue wants to orbit Red
            blue_blue: -0.2,  // Weak repulsion - Blue particles don't clump
            blue_green: -0.1, // Weak repulsion - Blue avoids Green slightly
            
            green_red: 0.2,   // Weak attraction - Green forms bridges to Red
            green_blue: 0.4,  // Moderate attraction - Green connects to Blue
            green_green: -0.3, // Moderate repulsion - Green spreads out
        };
    }
}