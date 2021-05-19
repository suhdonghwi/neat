mod helper;

use neat::network::network_graph::NetworkGraph;
use neat::{innovation_record::InnovationRecord, node_data::NodeData, node_kind::NodeKind};

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

fn calculate_y(total_count: usize, nth: usize, rect: &graphics::Rect) -> f32 {
    let delta = 40.0;
    let height = delta * (total_count - 1) as f32;
    (rect.h as f32 - height) / 2.0 + delta * nth as f32
}

fn node_draw_info(
    node_data: &NodeData,
    graph: &NetworkGraph,
    rect: &graphics::Rect,
) -> (na::Point2<f32>, graphics::Color) {
    let left_right_space = 60.0;

    match node_data.kind() {
        NodeKind::Input | NodeKind::Bias => {
            let nth = if node_data.kind() == NodeKind::Bias {
                0
            } else {
                node_data.id() + 1
            };

            let total_count = graph.input_number() + 1;
            let color = if node_data.kind() == NodeKind::Input {
                graphics::Color::from_rgb(255, 107, 107)
            } else {
                graphics::Color::from_rgb(252, 196, 25)
            };

            (
                na::Point2::new(
                    rect.x as f32 + left_right_space,
                    calculate_y(total_count, nth, rect),
                ),
                color,
            )
        }
        NodeKind::Output => {
            let nth = node_data.id() - graph.input_number();
            let total_count = graph.output_number();
            (
                na::Point2::new(
                    rect.x as f32 + rect.w as f32 - left_right_space,
                    calculate_y(total_count, nth, rect),
                ),
                graphics::Color::from_rgb(92, 124, 250),
            )
        }
        _ => (na::Point2::new(0.0, 0.0), graphics::WHITE),
    }
}

struct GraphVisual {
    rect: graphics::Rect,

    node_draw_points: Vec<(na::Point2<f32>, graphics::Color)>,
}

impl GraphVisual {
    pub fn new(graph: NetworkGraph, rect: graphics::Rect) -> GraphVisual {
        let mut node_draw_points = Vec::new();

        for node in graph.inner_data().raw_nodes() {
            node_draw_points.push(node_draw_info(&node.weight, &graph, &rect));
        }

        GraphVisual {
            rect,
            node_draw_points,
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            graphics::Color::from_rgb(233, 236, 239),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        let node_radius = 6.0;
        for (point, color) in &self.node_draw_points {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [0.0, 0.0],
                node_radius,
                0.5,
                *color,
            )?;

            graphics::draw(ctx, &circle, (*point,))?;
        }

        Ok(())
    }
}

struct MainState {
    graph_visual: GraphVisual,
    innov_record: InnovationRecord,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let mut innov_record = InnovationRecord::new(4, 3);
        let network = NetworkGraph::new(4, 3, &mut innov_record);

        Ok(MainState {
            graph_visual: GraphVisual::new(network, [600.0, 0.0, 350.0, 350.0].into()),
            innov_record,
        })
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
    let state = &mut MainState::new()?;
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
