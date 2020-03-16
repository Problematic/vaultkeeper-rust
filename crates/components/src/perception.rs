use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct Perception {
  pub range: i32,
}
