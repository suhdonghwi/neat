mod helper;

use std::{path::Path, time::Duration};

use ggez::graphics;
use ggez::{conf::WindowSetup, event};

use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};
use neat::{network::Network, parameters::Parameters};

use helper::{fitness_plot::FitnessPlot, graph_visual::GraphVisual};

struct MainState {
    graph_visual: Option<GraphVisual>,
    fitness_plot: FitnessPlot,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    timer: Duration,
    params: Parameters,
}

impl MainState {
    fn new() -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/xor.toml");

        let mut innov_record = InnovationRecord::new(2, 1);
        let pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

        MainState {
            graph_visual: None,
            fitness_plot: FitnessPlot::new([600.0, 350.0, 350.0, 300.0].into(), 4.0, 1.0, 1.0),
            innov_record,
            pool,
            timer: Duration::new(1, 0),
            params,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.timer += ggez::timer::delta(ctx);

        if self.timer >= Duration::from_secs_f64(0.5) {
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

            self.fitness_plot.add_data(best_fitness);
            self.graph_visual = Some(GraphVisual::new(
                ctx,
                best_network.graph().clone(),
                [600.0, 0.0, 350.0, 350.0].into(),
                self.params.mutation.weight_max.abs(),
                generation,
                best_fitness,
            ));

            self.pool.evolve(&mut self.innov_record);
            self.timer = Duration::new(0, 0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(248, 249, 250));

        if let Some(graph) = &self.graph_visual {
            graph.draw(ctx)?;
        }

        self.fitness_plot.draw(ctx)?;

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("neat", "suhdonghwi")
        .window_mode(ggez::conf::WindowMode::default().dimensions(950.0, 650.0))
        .window_setup(WindowSetup::default().title("XOR"))
        .add_resource_path(Path::new("./resources"));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}
