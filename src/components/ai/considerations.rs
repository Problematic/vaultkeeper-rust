use super::AIContext;
use crate::components::ai::{AIInterest, Need, ResponseCurve};
use crate::utils;
use specs::prelude::*;

pub trait AIConsideration: Send + Sync + std::fmt::Debug {
  fn score(&self, _context: &AIContext) -> f32;
}

#[derive(Debug)]
pub struct NeedConsideration {
  pub need: Need,
}

impl AIConsideration for NeedConsideration {
  fn score(&self, context: &AIContext) -> f32 {
    // ResponseCurve::InverseLinear
    //   .evaluate(*context.agent.needs.0.get(&self.need).unwrap_or(&0.0) / 100.0)

    0.0
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
  pub need: Need,
}

impl AIConsideration for DistanceToInterestConsideration {
  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &AIContext) -> f32 {
    (context.points_of_interest, context.positions)
      .join()
      .filter(|(poi, _pos)| poi.need == self.need)
      .map(|(_poi, pos)| utils::geom::chebyshev_dist(*context.agent.position, *pos))
      .min()
      .map_or(0.0, |dist| {
        ResponseCurve::CustomLinear(-1.0, 1.0, 1.1, 0.0)
          .evaluate(dist as f32 / context.agent.perception.range as f32)
      })
  }
}
