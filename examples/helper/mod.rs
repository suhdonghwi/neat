use neat::parameters::Parameters;
use std::fs;

pub mod cli;
pub mod fitness_plot;
pub mod graph_visual;
pub mod text;
pub mod main_layout;

pub fn read_parameters_file(path: &str) -> Parameters {
    let params_str;
    if let Ok(str) = fs::read_to_string(path) {
        params_str = str;
    } else {
        panic!("Couldn't read params file path: {}", path);
    }

    toml::from_str(&params_str).unwrap()
}

#[allow(dead_code)]
pub fn main() {}
