use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cake {
  pub slices_remaining: u8,
}

#[typetag::serde]
impl Interactable for Cake {
  fn evaluate(
    &mut self,
    _context: &mut AIContext,
  ) -> Option<(Box<dyn Interaction<State = Self>>, f32)> {
    None
  }
}

#[derive(Debug, Clone, Copy)]
pub struct EatCakeInteraction {}

impl Interaction for EatCakeInteraction {
  type State = Cake;

  fn start(&mut self, _: &mut Self::State, _: &mut AIContext) {
    log::info!("om...");
  }

  fn update(&mut self, _: &mut Self::State, _: &mut AIContext) {
    log::info!("... nom ...");
  }

  fn stop(&mut self, state: &mut Self::State, _: &mut AIContext) {
    log::info!("... glomph!");
    state.slices_remaining -= 1;
  }
}
