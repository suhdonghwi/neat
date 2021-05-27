#![recursion_limit = "512"]
mod helper;

use std::{path::Path, time::Duration};

use ggez::event;
use ggez::graphics;
use ggez::mint;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::{main_layout::MainLayout, opencolor, plot::Axis};

struct Bird {
    pos: mint::Point2<f32>,
    y_velocity: f32,
    y_accel: f32,
}

impl Bird {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let rect = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            3.0,
            0.3,
            *opencolor::YELLOW3,
        )?;

        graphics::draw(ctx, &rect, (mint::Point2 { x: 0.0, y: 0.0 },))
    }
}

struct MainState {
    layout: MainLayout,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    timer: Duration,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/flappy.toml");

        let mut innov_record = InnovationRecord::new(3, 2);
        let pool = Pool::<Feedforward>::new(params.clone(), args.verbosity, &mut innov_record);

        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        let layout = MainLayout::new(
            params.mutation.weight_max,
            "fitness-generation graph",
            Axis::new(1.0, 10.0, 2.0),
            Axis::new(0.0, 4.0, 1.0),
            font,
        );

        MainState {
            innov_record,
            pool,
            timer: Duration::new(1, 0),
            layout,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.timer += ggez::timer::delta(ctx);

        if self.timer >= Duration::from_secs_f64(0.2) {
            let data = vec![
                (vec![0.0, 0.0], 0.0),
                (vec![0.0, 1.0], 1.0),
                (vec![1.0, 0.0], 1.0),
                (vec![1.0, 1.0], 0.0),
            ];

            let generation = self.pool.generation();
            let mut best_network = self
                .pool
                .evaluate(|_, network| {
                    let mut err = 0.0;

                    for (inputs, expected) in &data {
                        let output = network.activate(inputs).unwrap()[0];
                        err += (output - expected) * (output - expected);
                    }

                    network.evaluate(4.0 - err);
                })
                .clone();
            let best_fitness = best_network.fitness().unwrap();

            self.layout
                .update(best_network.graph_mut(), best_fitness, generation);
            self.pool.evolve(&mut self.innov_record);
            self.timer = Duration::new(0, 0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.layout.draw(ctx)?;

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = MainLayout::builder("Flappy Bird");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx);

    event::run(ctx, event_loop, state)
}
