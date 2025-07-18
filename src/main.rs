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
            let debug_text = format!(
                "FPS: {}\nParticles: {}\nStatus: {}\nCursor: ({:.1}, {:.1})",
                self.current_fps, 
                self.world.particle_count(), 
                if self.paused { "PAUSED" } else { "RUNNING" },
                self.cursor_pos.x, 
                self.cursor_pos.y
            );
            
            let text = Text::new(debug_text);
            canvas.draw(&text, DrawParam::default().dest(Vec2::new(10.0, 10.0)).color(Color::WHITE));
        }
        
        // Draw controls
        let controls_text = Text::new(
            "SPACE: Pause/Resume\n\
             R: Reset\n\
             D: Toggle Debug\n\
             1-2: Load Presets\n\
             ESC: Exit\n\
             Left Click: Add Red Particles\n\
             Right Click: Add Blue Particles"
        );
        canvas.draw(&controls_text, DrawParam::default().dest(Vec2::new(10.0, WINDOW_HEIGHT - 150.0)).color(Color::WHITE));
        
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
            _ => {}
        }
        
        Ok(())
    }
    
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        self.cursor_pos = Vec2::new(x, y);
        
        let particle_type = match button {
            MouseButton::Left => ParticleType::Red,
            MouseButton::Right => ParticleType::Blue,
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