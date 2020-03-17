use super::Context;

pub trait Consideration: Send + Sync + std::fmt::Debug {
  fn score(&self, context: &mut dyn Context) -> f32;
}
