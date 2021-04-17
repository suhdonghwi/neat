use neatlib::network::Network;

fn main() {
    let network = Network::new(10, 1);
    println!("{:#?}", network);
}
