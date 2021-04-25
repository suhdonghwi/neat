use petgraph::graph::{EdgeIndex, NodeIndex};

use self::network_graph::NetworkGraph;
use crate::innovation_record::InnovationRecord;

pub mod feedforward;
mod network_graph;

pub trait Network {
    fn new(input_number: usize, output_number: usize, innov_record: &mut InnovationRecord) -> Self;
    fn activate(&mut self, inputs: &Vec<f64>) -> Option<Vec<f64>>;

    fn graph(&self) -> &NetworkGraph;
    fn graph_mut(&mut self) -> &mut NetworkGraph;

    fn mutate_add_node(&mut self, index: EdgeIndex, innov_record: &mut InnovationRecord) -> bool;
    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: f64,
        innov_record: &mut InnovationRecord,
    ) -> bool;

    fn mutate_assign_weight(&mut self, index: EdgeIndex, weight: f64) -> bool {
        let edge = self.graph_mut().edge_mut(index);
        edge.set_weight(weight);
        true
    }

    fn mutate_perturb_weight(&mut self, index: EdgeIndex, delta: f64) -> bool {
        let edge = self.graph_mut().edge_mut(index);
        edge.set_weight(edge.get_weight() + delta);
        true
    }

    fn crossover(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    fn evaluate(&mut self, fitness: f64);
    fn fitness(&self) -> Option<f64>;
}
