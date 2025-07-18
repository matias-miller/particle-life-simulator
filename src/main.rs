use ggez::{
    event::{self, EventHandler},
    graphics::{Canvas, Color, DrawParam, Text},
    input::keyboard::{KeyInput, KeyCode},
    input::mouse::MouseButton,
    Context, GameResult,
};
use glam::Vec2;
use rand::Rng;
use std::time::Instant;

mod particle;
mod simulation;
mod utils;

use particle::{Particle, ParticleType};
use simulation::World;
use utils::math::Vec2 as MyVec2;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

struct ParticleLifeGame {
    world: World,
    paused: bool,
    show_debug: bool,
    fps_timer: Instant,
    frame_count: u32,
    current_fps: u32,
    cursor_pos: Vec2,
    selected_param: Option<usize>, // 0: red_red, 1: red_blue, 2: blue_red, 3: blue_blue
}

impl ParticleLifeGame {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut world = World::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        
        // Load preset 1 automatically on application start
        world.load_preset(1);
        println!("Loaded preset 1 on application start");
        
        Ok(Self {
            world,
            paused: false,
            show_debug: false,
            fps_timer: Instant::now(),
            frame_count: 0,
            current_fps: 0,
            cursor_pos: Vec2::ZERO,
            selected_param: None,
        })
    }
    
    fn add_particle_at_cursor(&mut self, particle_type: ParticleType) {
        let mut rng = rand::thread_rng();
        let velocity = MyVec2::new(
            rng.gen_range(-50.0..50.0),
            rng.gen_range(-50.0..50.0),
        );
        
        self.world.add_particle(Particle::new(
            MyVec2::new(self.cursor_pos.x, self.cursor_pos.y),
            velocity,
            particle_type,
            1.0,
            3.0,
        ));
    }
    
    fn adjust_interaction_param(&mut self, delta: f32) {
        if let Some(param) = self.selected_param {
            let matrix = self.world.get_interaction_matrix_mut();
            match param {
                0 => matrix.red_red += delta,
                1 => matrix.red_blue += delta,
                2 => matrix.red_green += delta,
                3 => matrix.blue_red += delta,
                4 => matrix.blue_blue += delta,
                5 => matrix.blue_green += delta,
                6 => matrix.green_red += delta,
                7 => matrix.green_blue += delta,
                8 => matrix.green_green += delta,
                _ => {}
            }
            println!("Adjusted interaction parameter {} by {:.2}", param, delta);
        }
    }
}

