use log::{info, trace};
use rand::{
    distributions::{Distribution, Open01, Uniform},
    RngCore,
};

use crate::{
    innovation_record::InnovationRecord, network::Network, parameters::Parameters, specie::Specie,
};
use std::fmt::Debug;

fn random(rng: &mut impl RngCore) -> f64 {
    Open01.sample(rng)
}

pub struct Pool<T: Network + Debug + Clone> {
    list: Vec<T>,
    innov_record: InnovationRecord,
    params: Parameters,
}

impl<T: Network + Debug + Clone> Pool<T> {
    pub fn new(params: Parameters) -> Self {
        let mut list: Vec<T> = Vec::new();
        let mut innov_record = InnovationRecord::new(params.input_number, params.output_number);

        for _ in 0..params.population {
            let mut network = T::new(params.input_number, params.output_number, &mut innov_record);
            network
                .graph_mut()
                .randomize_weights(params.mutation.weight_min, params.mutation.weight_max);
            list.push(network);
        }

        Self {
            list,
            innov_record,
            params,
        }
    }

    pub fn networks(&mut self) -> impl Iterator<Item = &mut T> {
        self.list.iter_mut()
    }

    fn mutate(&mut self, network: &mut T, rng: &mut impl RngCore) {
        let delta_uniform = Uniform::new(
            self.params.mutation.perturb_min,
            self.params.mutation.perturb_max,
        );
        let assign_uniform = Uniform::new(
            self.params.mutation.weight_min,
            self.params.mutation.weight_max,
        );

        if random(rng) < self.params.mutation.weight_perturbation {
            if let Some(to_mutate) = network.graph().random_edge(rng) {
                network.mutate_perturb_weight(
                    to_mutate,
                    delta_uniform.sample(rng),
                    self.params.mutation.weight_min,
                    self.params.mutation.weight_max,
                );
            }
        }

        if random(rng) < self.params.mutation.weight_assign {
            if let Some(to_mutate) = network.graph().random_edge(rng) {
                network.mutate_assign_weight(to_mutate, assign_uniform.sample(rng));
            }
        }

        if random(rng) < self.params.mutation.add_node {
            if let Some(to_add) = network.graph().random_edge(rng) {
                network.mutate_add_node(to_add, &mut self.innov_record);
            }
        }

        if random(rng) < self.params.mutation.remove_node {
            let to_remove = network.graph().random_node(rng);
            network.mutate_remove_node(to_remove);
        }

        if random(rng) < self.params.mutation.add_connection {
            let source = network.graph().random_node(rng);
            let target = network.graph().random_node(rng);

            network.mutate_add_connection(
                source,
                target,
                assign_uniform.sample(rng),
                &mut self.innov_record,
            );
        }

        if random(rng) < self.params.mutation.remove_connection {
            if let Some(to_remove) = network.graph().random_edge(rng) {
                network.mutate_remove_connection(to_remove);
            }
        }

        if random(rng) < self.params.mutation.toggle_connection {
            if let Some(to_toggle) = network.graph().random_edge(rng) {
                network.mutate_toggle_connection(to_toggle);
            }
        }
    }

    fn sort_by_fitness(&mut self) {
        self.list.sort_by(|a, b| b.compare(a).unwrap());
    }

    pub fn reproduce(&mut self) -> bool {
        // assumes gene pool is sorted by fitness correctly

        let rng = &mut rand::thread_rng();
        let uniform = Uniform::new(0, 15);
        let mut new_list = Vec::new();

        for _ in 0..self.list.len() {
            let index1 = uniform.sample(rng);
            let mut index2 = uniform.sample(rng);
            while index1 == index2 {
                index2 = uniform.sample(rng);
            }

            let parent1 = &self.list[index1];
            let parent2 = &self.list[index2];

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

    fn speciate(&self) -> Vec<Specie<T>> {
        // also assumes gene pool is sorted by fitness correctly

        let mut species: Vec<Specie<T>> = Vec::new();

        for network in &self.list {
            let mut assigned = false;

            for specie in &mut species {
                if specie.try_assign(network, &self.params) {
                    assigned = true;
                    break;
                }
            }

            if !assigned {
                let new_specie = Specie::new(network);
                species.push(new_specie);
            }
        }

        species
    }

    pub fn evolve<F: Fn(&mut Vec<T>) -> ()>(
        &mut self,
        generation: usize,
        fitness_threshold: f64,
        evaluate: F,
    ) {
        for current_generation in 1..=generation {
            evaluate(&mut self.list);
            self.sort_by_fitness();

            let best_fitness = self.list[0].fitness().unwrap();
            info!(
                "Generation {} [best fitness : {}, complexity : ({}, {})]",
                current_generation,
                best_fitness,
                self.list[0].graph().node_count(),
                self.list[0].graph().edge_count()
            );
            trace!("{:#?}", self.list[0]);

            if best_fitness > fitness_threshold {
                break;
            }

            let species = self.speciate();
            info!("There are {} species", species.len());

            self.reproduce();
        }
    }
}
