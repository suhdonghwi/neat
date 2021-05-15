use crate::{network::Network, parameters::Parameters};
use std::fmt::Debug;

pub struct Specie<'a, T: Network + Debug + Clone> {
    list: Vec<&'a T>,
    representative: &'a T,
}

impl<'a, T: Network + Debug + Clone> Specie<'a, T> {
    pub fn new(representative: &'a T) -> Specie<'a, T> {
        return Specie {
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
            return true;
        } else {
            return false;
        }
    }
}
