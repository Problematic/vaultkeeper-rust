use super::consideration::Consideration;
use super::traits::Input;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision<TInput, TAction> {
  pub name: String,
  pub considerations: Vec<Consideration<TInput>>,
  action: TAction,
}

impl<'a, TInput, TAction> Decision<TInput, TAction>
where
  TInput: Input<'a>,
  TAction: Copy,
{
  pub fn action(&self) -> TAction {
    self.action
  }

  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &TInput::Context, initial: f32) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let mut result = initial;
    for consideration in &self.considerations {
      let score = consideration.score(context);
      log::debug!("{}: {}", consideration.name, score);

      result *= consideration.score(context);
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - result) * mod_factor;

    result + (make_up_value * result)
  }
}
