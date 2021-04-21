use std::collections::HashMap;

pub struct InnovationRecord {
    node_counter: usize,
    connection_counter: usize,
    connection_record: HashMap<(usize, usize), usize>,
}

impl InnovationRecord {
    pub fn new(input_number: usize, output_number: usize) -> Self {
        Self {
            node_counter: input_number + output_number + 1,
            connection_counter: 0,
            connection_record: HashMap::new(),
        }
    }

    pub fn new_node(&mut self) -> usize {
        self.node_counter += 1;
        self.node_counter - 1
    }

    pub fn new_connection(&mut self, source: usize, target: usize) -> usize {
        match self.connection_record.get(&(source, target)) {
            None => {
                self.connection_record
                    .insert((source, target), self.connection_counter);
                self.connection_counter += 1;
                self.connection_counter - 1
            }
            Some(&n) => n,
        }
    }
}
