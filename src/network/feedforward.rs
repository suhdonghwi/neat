use petgraph::graph::NodeIndex;

use super::{network_graph::NetworkGraph, Network};
use crate::{innovation_record::InnovationRecord, node_data::NodeData, node_kind::NodeKind};

#[derive(Debug, Clone)]
pub struct Feedforward {
    graph: NetworkGraph,
    fitness: Option<f64>,
}

impl Network for Feedforward {
    fn new(input_number: usize, output_number: usize, innov_record: &mut InnovationRecord) -> Self {
        Self {
            graph: NetworkGraph::new(input_number, output_number, innov_record),
            fitness: None,
        }
    }

    fn from_graph(graph: NetworkGraph) -> Self {
        Self {
            graph,
            fitness: None,
        }
    }

    fn activate(&mut self, inputs: &[f64]) -> Option<Vec<f64>> {
        let input_nodes: Vec<&mut NodeData> = self.graph.input_nodes_mut().collect();
        if input_nodes.len() != inputs.len() {
            return None;
        }

        // Set input to input nodes
        for (node_data, &input) in input_nodes.into_iter().zip(inputs.iter()) {
            node_data.add_input(input);
        }

        let bias_node = self.graph.bias_node_mut();
        bias_node.add_input(1.0);

        // Activate nodes in topological order
        // TODO: Cache toposort result?
        self.graph.activate_topo();

        let result = Some(self.graph.activate_output());
        self.graph.clear_sum();

        result
    }

    fn graph(&self) -> &NetworkGraph {
        &self.graph
    }

    fn graph_mut(&mut self) -> &mut NetworkGraph {
        &mut self.graph
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

    fn evaluate(&mut self, fitness: f64) {
        self.fitness = Some(fitness);
    }

    fn fitness(&self) -> Option<f64> {
        self.fitness
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
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert_eq!(
            network.activate(&[1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
    }

    #[test]
    fn disconnected_graph_should_result_zero() {
        let input_number = 1;
        let output_number = 1;

        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut graph = NetworkGraph::new(input_number, output_number, &mut innov_record);
        graph.add_node(0.into(), &mut innov_record);
        graph.remove_connetion(1.into());

        let mut network = Feedforward::from_graph(graph);

        assert_eq!(network.activate(&[0.0]), Some(vec![0.0]));
        assert_eq!(network.activate(&[1.0]), Some(vec![0.0]));
    }

    #[test]
    fn disabled_connection_should_not_propagate() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        assert!(network.mutate_add_node(0.into(), &mut innov_record));

        assert_eq!(
            network.activate(&[1.0, 2.0]),
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
            network.activate(&[1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
        assert_eq!(
            network.activate(&[1.0, 2.0]),
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

        assert_eq!(network.activate(&[1.0, 2.0]), Some(vec![sigmoid(0.0)]));
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
            network.activate(&[1.0, 2.0]),
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
