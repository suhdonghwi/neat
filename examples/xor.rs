mod helper;

use std::time::Duration;

use ggez::event;
use ggez::graphics;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::graph_visual::GraphVisual;

struct MainState {
    graph_visual: Option<GraphVisual>,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    timer: Duration,
}

impl MainState {
    fn new() -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/xor.toml");

        let mut innov_record = InnovationRecord::new(2, 1);
        let pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

        MainState {
            graph_visual: None,
            innov_record,
            pool,
            timer: Duration::new(0, 0),
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

            self.pool.evaluate(|_, network| {
                let mut err = 0.0;

                for (inputs, expected) in &data {
                    let output = network.activate(inputs).unwrap()[0];
                    err += (output - expected) * (output - expected);
                }

                network.evaluate(4.0 - err);
            });

            let best_network = self.pool.evolve(&mut self.innov_record);
            self.graph_visual = Some(GraphVisual::new(
                best_network.graph().clone(),
                [600.0, 0.0, 350.0, 350.0].into(),
            ));

            self.timer = Duration::new(0, 0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(248, 249, 250));

        if let Some(graph) = &self.graph_visual {
            graph.draw(ctx)?;
        }

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("neat", "suhdonghwi")
        .window_mode(ggez::conf::WindowMode::default().dimensions(950.0, 650.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}
