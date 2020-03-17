use super::PauseState;
use crate::game::*;
use bracket_lib::prelude::*;

#[derive(Default, Debug)]
pub struct DelveState;

impl VaultkeeperState for DelveState {
  fn update(&mut self, term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    if let Some(VirtualKeyCode::Space) = term.key {
      return Transition::Push(Box::new(PauseState));
    }

    Transition::None
  }
}
