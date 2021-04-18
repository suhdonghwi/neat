pub fn sigmoid(s: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-s))
}
