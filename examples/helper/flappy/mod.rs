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
        let rect = graphics::Rect::new(pos.x, pos.y, 34.0 * 1.5, 24.0 * 1.5);

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

    pub fn draw_param(&self) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .dest(self.rect.point())
            .scale(na::Vector2::new(1.5, 1.5))
    }

    pub fn jump(&mut self) {
        if self.y_velocity >= 0.0 {
            self.y_velocity = -7.0;
        }
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

pub struct PipePair {
    pipe_image: graphics::Image,
    upper_rect: graphics::Rect,
    lower_rect: graphics::Rect,
}

impl PipePair {
    pub fn new(image: graphics::Image, pos: na::Point2<f32>) -> Self {
        PipePair {
            pipe_image: image,
            upper_rect: graphics::Rect::new(pos.x, pos.y - 400.0, 65.0, 400.0),
            lower_rect: graphics::Rect::new(pos.x, pos.y + 100.0, 65.0, 400.0),
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        /*
        let param = graphics::DrawParam::new()
            .dest(na::Point2::new(0.0, 0.0))
            .scale(na::Vector2::new(1.25, 1.25));
        graphics::draw(ctx, &self.pipe_image, param)
        */

        let upper = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.upper_rect,
            *opencolor::GRAY5,
        )?;
        let lower = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.lower_rect,
            *opencolor::GRAY5,
        )?;
        graphics::draw(ctx, &upper, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &lower, (na::Point2::new(0.0, 0.0),))
    }
}
