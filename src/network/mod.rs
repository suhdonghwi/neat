use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::innovation_record::InnovationRecord;

pub mod feedforward;
mod network_graph;

pub trait Network {
    fn new(input_number: usize, output_number: usize, innov_record: &mut InnovationRecord) -> Self;
    fn activate(&mut self, inputs: &Vec<f64>) -> Option<Vec<f64>>;

    fn randomize_weights(&mut self, low: f64, high: f64);

    fn mutate_add_node(&mut self, index: EdgeIndex, innov_record: &mut InnovationRecord) -> bool;
    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: f64,
        innov_record: &mut InnovationRecord,
    ) -> bool;
    fn mutate_assign_weight(&mut self, index: EdgeIndex, weight: f64) -> bool;
    fn mutate_perturb_weight(&mut self, index: EdgeIndex, delta: f64) -> bool;

    fn evaluate(&mut self, fitness: f64);
    fn fitness(&self) -> Option<f64>;
}
