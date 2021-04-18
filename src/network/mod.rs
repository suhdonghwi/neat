pub mod feedforward;
mod network_graph;

pub trait Network {
    fn activate(&mut self, inputs: Vec<f64>) -> Option<Vec<f64>>;
}
