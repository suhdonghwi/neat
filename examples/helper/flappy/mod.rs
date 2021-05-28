use ggez::graphics;
use ggez::nalgebra as na;

use super::opencolor;

pub struct Bird {
    rect: graphics::Rect,
    y_velocity: f32,
    y_accel: f32,
    fitness: Option<f64>,
}

impl Bird {
    pub fn new(pos: na::Point2<f32>, velocity: f32, accel: f32) -> Self {
        let rect = graphics::Rect::new(pos.x, pos.y, 50.0, 30.0);

        Bird {
            rect,
            y_velocity: velocity,
            y_accel: accel,
            fitness: None,
        }
    }

    pub fn update(&mut self) {
        self.rect.y += self.y_velocity;
        self.y_velocity += self.y_accel;
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            *opencolor::GRAY5,
        )?;

        graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0),))
    }

    pub fn jump(&mut self) {
        self.y_velocity -= 5.0;
    }

    pub fn is_dead(&self) -> bool {
        self.fitness.is_some()
    }

    pub fn rect(&self) -> graphics::Rect {
        self.rect
    }

    pub fn y_velocity(&self) -> f32 {
        self.y_velocity
    }

    pub fn kill(&mut self, fitness: f64) {
        self.fitness = Some(fitness);
    }

    pub fn fitness(&self) -> Option<f64> {
        self.fitness
    }
}
