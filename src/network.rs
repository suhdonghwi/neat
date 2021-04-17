use petgraph::graph::DiGraph;

use crate::edge::Edge;
use crate::node::Node;

pub struct Network {
  graph: DiGraph<Node, Edge>,
}
