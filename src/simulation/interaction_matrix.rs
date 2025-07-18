#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InteractionMatrix {
    pub red_red: f32,
    pub red_blue: f32,
    pub red_green: f32,
    pub red_pink: f32,
    pub blue_red: f32,
    pub blue_blue: f32,
    pub blue_green: f32,
    pub blue_pink: f32,
    pub green_red: f32,
    pub green_blue: f32,
    pub green_green: f32,
    pub green_pink: f32,
    pub pink_red: f32,
    pub pink_blue: f32,
    pub pink_green: f32,
    pub pink_pink: f32,
}

impl InteractionMatrix {
    pub fn new(
        red_red: f32,
        red_blue: f32,
        red_green: f32,
        red_pink: f32,
        blue_red: f32,
        blue_blue: f32,
        blue_green: f32,
        blue_pink: f32,
        green_red: f32,
        green_blue: f32,
        green_green: f32,
        green_pink: f32,
        pink_red: f32,
        pink_blue: f32,
        pink_green: f32,
        pink_pink: f32,
    ) -> Self {
        Self {
            red_red,
            red_blue,
            red_green,
            red_pink,
            blue_red,
            blue_blue,
            blue_green,
            blue_pink,
            green_red,
            green_blue,
            green_green,
            green_pink,
            pink_red,
            pink_blue,
            pink_green,
            pink_pink,
        }
    }
    
    pub fn default() -> Self {
        Self {
            red_red: -0.2,     // Repulsion between red particles
            red_blue: 0.15,    // Attraction from red to blue
            red_green: -0.1,   // Slight repulsion from red to green
            red_pink: 0.3,     // Strong attraction from red to pink
            blue_red: 0.15,    // Attraction from blue to red
            blue_blue: 0.1,    // Attraction between blue particles
            blue_green: 0.05,  // Weak attraction from blue to green
            blue_pink: -0.25,  // Strong repulsion from blue to pink
            green_red: -0.1,   // Slight repulsion from green to red
            green_blue: 0.05,  // Weak attraction from green to blue
            green_green: -0.3, // Strong repulsion between green particles
            green_pink: 0.2,   // Moderate attraction from green to pink
            pink_red: 0.3,     // Strong attraction from pink to red
            pink_blue: -0.25,  // Strong repulsion from pink to blue
            pink_green: 0.2,   // Moderate attraction from pink to green
            pink_pink: -0.4,   // Very strong repulsion between pink particles
        }
    }
    
    pub fn get_force(&self, source: super::ParticleType, target: super::ParticleType) -> f32 {
        match (source, target) {
            (super::ParticleType::Red, super::ParticleType::Red) => self.red_red,
            (super::ParticleType::Red, super::ParticleType::Blue) => self.red_blue,
            (super::ParticleType::Red, super::ParticleType::Green) => self.red_green,
            (super::ParticleType::Red, super::ParticleType::NeonPink) => self.red_pink,
            (super::ParticleType::Blue, super::ParticleType::Red) => self.blue_red,
            (super::ParticleType::Blue, super::ParticleType::Blue) => self.blue_blue,
            (super::ParticleType::Blue, super::ParticleType::Green) => self.blue_green,
            (super::ParticleType::Blue, super::ParticleType::NeonPink) => self.blue_pink,
            (super::ParticleType::Green, super::ParticleType::Red) => self.green_red,
            (super::ParticleType::Green, super::ParticleType::Blue) => self.green_blue,
            (super::ParticleType::Green, super::ParticleType::Green) => self.green_green,
            (super::ParticleType::Green, super::ParticleType::NeonPink) => self.green_pink,
            (super::ParticleType::NeonPink, super::ParticleType::Red) => self.pink_red,
            (super::ParticleType::NeonPink, super::ParticleType::Blue) => self.pink_blue,
            (super::ParticleType::NeonPink, super::ParticleType::Green) => self.pink_green,
            (super::ParticleType::NeonPink, super::ParticleType::NeonPink) => self.pink_pink,
        }
    }
}