use std::fmt::Debug;

use crate::{innovation_record::InnovationRecord, network::Network};

pub struct Pool<T: Network + Debug> {
    list: Vec<T>,
    innov_record: InnovationRecord,
}

impl<T: Network + Debug> Pool<T> {
    pub fn new(input_number: usize, output_number: usize, population: usize) -> Self {
        let mut list: Vec<T> = Vec::new();
        let mut innov_record = InnovationRecord::new(input_number, output_number);

        for _ in 0..population {
            let mut network = T::new(input_number, output_number, &mut innov_record);
            network.randomize_weights(-1.0, 1.0);
            list.push(network);
        }

        Self { list, innov_record }
    }

    pub fn networks(&mut self) -> impl Iterator<Item = &mut T> {
        self.list.iter_mut()
    }

    pub fn evolve(&mut self) -> Option<Pool<T>> {
        if self.list.iter().any(|n| n.fitness().is_none()) {
            return None;
        }

        self.list
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

        dbg!(&self.list);
        None
    }
}
