use super::State;

#[allow(dead_code)]
pub enum Transition<T> {
  None,
  Push(Box<dyn State<T>>),
  Switch(Box<dyn State<T>>),
  Pop,
  Quit,
}
