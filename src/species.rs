use rand::{distributions::Uniform, prelude::Distribution, RngCore};

use crate::{network::Network, parameters::Parameters};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SpeciesInfo<T: Network + Debug + Clone> {
    id: usize,
    representative: T,
    age: usize,
}

impl<T: Network + Debug + Clone> SpeciesInfo<T> {
    pub fn new(id: usize, representative: T, age: usize) -> Self {
        Self {
            id,
            representative,
            age,
        }
    }

    pub fn age(&mut self) {
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

    pub fn try_assign(&mut self, network: &'a T, params: &Parameters) -> bool {
        let metric = self.info.representative.graph().compatibility_metric(
            network.graph(),
            params.speciation.c1,
            params.speciation.c2,
        );

        if metric <= params.speciation.compatibility_threshold {
            self.list.push(network);
            true
        } else {
            false
        }
    }

    pub fn kill_worst(&mut self, survival_rate: f64) {
        self.list
            .truncate(((self.list.len() as f64) * survival_rate).floor() as usize);
    }

    pub fn adjusted_fitness_average(&self) -> Option<f64> {
        let mut sum = 0.0;
        for network in &self.list {
            sum += network.fitness()?;
        }

        if self.list.len() == 0 {
            Some(0.0)
        } else {
            Some(sum / (self.list.len() as f64) / (self.list.len() as f64))
        }
    }

    pub fn genome_count(&self) -> usize {
        self.list.len()
    }

    pub fn mate(&self, rng: &mut impl RngCore) -> Option<T> {
        let uniform = Uniform::new(0, self.list.len());

        let index1 = uniform.sample(rng);
        let mut index2 = uniform.sample(rng);
        while index1 == index2 {
            index2 = uniform.sample(rng);
        }

        let parent1 = &self.list[index1];
        let parent2 = &self.list[index2];

        parent1.crossover(parent2)
    }

    pub fn elites(&self, count: usize) -> Vec<T> {
        self.list.iter().take(count).cloned().cloned().collect()
    }
}
