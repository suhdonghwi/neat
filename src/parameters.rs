use serde::Deserialize;

#[derive(Deserialize)]
pub struct Parameters {
    pub input_number: usize,
    pub output_number: usize,
    pub population: usize,

    pub mutation: MutationParameters,
    pub speciation: SpeciationParameters,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct SpeciationParameters {
    pub c1: f64, // mismatch gene coefficient
    pub c2: f64, // weight difference cofficient

    pub compatibility_threshold: f64,
    pub survival_rate: f64,

    pub elitism: usize,
}
