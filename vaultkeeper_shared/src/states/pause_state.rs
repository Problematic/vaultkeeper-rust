use crate::ui::Input as VKInput;
use crate::{State, StateContext, Transition};

#[derive(Debug)]
pub struct PauseState;

impl<TData> State<TData> for PauseState {
  fn handle_input(&mut self, _context: StateContext<TData>, input: VKInput) -> Transition<TData> {
    match input {
      VKInput::TogglePause => Transition::Pop,
      _ => Transition::None,
    }
  }
}
