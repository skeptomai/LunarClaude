use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use log::info;
use rand::Rng;

pub struct Particle {
    position: Point2<f32>,
    velocity: Point2<f32>,
    lifetime: f32,
    initial_lifetime: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let speed = rng.gen_range(50.0..200.0);
        let lifetime = rng.gen_range(0.5..1.5);

        Particle {
            position: Point2 { x, y },
            velocity: Point2 {
                x: speed * angle.cos(),
                y: speed * angle.sin(),
            },
            lifetime,
            initial_lifetime: lifetime,
        }
    }

    fn update(&mut self) {
        const DT: f32 = 1.0 / 60.0;
        self.position.x += self.velocity.x * DT;
        self.position.y += self.velocity.y * DT;
        self.lifetime -= DT;

        // Add some gravity effect
        self.velocity.y -= 1.0;
    }

    fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

pub struct Explosion {
    particles: Vec<Particle>,
    notified_finished: bool,
}

impl Explosion {
    pub fn new(x: f32, y: f32) -> Self {
        let mut particles = Vec::new();
        // Create more particles for a bigger explosion
        for _ in 0..100 {
            particles.push(Particle::new(x, y));
        }
        Explosion {
            particles,
            notified_finished: false,
        }
    }

    pub fn update(&mut self) {
        if self.is_finished() && !self.notified_finished {
            info!("Explosion finished!");
            self.notified_finished = true;
        }
        // Update all particles and remove dead ones
        for particle in &mut self.particles {
            particle.update();
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for particle in &self.particles {
            let alpha = particle.lifetime / particle.initial_lifetime;
            let size = 2.0 * (particle.lifetime / particle.initial_lifetime);

            let color = if particle.lifetime > particle.initial_lifetime * 0.6 {
                // White/yellow core
                Color::new(1.0, 1.0, 0.8, alpha)
            } else {
                // Orange/red fade
                Color::new(1.0, 0.5 * alpha, 0.0, alpha)
            };

            let particle_mesh =
                Mesh::new_circle(ctx, DrawMode::fill(), particle.position, size, 0.1, color)?;

            canvas.draw(&particle_mesh, graphics::DrawParam::default());
        }
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.particles.is_empty()
    }
}
