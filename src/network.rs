use petgraph::graph::{DiGraph, NodeIndex};
use rand::distributions::{Distribution, Uniform}; // 0.6.5

use crate::edge_data::EdgeData;
use crate::node_data::{NodeData, NodeKind};

#[derive(Debug)]
pub struct Network {
    graph: DiGraph<NodeData, EdgeData>,
    input_number: usize,
    output_number: usize,
}

impl Network {
    pub fn new(input_number: usize, output_number: usize) -> Self {
        let mut graph = DiGraph::new();

        for _ in 0..input_number {
            graph.add_node(NodeData {
                kind: NodeKind::Input,
            });
        }

        for _ in 0..output_number {
            graph.add_node(NodeData {
                kind: NodeKind::Output,
            });
        }

        for i in 0..input_number {
            for j in 0..output_number {
                let edge_data = EdgeData {
                    weight: 0.0,
                    disabled: false,
                };

                graph.add_edge(
                    NodeIndex::new(i),
                    NodeIndex::new(input_number + j),
                    edge_data,
                );
            }
        }

        let mut result = Self {
            graph,
            input_number,
            output_number,
        };

        result.randomize_weights();
        return result;
    }

    fn randomize_weights(&mut self) {
        let uniform = Uniform::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();
        for edge_data in self.graph.edge_weights_mut() {
            edge_data.weight = uniform.sample(&mut rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_eq<N, E, Ty, Ix>(
        a: &petgraph::Graph<N, E, Ty, Ix>,
        b: &petgraph::Graph<N, E, Ty, Ix>,
    ) -> bool
    // NOTE: Does not check equality of edge weights
    where
        N: PartialEq,
        E: PartialEq,
        Ty: petgraph::EdgeType,
        Ix: petgraph::graph::IndexType + PartialEq,
    {
        let a_ns = a.raw_nodes().iter().map(|n| &n.weight);
        let b_ns = b.raw_nodes().iter().map(|n| &n.weight);
        let a_es = a.raw_edges().iter().map(|e| (e.source(), e.target()));
        let b_es = b.raw_edges().iter().map(|e| (e.source(), e.target()));
        a_ns.eq(b_ns) && a_es.eq(b_es)
    }

    #[test]
    fn network_initialization() {
        let network = Network::new(2, 2);

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for &kind in &[
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Output,
        ] {
            graph.add_node(NodeData { kind });
        }

        for &(a, b) in &[(0, 2), (0, 3), (1, 2), (1, 3)] {
            graph.add_edge(
                a.into(),
                b.into(),
                EdgeData {
                    weight: 1.0,
                    disabled: false,
                },
            );
        }

        assert!(graph_eq(&network.graph, &graph));
    }
}
