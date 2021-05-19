mod helper;

use std::collections::HashMap;

use neat::network::network_graph::NetworkGraph;
use neat::{
    edge_data::EdgeData, innovation_record::InnovationRecord, node_data::NodeData,
    node_kind::NodeKind,
};

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use rand::Rng;

fn calculate_y(total_count: usize, nth: usize, rect: &graphics::Rect) -> f32 {
    let delta = 40.0;
    let height = delta * (total_count - 1) as f32;
    (rect.h as f32 - height) / 2.0 + delta * nth as f32
}

#[derive(Debug, Clone)]
struct NodeDrawInfo {
    pos: na::Point2<f32>,
    color: graphics::Color,
}

impl NodeDrawInfo {
    fn new(node_data: &NodeData, graph: &NetworkGraph, rect: &graphics::Rect) -> NodeDrawInfo {
        let left_right_space = 60.0;
        let mut rng = rand::thread_rng();

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

                NodeDrawInfo {
                    pos: na::Point2::new(
                        rect.x as f32 + left_right_space,
                        calculate_y(total_count, nth, rect),
                    ),
                    color,
                }
            }
            NodeKind::Output => {
                let nth = node_data.id() - graph.input_number();
                let total_count = graph.output_number();

                NodeDrawInfo {
                    pos: na::Point2::new(
                        rect.x as f32 + rect.w as f32 - left_right_space,
                        calculate_y(total_count, nth, rect),
                    ),
                    color: graphics::Color::from_rgb(92, 124, 250),
                }
            }
            NodeKind::Hidden => NodeDrawInfo {
                pos: na::Point2::new(
                    rng.gen_range(
                        rect.x + left_right_space + 30.0..rect.x + rect.w - left_right_space - 30.0,
                    ),
                    rng.gen_range(rect.y + 60.0..rect.y + rect.h - 60.0),
                ),
                color: graphics::Color::from_rgb(32, 201, 151),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct EdgeDrawInfo {
    from: na::Point2<f32>,
    to: na::Point2<f32>,
    width: f32,
    color: graphics::Color,
}

impl EdgeDrawInfo {
    fn new(from: na::Point2<f32>, to: na::Point2<f32>) -> EdgeDrawInfo {
        EdgeDrawInfo {
            from,
            to,
            width: 2.0,
            color: graphics::Color::from_rgba(73, 80, 87, 150),
        }
    }
}
struct GraphVisual {
    rect: graphics::Rect,
    node_draw_info_list: Vec<NodeDrawInfo>,
    edge_draw_info_list: Vec<EdgeDrawInfo>,
}

impl GraphVisual {
    pub fn new(graph: NetworkGraph, rect: graphics::Rect) -> GraphVisual {
        let mut node_info_map = HashMap::new();
        let mut edge_draw_info_list = Vec::new();

        for node in graph.inner_data().raw_nodes() {
            let info = NodeDrawInfo::new(&node.weight, &graph, &rect);
            node_info_map.insert(node.weight.id(), info);
        }

        for edge in graph.inner_data().raw_edges() {
            if edge.weight.is_disabled() {
                continue;
            }

            let from_id = graph.inner_data()[edge.source()].id();
            let to_id = graph.inner_data()[edge.target()].id();

            let from_pos = node_info_map.get(&from_id).unwrap().pos;
            let to_pos = node_info_map.get(&to_id).unwrap().pos;

            edge_draw_info_list.push(EdgeDrawInfo::new(from_pos, to_pos));
        }

        GraphVisual {
            rect,
            node_draw_info_list: node_info_map.values().cloned().collect(),
            edge_draw_info_list,
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            graphics::Color::from_rgb(233, 236, 239),
        )?;
        graphics::draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),))?;

        for info in &self.edge_draw_info_list {
            let line =
                graphics::Mesh::new_line(ctx, &[info.from, info.to], info.width, info.color)?;

            graphics::draw(ctx, &line, (na::Point2::new(0.0, 0.0),))?;
        }

        let node_radius = 6.0;
        for info in &self.node_draw_info_list {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                [0.0, 0.0],
                node_radius,
                0.3,
                info.color,
            )?;

            graphics::draw(ctx, &circle, (info.pos,))?;
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
        let mut network = NetworkGraph::new(4, 3, &mut innov_record);
        network.add_node(0.into(), &mut innov_record);
        network.add_node(1.into(), &mut innov_record);
        network.add_node(2.into(), &mut innov_record);
        network.add_node(3.into(), &mut innov_record);

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
