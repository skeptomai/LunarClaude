use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Text, TextFragment, PxScale};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::mint::Point2;
use log::debug;
use rand::Rng;

use crate::lander::LunarLander;
use crate::terrain::{Terrain, generate_terrain};
use crate::particles::Explosion;

pub struct MainState {
    lander: LunarLander,
    terrain: Terrain,
    stars: Vec<Point2<f32>>,
    game_over: bool,
    explosion: Option<Explosion>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let terrain = generate_terrain(ctx)?;
        let stars = generate_stars();
        
        Ok(MainState {
            lander: LunarLander::new(400.0, 100.0),
            terrain,
            stars,
            game_over: false,
            explosion: None,
        })
    }

    fn draw_hud(&self, canvas: &mut Canvas, _ctx: &mut Context) -> GameResult {
        let fuel_text = Text::new(
            TextFragment::new(format!("Fuel: {:.1}%", self.lander.fuel))
                .scale(PxScale::from(20.0))
        );
        let velocity_text = Text::new(
            TextFragment::new(format!(
                "Velocity: ({:.1}, {:.1})",
                self.lander.velocity.x,
                self.lander.velocity.y
            ))
            .scale(PxScale::from(20.0))
        );
        let angle_text = Text::new(
            TextFragment::new(format!("Angle: {:.1}°", self.lander.angle.to_degrees()))
                .scale(PxScale::from(20.0))
        );

        canvas.draw(
            &fuel_text,
            graphics::DrawParam::default()
                .dest([10.0, 10.0])
                .color(Color::WHITE),
        );
        canvas.draw(
            &velocity_text,
            graphics::DrawParam::default()
                .dest([10.0, 40.0])
                .color(Color::WHITE),
        );
        canvas.draw(
            &angle_text,
            graphics::DrawParam::default()
                .dest([10.0, 70.0])
                .color(Color::WHITE),
        );

        if self.game_over {
            let game_over_text = if self.lander.is_landed_safely() {
                "Successful Landing!"
            } else {
                "Crash Landing!"
            };
            let text = Text::new(
                TextFragment::new(game_over_text)
                    .scale(PxScale::from(40.0))
            );
            let screen_center = Point2 {
                x: 400.0,
                y: 300.0,
            };
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .dest(screen_center)
                    .offset([0.5, 0.5]) // Center the text
                    .color(if self.lander.is_landed_safely() {
                        Color::GREEN
                    } else {
                        Color::RED
                    }),
            );

            let restart_text = Text::new(
                TextFragment::new("Press R to restart")
                    .scale(PxScale::from(20.0))
            );
            canvas.draw(
                &restart_text,
                graphics::DrawParam::default()
                    .dest([400.0, 350.0])
                    .offset([0.5, 0.5])
                    .color(Color::WHITE),
            );
        }

        Ok(())
    }
}

fn generate_stars() -> Vec<Point2<f32>> {
    let mut rng = rand::thread_rng();
    let mut stars = Vec::new();
    for _ in 0..100 {
        stars.push(Point2 {
            x: rng.gen_range(0.0..800.0),
            y: rng.gen_range(0.0..600.0),
        });
    }
    stars
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game_over {
            self.lander.update();
            
            // Check collision with terrain
            if self.terrain.check_collision(&mut self.lander) {
                self.game_over = true;
                if !self.lander.is_landed_safely() {
                    self.explosion = Some(Explosion::new(
                        self.lander.position.x,
                        self.lander.position.y,
                    ));
                }
            }
        } else if let Some(explosion) = &mut self.explosion {
            explosion.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new Canvas
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 0.08, 1.0]), // Dark blue background
        );
        
        // Draw stars
        for &star in &self.stars {
            let star_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                star,
                1.0,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&star_mesh, graphics::DrawParam::default());
        }
        
        // Draw terrain
        self.terrain.draw(&mut canvas)?;
        
        // Draw lander if not crashed
        if !self.game_over || self.lander.is_landed_safely() {
            self.lander.draw(ctx, &mut canvas)?;
        }
        
        // Draw explosion if crashed
        if let Some(explosion) = &self.explosion {
            explosion.draw(ctx, &mut canvas)?;
        }
        
        // Draw HUD
        self.draw_hud(&mut canvas, ctx)?;
        
        // Present the canvas
        canvas.finish(ctx)?;
        
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if !self.game_over {
            match input.keycode {
                Some(KeyCode::Up) => self.lander.apply_thrust(1.0),
                Some(KeyCode::Left) => self.lander.rotate(-0.1),
                Some(KeyCode::Right) => self.lander.rotate(0.1),
                Some(KeyCode::Space) => self.lander.apply_thrust(0.5), // Half thrust option
                Some(KeyCode::R) => { // Reset game
                    debug!("Resetting game...");
                    self.lander = LunarLander::new(400.0, 100.0);
                    self.game_over = false;
                    self.explosion = None;
                }
                _ => (),
            }
        } else if let Some(KeyCode::R) = input.keycode {
            // Allow reset even when game is over
            self.lander = LunarLander::new(400.0, 100.0);
            self.game_over = false;
            self.explosion = None;
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
    ) -> GameResult {
        if !self.game_over {
            match input.keycode {
                Some(KeyCode::Up) | Some(KeyCode::Space) => self.lander.apply_thrust(0.0),
                _ => (),
            }
        }
        Ok(())
    }
}
