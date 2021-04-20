pub struct InnovationRecord {
    node_identifying_number: usize,
}

impl InnovationRecord {
    pub fn new() -> Self {
        Self {
            node_identifying_number: 0,
        }
    }

    pub fn new_node(&mut self) -> usize {
        self.node_identifying_number += 1;
        self.node_identifying_number - 1
    }
}
