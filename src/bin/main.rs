use neatlib::{network::feedforward::Feedforward, pool::Pool};

fn main() {
    let input_number = 2;
    let output_number = 1;

    let pool = Pool::<Feedforward>::new(input_number, output_number, 100);
}
