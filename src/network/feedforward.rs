use petgraph::graph::{EdgeIndex, NodeIndex};

use super::{network_graph::NetworkGraph, Network};
use crate::{
    edge_data::EdgeData,
    node_data::{NodeData, NodeKind},
};

struct Feedforward {
    graph: NetworkGraph,
}

impl Network for Feedforward {
    fn activate(&mut self, inputs: Vec<f64>) -> Option<Vec<f64>> {
        let input_nodes: Vec<&mut NodeData> = self.graph.input_nodes_mut().collect();
        if input_nodes.len() != inputs.len() {
            return None;
        }

        // Set input to input nodes
        for (node_data, input) in input_nodes.into_iter().zip(inputs.into_iter()) {
            node_data.add_input(input);
        }

        // Activate nodes in topological order
        let sorted = self.graph.toposort();
        for index in sorted? {
            self.activate_node(index);
        }

        Some(self.graph.activate_output())
    }

    fn mutate_add_node(&mut self, index: EdgeIndex) -> bool {
        self.graph.add_node(index);
        true
    }

    fn mutate_add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        edge_data: EdgeData,
    ) -> bool {
        if self.graph.node(source).kind() == NodeKind::Output
            || self.graph.node(target).kind() == NodeKind::Input
            || self.graph.has_connection(source, target)
        {
            return false;
        }

        let new_edge_index = self.graph.add_connection(source, target, edge_data);
        if self.graph.has_cycle() {
            self.graph.remove_connetion(new_edge_index);
            return false;
        }

        true
    }

    fn mutate_weight(&mut self, index: EdgeIndex, weight: f64) -> bool {
        let edge = self.graph.edge_mut(index);
        edge.weight = weight;
        true
    }
}

impl Feedforward {
    fn new(input_number: usize, output_number: usize) -> Feedforward {
        Self {
            graph: NetworkGraph::new(input_number, output_number),
        }
    }

    fn activate_node(&mut self, index: NodeIndex) {
        let activation = self.graph.node(index).activate();
        let targets = self.graph.outgoing(index);

        for (edge_index, target_index) in targets {
            let weight: f64;

            {
                let edge = self.graph.edge(edge_index);
                if edge.disabled {
                    continue;
                };
                weight = edge.weight;
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
        let mut network = Feedforward::new(2, 1);
        assert_eq!(
            network.activate(vec![1.0, 2.0]),
            Some(vec![sigmoid(1.0 + 2.0)])
        );
    }

    #[test]
    fn disabled_connection_should_not_propagate() {
        let mut network = Feedforward::new(2, 1);
        assert!(network.mutate_add_node(0.into()));

        assert_eq!(
            network.activate(vec![1.0, 2.0]),
            Some(vec![sigmoid(sigmoid(1.0) + 2.0)])
        );
    }

    #[test]
    fn mutate_add_connection_should_connect_two_nodes() {
        let mut network = Feedforward::new(2, 1);
        assert!(network.mutate_add_node(0.into()));
        assert!(network.mutate_add_connection(
            1.into(),
            3.into(),
            EdgeData {
                weight: 2.0,
                disabled: false,
            },
        ));

        assert_eq!(
            network.activate(vec![1.0, 2.0]),
            Some(vec![sigmoid(sigmoid(5.0) + 2.0)])
        );
    }

    #[test]
    fn mutate_add_connection_should_be_unique() {
        let mut network = Feedforward::new(2, 1);
        assert_eq!(
            network.mutate_add_connection(
                0.into(),
                2.into(),
                EdgeData {
                    weight: 1.0,
                    disabled: false,
                },
            ),
            false
        );
    }

    #[test]
    fn mutate_add_connection_should_not_form_cycle() {
        let mut network = Feedforward::new(2, 1);
        assert!(network.mutate_add_node(0.into()));
        assert!(network.mutate_add_node(1.into()));
        assert!(network.mutate_add_connection(
            3.into(),
            4.into(),
            EdgeData {
                weight: 1.0,
                disabled: false
            }
        ));
        assert_eq!(
            network.mutate_add_connection(
                4.into(),
                3.into(),
                EdgeData {
                    weight: 1.0,
                    disabled: false
                }
            ),
            false
        );
    }

    #[test]
    fn mutate_add_connection_should_not_start_from_output_nor_end_to_input() {
        let mut network = Feedforward::new(2, 1);
        assert!(network.mutate_add_node(0.into()));
        assert_eq!(
            network.mutate_add_connection(
                2.into(),
                3.into(),
                EdgeData {
                    weight: 1.0,
                    disabled: false
                }
            ),
            false
        );
        assert_eq!(
            network.mutate_add_connection(
                3.into(),
                0.into(),
                EdgeData {
                    weight: 1.0,
                    disabled: false
                }
            ),
            false
        );
    }
}
