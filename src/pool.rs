use rand::{
    distributions::{Distribution, Open01, Uniform},
    RngCore,
};

use crate::{innovation_record::InnovationRecord, network::Network};
use std::fmt::Debug;

fn random(rng: &mut impl RngCore) -> f64 {
    Open01.sample(rng)
}

pub struct Pool<T: Network + Debug + Clone> {
    list: Vec<T>,
    innov_record: InnovationRecord,
}

impl<T: Network + Debug + Clone> Pool<T> {
    pub fn new(input_number: usize, output_number: usize, population: usize) -> Self {
        let mut list: Vec<T> = Vec::new();
        let mut innov_record = InnovationRecord::new(input_number, output_number);

        for _ in 0..population {
            let mut network = T::new(input_number, output_number, &mut innov_record);
            network.graph_mut().randomize_weights(-30.0, 30.0);
            list.push(network);
        }

        Self { list, innov_record }
    }

    pub fn networks(&mut self) -> impl Iterator<Item = &mut T> {
        self.list.iter_mut()
    }

    fn mutate(&mut self, network: &mut T, rng: &mut impl RngCore) {
        let weight_perbutation = 0.8;
        let weight_assign = 0.1;
        let add_connection = 0.5;
        let remove_connection = 0.5;
        let add_node = 0.2;
        let remove_node = 0.2;

        let delta_uniform = Uniform::new(-1.0, 1.0);
        let assign_uniform = Uniform::new(-30.0, 30.0);

        if random(rng) < weight_perbutation {
            if let Some(to_mutate) = network.graph().random_edge(rng) {
                network.mutate_perturb_weight(to_mutate, delta_uniform.sample(rng));
            }
        }

        if random(rng) < weight_assign {
            if let Some(to_mutate) = network.graph().random_edge(rng) {
                network.mutate_assign_weight(to_mutate, assign_uniform.sample(rng));
            }
        }

        if random(rng) < add_connection {
            let source = network.graph().random_node(rng);
            let target = network.graph().random_node(rng);

            network.mutate_add_connection(
                source,
                target,
                assign_uniform.sample(rng),
                &mut self.innov_record,
            );
        }

        if random(rng) < remove_connection {
            if let Some(to_remove) = network.graph().random_edge(rng) {
                network.mutate_remove_connection(to_remove);
            }
        }

        if random(rng) < add_node {
            if let Some(to_add) = network.graph().random_edge(rng) {
                network.mutate_add_node(to_add, &mut self.innov_record);
            }
        }

        if random(rng) < remove_node {
            let to_remove = network.graph().random_node(rng);
            network.mutate_remove_node(to_remove);
        }
    }

    fn sort_by_fitness(&mut self) {
        self.list
            .sort_by(|a, b| b.fitness().partial_cmp(&a.fitness()).unwrap());
        dbg!(&self.list[0]);
    }

    pub fn evolve(&mut self) -> bool {
        if self.list.iter().any(|n| n.fitness().is_none()) {
            return false;
        }

        self.sort_by_fitness();

        let rng = &mut rand::thread_rng();
        let uniform = Uniform::new(0, 5);
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
