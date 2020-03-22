use crate::ui::Input as VKInput;
use crate::{State, Transition};

#[derive(Debug)]
pub struct PauseState;

impl<TContext> State<TContext> for PauseState {
  fn handle_input(&mut self, _context: &mut TContext, input: VKInput) -> Transition<TContext> {
    match input {
      VKInput::TogglePause => Transition::Pop,
      _ => Transition::None,
    }
  }
}
