use crate::game::*;
use bracket_lib::prelude::*;

#[derive(Debug)]
pub struct PauseState;

impl VaultkeeperState for PauseState {
  fn update(&mut self, term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    if let Some(VirtualKeyCode::Space) = term.key {
      Transition::Pop
    } else {
      Transition::None
    }
  }
}
