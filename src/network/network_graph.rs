use std::collections::HashMap;

use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::{algo::toposort, graph::Edge};
use rand::{
    distributions::{Bernoulli, Distribution, Uniform},
    RngCore,
};

use crate::node_data::{NodeData, NodeKind};
use crate::{edge_data::EdgeData, innovation_record::InnovationRecord};

#[derive(Debug, Clone)]
pub struct NetworkGraph {
    graph: DiGraph<NodeData, EdgeData>,
    input_number: usize,
    output_number: usize,
    toposort_cache: Option<Vec<NodeIndex>>,
}

impl NetworkGraph {
    fn new_disconnected(input_number: usize, output_number: usize) -> Self {
        let mut graph = DiGraph::new();

        for i in 0..input_number {
            graph.add_node(NodeData::new(NodeKind::Input, i));
        }

        for i in 0..output_number {
            graph.add_node(NodeData::new(NodeKind::Output, input_number + i));
        }

        graph.add_node(NodeData::new(NodeKind::Bias, input_number + output_number));

        Self {
            graph,
            input_number,
            output_number,
            toposort_cache: None,
        }
    }

    pub fn new(
        input_number: usize,
        output_number: usize,
        innov_record: &mut InnovationRecord,
    ) -> Self {
        let mut network = NetworkGraph::new_disconnected(input_number, output_number);

        for i in 0..input_number {
            for j in 0..output_number {
                let innov_number = innov_record.new_connection(i, input_number + j);
                let edge_data = EdgeData::new(1.0, innov_number);

                network.graph.add_edge(
                    NodeIndex::new(i),
                    NodeIndex::new(input_number + j),
                    edge_data,
                );
            }
        }

        network
    }

    pub fn randomize_weights(&mut self, low: f64, high: f64) {
        let uniform = Uniform::new(low, high);
        let mut rng = rand::thread_rng();
        for edge_data in self.graph.edge_weights_mut() {
            edge_data.set_weight(uniform.sample(&mut rng));
        }
    }

    pub fn clear_sum(&mut self) {
        for node_data in self.graph.node_weights_mut() {
            node_data.clear_sum();
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
            let node = &self.graph[NodeIndex::new(index)];
            result.push(node.activate());
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

    pub fn random_edge(&self, rng: &mut impl RngCore) -> Option<EdgeIndex> {
        if self.graph.edge_count() == 0 {
            None
        } else {
            let uniform = Uniform::from(0..self.graph.edge_count());
            Some(EdgeIndex::new(uniform.sample(rng)))
        }
    }

    pub fn random_node(&self, rng: &mut impl RngCore) -> NodeIndex {
        let uniform = Uniform::from(0..self.graph.node_count());
        NodeIndex::new(uniform.sample(rng))
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
        let new_node_id: usize;

        {
            let edge_data = self.graph.edge_weight_mut(edge).unwrap();
            edge_data.set_disabled(true);
            previous_weight = edge_data.get_weight();

            new_node_id = innov_record.new_node();
            new_node_index = self
                .graph
                .add_node(NodeData::new(NodeKind::Hidden, new_node_id));
        }

        let (source, target) = self.graph.edge_endpoints(edge).unwrap();
        let source_id = self.graph[source].id();
        let target_id = self.graph[target].id();
        self.graph.add_edge(
            source,
            new_node_index,
            EdgeData::new(
                previous_weight,
                innov_record.new_connection(source_id, new_node_id),
            ),
        );
        self.graph.add_edge(
            new_node_index,
            target,
            EdgeData::new(1.0, innov_record.new_connection(new_node_id, target_id)),
        );

        new_node_index
    }

    pub fn remove_node(&mut self, node: NodeIndex) {
        self.graph.remove_node(node);
    }

    pub fn add_connection(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        weight: f64,
        innov_record: &mut InnovationRecord,
    ) -> EdgeIndex {
        let source_id = self.graph[source].id();
        let target_id = self.graph[target].id();
        let innov_number = innov_record.new_connection(source_id, target_id);
        let edge_data = EdgeData::new(weight, innov_number);
        self.graph.add_edge(source, target, edge_data)
    }

    pub fn remove_connetion(&mut self, edge: EdgeIndex) {
        self.graph.remove_edge(edge);
    }

    fn endpoints(&self, edge: &Edge<EdgeData>) -> (Edge<EdgeData>, &NodeData, &NodeData) {
        let source = &self.graph[edge.source()];
        let target = &self.graph[edge.target()];
        (edge.clone(), source, target)
    }

    pub fn crossover(
        &self,
        other: &NetworkGraph,
        more_fit: bool,
        rng: &mut impl RngCore,
    ) -> Option<NetworkGraph> {
        if self.input_number != other.input_number || self.output_number != other.output_number {
            return None;
        }

        let mut network = NetworkGraph::new_disconnected(self.input_number, self.output_number);
        let mut new_genes = Vec::new();

        let my_edges = self.graph.raw_edges();
        let mut other_edges: Vec<&Edge<EdgeData>> = other.graph.raw_edges().iter().collect();

        let dist = Bernoulli::new(0.5).unwrap();
        for my_edge in my_edges {
            let mut matched = None;

            for (i, &other_edge) in other_edges.iter().enumerate() {
                if other_edge.weight.innov_number() == my_edge.weight.innov_number() {
                    if dist.sample(rng) {
                        new_genes.push(self.endpoints(my_edge));
                    } else {
                        new_genes.push(other.endpoints(other_edge));
                    }

                    matched = Some(i);
                    break;
                }
            }

            if let Some(i) = matched {
                other_edges.remove(i);
            } else if more_fit {
                new_genes.push(self.endpoints(my_edge));
            }
        }

        if !more_fit {
            for edge in other_edges {
                new_genes.push(other.endpoints(edge));
            }
        }

        // node_map is used to prevent adding nodes with the same innovation number
        let mut node_map: HashMap<usize, NodeIndex> = HashMap::new();
        let mut get_index = |data: &NodeData, network: &mut Self| {
            if data.kind() != NodeKind::Hidden {
                return NodeIndex::new(data.id()); // Index of default nodes is same as ID
            }

            match node_map.get(&data.id()) {
                None => {
                    let index = network.graph.add_node(data.clone());
                    node_map.insert(data.id(), index);
                    index
                }
                Some(&i) => i,
            }
        };
        for (gene, source, target) in new_genes {
            let source_index = get_index(source, &mut network);
            let target_index = get_index(target, &mut network);

            network
                .graph
                .add_edge(source_index, target_index, gene.weight.clone());
        }

        Some(network)
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
        let input_number = 2;
        let output_number = 2;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let network = NetworkGraph::new(input_number, output_number, &mut innov_record);

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for (i, &kind) in [
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Output,
            NodeKind::Bias,
        ]
        .iter()
        .enumerate()
        {
            graph.add_node(NodeData::new(kind, i));
        }

        for (i, &(a, b)) in [(0, 2), (0, 3), (1, 2), (1, 3)].iter().enumerate() {
            graph.add_edge(a.into(), b.into(), EdgeData::new(0.0, i));
        }

        assert!(graph_eq(&network.graph, &graph));
    }

    #[test]
    fn all_input_output_bias_nodes_should_be_assigned_same_id() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);

        let network1 = NetworkGraph::new(input_number, output_number, &mut innov_record);
        let network2 = NetworkGraph::new(input_number, output_number, &mut innov_record);

        let mut graph = DiGraph::<NodeData, EdgeData>::new();
        for (i, &kind) in [
            NodeKind::Input,
            NodeKind::Input,
            NodeKind::Output,
            NodeKind::Bias,
        ]
        .iter()
        .enumerate()
        {
            graph.add_node(NodeData::new(kind, i));
        }

        for (i, &(a, b)) in [(0, 2), (1, 2)].iter().enumerate() {
            graph.add_edge(a.into(), b.into(), EdgeData::new(0.0, i));
        }

        assert!(graph_eq(&network1.graph, &graph));
        assert!(graph_eq(&network2.graph, &graph));
    }

