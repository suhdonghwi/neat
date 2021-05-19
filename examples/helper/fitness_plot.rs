use float_cmp::ApproxEq;
use ggez::nalgebra as na;
use ggez::{graphics, mint};

use super::text::Text;

pub struct FitnessPlot {
    rect: graphics::Rect,
    fitness_list: Vec<f64>,
}

impl FitnessPlot {
    pub fn new(rect: graphics::Rect) -> FitnessPlot {
        FitnessPlot {
            rect,
            fitness_list: Vec::new(),
        }
    }

    pub fn add_data(&mut self, v: f64) {
        self.fitness_list.push(v);
    }

    fn draw_axis(&self, ctx: &mut ggez::Context, rect: &graphics::Rect) -> ggez::GameResult<()> {
        let vertical = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(0.0, 0.0), na::Point2::new(0.0, rect.h)],
            1.5,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &vertical, (rect.point(),))?;

        let horizontal = graphics::Mesh::new_line(
            ctx,
            &[
                na::Point2::new(0.0, rect.h),
                na::Point2::new(rect.w, rect.h),
            ],
            1.5,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &horizontal, (rect.point(),))
    }

    fn draw_vertical(
        &self,
        ctx: &mut ggez::Context,
        gen: usize,
        x: f32,
        rect: &graphics::Rect,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(x, rect.h), na::Point2::new(x, 0.0)],
            1.5,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;

        graphics::draw(ctx, &line, (rect.point(),))?;

        let text = Text::new(ctx, &format!("{}", gen + 1), 28.0);
        let width = text.width(ctx) as f32;
        text.draw(
            ctx,
            na::Point2::new(rect.x + x - width / 4.0, rect.y + rect.h + 8.0),
            graphics::BLACK,
        )
    }

    fn draw_horizontal(
        &self,
        ctx: &mut ggez::Context,
        y: f32,
        fitness: f32,
        rect: &graphics::Rect,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(0.0, y), na::Point2::new(rect.w, y)],
            1.5,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;

        graphics::draw(ctx, &line, (rect.point(),))?;

        let text = Text::new(ctx, &format!("{:.1}", fitness), 28.0);
        let width = text.width(ctx);
        text.draw(
            ctx,
            na::Point2::new(rect.x - width as f32 + 7.0, rect.y + y - 7.0),
            graphics::BLACK,
        )
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.fitness_list.len() < 2 {
            return Ok(());
        }

        let top_padding = 60.0;
        let right_padding = 40.0;
        let bottom_padding = 40.0;
        let left_padding = 40.0;

        let actual_rect = graphics::Rect::new(
            self.rect.x + left_padding,
            self.rect.y + top_padding,
            self.rect.w - left_padding - right_padding,
            self.rect.h - top_padding - bottom_padding,
        );

        let max_points = 40;
        let to_show: Vec<f64> = self
            .fitness_list
            .iter()
            .rev()
            .take(max_points)
            .rev()
            .cloned()
            .collect();

        let max = 4.0;
        let min = 2.0;
        let current_gen = self.fitness_list.len();

        self.draw_axis(ctx, &actual_rect)?;

        let gen_delta = (to_show.len() as f64 / 4.0).ceil() as usize;
        let fitness_delta = 1.0;

        let gen_start = if self.fitness_list.len() < max_points {
            0
        } else {
            self.fitness_list.len() - max_points
        };
        let mut gen = gen_start;
        while gen < current_gen {
            let x = (gen - gen_start) as f32 / to_show.len() as f32 * actual_rect.w;
            self.draw_vertical(ctx, gen, x, &actual_rect)?;
            gen += gen_delta;
        }
        self.draw_vertical(ctx, current_gen, actual_rect.w, &actual_rect)?;

        let mut fitness: f32 = 0.0;
        while !fitness.approx_eq(max, (0.0, 2)) {
            self.draw_horizontal(
                ctx,
                actual_rect.h - actual_rect.h * fitness / max,
                fitness,
                &actual_rect,
            )?;
            fitness += fitness_delta;
        }
        self.draw_horizontal(ctx, 0.0, max, &actual_rect)?;

        let delta = if to_show.len() <= 1 {
            0.0
        } else {
            actual_rect.w / (to_show.len() - 1) as f32
        };

        let points: Vec<na::Point2<f32>> = to_show
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                na::Point2::new(
                    i as f32 * delta,
                    actual_rect.h - (actual_rect.h * (y as f32 - min) / (max - min)) as f32,
                )
            })
            .collect();

        let line = graphics::Mesh::new_line(ctx, &points, 2.0, graphics::BLACK)?;
        graphics::draw(ctx, &line, (actual_rect.point(),))?;

        Ok(())
    }
}
