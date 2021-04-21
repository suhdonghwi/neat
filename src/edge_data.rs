#[derive(Debug, PartialEq, Clone, Default)]
pub struct EdgeData {
    weight: f64,
    disabled: bool,
}

impl EdgeData {
    pub fn new(weight: f64) -> Self {
        Self {
            weight,
            disabled: false,
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
}
