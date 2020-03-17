use super::{Consideration, Context};
use std::borrow::Cow;

pub struct Decision {
  pub name: Cow<'static, str>,
  pub weight: f32,
  pub considerations: Vec<Box<dyn Consideration>>,
}

impl Decision {
  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &mut dyn Context) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let mut result = self.weight;
    for consideration in &self.considerations {
      if result <= 0.0 {
        break;
      }

      let score = consideration.score(context);

      log::debug!("\u{2502}    {:?} => {}", consideration, score);

      result *= score;
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - result) * mod_factor;

    result + (make_up_value * result)
  }
}

use std::fmt;
impl fmt::Debug for Decision {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}
