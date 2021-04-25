use petgraph::graph::{EdgeIndex, NodeIndex};

use super::{network_graph::NetworkGraph, Network};
use crate::{
    innovation_record::InnovationRecord,
    node_data::{NodeData, NodeKind},
};

#[derive(Debug)]
pub struct Feedforward {
    graph: NetworkGraph,
    fitness: Option<f64>,
}

impl Network for Feedforward {
    fn new(
        input_number: usize,
        output_number: usize,
        innov_record: &mut InnovationRecord,
    ) -> Feedforward {
        Self {
            graph: NetworkGraph::new(input_number, output_number, innov_record),
            fitness: None,
        }
    }

    fn activate(&mut self, inputs: &Vec<f64>) -> Option<Vec<f64>> {
        self.graph.clear_sum();

        let input_nodes: Vec<&mut NodeData> = self.graph.input_nodes_mut().collect();
        if input_nodes.len() != inputs.len() {
            return None;
        }

        // Set input to input nodes
        for (node_data, &input) in input_nodes.into_iter().zip(inputs.into_iter()) {
            node_data.add_input(input);
        }

        let bias_node = self.graph.bias_node_mut();
        bias_node.add_input(1.0);

        // Activate nodes in topological order
        let sorted = self.graph.toposort();
        for index in sorted? {
            self.activate_node(index);
        }

        Some(self.graph.activate_output())
    }

    fn graph(&self) -> &NetworkGraph {
        &self.graph
    }

    fn graph_mut(&mut self) -> &mut NetworkGraph {
        &mut self.graph
    }

    fn mutate_add_node(&mut self, index: EdgeIndex, innov_record: &mut InnovationRecord) -> bool {
        self.graph.add_node(index, innov_record);
        true
    }

    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: f64,
        innov_record: &mut InnovationRecord,
    ) -> bool {
        let source_kind = self.graph.node(source).kind();
        let target_kind = self.graph.node(target).kind();
        if source == target
            || source_kind == NodeKind::Output
            || target_kind == NodeKind::Input
            || target_kind == NodeKind::Bias
            || self.graph.has_connection(source, target)
        {
            return false;
        }

        let new_edge_index = self
            .graph
            .add_connection(source, target, weight, innov_record);
        if self.graph.has_cycle() {
            self.graph.remove_connetion(new_edge_index);
            return false;
        }

        true
    }

    fn crossover(&self, other: &Self) -> Option<Self> {
        if let (Some(my_fitness), Some(other_fitness)) = (self.fitness(), other.fitness()) {
            let rng = &mut rand::thread_rng();
            let new_graph = self
                .graph
                .crossover(&other.graph, my_fitness >= other_fitness, rng)?;

            Some(Self {
                graph: new_graph,
                fitness: None,
            })
        } else {
            None
        }
    }

    /*
    fn random_edge(&self, rng: &mut impl RngCore) -> EdgeIndex {
        let uniform = Uniform::from(0..self.graph.edge_count());
        EdgeIndex::new(uniform.sample(rng))
    }

    fn random_node(&self, rng: &mut impl RngCore) -> NodeIndex {
        let uniform = Uniform::from(0..self.graph.node_count());
        NodeIndex::new(uniform.sample(rng))
    }*/

    fn evaluate(&mut self, fitness: f64) {
        self.fitness = Some(fitness);
    }

    fn fitness(&self) -> Option<f64> {
        self.fitness
    }
}

impl Feedforward {
    fn activate_node(&mut self, index: NodeIndex) {
        let activation = self.graph.node_mut(index).activate();
        let targets = self.graph.outgoing(index);

        for (edge_index, target_index) in targets {
            let weight: f64;

            {
                let edge = self.graph.edge(edge_index);
                if edge.is_disabled() {
                    continue;
                };
                weight = edge.get_weight();
            }

            let target = self.graph.node_mut(target_index);
            target.add_input(activation * weight);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activations::sigmoid;

    #[test]
    fn initial_network_activation_should_sum_input_and_squash() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(2, 1, &mut innov_record);
        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
    }

    #[test]
    fn disabled_connection_should_not_propagate() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert!(network.mutate_add_node(0.into(), &mut innov_record));

        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(sigmoid(1.0) + 2.0)])
        );
    }

    #[test]
    fn activation_result_should_be_consistent() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(2, 1, &mut innov_record);

        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
    }

    #[test]
    fn bias_node_should_sum_weight_as_is() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert!(network.mutate_add_connection(3.into(), 2.into(), -3.0, &mut innov_record));

        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0 - 3.0)])
        );
    }

    #[test]
    fn mutate_add_connection_should_connect_nodes() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(2, 1, &mut innov_record);
        assert!(network.mutate_add_node(0.into(), &mut innov_record));
        assert!(network.mutate_add_connection(1.into(), 4.into(), 2.0, &mut innov_record));

        assert_eq!(
            network.activate(&vec![1.0, 2.0]),
            Some(vec![sigmoid(sigmoid(5.0) + 2.0)])
        );
    }

    #[test]
    fn mutate_add_connection_should_be_unique() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(2, 1, &mut innov_record);
        assert_eq!(
            network.mutate_add_connection(0.into(), 2.into(), 1.0, &mut innov_record),
            false
        );
    }

    #[test]
    fn mutate_add_connection_should_not_form_cycle() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert!(network.mutate_add_node(0.into(), &mut innov_record));
        assert!(network.mutate_add_node(1.into(), &mut innov_record));
        assert!(network.mutate_add_connection(4.into(), 5.into(), 1.0, &mut innov_record));
        assert_eq!(
            network.mutate_add_connection(5.into(), 4.into(), 1.0, &mut innov_record),
            false
        );
    }

    #[test]
    fn mutate_add_connection_should_start_or_end_at_valid_node() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert!(network.mutate_add_node(0.into(), &mut innov_record));
        assert_eq!(
            network.mutate_add_connection(2.into(), 4.into(), 1.0, &mut innov_record),
            false
        );

        assert_eq!(
            network.mutate_add_connection(4.into(), 0.into(), 1.0, &mut innov_record),
            false
        );

        assert_eq!(
            network.mutate_add_connection(4.into(), 3.into(), 1.0, &mut innov_record),
            false
        );
    }
}
