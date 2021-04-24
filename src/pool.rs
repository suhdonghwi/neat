use rand::distributions::{Distribution, WeightedIndex};

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

    pub fn evolve(&mut self) -> bool {
        if self.list.iter().any(|n| n.fitness().is_none()) {
            return false;
        }

        self.list
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());

        let rng = &mut rand::thread_rng();
        let dist = WeightedIndex::new(self.list.iter().map(|x| x.fitness().unwrap())).unwrap();
        let mut new_list = Vec::new();

        for _ in 0..self.list.len() {
            let parent1 = &self.list[dist.sample(rng)];
            let parent2 = &self.list[dist.sample(rng)];

            if let Some(offspring) = parent1.crossover(parent2) {
                new_list.push(offspring);
            } else {
                return false;
            }
        }

        self.list = new_list;
        true
    }
}
