use petgraph::graph::DiGraph;

pub struct Network {
  graph: DiGraph<u32, f32>,
}
