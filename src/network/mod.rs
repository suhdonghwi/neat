use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::edge_data::EdgeData;

pub mod feedforward;
mod network_graph;

pub trait Network {
    fn activate(&mut self, inputs: Vec<f64>) -> Option<Vec<f64>>;

    fn mutate_add_node(&mut self, index: EdgeIndex) -> bool;
    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        edge_data: EdgeData,
    ) -> bool;
}
