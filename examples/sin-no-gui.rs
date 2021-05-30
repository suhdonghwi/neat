#![recursion_limit = "512"]
mod helper;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

pub fn main() {
    let args = helper::cli::get_arguments();
    let params = helper::read_parameters_file("./params/sin.toml");

    let mut innov_record = InnovationRecord::new(params.input_number, params.output_number);
    let mut pool = Pool::<Feedforward>::new(params, args.verbosity, &mut innov_record);

    for _ in 0..100 {
        pool.evaluate(|_, network| {
            let n = 50;
            let mut error_sum = 0.0;

            for i in -n..=n {
                let x = i as f64 / n as f64;

                let output = network.activate(&[x]).unwrap()[0];
                let expected = (x * std::f64::consts::PI).sin();
                let err = output - expected;

                error_sum += err * err;
            }

            let error_mean = error_sum / (n * 2 + 1) as f64;
            network.evaluate(4.0 - error_mean);
        });
        pool.evolve(&mut innov_record);
    }
}
