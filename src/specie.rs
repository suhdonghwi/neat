use crate::{network::Network, parameters::Parameters};
use std::fmt::Debug;

pub struct Species<'a, T: Network + Debug + Clone> {
    list: Vec<&'a T>,
    representative: &'a T,
}

impl<'a, T: Network + Debug + Clone> Species<'a, T> {
    pub fn new(representative: &'a T) -> Species<'a, T> {
        return Species {
            list: vec![representative],
            representative,
        };
    }

    pub fn try_assign(&mut self, network: &'a T, params: &Parameters) -> bool {
        let metric = self.representative.graph().compatibility_metric(
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
