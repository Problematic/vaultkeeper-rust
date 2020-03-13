use super::{AIContext, Need, ResponseCurve};
use crate::utils;
use specs::prelude::*;
use std::time::Duration;

pub trait AIConsideration: Send + Sync + std::fmt::Debug {
  fn score(&self, _context: &mut AIContext) -> f32;
}

#[derive(Debug)]
pub struct NeedConsideration {
  pub need: Need,
}

impl AIConsideration for NeedConsideration {
  fn score(&self, context: &mut AIContext) -> f32 {
    ResponseCurve::InverseLinear
      .evaluate(*context.agent.needs.0.get(&self.need).unwrap_or(&0.0) / 100.0)
  }
}

#[derive(Debug)]
pub struct ConstantConsideration {
  pub value: f32,
}

impl AIConsideration for ConstantConsideration {
  fn score(&self, _: &mut AIContext) -> f32 {
    self.value
  }
}

#[derive(Debug)]
pub struct DistanceToInterestConsideration {
  pub need: Need,
}

impl AIConsideration for DistanceToInterestConsideration {
  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &mut AIContext) -> f32 {
    (context.points_of_interest, context.positions)
      .join()
      .filter_map(|(poi, pos)| {
        if poi.need == self.need {
          Some(context.distance_to_pos(*pos))
        } else {
          None
        }
      })
      .min()
      .map_or(0.0, |dist| {
        ResponseCurve::CustomLinear(-1.0, 1.0, 1.1, 0.0)
          .evaluate(dist as f32 / context.agent.perception.range as f32)
      })
  }
}

#[derive(Debug)]
pub struct NearTargetConsideration {
  pub max_distance: f32,
}

impl AIConsideration for NearTargetConsideration {
  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &mut AIContext) -> f32 {
    if let Some(target) = context.blackboard.target {
      if let Some(pos) = context.positions.get(target) {
        return ResponseCurve::CustomLinear(-1.0, 1.0, 1.1, 0.0)
          .evaluate(context.distance_to_pos(*pos) as f32 / self.max_distance);
      }
    }

    0.0
  }
}

#[derive(Debug)]
pub struct NearestPOIPicker {
  pub need: Need,
}

impl AIConsideration for NearestPOIPicker {
  fn score(&self, context: &mut AIContext) -> f32 {
    let result = (
      context.entities,
      context.points_of_interest,
      context.positions,
    )
      .join()
      .filter(|(_entity, poi, pos)| {
        poi.need == self.need
          && (poi.is_global || context.distance_to_pos(**pos) <= context.agent.perception.range)
      })
      .min_by_key(|(_entity, _poi, pos)| context.distance_to_pos(**pos));

    if let Some((entity, _, _)) = result {
      context.blackboard.target = Some(entity);
      1.0
    } else {
      0.0
    }
  }
}

#[derive(Debug)]
pub struct CooldownConsideration {
  key: &'static str,
  duration: Duration,
}

impl CooldownConsideration {
  pub fn new(key: &'static str, duration: Duration) -> Self {
    Self { key, duration }
  }
}

impl AIConsideration for CooldownConsideration {
  fn score(&self, context: &mut AIContext) -> f32 {
    let remaining = context.blackboard.cooldowns.entry(self.key).or_default();

    if *remaining == utils::ZERO_DURATION {
      context.blackboard.cooldowns.insert(self.key, self.duration);
      1.0
    } else {
      0.0
    }
  }
}
