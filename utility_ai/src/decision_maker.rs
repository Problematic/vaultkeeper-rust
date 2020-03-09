use super::decision::Decision;
use super::traits::Input;
use serde::{Deserialize, Serialize};
use utils::Weighted;

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionMaker<TInput, TAction> {
  decisions: Vec<Weighted<Decision<TInput, TAction>>>,
}

impl<'a, TInput, TAction> DecisionMaker<TInput, TAction>
where
  TInput: Input<'a>,
  TAction: Copy,
{
  #[must_use]
  pub fn evaluate(&self, context: &TInput::Context) -> (Option<&Decision<TInput, TAction>>, f32) {
    let mut threshold = 0.0;
    let mut selected = None;

    for Weighted {
      item: decision,
      weight,
    } in &self.decisions
    {
      if *weight <= threshold {
        continue;
      }

      let score = decision.score(context, *weight);
      log::debug!("{}: {}", decision.name, score);

      if score > threshold {
        threshold = score;
        selected = Some(decision);
      }
    }

    (selected, threshold)
  }
}
