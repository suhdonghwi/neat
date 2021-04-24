use rand::{
    distributions::{Distribution, Open01, Uniform, WeightedIndex},
    RngCore,
};

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

    fn mutate(&mut self, network: &mut T, rng: &mut impl RngCore) {
        let weight_perbutation = 0.6;
        let weight_assign = 0.1;
        let add_connection = 0.2;
        let add_node = 0.2;

        let rand: f64 = Open01.sample(rng);
        let weight_uniform = Uniform::new(-1.0, 1.0);

        if rand < weight_perbutation {
            network.mutate_perturb_weight(network.random_edge(rng), weight_uniform.sample(rng));
        }
        if rand < weight_assign {
            network.mutate_assign_weight(network.random_edge(rng), weight_uniform.sample(rng));
        }
        if rand < add_connection {
            let source = network.random_node(rng);
            let mut target = network.random_node(rng);
            while target == source {
                target = network.random_node(rng);
            }

            network.mutate_add_connection(
                source,
                target,
                weight_uniform.sample(rng),
                &mut self.innov_record,
            );
        }
        if rand < add_node {
            network.mutate_add_node(network.random_edge(rng), &mut self.innov_record);
        }
    }

    pub fn evolve(&mut self) -> bool {
        if self.list.iter().any(|n| n.fitness().is_none()) {
            return false;
        }

        self.list
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        dbg!(self.list[0].fitness());

        let rng = &mut rand::thread_rng();
        let uniform = Uniform::new(0, 10);
        let mut new_list = Vec::new();

        for _ in 0..self.list.len() {
            let parent1 = &self.list[uniform.sample(rng)];
            let parent2 = &self.list[uniform.sample(rng)];

            if let Some(mut offspring) = parent1.crossover(parent2) {
                self.mutate(&mut offspring, rng);
                new_list.push(offspring);
            } else {
                return false;
            }
        }

        self.list = new_list;
        true
    }
}
