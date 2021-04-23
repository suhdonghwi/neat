use neatlib::network::feedforward::Feedforward;
use neatlib::{innovation_record::InnovationRecord, network::Network};

fn main() {
    let input_number = 2;
    let output_number = 1;

    let mut innov_record = InnovationRecord::new(input_number, output_number);
    let mut population: Vec<Feedforward> = Vec::new();

    for _ in 0..100 {
        let mut network = Feedforward::new(input_number, output_number, &mut innov_record);
        network.randomize_weights(-1.0, 1.0);
        population.push(network);
    }

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    for mut network in population {
        let mut err: f64 = 0.0;

        for (inputs, expected) in &data {
            let output = network.activate(inputs).unwrap()[0];
            err += (expected - output).powf(2.0);
        }

        network.evaluate(err);
    }
}
