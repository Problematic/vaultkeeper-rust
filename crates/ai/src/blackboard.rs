use legion::prelude::Entity;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Blackboard {
  pub target: Option<Entity>,
  pub cooldowns: HashMap<&'static str, Duration>,
}
