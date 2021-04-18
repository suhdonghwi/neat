use crate::activations;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind {
    Input,
    Output,
    Hidden,
    Bias,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeData {
    kind: NodeKind,
    input_sum: f64,
}

impl NodeData {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind,
            input_sum: 0.0,
        }
    }

    pub fn add_input(&mut self, input: f64) {
        self.input_sum += input;
    }

    pub fn activate(&self) -> f64 {
        activations::sigmoid(self.input_sum)
    }
}
