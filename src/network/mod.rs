use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::{edge_data::EdgeData, innovation_record::InnovationRecord};

pub mod feedforward;
mod network_graph;

pub trait Network {
    fn activate(&mut self, inputs: Vec<f64>) -> Option<Vec<f64>>;

    fn randomize_weights(&mut self, low: f64, high: f64);

    fn mutate_add_node(&mut self, index: EdgeIndex, innov_record: &mut InnovationRecord) -> bool;
    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        edge_data: EdgeData,
    ) -> bool;
    fn mutate_assign_weight(&mut self, index: EdgeIndex, weight: f64) -> bool;
    fn mutate_perturb_weight(&mut self, index: EdgeIndex, delta: f64) -> bool;
}
