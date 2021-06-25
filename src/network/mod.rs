use std::cmp::Ordering;

use petgraph::graph::{EdgeIndex, NodeIndex};

use self::network_graph::NetworkGraph;
use crate::node_kind::NodeKind;
use crate::{activations::ActivationKind, innovation_record::InnovationRecord};

pub mod feedforward;
pub mod network_graph;

pub trait Network {
    fn new(
        input_number: usize,
        output_number: usize,
        hidden_func: ActivationKind,
        output_func: ActivationKind,
        innov_record: &mut InnovationRecord,
    ) -> Self;
    fn from_graph(
        graph: NetworkGraph,
        hidden_func: ActivationKind,
        output_func: ActivationKind,
    ) -> Self;

    fn activate(&mut self, inputs: &[f64]) -> Option<Vec<f64>>;

    fn graph(&self) -> &NetworkGraph;
    fn graph_mut(&mut self) -> &mut NetworkGraph;

    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: f64,
        innov_record: &mut InnovationRecord,
    ) -> bool;

    fn mutate_remove_connection(&mut self, index: EdgeIndex) -> bool {
        self.graph_mut().remove_connetion(index);
        true
    }

    fn mutate_toggle_connection(&mut self, index: EdgeIndex) -> bool {
        let edge = self.graph_mut().edge_mut(index);
        edge.set_disabled(!edge.is_disabled());
        true
    }

    fn mutate_add_node(&mut self, index: EdgeIndex, innov_record: &mut InnovationRecord) -> bool {
        self.graph_mut().add_node(index, innov_record);
        true
    }

    fn mutate_remove_node(
        &mut self,
        index: NodeIndex,
        innov_record: &mut InnovationRecord,
    ) -> bool {
        let node = self.graph().node(index);
        if node.kind() == NodeKind::Hidden {
            self.graph_mut().remove_node(index, innov_record);
            true
        } else {
            false
        }
    }

    fn mutate_assign_weight(&mut self, index: EdgeIndex, weight: f64) -> bool {
        let edge = self.graph_mut().edge_mut(index);
        edge.set_weight(weight);
        true
    }

    fn mutate_perturb_weight(&mut self, index: EdgeIndex, delta: f64, min: f64, max: f64) -> bool {
        let edge = self.graph_mut().edge_mut(index);
        edge.set_weight((edge.get_weight() + delta).clamp(min, max));
        true
    }

    fn crossover(
        &self,
        other: &Self,
        hidden_func: ActivationKind,
        output_func: ActivationKind,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let (my_fitness, other_fitness) = (self.fitness()?, other.fitness()?);
        let rng = &mut rand::thread_rng();
        let new_graph = self
            .graph()
            .crossover(&other.graph(), my_fitness >= other_fitness, rng)?;

        Some(Self::from_graph(new_graph, hidden_func, output_func))
    }

    fn evaluate(&mut self, fitness: f64);
    fn fitness(&self) -> Option<f64>;

    fn compare(&self, other: &Self) -> Option<Ordering> {
        self.fitness().partial_cmp(&other.fitness())
        /*
        let result = self.fitness().partial_cmp(&other.fitness());
        if result == Some(Ordering::Equal) {
            let my_complexity = self.graph().edge_count();
            let other_complexity = other.graph().edge_count();
            other_complexity.partial_cmp(&my_complexity)
        } else {
            result
        }
        */
    }
}
