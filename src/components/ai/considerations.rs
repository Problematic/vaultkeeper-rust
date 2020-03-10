use super::AIContext;
use crate::components::ai::{AIInterest, Need, ResponseCurve};

pub trait AIConsideration: Send + Sync + std::fmt::Debug {
  fn score(&self, _context: &AIContext) -> f32;
}

#[derive(Debug)]
pub struct NeedConsideration {
  pub need: Need,
}

impl AIConsideration for NeedConsideration {
  fn score(&self, context: &AIContext) -> f32 {
    ResponseCurve::InverseLinear
      .evaluate(*context.agent.needs.0.get(&self.need).unwrap_or(&0.0) / 100.0)
  }
}

#[derive(Debug)]
pub struct ConstantConsideration {
  pub value: f32,
}

impl AIConsideration for ConstantConsideration {
  fn score(&self, _: &AIContext) -> f32 {
    self.value
  }
}

#[derive(Debug)]
pub struct DistanceToInterestConsideration {
  pub interest: AIInterest,
}

impl AIConsideration for DistanceToInterestConsideration {
  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &AIContext) -> f32 {
    if let Some(dist) = (context.distance_to_interest)(self.interest) {
      ResponseCurve::CustomLinear(-1.0, 1.0, 1.1, 0.0).evaluate(dist)
    } else {
      0.0
    }
  }
}
