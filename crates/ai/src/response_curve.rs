use serde::{Deserialize, Serialize};
use utils;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCurve {
  Linear,
  InverseLinear,
  CustomLinear(f32, f32, f32, f32),
  Polynomial(f32, f32, f32, f32),
  Logistic,
  InverseLogistic,
  CustomLogistic(f32, f32, f32, f32),
  Logit,
  InverseLogit,
  CustomLogit(f32, f32, f32, f32),
  Normal(f32, f32, f32, f32),
  Sine(f32, f32, f32, f32),
}

impl ResponseCurve {
  #[allow(clippy::many_single_char_names)]
  pub fn evaluate(&self, x: f32) -> f32 {
    let x = utils::clamp(x, 0.0, 1.0);

    let y = match self {
      Self::Linear => x,
      Self::InverseLinear => -x + 1.0,
      Self::CustomLinear(m, _k, b, c) => m * (x - c) + b,
      Self::Polynomial(m, k, b, c) => m * (x - c).powf(*k) + b,
      Self::Logistic => 1.0 / (1.0 + (-10.0 * (x - 0.5)).exp()),
      Self::InverseLogistic => -1.0 / (1.0 + (-10.0 * (x - 0.5)).exp()) + 1.0,
      Self::CustomLogistic(m, k, b, c) => (m / (-10.0 * k * (x - 0.5 - c)).exp()) + b,
      Self::Logit => (x / (1.0 - x).ln()) / 5.0 + 0.5,
      Self::InverseLogit => ((1.0 - x) / x).ln() / 5.0 + 0.5,
      Self::CustomLogit(m, _k, b, c) => m * ((x - c) / (1.0 - (x - c))).ln() / 5.0 + 0.5 + b,
      Self::Normal(m, k, b, c) => m * (-30.0 * k * (x - 0.5 - c) * (x - 0.5 - c)).exp() + b,
      Self::Sine(m, _k, c, b) => 0.5 * m * (2.0 * std::f32::consts::PI * (x - c)).sin() + 0.5 + b,
    };

    utils::clamp(y, 0.0, 1.0)
  }
}
