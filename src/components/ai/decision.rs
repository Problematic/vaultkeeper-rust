use super::{AIAction, AIConsideration, AIContext};

#[derive(Debug)]
pub struct Decision {
  pub name: String,
  pub weight: f32,
  pub action: AIAction,
  pub considerations: Vec<Box<dyn AIConsideration>>,
}

impl Decision {
  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &AIContext) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let mut result = self.weight;
    for consideration in &self.considerations {
      let score = consideration.score(context);

      result *= score;
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - result) * mod_factor;

    result + (make_up_value * result)
  }
}

impl std::ops::Deref for Decision {
  type Target = AIAction;

  fn deref(&self) -> &Self::Target {
    &self.action
  }
}
