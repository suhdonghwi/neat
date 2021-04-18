use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use rand::distributions::{Distribution, Uniform};

use crate::edge_data::EdgeData;
use crate::node_data::{NodeData, NodeKind};

#[derive(Debug)]
pub struct NetworkInternal {
    graph: DiGraph<NodeData, EdgeData>,
    input_number: usize,
    output_number: usize,
}

impl NetworkInternal {
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

        return Self {
            graph,
            input_number,
            output_number,
        };
    }

    pub fn randomize_weights(&mut self) {
        let uniform = Uniform::new(-1.0, 1.0);
        let mut rng = rand::thread_rng();
        for edge_data in self.graph.edge_weights_mut() {
            edge_data.weight = uniform.sample(&mut rng);
        }
    }

    pub fn add_hidden_node(&mut self, edge: EdgeIndex) {
        let previous_weight: f64;
        let new_node_index: NodeIndex;

        {
            let edge_data = self.graph.edge_weight_mut(edge).unwrap();
            edge_data.disabled = true;
            previous_weight = edge_data.weight;

            new_node_index = self.graph.add_node(NodeData {
                kind: NodeKind::Hidden,
            });
        }

        let (source, target) = self.graph.edge_endpoints(edge).unwrap();
        self.graph.add_edge(
            source,
            new_node_index,
            EdgeData {
                weight: previous_weight,
                disabled: false,
            },
        );
        self.graph.add_edge(
            new_node_index,
            target,
            EdgeData {
                weight: 1.0,
                disabled: false,
            },
        );
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
        let network = NetworkInternal::new(2, 2);

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
                    weight: 0.0,
                    disabled: false,
                },
            );
        }

        assert!(graph_eq(&network.graph, &graph));
    }

    #[test]
    fn adding_hidden_node() {
        let mut network = NetworkInternal::new(2, 1);
        network.add_hidden_node(EdgeIndex::new(0));

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for &kind in &[
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Hidden,
        ] {
            graph.add_node(NodeData { kind });
        }

        for &(a, b) in &[(0, 2), (1, 2), (0, 3), (3, 2)] {
            graph.add_edge(
                a.into(),
                b.into(),
                EdgeData {
                    weight: 0.0,
                    disabled: false,
                },
            );
        }

        assert!(graph_eq(&network.graph, &graph));
    }
}
