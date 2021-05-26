use std::collections::HashMap;

use neat::network::network_graph::NetworkGraph;
use neat::{node_data::NodeData, node_kind::NodeKind};

use ggez::graphics::{self};
use ggez::nalgebra as na;

use super::{opencolor, text::Text};

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
    fn new(
        node_data: &NodeData,
        graph: &NetworkGraph,
        rect: &graphics::Rect,
        hidden_nth: usize,
    ) -> NodeDrawInfo {
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
                    *opencolor::GRAY7
                } else {
                    *opencolor::GRAY5
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
                    color: *opencolor::GRAY7,
                }
            }
            NodeKind::Hidden => {
                let hidden_count = graph.hidden_node_count();
                let hidden_per_line: usize = 5;
                let line_count = (hidden_count as f64 / hidden_per_line as f64).ceil() as usize;
                let nth_line = (hidden_nth / hidden_per_line) + 1;
                let delta = (rect.w - left_right_space * 2.0) / (line_count + 1) as f32;

                NodeDrawInfo {
                    pos: na::Point2::new(
                        rect.x + left_right_space + delta * nth_line as f32,
                        calculate_y(hidden_per_line, hidden_nth % hidden_per_line, rect),
                    ),
                    color: *opencolor::GRAY5,
                }
            }
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
    fn new(
        from: na::Point2<f32>,
        to: na::Point2<f32>,
        weight: f64,
        max_weight: f64,
    ) -> EdgeDrawInfo {
        EdgeDrawInfo {
            from,
            to,
            width: (7.0 * weight.abs() / max_weight) as f32,
            color: if weight > 0.0 {
                opencolor::with_alpha(*opencolor::GREEN5, 0.7)
            } else {
                opencolor::with_alpha(*opencolor::RED5, 0.7)
            },
        }
    }
}

pub struct GraphVisual {
    rect: graphics::Rect,
    node_draw_info_list: Vec<NodeDrawInfo>,
    edge_draw_info_list: Vec<EdgeDrawInfo>,
    text: Text,
}

impl GraphVisual {
    pub fn new(
        graph: &mut NetworkGraph,
        rect: graphics::Rect,
        max_weight: f64,
        generation: usize,
        fitness: f64,
        font: graphics::Font,
    ) -> GraphVisual {
        let mut node_info_map = HashMap::new();
        let mut edge_draw_info_list = Vec::new();

        let mut hidden_nth: usize = 0;
        for node_index in graph.toposort().unwrap() {
            let node_data = graph.node(node_index);
            let info = NodeDrawInfo::new(&node_data, &graph, &rect, hidden_nth);

            if node_data.kind() == NodeKind::Hidden {
                hidden_nth += 1;
            }

            node_info_map.insert(node_data.id(), info);
        }

        for edge in graph.inner_data().raw_edges() {
            if edge.weight.is_disabled() {
                continue;
            }

            let from_id = graph.inner_data()[edge.source()].id();
            let to_id = graph.inner_data()[edge.target()].id();

            let from_pos = node_info_map.get(&from_id).unwrap().pos;
            let to_pos = node_info_map.get(&to_id).unwrap().pos;

            edge_draw_info_list.push(EdgeDrawInfo::new(
                from_pos,
                to_pos,
                edge.weight.get_weight(),
                max_weight,
            ));
        }

        let text = Text::new(
            &format!(
                "#{} Best genome (fitness : {:.5})\n{} node(s), {} edge(s)",
                generation,
                fitness,
                graph.node_count(),
                graph.edge_count(),
            ),
            font,
            32.0,
        );

        GraphVisual {
            rect,
            node_draw_info_list: node_info_map.values().cloned().collect(),
            edge_draw_info_list,
            text,
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
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
                info.pos,
                node_radius,
                0.3,
                info.color,
            )?;

            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

        self.text.draw(
            ctx,
            na::Point2::new(self.rect.x + 15.0, self.rect.y + 15.0),
            graphics::BLACK,
        )
    }
}
