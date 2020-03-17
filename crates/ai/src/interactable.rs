use crate::Decision;

#[derive(Debug)]
pub struct Interactable {
  pub actions: Vec<Decision>,
}

impl Interactable {
  pub fn evaluate(&self) -> Option<(&Decision, f32)> {
    None
  }
}
