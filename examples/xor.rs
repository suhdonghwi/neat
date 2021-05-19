mod helper;

use ggez::event;
use ggez::graphics;

use neat::innovation_record::InnovationRecord;
use neat::network::network_graph::NetworkGraph;

use helper::graph_visual::GraphVisual;

struct MainState {
    graph_visual: GraphVisual,
    innov_record: InnovationRecord,
}

impl MainState {
    fn new() -> Self {
        let mut innov_record = InnovationRecord::new(2, 1);
        let network = NetworkGraph::new(2, 1, &mut innov_record);

        MainState {
            graph_visual: GraphVisual::new(network, [600.0, 0.0, 350.0, 350.0].into()),
            innov_record,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(248, 249, 250));

        self.graph_visual.draw(ctx)?;
        /*
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        */

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("neat", "suhdonghwi")
        .window_mode(ggez::conf::WindowMode::default().dimensions(950.0, 650.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new();
    event::run(ctx, event_loop, state)
}

/*
use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

fn main() {
    let args = helper::cli::get_arguments();
    let params = helper::read_parameters_file("./params/xor.toml");

    let mut innov_record = InnovationRecord::new(params.input_number, params.output_number);
    let mut pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for _ in 0..50 {
        pool.evaluate(|_, network| {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected) * (output - expected);
            }

            network.evaluate(4.0 - err);
        });
        pool.evolve(&mut innov_record);
    }
}
*/
