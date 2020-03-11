use specs::{prelude::*, Component};

#[derive(Component, Debug, Default)]
pub struct Blackboard {
  pub target: Option<Entity>,
}
