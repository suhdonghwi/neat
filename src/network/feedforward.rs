use super::{network_graph::NetworkGraph, Network};

struct Feedforward {
    graph: NetworkGraph,
}

impl Network for Feedforward {
    fn activate(&self, inputs: Vec<f64>) -> Option<Vec<f64>> {
        None
    }
}
