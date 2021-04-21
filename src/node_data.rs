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
    id: usize,
    input_sum: f64,
}

impl NodeData {
    pub fn new(kind: NodeKind, id: usize) -> Self {
        Self {
            kind,
            id,
            input_sum: 0.0,
        }
    }

    pub fn add_input(&mut self, input: f64) {
        self.input_sum += input;
    }

    pub fn activate(&self) -> f64 {
        if self.kind == NodeKind::Input || self.kind == NodeKind::Bias {
            self.input_sum
        } else {
            activations::sigmoid(self.input_sum)
        }
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
