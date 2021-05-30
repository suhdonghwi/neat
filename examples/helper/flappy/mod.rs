use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;

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

    pub fn draw_param(&self) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .dest(self.rect.point())
            .scale(na::Vector2::new(1.5, 1.5))
    }

    pub fn jump(&mut self) {
        self.y_velocity = -10.0;
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
    out: bool,
}

impl PipePair {
    pub fn new(image: graphics::Image, pos: na::Point2<f32>) -> Self {
        PipePair {
            pipe_image: image,
            upper_rect: graphics::Rect::new(pos.x, pos.y - 480.0, 78.0, 480.0),
            lower_rect: graphics::Rect::new(pos.x, pos.y + 150.0, 78.0, 480.0),
            out: false,
        }
    }

    pub fn overlaps(&self, other: &graphics::Rect) -> bool {
        self.upper_rect.overlaps(other) || self.lower_rect.overlaps(other)
    }

    pub fn update(&mut self, ctx: &mut ggez::Context) {
        if self.out {
            return;
        }

        self.upper_rect.x -= 4.0 * timer::delta(ctx).as_secs_f32() * 60.0;
        self.lower_rect.x -= 4.0 * timer::delta(ctx).as_secs_f32() * 60.0;

        if self.upper_rect.right() < 0.0 {
            self.out = true;
        }
    }

    pub fn reset(&mut self) {
        self.upper_rect.x = 500.0;
        self.lower_rect.x = 500.0;
    }

    pub fn past(&self, x: f32) -> bool {
        self.upper_rect.right() < x
    }

    pub fn upper_rect(&self) -> graphics::Rect {
        self.upper_rect
    }

    pub fn lower_rect(&self) -> graphics::Rect {
        self.lower_rect
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.out {
            return Ok(());
        }

        let param = graphics::DrawParam::new()
            .dest(na::Point2::new(
                self.upper_rect.x + self.upper_rect.w / 2.0,
                self.upper_rect.y + self.upper_rect.h / 2.0,
            ))
            .rotation(std::f32::consts::PI)
            .scale(na::Vector2::new(-1.5, 1.5))
            .offset(na::Point2::new(0.5, 0.5));
        graphics::draw(ctx, &self.pipe_image, param)?;

        let param = graphics::DrawParam::new()
            .dest(self.lower_rect.point())
            .scale(na::Vector2::new(1.5, 1.5));
        graphics::draw(ctx, &self.pipe_image, param)

        /*
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
        */
    }
}
