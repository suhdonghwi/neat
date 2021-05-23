#![recursion_limit = "512"]
mod helper;

use std::{path::Path, time::Duration};

use ggez::event;
use ggez::graphics;
use ggez::mint;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::{
    main_layout::MainLayout,
    opencolor,
    plot::{Axis, Plot},
};

struct MainState {
    layout: MainLayout,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    timer: Duration,

    xor_plot: Plot,
    xor_points: Vec<(mint::Point2<f32>, graphics::Color)>,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/xor.toml");

        let mut innov_record = InnovationRecord::new(2, 1);
        let pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        let layout = MainLayout::new(
            params.mutation.weight_max,
            "fitness-generation graph",
            Axis::new(1.0, 10.0, 2.0),
            Axis::new(0.0, 4.0, 1.0),
            font,
        );

        let xor_plot = Plot::new(
            graphics::Rect::new(0.0, 0.0, 400.0, 400.0),
            Axis::new(0.0, 1.0, 0.2),
            Axis::new(0.0, 1.0, 0.2),
            "XOR",
            font,
        );
        let mut xor_points = Vec::new();

        for i in 0..=20 {
            for j in 0..=20 {
                let x = 1.0 * i as f32 / 20.0;
                let y = 1.0 * j as f32 / 20.0;

                xor_points.push((mint::Point2 { x, y }, *opencolor::RED5));
            }
        }

        MainState {
            innov_record,
            pool,
            timer: Duration::new(1, 0),
            layout,
            xor_plot,
            xor_points,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.timer += ggez::timer::delta(ctx);

        if self.timer >= Duration::from_secs_f64(0.3) {
            let data = vec![
                (vec![0.0, 0.0], 0.0),
                (vec![0.0, 1.0], 1.0),
                (vec![1.0, 0.0], 1.0),
                (vec![1.0, 1.0], 0.0),
            ];

            let generation = self.pool.generation();
            let best_network = self.pool.evaluate(|_, network| {
                let mut err = 0.0;

                for (inputs, expected) in &data {
                    let output = network.activate(inputs).unwrap()[0];
                    err += (output - expected) * (output - expected);
                }

                network.evaluate(4.0 - err);
            });
            let best_fitness = best_network.fitness().unwrap();

            self.layout
                .update(best_network.graph().clone(), best_fitness, generation);
            self.pool.evolve(&mut self.innov_record);
            self.timer = Duration::new(0, 0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.layout.draw(ctx)?;

        self.xor_plot
            .draw_plane(ctx, |x| format!("{:.1}", x), |y| format!("{:.1}", y))?;

        self.xor_plot.start_plotting();
        for (point, color) in &self.xor_points {
            self.xor_plot.draw_point(point, 3.0, *color)?;
        }
        self.xor_plot.finish_plotting(ctx)?;

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = MainLayout::builder("XOR");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx);

    event::run(ctx, event_loop, state)
}
