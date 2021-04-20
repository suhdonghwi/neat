pub struct InnovationRecord {
    node_identifying_number: usize,
}

impl InnovationRecord {
    fn new() -> Self {
        Self {
            node_identifying_number: 0,
        }
    }

    fn new_node(&mut self) -> usize {
        self.node_identifying_number += 1;
        self.node_identifying_number - 1
    }
}