impl EventHandler for ParticleLifeGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update cursor position
        self.cursor_pos = ctx.mouse.position().into();
        
        // Update FPS counter
        self.frame_count += 1;
        if self.fps_timer.elapsed().as_secs() >= 1 {
            self.current_fps = self.frame_count;
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
        
        // Update simulation
        if !self.paused {
            let dt = ctx.time.delta().as_secs_f32();
            // Cap delta time to prevent large jumps
            let dt = dt.min(1.0 / 30.0);
            self.world.update(dt);
        }
        
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        
        // Draw particles
        let particles = self.world.get_particles();
        for particle in particles {
            let color = match particle.particle_type {
                ParticleType::Red => Color::RED,
                ParticleType::Blue => Color::BLUE,
                ParticleType::Green => Color::GREEN,
                ParticleType::NeonPink => Color::new(1.0, 0.0, 0.5, 1.0), // Neon pink color
            };
            
            let circle = ggez::graphics::Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                Vec2::new(particle.position.x, particle.position.y),
                particle.radius,
                0.1,
                color,
            )?;
            
            canvas.draw(&circle, DrawParam::default());
        }
        
        // Draw debug info
        if self.show_debug {
            // Draw debug background
            let debug_bg = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(5.0, 5.0, 300.0, 300.0),
                Color::new(0.0, 0.0, 0.0, 0.8),
            )?;
            canvas.draw(&debug_bg, DrawParam::default());

            let matrix = self.world.get_interaction_matrix();
            let debug_text = format!(
                "FPS: {}\nParticles: {}\nStatus: {}\nCursor: ({:.1}, {:.1})\n\
                 Interaction Matrix:\n\
                 Red-Red: {:.2}\nRed-Blue: {:.2}\nRed-Green: {:.2}\nRed-Pink: {:.2}\n\
                 Blue-Red: {:.2}\nBlue-Blue: {:.2}\nBlue-Green: {:.2}\nBlue-Pink: {:.2}\n\
                 Green-Red: {:.2}\nGreen-Blue: {:.2}\nGreen-Green: {:.2}\nGreen-Pink: {:.2}\n\
                 Pink-Red: {:.2}\nPink-Blue: {:.2}\nPink-Green: {:.2}\nPink-Pink: {:.2}\n\
                 Selected Param: {}",
                self.current_fps, 
                self.world.particle_count(), 
                if self.paused { "PAUSED" } else { "RUNNING" },
                self.cursor_pos.x, 
                self.cursor_pos.y,
                matrix.red_red,
                matrix.red_blue,
                matrix.red_green,
                matrix.red_pink,
                matrix.blue_red,
                matrix.blue_blue,
                matrix.blue_green,
                matrix.blue_pink,
                matrix.green_red,
                matrix.green_blue,
                matrix.green_green,
                matrix.green_pink,
                matrix.pink_red,
                matrix.pink_blue,
                matrix.pink_green,
                matrix.pink_pink,
                match self.selected_param {
                    Some(0) => "Red-Red",
                    Some(1) => "Red-Blue",
                    Some(2) => "Red-Green",
                    Some(3) => "Blue-Red",
                    Some(4) => "Blue-Blue",
                    Some(5) => "Blue-Green",
                    Some(6) => "Green-Red",
                    Some(7) => "Green-Blue",
                    Some(8) => "Green-Green",
                    Some(_) => "Invalid",
                    None => "None",
                }
            );
            
            let text = Text::new(debug_text);
            canvas.draw(&text, DrawParam::default().dest(Vec2::new(10.0, 10.0)).color(Color::WHITE));
        }
        
        // Draw controls background
        let controls_bg = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(5.0, WINDOW_HEIGHT - 270.0, 300.0, 260.0),
            Color::new(0.0, 0.0, 0.0, 0.8),
        )?;
        canvas.draw(&controls_bg, DrawParam::default());

        // Draw controls
        let controls_text = Text::new(
        "SPACE: Pause/Resume\n\
         R: Reset\n\
         D: Toggle Debug\n\
         1-6: Load Presets\n\
             ESC: Exit\n\
             Left Click: Add Red Particles\n\
             Shift + Left Click: Add Neon Pink Particles\n\
             Right Click: Add Blue Particles\n\
             Middle Click: Add Green Particles\n\
             F1-F9: Select Interaction Param\n\
             Numpad +/-: Adjust Selected Param\n\
             =/- Keys: Adjust Selected Param"
        );
        canvas.draw(&controls_text, DrawParam::default().dest(Vec2::new(10.0, WINDOW_HEIGHT - 260.0)).color(Color::WHITE));
        
        canvas.finish(ctx)?;
        
        Ok(())
    }
    
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) => {
                self.paused = !self.paused;
                println!("Simulation {}", if self.paused { "paused" } else { "resumed" });
            }
            Some(KeyCode::R) => {
                self.world.clear();
                self.world.load_preset(1); // Reset loads preset 1
                println!("Simulation reset to preset 1");
            }
            Some(KeyCode::D) => {
                self.show_debug = !self.show_debug;
                println!("Debug display {}", if self.show_debug { "enabled" } else { "disabled" });
            }
            Some(KeyCode::Escape) => {
                ctx.request_quit();
            }
            Some(KeyCode::Key1) => {
                self.world.load_preset(1);
                println!("Loaded preset 1");
            }
            Some(KeyCode::Key2) => {
                self.world.load_preset(2);
                println!("Loaded preset 2");
            }
            Some(KeyCode::Key3) => {
                self.world.load_preset(3);
                println!("Loaded preset 3");
            }
            Some(KeyCode::Key4) => {
                self.world.load_preset(4);
                println!("Loaded preset 4");
            }
            Some(KeyCode::Key5) => {
                self.world.load_preset(5);
                println!("Loaded preset 5");
            }
            Some(KeyCode::Key6) => {
                self.world.load_preset(6);
                println!("Loaded preset 6");
            }
            Some(KeyCode::F1) => {
                self.selected_param = Some(0);
                println!("Selected Red-Red interaction");
            }
            Some(KeyCode::F2) => {
                self.selected_param = Some(1);
                println!("Selected Red-Blue interaction");
            }
            Some(KeyCode::F3) => {
                self.selected_param = Some(2);
                println!("Selected Blue-Red interaction");
            }
            Some(KeyCode::F4) => {
                self.selected_param = Some(3);
                println!("Selected Blue-Red interaction");
            }
            Some(KeyCode::F5) => {
                self.selected_param = Some(4);
                println!("Selected Blue-Blue interaction");
            }
            Some(KeyCode::F6) => {
                self.selected_param = Some(5);
                println!("Selected Blue-Green interaction");
            }
            Some(KeyCode::F7) => {
                self.selected_param = Some(6);
                println!("Selected Green-Red interaction");
            }
            Some(KeyCode::F8) => {
                self.selected_param = Some(7);
                println!("Selected Green-Blue interaction");
            }
            Some(KeyCode::F9) => {
                self.selected_param = Some(8);
                println!("Selected Green-Green interaction");
            }
            Some(KeyCode::NumpadAdd) => {
                self.adjust_interaction_param(0.05);
            }
            Some(KeyCode::NumpadSubtract) => {
                self.adjust_interaction_param(-0.05);
            }
            Some(KeyCode::Equals) => {
                self.adjust_interaction_param(0.05);
            }
            Some(KeyCode::Minus) => {
                self.adjust_interaction_param(-0.05);
            }

            _ => {}
        }
        
        Ok(())
    }
    
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        self.cursor_pos = Vec2::new(x, y);
        
        let particle_type = match button {
            MouseButton::Left => {
                if ctx.keyboard.is_key_pressed(KeyCode::LShift) || ctx.keyboard.is_key_pressed(KeyCode::RShift) {
                    ParticleType::NeonPink
                } else {
                    ParticleType::Red
                }
            }
            MouseButton::Right => ParticleType::Blue,
            MouseButton::Middle => ParticleType::Green,
            _ => return Ok(()),
        };
        
        self.add_particle_at_cursor(particle_type);
        
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("particle-life-game", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Particle Life Game"))
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
            .resizable(false)
        );
    
    let (mut ctx, event_loop) = cb.build()?;
    
    let game = ParticleLifeGame::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}