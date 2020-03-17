use crate::*;
use components::*;
use legion::prelude::*;

pub fn build_action_selection_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("action_selection")
    .with_query(<(Read<Position>, Read<Viewshed>, Write<Option<Action>>)>::query())
    .with_query(<(
      Read<Position>,
      Read<Interactable>,
      Read<Option<PointOfInterest>>,
    )>::query())
    .build(|_, mut _world, _, _query| {})
}
