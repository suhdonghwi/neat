use petgraph::graph::{DiGraph, NodeIndex};

use crate::edge::Edge;
use crate::node::{Node, NodeKind};

struct Network {
  graph: DiGraph<Node, Edge>,
  input_number: usize,
  output_number: usize,
}

impl Network {
  pub fn new(input_number: usize, output_number: usize) -> Self {
    let mut graph = DiGraph::new();

    for _ in 0..input_number {
      graph.add_node(Node {
        kind: NodeKind::Input,
      });
    }

    for _ in 0..output_number {
      graph.add_node(Node {
        kind: NodeKind::Output,
      });
    }

    Self {
      graph,
      input_number,
      output_number,
    }
  }

  pub fn input_nodes(&self) -> Vec<&Node> {
    (0..self.input_number)
      .map(|i| &self.graph[NodeIndex::new(i)])
      .collect()
  }

  pub fn output_nodes(&self) -> Vec<&Node> {
    (self.input_number..self.input_number + self.output_number)
      .map(|i| &self.graph[NodeIndex::new(i)])
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn network_initialization() {
    let network = Network::new(2, 2);

    assert!(
      network.input_nodes()
        == vec![
          &Node {
            kind: NodeKind::Input
          };
          2
        ]
    );
    assert!(
      network.output_nodes()
        == vec![
          &Node {
            kind: NodeKind::Output
          };
          2
        ]
    );
  }
}
