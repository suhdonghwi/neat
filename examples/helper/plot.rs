use ggez::graphics;
use ggez::nalgebra as na;

use super::{opencolor, text::Text};

pub struct Axis {
    min: f32,
    max: f32,
    delta: f32,
}

impl Axis {
    pub fn new(min: f32, max: f32, delta: f32) -> Self {
        Axis { min, max, delta }
    }

    pub fn ticks(&self) -> Vec<f32> {
        let mut list = Vec::new();

        let mut n = self.min;
        while n < self.max {
            list.push(n);
            n += self.delta;
        }

        list.push(self.max);
        list
    }

    pub fn value_proportion(&self, v: f32) -> f32 {
        (v - self.min) / (self.max - self.min)
    }

    pub fn set_range(&mut self, min: f32, max: f32, delta: f32) {
        self.min = min;
        self.max = max;
        self.delta = delta;
    }
}

pub struct Plot {
    rect: graphics::Rect,
    actual_rect: graphics::Rect,

    title_text: Text,

    x_axis: Axis,
    y_axis: Axis,

    font: graphics::Font,
    mesh_builder: graphics::MeshBuilder,
}

impl Plot {
    pub fn new(
        rect: graphics::Rect,
        x_axis: Axis,
        y_axis: Axis,
        title_str: &str,
        font: graphics::Font,
    ) -> Plot {
        let title_text = Text::new(title_str, font, 35.0);

        let top_padding = 60.0;
        let right_padding = 30.0;
        let bottom_padding = 50.0;
        let left_padding = 60.0;

        let actual_rect = graphics::Rect::new(
            rect.x + left_padding,
            rect.y + top_padding,
            rect.w - left_padding - right_padding,
            rect.h - top_padding - bottom_padding,
        );

        Plot {
            rect,
            actual_rect,
            title_text,
            x_axis,
            y_axis,
            font,
            mesh_builder: graphics::MeshBuilder::new(),
        }
    }

    pub fn x_axis_mut(&mut self) -> &mut Axis {
        &mut self.x_axis
    }

    fn draw_axes(&self, ctx: &mut ggez::Context, rect: &graphics::Rect) -> ggez::GameResult<()> {
        let y_line = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(0.0, 0.0), na::Point2::new(0.0, rect.h)],
            1.5,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &y_line, (rect.point(),))?;

        let x_line = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(0.0, rect.h),
                na::Point2::new(rect.w, rect.h),
            ],
            1.5,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &x_line, (rect.point(),))
    }

    fn draw_guide_line(
        &self,
        ctx: &mut ggez::Context,
        from: na::Point2<f32>,
        to: na::Point2<f32>,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(ctx, &[from, to], 1.5, *opencolor::GRAY3)?;
        graphics::draw(ctx, &line, (self.actual_rect.point(),))
    }

    fn draw_vertical_guide(
        &self,
        ctx: &mut ggez::Context,
        text: &Text,
        x: f32,
        text_delta: f32,
    ) -> ggez::GameResult<()> {
        self.draw_guide_line(
            ctx,
            na::Point2::new(x, self.actual_rect.h),
            na::Point2::new(x, 0.0),
        )?;

        let text_width = text.width(ctx);
        text.draw(
            ctx,
            na::Point2::new(
                self.actual_rect.x + x - text_width / 2.0 + text_delta,
                self.actual_rect.y + self.actual_rect.h + 8.0,
            ),
            graphics::BLACK,
        )
    }

    fn draw_horizontal_guide(
        &self,
        ctx: &mut ggez::Context,
        text: &Text,
        y: f32,
        text_delta: f32,
    ) -> ggez::GameResult<()> {
        self.draw_guide_line(
            ctx,
            na::Point2::new(0.0, y),
            na::Point2::new(self.actual_rect.w, y),
        )?;

        let text_width = text.width(ctx);
        text.draw(
            ctx,
            na::Point2::new(
                self.actual_rect.x - text_width - 10.0,
                self.actual_rect.y + y - 7.0 + text_delta,
            ),
            graphics::BLACK,
        )
    }

    pub fn draw_plane<F1: Fn(f32) -> String, F2: Fn(f32) -> String>(
        &self,
        ctx: &mut ggez::Context,
        x_format: F1,
        y_format: F2,
    ) -> ggez::GameResult<()> {
        let title_point = na::Point2::new(
            self.rect.x + (self.rect.w - self.title_text.width(ctx)) / 2.0,
            self.rect.y + 20.0,
        );
        self.title_text.draw(ctx, title_point, graphics::BLACK)?;

        // TODO: Generalize this to avoid repeating the similar codes?
        let x_ticks = self.x_axis.ticks();
        let x_len = x_ticks.len();
        for (i, n) in x_ticks.into_iter().enumerate() {
            let x =
                (n - self.x_axis.min) / (self.x_axis.max - self.x_axis.min) * self.actual_rect.w;
            let guide_text = Text::new(&x_format(n), self.font, 28.0);

            let delta = match i {
                0 => guide_text.width(ctx) / 2.0,
                _ if i == x_len - 1 => -guide_text.width(ctx) / 2.0,
                _ => 0.0,
            };
            self.draw_vertical_guide(ctx, &guide_text, x, delta)?;
        }

        let y_ticks = self.y_axis.ticks();
        let y_len = y_ticks.len();
        for (i, n) in y_ticks.into_iter().enumerate() {
            let y = self.actual_rect.h
                - (n - self.y_axis.min) / (self.y_axis.max - self.y_axis.min) * self.actual_rect.h;
            let guide_text = Text::new(&y_format(n), self.font, 28.0);

            let delta = match i {
                0 => -guide_text.height(ctx) / 2.0,
                _ if i == y_len - 1 => guide_text.height(ctx) / 2.0,
                _ => 0.0,
            };
            self.draw_horizontal_guide(ctx, &guide_text, y, delta)?;
        }

        self.draw_axes(ctx, &self.actual_rect)?;

        Ok(())
    }

    pub fn start_plotting(&mut self) {
        self.mesh_builder = graphics::MeshBuilder::new();
    }

    pub fn finish_plotting(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if let Ok(mesh) = self.mesh_builder.build(ctx) {
            graphics::draw(ctx, &mesh, (self.actual_rect.point(),))?;
        }

        Ok(())
    }

    fn convert_point(&self, point: &na::Point2<f32>) -> na::Point2<f32> {
        na::Point2::new(
            self.x_axis.value_proportion(point.x) * self.actual_rect.w,
            (1.0 - self.y_axis.value_proportion(point.y)) * self.actual_rect.h,
        )
    }

    pub fn draw_line(
        &mut self,
        points: &[na::Point2<f32>],
        color: graphics::Color,
    ) -> ggez::GameResult<()> {
        if points.len() < 2 {
            return Ok(());
        }

        let mut converted_points: Vec<na::Point2<f32>> = Vec::new();

        for point in points {
            if point.x < self.x_axis.min
                || point.x > self.x_axis.max
                || point.y < self.y_axis.min
                || point.y > self.y_axis.max
            {
                continue;
            }

            converted_points.push(self.convert_point(point));
        }

        if converted_points.len() > 1 {
            self.mesh_builder.line(&converted_points, 3.0, color)?;
        }
        Ok(())
        //graphics::draw(ctx, &line, (self.actual_rect.point(),))
    }

    pub fn draw_point(&mut self, point: &na::Point2<f32>, radius: f32, color: graphics::Color) {
        let converted_point = self.convert_point(&point);
        self.mesh_builder.circle(
            graphics::DrawMode::fill(),
            converted_point,
            radius,
            0.1,
            color,
        );
    }
}
