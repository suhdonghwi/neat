use crate::node_data::NodeData;

use super::{network_graph::NetworkGraph, Network};

struct Feedforward {
    graph: NetworkGraph,
}

impl Network for Feedforward {
    fn activate(&mut self, inputs: Vec<f64>) -> Option<Vec<f64>> {
        let input_nodes: Vec<&mut NodeData> = self.graph.input_nodes_mut().collect();
        if input_nodes.len() != inputs.len() {
            return None;
        }

        for (node_data, input) in input_nodes.into_iter().zip(inputs.into_iter()) {
            node_data.add_input(input);
        }

        let sorted = self.graph.toposort()?;
        for index in sorted {
            self.graph.activate_node(index);
        }

        Some(self.graph.get_output())
    }
}
