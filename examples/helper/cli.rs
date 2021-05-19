use clap::{clap_app, value_t};

pub struct Arguments {
    pub verbosity: usize,
}

pub fn get_arguments() -> Arguments {
    let matches = clap_app!(neat =>
        (version: "0.1")
        (author: "Suh Donghwi <hwidongsuh@gmail.com>")
        (about: "NEAT(NeuroEvolution of Augmenting Topologies) implementation written in Rust")
        (@arg VERBOSITY: -v --verbosity +takes_value default_value("0") possible_values(&["0", "1", "2"]) "Sets verbosity of log")
    )
    .get_matches();

    let verbosity = value_t!(matches.value_of("VERBOSITY"), usize).unwrap();

    Arguments { verbosity }
}
