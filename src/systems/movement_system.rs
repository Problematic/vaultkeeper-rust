use crate::components::{actions::MoveAction, Energy, Position};
use crate::resources::WorldMap;
use legion::prelude::*;

pub fn build_movement_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("movement")
    .with_query(<(Read<MoveAction>, Write<Energy>, Write<Position>)>::query())
    .write_resource::<WorldMap>()
    .build(|cmd, mut world, map, query| {
      for (entity, (action, mut energy, mut pos)) in query.iter_entities_mut(&mut world) {
        if !energy.can_act() {
          continue;
        }

        let dest = *pos + action.direction.as_delta_pos();
        if map[dest].is_walkable() {
          map[*pos].occupant = None;
          *pos = dest;
          map[dest].occupant = Some(entity);
        }

        cmd.remove_component::<MoveAction>(entity);
        energy.spend();
      }
    })
}
