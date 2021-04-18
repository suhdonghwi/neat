pub trait Network {
    fn activate(&self, inputs: Vec<f64>) -> Vec<f64>;
}
