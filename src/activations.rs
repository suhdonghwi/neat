pub fn sigmoid(s: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-s))
}

pub fn tanh(s: f64) -> f64 {
    s.tanh()
}

pub fn linear(v: f64) -> f64 {
    v
}
