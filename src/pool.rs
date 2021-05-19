use indoc::indoc;
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
    prev_species_info: Vec<SpeciesInfo<T>>,
    generation: usize,
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
            prev_species_info: Vec::new(),
            generation: 0,
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

    fn speciate(&'a self, innov_record: &mut InnovationRecord) -> Vec<Species<T>> {
        // assumes genomes are sorted by fitness
        let mut new_species_set: Vec<Species<T>> = Vec::new();

        for mut species_info in self.prev_species_info.clone() {
            species_info.add_age();
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

    fn log(&self, verbosity: usize, message: &str) {
        if verbosity <= self.verbosity {
            println!("{}", message);
        }
    }

    fn list_stats(&self, list: &[f64]) -> (f64, f64) {
        let mut sum = 0.0;
        for element in list {
            sum += element;
        }
        let mean = sum / list.len() as f64;

        let mut delta_sum = 0.0;
        for element in list {
            delta_sum += (mean - element).powf(2.0) as f64;
        }
        let std_deviation = (delta_sum / list.len() as f64).sqrt();

        (mean, std_deviation)
    }

    fn log_evaluation(&self, fitness_list: &[f64]) {
        let (fitness_mean, fitness_std_deviation) = self.list_stats(&fitness_list);

        let message = &format!(
            indoc! {"
        # Evaluation result
          - fitness max: {} ({} nodes, {} edges)
          - fitness mean: {} (σ = {})
        "},
            fitness_list[0],
            self.list[0].graph().node_count(),
            self.list[0].graph().edge_count(),
            fitness_mean,
            fitness_std_deviation
        );
        self.log(1, message);
        self.log(2, &format!("  - best genome: {:#?}", self.list[0]));
    }

    fn log_speciation(
        &self,
        species_set: &[Species<T>],
        adj_fitness_list: &[f64],
        count_list: &[usize],
    ) {
        let mut speciation_log = format!(
            indoc! {"
            # Speciation result:
              {:^6} | {:^5} | {:^6} | {:^11} | {:^10}
              ====================================================
            "},
            "id", "age", "size", "offspring", "adj fit avg."
        );
        for i in 0..species_set.len() {
            speciation_log += &format!(
                "  {:^6} | {:^5} | {:^6} | {:^11} | {:^10.4}\n",
                species_set[i].id(),
                species_set[i].age(),
                species_set[i].genome_count(),
                count_list[i],
                adj_fitness_list[i]
            );
        }
        self.log(1, &speciation_log);
    }

    pub fn evaluate<F: Fn(usize, &mut T)>(&mut self, evaluate: F) {
        for (i, network) in self.list.iter_mut().enumerate() {
            evaluate(i, network);
            assert!(network.fitness().is_some());
        }
    }

    pub fn evolve(&mut self, innov_record: &mut InnovationRecord) -> &T {
        self.log(1, &format!("[Generation {}]", self.generation));

        self.list.sort_by(|a, b| b.compare(a).unwrap());
        let fitness_list: Vec<f64> = self.list.iter().map(|g| g.fitness().unwrap()).collect();

        self.log_evaluation(&fitness_list);

        let mut species_set = self.speciate(innov_record);
        for species in &mut species_set {
            species.kill_worst(self.params.speciation.survival_rate);
        }

        species_set = species_set
            .into_iter()
            .filter(|s| s.genome_count() > 2)
            .collect();
        if species_set.is_empty() {
            panic!("remaining species_set size is 0; maybe compatibility threshold is too small?");
        }

        let mut offspring_list: Vec<T> = Vec::new();
        for species in &species_set {
            offspring_list.extend(species.elites(self.params.speciation.elitism).to_owned());
        }

        let target_count = self.params.population - offspring_list.len();
        let adj_fitness_list: Vec<f64> = species_set
            .iter()
            .map(|s| s.adjusted_fitness_average().unwrap())
            .collect();
        let adj_fitness_sum: f64 = adj_fitness_list.iter().sum();

        let mut count_list: Vec<usize> = adj_fitness_list
            .iter()
            .map(|f| (target_count as f64 * (f / adj_fitness_sum)).ceil() as usize)
            .collect();
        let total_count: usize = count_list.iter().sum();

        for i in 0..total_count - target_count {
            count_list[i % species_set.len()] -= 1;
        }
        self.log_speciation(&species_set, &adj_fitness_list, &count_list);

        let rng = &mut rand::thread_rng();
        for (i, count) in count_list.into_iter().enumerate() {
            for _ in 0..count {
                let mut offspring = species_set[i].mate(rng).unwrap();
                self.mutate(&mut offspring, innov_record, rng);

                offspring_list.push(offspring);
            }
        }

        self.prev_species_info = species_set.into_iter().map(|s| s.info()).collect();
        self.list = offspring_list;

        self.log(1, "\n---------------------------------\n");
        self.generation += 1;

        &self.list[0]
    }
}
