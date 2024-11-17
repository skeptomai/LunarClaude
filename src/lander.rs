use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, MeshBuilder};
use ggez::mint::Point2;
use glam::Vec2;
use log::info;

const GRAVITY: f32 = 1.62; // Lunar gravity (m/s²)
const THRUST_POWER: f32 = 3.5;
const MAX_SAFE_LANDING_VELOCITY: f32 = 2.0; // m/s
const MAX_SAFE_LANDING_ANGLE: f32 = 0.15; // radians (approximately 8.6 degrees)
const DT: f32 = 1.0 / 60.0; // 60 FPS

pub struct LunarLander {
    pub position: Point2<f32>,
    pub velocity: Vec2,
    pub angle: f32,
    pub thrust: f32,
    pub fuel: f32,
    landing_safety_checked: bool,
    landed_safely: bool,
}

impl LunarLander {
    pub fn new(x: f32, y: f32) -> Self {
        LunarLander {
            position: Point2 { x, y },
            velocity: Vec2::ZERO,
            angle: 0.0,
            thrust: 0.0,
            fuel: 100.0,
            landing_safety_checked: false,
            landed_safely: false,
        }
    }

    pub fn update(&mut self) {
        if self.fuel > 0.0 && self.thrust > 0.0 {
            // Apply thrust     
            let thrust_vector = Vec2::new(
                -self.thrust * self.angle.cos() * THRUST_POWER,  // Negative because right is positive x
                self.thrust * self.angle.sin() * THRUST_POWER    // Positive because up is positive y
            );

            info!("Thrust: {}, Angle: {}, Vector: {:?}", self.thrust, self.angle, thrust_vector); // Debug

            self.velocity += thrust_vector * DT;
            self.fuel -= self.thrust * 0.5;
        }

        // Apply gravity
        //self.velocity.y -= GRAVITY * DT;
        // Should be
        self.velocity.y -= GRAVITY * DT;  // Add gravity since positive y is up

        // Update position
        self.position.x += self.velocity.x * DT;
        self.position.y -= self.velocity.y * DT;

        // Keep lander in bounds
        self.position.x = self.position.x.clamp(0.0, 800.0);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        // Draw lander body
        let body_mesh = self.create_body_mesh(ctx)?;
        canvas.draw(&body_mesh, graphics::DrawParam::default());

        // Draw thrust flame if thrusting
        if self.thrust > 0.0 && self.fuel > 0.0 {
            let flame_mesh = self.create_flame_mesh(ctx)?;
            canvas.draw(&flame_mesh, graphics::DrawParam::default());
        }

        Ok(())
    }

    fn create_body_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let points = self.get_vertices();
        let legs = self.get_legs_points();
        
        let mut mb = MeshBuilder::new();
        
        // Draw main body
        mb.polygon(DrawMode::fill(), &points, Color::WHITE)?;
        
        // Draw legs
        mb.line(&[legs[0], points[1]], 2.0, Color::WHITE)?;
        mb.line(&[legs[1], points[2]], 2.0, Color::WHITE)?;
        
        Ok(Mesh::from_data(ctx, mb.build()))
    }

    fn create_flame_mesh(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let flame_points = self.get_flame_vertices();
        
        let mut mb = MeshBuilder::new();
        mb.polygon(DrawMode::fill(), &flame_points, Color::new(1.0, 0.5, 0.0, self.thrust))?;
        
        Ok(Mesh::from_data(ctx, mb.build()))
    }

    fn get_vertices(&self) -> Vec<Point2<f32>> {
        let cos_angle = self.angle.cos();
        let sin_angle = self.angle.sin();
        
        vec![
            Point2 { // Nose
                x: self.position.x + (0.0 * cos_angle - 15.0 * sin_angle),
                y: self.position.y + (0.0 * sin_angle + 15.0 * cos_angle),
            },
            Point2 { // Left side
                x: self.position.x + (-10.0 * cos_angle - (-10.0) * sin_angle),
                y: self.position.y + (-10.0 * sin_angle + (-10.0) * cos_angle),
            },
            Point2 { // Right side
                x: self.position.x + (10.0 * cos_angle - (-10.0) * sin_angle),
                y: self.position.y + (10.0 * sin_angle + (-10.0) * cos_angle),
            },
        ]
    }

    fn get_flame_vertices(&self) -> Vec<Point2<f32>> {
        let cos_angle = self.angle.cos();
        let sin_angle = self.angle.sin();
        
        vec![
            Point2 {
                x: self.position.x + (-5.0 * cos_angle - (-8.0) * sin_angle),
                y: self.position.y + (-5.0 * sin_angle + (-8.0) * cos_angle),
            },
            Point2 {
                x: self.position.x + (5.0 * cos_angle - (-8.0) * sin_angle),
                y: self.position.y + (5.0 * sin_angle + (-8.0) * cos_angle),
            },
            Point2 {
                x: self.position.x + (0.0 * cos_angle - (-20.0) * sin_angle),
                y: self.position.y + (0.0 * sin_angle + (-20.0) * cos_angle),
            },
        ]
    }

    pub fn get_legs_points(&self) -> Vec<Point2<f32>> {
        let cos_angle = self.angle.cos();
        let sin_angle = self.angle.sin();
        
        vec![
            Point2 {
                x: self.position.x + (-15.0 * cos_angle - (-5.0) * sin_angle),
                y: self.position.y + (-15.0 * sin_angle + (-5.0) * cos_angle),
            },
            Point2 {
                x: self.position.x + (15.0 * cos_angle - (-5.0) * sin_angle),
                y: self.position.y + (15.0 * sin_angle + (-5.0) * cos_angle),
            },
        ]
    }

    pub fn apply_thrust(&mut self, amount: f32) {
        self.thrust = if self.fuel > 0.0 {
            let thrust = amount.clamp(0.0, 1.0);
            info!("Applying thrust: {}", thrust); // Debug log
            thrust
        } else {
            0.0
        };
    }

    pub fn rotate(&mut self, amount: f32) {
        self.angle = (self.angle + amount) % (2.0 * std::f32::consts::PI);
    }

    pub fn check_landing_safety(&mut self, surface_angle: f32) {
        if !self.landing_safety_checked {
            let velocity_magnitude = self.velocity.length();
            let relative_angle = (self.angle - surface_angle).abs();
            
            self.landed_safely = velocity_magnitude <= MAX_SAFE_LANDING_VELOCITY 
                && relative_angle <= MAX_SAFE_LANDING_ANGLE;
            self.landing_safety_checked = true;
        }
    }

    pub fn is_landed_safely(&self) -> bool {
        self.landed_safely
    }
}