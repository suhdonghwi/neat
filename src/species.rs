use crate::{network::Network, parameters::Parameters};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SpeciesInfo<T: Network + Debug + Clone> {
    representative: T,
    age: usize,
}

impl<T: Network + Debug + Clone> SpeciesInfo<T> {
    pub fn new(representative: T, age: usize) -> Self {
        Self {
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
            sum += network.fitness()? / self.list.len() as f64;
        }

        Some(sum / self.list.len() as f64)
    }
}
