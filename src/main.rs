use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, GameResult};

use log::debug;
mod game;
mod lander;
mod particles;
mod terrain;

fn main() -> GameResult {
    // Initialize logger
    env_logger::init();

    // Your existing ggez setup
    debug!("Starting game...");

    let window_setup = WindowSetup::default().title("Lunar Lander").vsync(true);

    let window_mode = WindowMode::default()
        .dimensions(800.0, 600.0)
        .resizable(false);

    let (mut ctx, event_loop) = ContextBuilder::new("Lunar Lander", "Christopher Brown")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let game_state = game::MainState::new(&mut ctx)?;
    ggez::event::run(ctx, event_loop, game_state)
}
