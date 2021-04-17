pub enum NodeKind {
  Input,
  Output,
  Hidden,
  Bias,
}

pub struct Node {
  kind: NodeKind,
}
