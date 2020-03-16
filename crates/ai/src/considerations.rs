use super::AIContext;

pub trait AIConsideration: Send + Sync + std::fmt::Debug {
  fn score(&self, _context: &mut AIContext) -> f32;
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
