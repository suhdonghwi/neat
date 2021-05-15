use std::fs;

use neatlib::{network::feedforward::Feedforward, pool::Pool};
use neatlib::{network::Network, parameters::Parameters};

fn main() {
    env_logger::init();

    let params_file_path = "./params/xor.toml";
    let params_str;
    if let Ok(str) = fs::read_to_string(params_file_path) {
        params_str = str;
    } else {
        panic!("Couldn't read params file path: {}", params_file_path);
    }

    let params: Parameters = toml::from_str(&params_str).unwrap();
    let mut pool = Pool::<Feedforward>::new(params);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    pool.evolve(300, 3.9, |networks| {
        for network in networks {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected) * (output - expected);
            }

            network.evaluate(4.0 - err);
        }
    });
}
