#![recursion_limit = "512"]
mod helper;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

pub fn main() {
    println!("<Case Start>");

    let args = helper::cli::get_arguments();
    let params = helper::read_parameters_file("./params/xor.toml");

    let mut innov_record = InnovationRecord::new(2, 1);
    let mut pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for _ in 0..100 {
        let best_genome = pool.evaluate(|_, network| {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected) * (output - expected);
            }

            network.evaluate(4.0 - err);
        });

        if best_genome.fitness().unwrap() > 3.9 {
            break;
        }
        pool.evolve(&mut innov_record);
    }
}
