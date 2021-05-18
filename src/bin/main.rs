use std::fs;

use clap::{clap_app, value_t};
use neatlib::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};
use neatlib::{network::Network, parameters::Parameters};

fn main() {
    let matches = clap_app!(neat =>
        (version: "0.1")
        (author: "Suh Donghwi <hwidongsuh@gmail.com>")
        (about: "NEAT(NeuroEvolution of Augmenting Topologies) implementation written in Rust")
        (@arg VERBOSITY: -v --verbosity +takes_value default_value("0") possible_values(&["0", "1", "2"]) "Sets verbosity of log")
    )
    .get_matches();

    let verbosity = value_t!(matches.value_of("VERBOSITY"), usize).unwrap();

    let params_file_path = "./params/xor.toml";
    let params_str;
    if let Ok(str) = fs::read_to_string(params_file_path) {
        params_str = str;
    } else {
        panic!("Couldn't read params file path: {}", params_file_path);
    }

    let params: Parameters = toml::from_str(&params_str).unwrap();
    let mut innov_record = InnovationRecord::new(params.input_number, params.output_number);
    let mut pool = Pool::<Feedforward>::new(params, verbosity, &mut innov_record);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for _ in 0..50 {
        pool.evaluate(|_i, network| {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected) * (output - expected);
            }

            network.evaluate(4.0 - err);
        });
        pool.evolve(&mut innov_record);
    }
}
