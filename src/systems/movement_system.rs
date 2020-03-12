use crate::components::{Navigation, Position};
use specs::prelude::*;

#[derive(Default)]
pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
  type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Navigation>);

  fn run(&mut self, (mut positions, mut navigations): Self::SystemData) {
    for (position, navigation) in (&mut positions, &mut navigations).join() {
      if let Some(pos) = navigation.next() {
        *position = pos;
      }
    }
  }
}
