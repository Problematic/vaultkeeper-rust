use crate::components::AIContext;
use serde::{Deserialize, Serialize};
use specs::{prelude::*, Component};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

pub trait Interaction: std::fmt::Debug {
  type State: Interactable;

  /// Called once when an agent begins interacting with this gadget
  fn start(&mut self, state: &mut Self::State, context: &mut AIContext);

  /// Called once per frame by an agent who is currently interacting
  /// with this gadget
  fn update(&mut self, state: &mut Self::State, context: &mut AIContext);

  /// Called once when an agent stops interacting with this gadget due
  /// to an action reselection or other interrupt. Called internally
  /// when an `Interactable` decides that a specific interaction is over
  fn stop(&mut self, state: &mut Self::State, context: &mut AIContext);
}

#[typetag::serde(tag = "type")]
pub trait Interactable: Send + Sync {
  fn tick(&mut self, _dt: Duration) {}

  fn evaluate(
    &mut self,
    context: &mut AIContext,
  ) -> Option<(Box<dyn Interaction<State = Self>>, f32)>
  where
    Self: Sized;
}

#[derive(Component, Serialize, Deserialize)]
pub struct InteractableComponent(pub Box<dyn Interactable>);

impl Deref for InteractableComponent {
  type Target = dyn Interactable;

  fn deref(&self) -> &Self::Target {
    &*self.0
  }
}

impl DerefMut for InteractableComponent {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut *self.0
  }
}
