pub struct Parameters {
    pub input_number: usize,
    pub output_number: usize,
    pub population: usize,

    pub mutation: MutationParameters,
}

pub struct MutationParameters {
    pub weight_perturbation: f64,
    pub weight_assign: f64,
    pub add_connection: f64,
    pub remove_connection: f64,
    pub toggle_connection: f64,
    pub add_node: f64,
    pub remove_node: f64,

    pub weight_min: f64, // -30.0
    pub weight_max: f64, // 30.0

    pub perturb_min: f64, // -1.0
    pub perturb_max: f64, // 1.0
}
