use crate::{State, Transition, WorldContext};
use bracket_lib::prelude::*;

#[derive(Debug)]
pub struct PauseState;

impl State for PauseState {
  fn update(&mut self, term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    if let Some(VirtualKeyCode::Space) = term.key {
      Transition::Pop
    } else {
      Transition::None
    }
  }
}
