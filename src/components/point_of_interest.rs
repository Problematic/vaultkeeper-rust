use super::Need;
use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct PointOfInterest {
  pub need: Need,
  pub range: i32,
  pub is_global: bool,
}
