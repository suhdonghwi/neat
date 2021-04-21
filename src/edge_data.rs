#[derive(Debug, PartialEq, Clone, Default)]
pub struct EdgeData {
    weight: f64,
    disabled: bool,
    innov_number: usize,
}

impl EdgeData {
    pub fn new(weight: f64, innov_number: usize) -> Self {
        Self {
            weight,
            disabled: false,
            innov_number,
        }
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight;
    }

    pub fn get_weight(&self) -> f64 {
        self.weight
    }

    pub fn innov_number(&self) -> usize {
        self.innov_number
    }
}