    #[test]
    fn input_nodes_should_be_returned() {
        let input_number = 5;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = NetworkGraph::new(input_number, output_number, &mut innov_record);
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
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = NetworkGraph::new(input_number, output_number, &mut innov_record);
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

        for (i, &(a, b)) in [(0, 2), (1, 2), (0, 4), (4, 2)].iter().enumerate() {
            graph.add_edge(a.into(), b.into(), EdgeData::new(0.0, i));
        }

        assert!(graph_eq(&network.graph, &graph));
    }

    #[test]
    fn add_connection_should_connect_nodes() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = NetworkGraph::new(input_number, output_number, &mut innov_record);
        network.add_node(EdgeIndex::new(0), &mut innov_record);

        let result = network.add_connection(1.into(), 4.into(), 0.0, &mut innov_record);

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

        for (i, &(a, b)) in [(0, 2), (1, 2), (0, 4), (4, 2), (1, 4)].iter().enumerate() {
            graph.add_edge(a.into(), b.into(), EdgeData::new(0.0, i));
        }
        assert!(graph_eq(&network.graph, &graph));
    }

    #[test]
    fn toposort_should_work_on_dag() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);
        let mut network = NetworkGraph::new(input_number, output_number, &mut innov_record);
        network.add_node(EdgeIndex::new(0), &mut innov_record);

        let result = network.toposort();

        assert_eq!(
            result,
            Some(vec![3.into(), 1.into(), 0.into(), 4.into(), 2.into()])
        );
    }

    #[test]
    fn crossover_should_pass_only_from_more_fit_parent() {
        let input_number = 2;
        let output_number = 1;
        let mut innov_record = InnovationRecord::new(input_number, output_number);

        let mut network1 = NetworkGraph::new(input_number, output_number, &mut innov_record);
        let mut network2 = NetworkGraph::new(input_number, output_number, &mut innov_record);

        network1.add_node(EdgeIndex::new(0), &mut innov_record);
        network2.add_node(EdgeIndex::new(1), &mut innov_record);

        // Edge weight is same in network1, 2 - so constant seeding is not needed here.
        let mut rng = rand::thread_rng();
        if let Some(offspring) = network1.crossover(&network2, true, &mut rng) {
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

            for (i, &(a, b)) in [(0, 2), (1, 2), (0, 4), (4, 2)].iter().enumerate() {
                graph.add_edge(a.into(), b.into(), EdgeData::new(0.0, i));
            }

            assert!(graph_eq(&offspring.graph, &graph));
        } else {
            panic!("crossover result is None");
        }
    }
}
