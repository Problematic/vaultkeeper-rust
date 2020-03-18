use super::State;

#[allow(dead_code)]
pub enum Transition {
  None,
  Push(Box<dyn State>),
  Switch(Box<dyn State>),
  Pop,
  Quit,
}
