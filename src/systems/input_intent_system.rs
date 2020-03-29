use crate::components::{actions::*, markers::HasAction, Input, Navigation, Position, Target};
use crate::resources::WorldMap;
use legion::prelude::*;

pub fn build_input_intent_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("input->intent")
    .with_query(<(Write<Option<Input>>, Read<Position>, Write<Navigation>)>::query())
    .read_resource::<WorldMap>()
    .build(|cmd, mut world, map, query| {
      for (entity, (mut player_input, pos, mut nav)) in query.iter_entities_mut(&mut world) {
        if let Some(input) = *player_input {
          use Input::*;

          let handled = match input {
            Move(direction) => {
              let dest = *pos + direction.as_delta_pos();
              if map[dest].occupant.is_some() {
                cmd.add_component(entity, AttackAction::new(direction));
                cmd.add_component(entity, HasAction);
              } else {
                nav.goal = Some(Target::Position(dest));
                nav.path.clear();
                nav.path.push_front(dest);
              }

              true
            }
            Wait => {
              cmd.add_component(entity, WaitAction);
              cmd.add_component(entity, HasAction);

              true
            }
            _ => false,
          };

          if handled {
            *player_input = None;
          }
        }
      }
    })
}
