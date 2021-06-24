use serde::Deserialize;

use crate::activations::ActivationKind;

#[derive(Deserialize, Clone)]
pub struct Parameters {
    pub input_number: usize,
    pub output_number: usize,
    pub population: usize,

    pub hidden_activation: ActivationKind,
    pub output_activation: ActivationKind,

    pub mutation: MutationParameters,
    pub speciation: SpeciationParameters,
    pub reproduction: ReproductionParameters,
}

#[derive(Deserialize, Clone, Copy)]
pub struct MutationParameters {
    pub weight_perturbation: f64,
    pub weight_assign: f64,
    pub add_connection: f64,
    pub remove_connection: f64,
    pub toggle_connection: f64,
    pub add_node: f64,
    pub remove_node: f64,

    pub weight_min: f64,
    pub weight_max: f64,

    pub perturb_min: f64,
    pub perturb_max: f64,
}

#[derive(Deserialize, Clone, Copy)]
pub struct SpeciationParameters {
    pub c1: f64, // mismatch gene coefficient
    pub c2: f64, // weight difference cofficient

    pub compatibility_threshold: f64,
    pub survival_rate: f64,
    pub stagnant_max: usize,

    pub elitism: usize,
}

#[derive(Deserialize, Clone, Copy)]
pub struct ReproductionParameters {
    pub crossover_rate: f64,
}
