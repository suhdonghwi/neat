use neatlib::network::Network;
use neatlib::{network::feedforward::Feedforward, pool::Pool};

fn main() {
    let input_number = 2;
    let output_number = 1;

    let mut pool = Pool::<Feedforward>::new(input_number, output_number, 100);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for network in pool.networks() {
        let mut err = 0.0;

        for (inputs, expected) in &data {
            let output = network.activate(inputs).unwrap()[0];
            err += (output - expected).powf(2.0);
        }

        network.evaluate(4.0 - err);
    }

    pool.evolve();
}
