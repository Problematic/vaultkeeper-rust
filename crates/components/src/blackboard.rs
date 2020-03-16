use specs::{prelude::*, Component};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Component, Debug, Default)]
pub struct Blackboard {
  pub target: Option<Entity>,
  pub cooldowns: HashMap<&'static str, Duration>,
}
