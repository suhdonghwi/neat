use crate::{innovation_record::InnovationRecord, network::Network};

pub struct Pool<T: Network> {
    list: Vec<T>,
    innov_record: InnovationRecord,
}

impl<T: Network> Pool<T> {
    pub fn new(input_number: usize, output_number: usize, population: usize) -> Self {
        let mut list: Vec<T> = Vec::new();
        let mut innov_record = InnovationRecord::new(input_number, output_number);

        for _ in 0..population {
            list.push(T::new(input_number, output_number, &mut innov_record));
        }

        Self { list, innov_record }
    }

    pub fn networks(&mut self) -> impl Iterator<Item = &mut T> {
        self.list.iter_mut()
    }
}
