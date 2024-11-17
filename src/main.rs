use ggez::{ContextBuilder, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

use log::debug;

mod game;
mod lander;
mod terrain;
mod particles;

fn main() -> GameResult {

    // Initialize logger
    env_logger::init();

    // Your existing ggez setup
    debug!("Starting game...");
    
    let window_setup = WindowSetup::default()
        .title("Lunar Lander")
        .vsync(true);
        
    let window_mode = WindowMode::default()
        .dimensions(800.0, 600.0)
        .resizable(false);
    
    let (mut ctx, event_loop) = ContextBuilder::new("lunar_lander", "Your Name")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let game_state = game::MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, game_state)
}