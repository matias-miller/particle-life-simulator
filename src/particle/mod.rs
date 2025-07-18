use crate::utils::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub particle_type: ParticleType,
    pub mass: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new(
        position: Vec2,
        velocity: Vec2,
        particle_type: ParticleType,
        mass: f32,
        radius: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            particle_type,
            mass: match particle_type {
                ParticleType::Red => mass,
                ParticleType::Blue => mass * 1.2, // Slightly heavier blue particles
                ParticleType::Green => mass * 0.8, // Lighter green particles
            },
            radius: match particle_type {
                ParticleType::Red => radius,
                ParticleType::Blue => radius * 1.1, // Slightly larger blue particles
                ParticleType::Green => radius * 0.9, // Smaller green particles
            },
        }
    }
}