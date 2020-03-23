pub mod components;
pub mod states;

pub trait Action {}

pub struct StateData {
  pub action: Option<Box<dyn Action>>,
}
