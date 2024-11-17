use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, DrawMode, Mesh, MeshBuilder};
use ggez::mint::Point2;
use rand::Rng;

use crate::lander::LunarLander;

pub struct Terrain {
    pub mesh: Mesh,
    points: Vec<TerrainPoint>,
}

struct TerrainPoint {
    position: Point2<f32>,
    is_landing_pad: bool,
}

pub fn generate_terrain(ctx: &mut Context) -> GameResult<Terrain> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    
    // Generate terrain points
    let num_points = 100;
    let dx = 800.0 / (num_points - 1) as f32;
    
    for i in 0..num_points {
        let x = i as f32 * dx;
        let y = rng.gen_range(400.0..500.0);
        points.push(TerrainPoint {
            position: Point2 { x, y },
            is_landing_pad: false,
        });
    }
    
    // Add landing pads
    for _ in 0..3 {
        let pad_start = rng.gen_range(5..90);
        let pad_width = 5;
        let pad_height = points[pad_start].position.y;
        
        for i in pad_start..pad_start + pad_width {
            points[i].position.y = pad_height;
            points[i].is_landing_pad = true;
        }
    }
    
    // Create mesh
    let mesh = create_terrain_mesh(ctx, &points)?;
    
    Ok(Terrain { mesh, points })
}

fn create_terrain_mesh(ctx: &mut Context, points: &[TerrainPoint]) -> GameResult<Mesh> {
    let mut mb = MeshBuilder::new();
    
    // Draw terrain body
    let mut mesh_points = Vec::new();
    for point in points {
        mesh_points.push(point.position);
    }
    
    // Add bottom points to close the shape
    mesh_points.push(Point2 { x: 800.0, y: 600.0 });
    mesh_points.push(Point2 { x: 0.0, y: 600.0 });
    
    mb.polygon(
        DrawMode::fill(),
        &mesh_points,
        Color::from_rgb(150, 150, 150),
    )?;
    
    // Draw landing pads with different color
    for i in 0..points.len() - 1 {
        if points[i].is_landing_pad {
            mb.line(
                &[points[i].position, points[i + 1].position],
                2.0,
                Color::from_rgb(0, 255, 0),
            )?;
        }
    }
    
    Ok(Mesh::from_data(ctx, mb.build()))
}

impl Terrain {
    pub fn draw(&self, canvas: &mut Canvas) -> GameResult {
        canvas.draw(&self.mesh, graphics::DrawParam::default());
        Ok(())
    }

    pub fn check_collision(&self, lander: &mut LunarLander) -> bool {
        let legs = lander.get_legs_points();
        
        for leg in legs {
            for i in 0..self.points.len() - 1 {
                let p1 = self.points[i].position;
                let p2 = self.points[i + 1].position;
                
                if point_in_segment(leg, p1, p2) {
                    // Calculate surface angle for landing check
                    let dx = p2.x - p1.x;
                    let dy = p2.y - p1.y;
                    let surface_angle = (dy / dx).atan();
                    
                    lander.check_landing_safety(surface_angle);
                    return true;
                }
            }
        }
        false
    }
}

fn point_in_segment(point: Point2<f32>, p1: Point2<f32>, p2: Point2<f32>) -> bool {
    if point.x < p1.x.min(p2.x) || point.x > p1.x.max(p2.x) {
        return false;
    }
    
    let t = (point.x - p1.x) / (p2.x - p1.x);
    let interpolated_y = p1.y + t * (p2.y - p1.y);
    
    point.y >= interpolated_y
}