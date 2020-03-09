use super::response_curve::ResponseCurve;
use super::traits::Input;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Consideration<TInput> {
  pub name: String,
  input: TInput,
  response_curve: ResponseCurve,
}

impl<'a, TInput> Consideration<TInput>
where
  TInput: Input<'a>,
{
  pub fn score(&self, context: &TInput::Context) -> f32 {
    self.response_curve.evaluate(self.input.score(context))
  }
}
