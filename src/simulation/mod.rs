use crate::particle::Particle;
use crate::particle::ParticleType;
use crate::utils::math::Vec2;

pub struct World {
    particles: Vec<Particle>,
    width: f32,
    height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
        }
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
        // Simple physics update - will be improved later
        for particle in &mut self.particles {
            // Update position
            particle.position.x += particle.velocity.x * dt;
            particle.position.y += particle.velocity.y * dt;
            
            // Simple boundary handling
            if particle.position.x < 0.0 || particle.position.x > self.width {
                particle.velocity.x *= -0.8;
                particle.position.x = particle.position.x.max(0.0).min(self.width);
            }
            
            if particle.position.y < 0.0 || particle.position.y > self.height {
                particle.velocity.y *= -0.8;
                particle.position.y = particle.position.y.max(0.0).min(self.height);
            }
            
            // Simple damping
            particle.velocity.x *= 0.99;
            particle.velocity.y *= 0.99;
        }
    }
    
    pub fn load_preset(&mut self, preset: u32) {
        self.clear();
        
        match preset {
            1 => self.create_preset_1(),
            2 => self.create_preset_2(),
            _ => {}
        }
    }
    
    fn create_preset_1(&mut self) {
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
        }
    }
    
    fn create_preset_2(&mut self) {
        // Red and Blue particles in grid pattern
        for x in 0..20 {
            for y in 0..15 {
                let px = (x as f32) * (self.width / 20.0);
                let py = (y as f32) * (self.height / 15.0);
                
                let particle_type = if (x + y) % 2 == 0 {
                    ParticleType::Red
                } else {
                    ParticleType::Blue
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
}