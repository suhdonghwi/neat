use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum ActivationKind {
    Sigmoid,
    Tanh,
    Linear,
}

pub fn activate(kind: ActivationKind, v: f64) -> f64 {
    match kind {
        ActivationKind::Sigmoid => sigmoid(v),
        ActivationKind::Tanh => tanh(v),
        ActivationKind::Linear => linear(v),
    }
}

pub fn sigmoid(s: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-s))
}

pub fn tanh(s: f64) -> f64 {
    s.tanh()
}

pub fn linear(v: f64) -> f64 {
    v
}
