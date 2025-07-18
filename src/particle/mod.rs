use crate::utils::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Red,
    Blue,
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
            mass,
            radius,
        }
    }
}