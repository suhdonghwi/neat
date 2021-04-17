#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind {
    Input,
    Output,
    Hidden,
    Bias,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NodeData {
    pub kind: NodeKind,
}
