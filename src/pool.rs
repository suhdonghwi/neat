use rand::{
    distributions::{Distribution, Open01, Uniform},
    RngCore,
};

use crate::{
    innovation_record::InnovationRecord,
    network::Network,
    parameters::Parameters,
    species::{Species, SpeciesInfo},
};
use std::fmt::Debug;

fn random(rng: &mut impl RngCore) -> f64 {
    Open01.sample(rng)
}

pub struct Pool<T: Network + Debug + Clone> {
    list: Vec<T>,
    params: Parameters,
    verbosity: usize,
}

impl<'a, T: Network + Debug + Clone> Pool<T> {
    pub fn new(params: Parameters, verbosity: usize, innov_record: &mut InnovationRecord) -> Self {
        let mut list: Vec<T> = Vec::new();

        for _ in 0..params.population {
            let mut network = T::new(params.input_number, params.output_number, innov_record);
            network
                .graph_mut()
                .randomize_weights(params.mutation.weight_min, params.mutation.weight_max);
            list.push(network);
        }

        Self {
            list,
            params,
            verbosity,
        }
    }

    fn mutate(&self, network: &mut T, innov_record: &mut InnovationRecord, rng: &mut impl RngCore) {
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
                network.mutate_add_node(to_add, innov_record);
            }
        }

        if random(rng) < self.params.mutation.remove_node {
            let to_remove = network.graph().random_node(rng);
            network.mutate_remove_node(to_remove);
        }

        if random(rng) < self.params.mutation.add_connection {
            let source = network.graph().random_node(rng);
            let target = network.graph().random_node(rng);

            network.mutate_add_connection(source, target, assign_uniform.sample(rng), innov_record);
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

    fn speciate(
        &'a self,
        prev_species_set: Vec<SpeciesInfo<T>>,
        innov_record: &mut InnovationRecord,
    ) -> Vec<Species<T>> {
        // assumes genomes are sorted by fitness

        let mut new_species_set: Vec<Species<T>> = Vec::new();

        for mut species_info in prev_species_set {
            species_info.age();
            new_species_set.push(Species::new(species_info));
        }

        for network in &self.list {
            let mut found = false;

            for species in &mut new_species_set {
                if species.try_assign(network, &self.params) {
                    found = true;
                    break;
                }
            }

            if !found {
                let id = innov_record.new_species();
                new_species_set.push(Species::new(SpeciesInfo::new(id, network.clone(), 0)));
            }
        }

        new_species_set
    }

    fn log(&self, verbosity: usize, message: String) {
        if verbosity <= self.verbosity {
            println!("{}", message);
        }
    }

    pub fn evolve<F: Fn(&mut Vec<T>) -> ()>(
        &mut self,
        generation: usize,
        fitness_threshold: f64,
        innov_record: &mut InnovationRecord,
        evaluate: F,
    ) {
        let mut prev_species_info: Vec<SpeciesInfo<T>> = Vec::new();

        for current_generation in 1..=generation {
            self.log(
                1,
                format!("\n===== Generation {} =====", current_generation),
            );

            evaluate(&mut self.list);
            self.sort_by_fitness();

            let best_fitness = self.list[0].fitness().unwrap();
            self.log(
                1,
                format!(
                    "[Evaluation result]\n- best fitness: {} ({} nodes, {} edges)",
                    best_fitness,
                    self.list[0].graph().node_count(),
                    self.list[0].graph().edge_count()
                ),
            );
            self.log(2, format!("- best genome: {:#?}", self.list[0]));

            if best_fitness > fitness_threshold {
                break;
            }

            let mut species_set = self.speciate(prev_species_info, innov_record);

            for species in &mut species_set {
                species.kill_worst(self.params.speciation.survival_rate);
            }

            species_set = species_set
                .into_iter()
                .filter(|s| s.genome_count() > 2)
                .collect();

            if species_set.len() == 0 {
                panic!(
                    "remaining species_set size is 0; maybe compatibility threshold is too small?"
                );
            }

            let mut offspring_list: Vec<T> = Vec::new();

            for species in &species_set {
                offspring_list.extend(species.elites(self.params.speciation.elitism));
            }

            let target_count = self.params.population - offspring_list.len();

            let fitness_list: Vec<f64> = species_set
                .iter()
                .map(|s| s.adjusted_fitness_average().unwrap())
                .collect();
            let fitness_sum: f64 = fitness_list.iter().sum();

            let mut count_list: Vec<usize> = fitness_list
                .iter()
                .map(|f| (target_count as f64 * (f / fitness_sum)).ceil() as usize)
                .collect();
            let total_count: usize = count_list.iter().sum();

            for i in 0..total_count - target_count {
                count_list[i % species_set.len()] -= 1;
            }

            /*
            info!(
                "[spec] {} species, assigned list : {:?}",
                species_set.len(),
                &count_list
            );
            */

            let rng = &mut rand::thread_rng();
            for (i, count) in count_list.into_iter().enumerate() {
                for _ in 0..count {
                    let mut offspring = species_set[i].mate(rng).unwrap();
                    self.mutate(&mut offspring, innov_record, rng);

                    offspring_list.push(offspring);
                }
            }

            prev_species_info = Vec::new();
            for species in species_set {
                prev_species_info.push(species.info());
            }

            self.list = offspring_list;
        }
    }
}
