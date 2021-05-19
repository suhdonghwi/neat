use ggez::graphics;
use ggez::nalgebra as na;

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

    fn draw_vertical(
        &self,
        ctx: &mut ggez::Context,
        gen: usize,
        x: f32,
        actual_height: f32,
        offset_pos: na::Point2<f32>,
    ) -> ggez::GameResult<()> {
        let line = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(x, actual_height), na::Point2::new(x, 0.0)],
            1.5,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;

        graphics::draw(ctx, &line, (offset_pos,))?;

        let text = Text::new(ctx, &format!("{}", gen + 1), 28.0);
        let width = text.width(ctx) as f32;
        text.draw(
            ctx,
            na::Point2::new(
                offset_pos.x + x - width / 4.0,
                offset_pos.y + actual_height + 4.0,
            ),
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

        let actual_width = self.rect.w - left_padding - right_padding;
        let actual_height = self.rect.h - top_padding - bottom_padding;
        let offset_pos = na::Point2::new(self.rect.x + left_padding, self.rect.y + top_padding);

        let max_points = 40;
        let to_show: Vec<f64> = self
            .fitness_list
            .iter()
            .rev()
            .take(max_points)
            .rev()
            .cloned()
            .collect();

        let delta = if to_show.len() <= 1 {
            0.0
        } else {
            actual_width / (to_show.len() - 1) as f32
        };

        let max = 4.0;
        let min = 2.0;
        let current_gen = self.fitness_list.len();

        let gen_delta = (to_show.len() as f64 / 4.0).ceil() as usize;
        let y_delta = 0.5;

        let gen_start = if self.fitness_list.len() < max_points {
            0
        } else {
            self.fitness_list.len() - max_points
        };
        let mut gen = gen_start;
        while gen < current_gen {
            let x = (gen - gen_start) as f32 / to_show.len() as f32 * actual_width;
            self.draw_vertical(ctx, gen, x, actual_height, offset_pos)?;
            gen += gen_delta;
        }
        self.draw_vertical(ctx, current_gen, actual_width, actual_height, offset_pos)?;

        let points: Vec<na::Point2<f32>> = to_show
            .iter()
            .enumerate()
            .map(|(i, &y)| {
                na::Point2::new(
                    i as f32 * delta,
                    actual_height - (actual_height * (y as f32 - min) / (max - min)) as f32,
                )
            })
            .collect();

        let line = graphics::Mesh::new_line(ctx, &points, 2.0, graphics::BLACK)?;
        graphics::draw(ctx, &line, (offset_pos,))?;

        Ok(())
    }
}
