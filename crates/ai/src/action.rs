use super::{AIContext, Decision, Position};
use legion::prelude::Entity;
use ordered_float::NotNan;
use std::borrow::Cow;

pub trait Action {
  fn select(&mut self, context: &mut AIContext);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AIInterest {
  Entity(Entity),
  Position(Position),
  BlackboardTarget,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AIAction {
  Goto(AIInterest),
  Interact(Entity),
}

#[derive(Debug)]
pub struct CurrentAction(pub AIAction);

use std::ops;

impl ops::Deref for CurrentAction {
  type Target = AIAction;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
