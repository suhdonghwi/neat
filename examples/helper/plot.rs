use ggez::graphics;
use ggez::mint;
use ggez::nalgebra as na;

use super::text::Text;

pub struct Axis {
    min: f32,
    max: f32,
    delta: f32,
}

impl Axis {
    pub fn new(min: f32, max: f32, delta: f32) -> Self {
        Axis { min, max, delta }
    }

    pub fn ticks(&self) -> impl Iterator<Item = f32> {
        let mut list = Vec::new();

        let mut n = self.min;
        while n < self.max {
            list.push(n);
            n += self.delta;
        }

        list.push(self.max);
        list.into_iter()
    }

    pub fn value_proportion(&self, v: f32) -> f32 {
        (v - self.min) / (self.max - self.min)
    }

    pub fn set_range(&mut self, min: f32, max: f32, tick_count: usize) {
        let delta = ((max - min) / tick_count as f32).ceil();

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

    fn draw_vertical_guide(
        &self,
        ctx: &mut ggez::Context,
        text_str: &str,
        x: f32,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(x, self.actual_rect.h),
                na::Point2::new(x, 0.0),
            ],
            1.5,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;

        graphics::draw(ctx, &line, (self.actual_rect.point(),))?;

        let text = Text::new(text_str, self.font, 28.0);
        let text_width = text.width(ctx);
        text.draw(
            ctx,
            na::Point2::new(
                self.actual_rect.x + x - text_width / 2.0,
                self.actual_rect.y + self.actual_rect.h + 8.0,
            ),
            graphics::BLACK,
        )
    }

    fn draw_horizontal_guide(
        &self,
        ctx: &mut ggez::Context,
        text_str: &str,
        y: f32,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(0.0, y),
                na::Point2::new(self.actual_rect.w, y),
            ],
            1.5,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;

        graphics::draw(ctx, &line, (self.actual_rect.point(),))?;

        let text = Text::new(text_str, self.font, 28.0);
        let text_width = text.width(ctx);
        text.draw(
            ctx,
            na::Point2::new(
                self.actual_rect.x - text_width - 10.0,
                self.actual_rect.y + y - 7.0,
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
        self.draw_axes(ctx, &self.actual_rect)?;

        let title_point = na::Point2::new(
            self.rect.x + (self.rect.w - self.title_text.width(ctx)) / 2.0,
            self.rect.y + 20.0,
        );
        self.title_text.draw(ctx, title_point, graphics::BLACK)?;

        for n in self.x_axis.ticks() {
            let x =
                (n - self.x_axis.min) / (self.x_axis.max - self.x_axis.min) * self.actual_rect.w;
            let guide_text = &x_format(n);
            self.draw_vertical_guide(ctx, guide_text, x)?;
        }

        for n in self.y_axis.ticks() {
            let y = self.actual_rect.h
                - (n - self.y_axis.min) / (self.y_axis.max - self.y_axis.min) * self.actual_rect.h;
            let guide_text = &y_format(n);
            self.draw_horizontal_guide(ctx, guide_text, y)?;
        }

        Ok(())
    }

    pub fn draw_line(
        &self,
        ctx: &mut ggez::Context,
        points: &[mint::Point2<f32>],
        color: graphics::Color,
    ) -> ggez::GameResult<()> {
        if points.len() < 2 {
            return Ok(());
        }

        let mut converted_points: Vec<mint::Point2<f32>> = Vec::new();

        for point in points {
            if point.x < self.x_axis.min
                || point.x > self.x_axis.max
                || point.y < self.y_axis.min
                || point.y > self.y_axis.max
            {
                continue;
            }

            converted_points.push(mint::Point2 {
                x: self.x_axis.value_proportion(point.x) * self.actual_rect.w,
                y: (1.0 - self.y_axis.value_proportion(point.y)) * self.actual_rect.h,
            });
        }

        let line = graphics::Mesh::new_line(ctx, &converted_points, 3.0, color)?;
        graphics::draw(ctx, &line, (self.actual_rect.point(),))
    }

    /*
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let top_padding = 60.0;
        let right_padding = 30.0;
        let bottom_padding = 50.0;
        let left_padding = 60.0;

        let actual_rect = graphics::Rect::new(
            self.rect.x + left_padding,
            self.rect.y + top_padding,
            self.rect.w - left_padding - right_padding,
            self.rect.h - top_padding - bottom_padding,
        );

        let max_points = 50;
        let to_show: Vec<f64> = self
            .fitness_list
            .iter()
            .rev()
            .take(max_points)
            .rev()
            .cloned()
            .collect();

        self.draw_axes(ctx, &actual_rect)?;

        let text_pos = na::Point2::new(
            self.rect.x + (self.rect.w - self.text.width(ctx)) / 2.0,
            self.rect.y + 20.0,
        );
        self.text.draw(ctx, text_pos, graphics::BLACK)?;

        let current_gen = self.fitness_list.len();
        let gen_delta = (to_show.len() as f64 / 5.0).ceil() as usize;
        let gen_start = if self.fitness_list.len() <= max_points {
            1
        } else {
            self.fitness_list.len() - max_points
        };
        let mut gen = gen_start;
        while gen < current_gen {
            let x = (gen - gen_start) as f32 / to_show.len() as f32 * actual_rect.w;
            self.draw_vertical_guide(ctx, gen, x, &actual_rect)?;
            gen += gen_delta;
        }
        self.draw_vertical_guide(ctx, current_gen, actual_rect.w, &actual_rect)?;

        let mut fitness: f32 = 0.0;
        while fitness < self.fitness_max - self.fitness_delta / 2.0 {
            self.draw_horizontal_guide(
                ctx,
                actual_rect.h - actual_rect.h * fitness / self.fitness_max,
                fitness,
                &actual_rect,
            )?;
            fitness += self.fitness_delta;
        }
        self.draw_horizontal_guide(ctx, 0.0, self.fitness_max, &actual_rect)?;

        let delta = if to_show.len() <= 1 {
            0.0
        } else {
            actual_rect.w / (to_show.len() - 1) as f32
        };

        if self.fitness_list.len() < 2 {
            return Ok(());
        }

        let points: Vec<na::Point2<f32>> = to_show
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                let y = actual_rect.h
                    - (actual_rect.h * (y as f32 - self.fitness_min)
                        / (self.fitness_max - self.fitness_min)) as f32;
                na::Point2::new(i as f32 * delta, y)
            })
            .collect();

        let line =
            graphics::Mesh::new_line(ctx, &points, 3.0, graphics::Color::from_rgb(92, 124, 250))?;
        graphics::draw(ctx, &line, (actual_rect.point(),))?;

        Ok(())
    }
    */
}
