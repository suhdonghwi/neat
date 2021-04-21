use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use rand::distributions::{Distribution, Uniform};

use crate::node_data::{NodeData, NodeKind};
use crate::{edge_data::EdgeData, innovation_record::InnovationRecord};

#[derive(Debug)]
pub struct NetworkGraph {
    graph: DiGraph<NodeData, EdgeData>,
    input_number: usize,
    output_number: usize,
    toposort_cache: Option<Vec<NodeIndex>>,
}

impl NetworkGraph {
    pub fn new(
        input_number: usize,
        output_number: usize,
        innov_record: &mut InnovationRecord,
    ) -> Self {
        let mut graph = DiGraph::new();

        for _ in 0..input_number {
            graph.add_node(NodeData::new(NodeKind::Input, innov_record.new_node()));
        }

        for _ in 0..output_number {
            graph.add_node(NodeData::new(NodeKind::Output, innov_record.new_node()));
        }

        graph.add_node(NodeData::new(NodeKind::Bias, innov_record.new_node()));

        for i in 0..input_number {
            for j in 0..output_number {
                let edge_data = EdgeData {
                    weight: 1.0,
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
            toposort_cache: None,
        };
    }

    pub fn randomize_weights(&mut self, low: f64, high: f64) {
        let uniform = Uniform::new(low, high);
        let mut rng = rand::thread_rng();
        for edge_data in self.graph.edge_weights_mut() {
            edge_data.weight = uniform.sample(&mut rng);
        }
    }

    pub fn input_nodes_mut(&mut self) -> impl Iterator<Item = &mut NodeData> {
        self.graph.node_weights_mut().take(self.input_number)
    }

    pub fn bias_node_mut(&mut self) -> &mut NodeData {
        &mut self.graph[NodeIndex::new(self.input_number + self.output_number)]
    }

    pub fn activate_output(&self) -> Vec<f64> {
        let mut result = Vec::new();
        for index in self.input_number..self.input_number + self.output_number {
            result.push(self.graph[NodeIndex::new(index)].activate());
        }

        result
    }

    pub fn node(&self, index: NodeIndex) -> &NodeData {
        &self.graph[index]
    }

    pub fn node_mut(&mut self, index: NodeIndex) -> &mut NodeData {
        &mut self.graph[index]
    }

    pub fn edge(&self, index: EdgeIndex) -> &EdgeData {
        &self.graph[index]
    }

    pub fn edge_mut(&mut self, index: EdgeIndex) -> &mut EdgeData {
        &mut self.graph[index]
    }

    pub fn outgoing(&self, index: NodeIndex) -> Vec<(EdgeIndex, NodeIndex)> {
        let mut result = Vec::new();
        let mut neighbors = self.graph.neighbors(index).detach();

        while let Some(n) = neighbors.next(&self.graph) {
            result.push(n);
        }

        result
    }

    pub fn toposort(&mut self) -> Option<Vec<NodeIndex>> {
        if self.toposort_cache.is_none() {
            self.toposort_cache = toposort(&self.graph, None).ok();
        }

        self.toposort_cache.clone()
    }

    pub fn has_connection(&self, source: NodeIndex, target: NodeIndex) -> bool {
        self.graph.contains_edge(source, target)
    }

    pub fn has_cycle(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }

    pub fn add_node(&mut self, edge: EdgeIndex, innov_record: &mut InnovationRecord) -> NodeIndex {
        let previous_weight: f64;
        let new_node_index: NodeIndex;

        {
            let edge_data = self.graph.edge_weight_mut(edge).unwrap();
            edge_data.disabled = true;
            previous_weight = edge_data.weight;

            new_node_index = self
                .graph
                .add_node(NodeData::new(NodeKind::Hidden, innov_record.new_node()));
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

        new_node_index
    }

    pub fn add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        edge_data: EdgeData,
    ) -> EdgeIndex {
        self.graph.add_edge(source, target, edge_data)
    }

    pub fn remove_connetion(&mut self, edge: EdgeIndex) {
        self.graph.remove_edge(edge);
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
    fn nodes_should_fully_connect_on_initialization() {
        let mut innov_record = InnovationRecord::new();
        let network = NetworkGraph::new(2, 2, &mut innov_record);

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        let mut i: usize = 0;
        for &kind in &[
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Output,
            NodeKind::Bias,
        ] {
            graph.add_node(NodeData::new(kind, i));
            i += 1;
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
    fn input_nodes_should_be_returned() {
        let mut innov_record = InnovationRecord::new();
        let mut network = NetworkGraph::new(5, 1, &mut innov_record);
        let input_nodes = network.input_nodes_mut().collect::<Vec<_>>();

        assert_eq!(
            input_nodes,
            vec![
                &NodeData::new(NodeKind::Input, 0),
                &NodeData::new(NodeKind::Input, 1),
                &NodeData::new(NodeKind::Input, 2),
                &NodeData::new(NodeKind::Input, 3),
                &NodeData::new(NodeKind::Input, 4),
            ]
        )
    }

    #[test]
    fn add_node_should_split_edge() {
        let mut innov_record = InnovationRecord::new();
        let mut network = NetworkGraph::new(2, 1, &mut innov_record);
        network.add_node(EdgeIndex::new(0), &mut innov_record);

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for (i, &kind) in [
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Bias,
            NodeKind::Hidden,
        ]
        .iter()
        .enumerate()
        {
            graph.add_node(NodeData::new(kind, i));
        }

        for &(a, b) in &[(0, 2), (1, 2), (0, 4), (4, 2)] {
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
    fn add_connection_should_connect_nodes() {
        let mut innov_record = InnovationRecord::new();
        let mut network = NetworkGraph::new(2, 1, &mut innov_record);
        network.add_node(EdgeIndex::new(0), &mut innov_record);

        let result = network.add_connection(
            1.into(),
            4.into(),
            EdgeData {
                weight: 0.0,
                disabled: false,
            },
        );

        assert_eq!(result, 4.into());

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for (i, &kind) in [
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Bias,
            NodeKind::Hidden,
        ]
        .iter()
        .enumerate()
        {
            graph.add_node(NodeData::new(kind, i));
        }

        for &(a, b) in &[(0, 2), (1, 2), (0, 4), (4, 2), (1, 4)] {
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
    fn toposort_should_work_on_dag() {
        let mut innov_record = InnovationRecord::new();
        let mut network = NetworkGraph::new(2, 1, &mut innov_record);
        network.add_node(EdgeIndex::new(0), &mut innov_record);

        let result = network.toposort();

        assert_eq!(
            result,
            Some(vec![3.into(), 1.into(), 0.into(), 4.into(), 2.into()])
        );
    }
}
