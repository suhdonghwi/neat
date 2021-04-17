use petgraph::data::FromElements;
use petgraph::graph::{DiGraph, Edge, Node, NodeIndex};

use crate::edge_data::EdgeData;
use crate::node_data::{NodeData, NodeKind};

struct Network {
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
                graph.add_edge(
                    NodeIndex::new(i),
                    NodeIndex::new(input_number + j),
                    EdgeData {
                        weight: 1.0,
                        disabled: false,
                    },
                );
            }
        }

        Self {
            graph,
            input_number,
            output_number,
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
    where
        N: PartialEq,
        E: PartialEq,
        Ty: petgraph::EdgeType,
        Ix: petgraph::graph::IndexType + PartialEq,
    {
        let a_ns = a.raw_nodes().iter().map(|n| &n.weight);
        let b_ns = b.raw_nodes().iter().map(|n| &n.weight);
        let a_es = a
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        let b_es = b
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
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
