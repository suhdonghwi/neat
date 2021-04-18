use neatlib::network_internal::NetworkInternal;

fn main() {
    let network = NetworkInternal::new(10, 1);
    println!("{:#?}", network);
}
