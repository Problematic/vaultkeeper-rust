use crate::components::{actions::*, Input, Position};
use crate::resources::WorldMap;
use legion::prelude::*;

pub fn build_input_intent_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("input->intent")
    .with_query(<(Read<Option<Input>>, Read<Position>)>::query())
    .read_resource::<WorldMap>()
    .build(|cmd, world, map, query| {
      for (entity, (input, pos)) in query.iter_entities(&world) {
        if let Some(input) = *input {
          use Input::*;

          #[allow(clippy::single_match)]
          match input {
            Move(direction) => {
              let dest = *pos + direction.as_delta_pos();
              if map[dest].occupant.is_some() {
                cmd.add_component(entity, AttackAction::new(direction));
              } else {
                cmd.add_component(entity, MoveAction::new(direction));
              }
            }
            _ => {}
          }
        }
      }
    })
}
