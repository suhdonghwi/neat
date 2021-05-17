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
    activated: bool,
}

impl NodeData {
    pub fn new(kind: NodeKind, id: usize) -> Self {
        Self {
            kind,
            id,
            input_sum: 0.0,
            activated: false,
        }
    }

    pub fn add_input(&mut self, input: f64) {
        self.input_sum += input;
        self.activated = true;
    }

    pub fn clear_sum(&mut self) {
        self.input_sum = 0.0;
        self.activated = false;
    }

    pub fn activate(&self) -> Option<f64> {
        if self.kind == NodeKind::Input || self.kind == NodeKind::Bias {
            Some(self.input_sum)
        } else if self.activated {
            Some(activations::sigmoid(self.input_sum))
        } else if self.kind == NodeKind::Output {
            Some(0.0)
        } else {
            None
        }
    }

    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
