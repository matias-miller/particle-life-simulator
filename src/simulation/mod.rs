use crate::particle::Particle;
use crate::particle::ParticleType;
use crate::utils::math::Vec2;
use rand::Rng;

pub mod interaction_matrix;
pub use interaction_matrix::InteractionMatrix;

const INTERACTION_RADIUS: f32 = 100.0;
const COLLISION_DAMPING: f32 = 0.8; // Energy loss during collision

pub struct World {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
    interaction_matrix: InteractionMatrix,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            interaction_matrix: InteractionMatrix::default(),
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
        // Calculate forces
        let mut forces = vec![Vec2::new(0.0, 0.0); self.particles.len()];
        
        // Interaction and collision detection
        for i in 0..self.particles.len() {
            for j in (i+1)..self.particles.len() {
                let interaction_force = self.calculate_interaction_force(i, j);
                
                // Apply interaction forces
                forces[i] += interaction_force;
                forces[j] -= interaction_force;
            }
        }
        
        // Collision detection (separate pass to avoid mutable borrow issues)
        for i in 0..self.particles.len() {
            for j in (i+1)..self.particles.len() {
                self.check_particle_collision(i, j);
            }
        }
        
        // Update particles and handle boundaries
        for i in 0..self.particles.len() {
            let mut particle = self.particles[i].clone();
            
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
            
            // Update the particle in the vector
            self.particles[i] = particle;
        }
    }
    
    fn check_particle_collision(&mut self, i: usize, j: usize) {
        let dx = self.particles[i].position.x - self.particles[j].position.x;
        let dy = self.particles[i].position.y - self.particles[j].position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        let min_distance = self.particles[i].radius + self.particles[j].radius;
        
        if distance < min_distance {
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
        let distance = (dx * dx + dy * dy).sqrt();
        
        // No interaction if too far
        if distance > INTERACTION_RADIUS {
            return Vec2::new(0.0, 0.0);
        }
        
        // Normalized direction
        let nx = dx / distance;
        let ny = dy / distance;
        
        // Get force strength from interaction matrix
        let force_strength = self.interaction_matrix.get_force(p1.particle_type, p2.particle_type);
        
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
        
        // Create 10,000 particles
        for _ in 0..10000 {
            let x = rng.gen_range(0.0..self.width);
            let y = rng.gen_range(0.0..self.height);
            
            // Randomly assign particle type (33% chance for each color)
            let particle_type = match rng.gen_range(0..3) {
                0 => ParticleType::Red,
                1 => ParticleType::Blue,
                _ => ParticleType::Green,
            };
            
            // Add slight random initial velocity
            let vel_x = rng.gen_range(-10.0..10.0);
            let vel_y = rng.gen_range(-10.0..10.0);
            
            self.add_particle(Particle::new(
                Vec2::new(x, y),
                Vec2::new(vel_x, vel_y),
                particle_type,
                1.0,
                2.0,
            ));
        }
        
        // Set specific interaction matrix for preset 5
        self.interaction_matrix = InteractionMatrix {
            red_red: -0.5,
            red_blue: 0.7,
            red_green: -0.3,
            blue_red: 0.7,
            blue_blue: -0.4,
            blue_green: 0.6,
            green_red: -0.3,
            green_blue: 0.6,
            green_green: -0.2,
        };
    }
}