use rand::{distributions::Uniform, prelude::Distribution, Rng, RngCore};

use crate::{activations::ActivationKind, network::Network};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SpeciesInfo<T: Network + Debug + Clone> {
    id: usize,
    representative: T,
    age: usize,

    previous_fitness: Option<f64>,
    stagnant: usize,
}

impl<T: Network + Debug + Clone> SpeciesInfo<T> {
    pub fn new(id: usize, representative: T, age: usize) -> Self {
        Self {
            id,
            representative,
            age,
            previous_fitness: None,
            stagnant: 0,
        }
    }

    pub fn add_age(&mut self) {
        self.age += 1;
    }
}

#[derive(Clone, Debug)]
pub struct Species<'a, T: Network + Debug + Clone> {
    info: SpeciesInfo<T>,
    list: Vec<&'a T>,
}

impl<'a, T: Network + Debug + Clone> Species<'a, T> {
    pub fn new(info: SpeciesInfo<T>) -> Self {
        Species {
            list: Vec::new(),
            info,
        }
    }

    pub fn info(self) -> SpeciesInfo<T> {
        self.info
    }

    pub fn try_assign(&mut self, network: &'a T, c1: f64, c2: f64, threshold: f64) -> bool {
        let metric = self
            .info
            .representative
            .graph()
            .compatibility_metric(network.graph(), c1, c2);

        if metric <= threshold {
            self.list.push(network);
            true
        } else {
            false
        }
    }

    pub fn force_assign(&mut self, network: &'a T) {
        self.list.push(network);
    }

    pub fn kill_worst(&mut self, survival_rate: f64) {
        let mut remaining = ((self.list.len() as f64) * survival_rate).floor() as usize;
        if remaining == 0 {
            remaining = 1;
        }
        self.list.truncate(remaining);
    }

    pub fn replace_representative(&mut self, rng: &mut impl RngCore) {
        let index = rng.gen_range(0..self.list.len());
        self.info.representative = self.list[index].clone();
    }

    pub fn update_adjusted_fitness(&mut self) -> Option<f64> {
        let mut sum = 0.0;
        let len = self.list.len() as f64;

        for network in &self.list {
            sum += network.fitness()?;
        }

        let fitness = if self.list.is_empty() {
            0.0
        } else {
            sum / len / len
        };

        if fitness <= self.info.previous_fitness.unwrap_or(0.0) {
            self.info.stagnant += 1;
        } else {
            self.info.stagnant = 0;
        }
        self.info.previous_fitness = Some(fitness);

        Some(fitness)
    }

    pub fn genome_count(&self) -> usize {
        self.list.len()
    }

    pub fn random_genome(&self, rng: &mut impl RngCore) -> T {
        let index = rng.gen_range(0..self.list.len());
        self.list[index].clone()
    }

    pub fn mate(
        &self,
        rng: &mut impl RngCore,
        hidden_func: ActivationKind,
        output_func: ActivationKind,
    ) -> Option<T> {
        let uniform = Uniform::new(0, self.list.len());

        let index1 = uniform.sample(rng);
        let mut index2 = index1;
        while index1 == index2 {
            index2 = uniform.sample(rng);
        }

        let parent1 = self.list[index1];
        let parent2 = self.list[index2];

        parent1.crossover(parent2, hidden_func, output_func)
    }

    pub fn elites(&self, count: usize) -> Vec<T> {
        let mut result = Vec::new();

        for i in 0..count.min(self.list.len()) {
            result.push(self.list[i].clone());
        }

        result
    }

    pub fn age(&self) -> usize {
        self.info.age
    }

    pub fn id(&self) -> usize {
        self.info.id
    }

    pub fn stagnant(&self) -> usize {
        self.info.stagnant
    }
}
