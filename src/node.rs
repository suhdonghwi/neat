#[derive(Debug, PartialEq)]
pub enum NodeKind {
  Input,
  Output,
  Hidden,
  Bias,
}

#[derive(Debug, PartialEq)]
pub struct Node {
  pub kind: NodeKind,
}
