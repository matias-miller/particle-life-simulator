#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InteractionMatrix {
    pub red_red: f32,
    pub red_blue: f32,
    pub blue_red: f32,
    pub blue_blue: f32,
}

impl InteractionMatrix {
    pub fn new(red_red: f32, red_blue: f32, blue_red: f32, blue_blue: f32) -> Self {
        Self {
            red_red,
            red_blue,
            blue_red,
            blue_blue,
        }
    }
    
    pub fn default() -> Self {
        Self {
            red_red: -0.2,    // Repulsion between red particles
            red_blue: 0.15,   // Attraction from red to blue
            blue_red: 0.15,   // Attraction from blue to red
            blue_blue: 0.1,   // Attraction between blue particles
        }
    }
    
    pub fn get_force(&self, source: super::ParticleType, target: super::ParticleType) -> f32 {
        match (source, target) {
            (super::ParticleType::Red, super::ParticleType::Red) => self.red_red,
            (super::ParticleType::Red, super::ParticleType::Blue) => self.red_blue,
            (super::ParticleType::Blue, super::ParticleType::Red) => self.blue_red,
            (super::ParticleType::Blue, super::ParticleType::Blue) => self.blue_blue,
        }
    }
}