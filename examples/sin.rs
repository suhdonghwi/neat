#![recursion_limit = "512"]
mod helper;

use std::{path::Path, time::Duration};

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

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

    sin_plot: Plot,
    sin_points: Vec<na::Point2<f32>>,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/sin.toml");

        let mut innov_record = InnovationRecord::new(1, 1);
        let pool = Pool::<Feedforward>::new(params.clone(), args.verbosity, &mut innov_record);

        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        let layout = MainLayout::new(
            params.mutation.weight_max,
            Axis::new(1.0, 10.0, 2.0),
            Axis::new(3.0, 4.0, 0.2),
            font,
        );

        let sin_plot = Plot::new(
            graphics::Rect::new(60.0, 70.0, 400.0, 400.0),
            Axis::new(-1.0, 1.0, 0.5),
            Axis::new(-1.0, 1.0, 0.5),
            "SIN",
            font,
        );
        let mut sin_points = Vec::new();
        for i in -50..=50 {
            sin_points.push(na::Point2::new(i as f32 / 50.0, 0.0));
        }

        MainState {
            innov_record,
            pool,
            timer: Duration::new(1, 0),
            layout,
            sin_plot,
            sin_points,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.timer += ggez::timer::delta(ctx);

        if self.timer >= Duration::from_secs_f64(0.1) {
            let generation = self.pool.generation();
            let mut best_network = self
                .pool
                .evaluate(|_, network| {
                    let n = 50;
                    let mut error_sum = 0.0;

                    for i in -n..=n {
                        let x = i as f64 / n as f64;

                        let output = network.activate(&[x]).unwrap()[0];
                        let expected = (x * std::f64::consts::PI).sin();
                        let err = output - expected;

                        error_sum += err * err;
                    }

                    let error_mean = error_sum / (n * 2 + 1) as f64;
                    network.evaluate(4.0 - error_mean);
                })
                .clone();
            let best_fitness = best_network.fitness().unwrap();

            for point in &mut self.sin_points {
                point.y = best_network.activate(&[point.x.into()]).unwrap()[0] as f32;
            }

            self.layout
                .update(best_network.graph_mut(), best_fitness, generation);
            self.pool.evolve(&mut self.innov_record);
            self.timer = Duration::new(0, 0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, *opencolor::GRAY0);
        self.layout.draw(ctx)?;

        self.sin_plot
            .draw_plane(ctx, |x| format!("{:.1}", x), |y| format!("{:.1}", y))?;

        self.sin_plot.start_plotting();
        self.sin_plot
            .draw_line(&self.sin_points, *opencolor::RED5)?;
        self.sin_plot.finish_plotting(ctx)?;

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = MainLayout::builder("sin function approximation");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx);

    event::run(ctx, event_loop, state)
}
