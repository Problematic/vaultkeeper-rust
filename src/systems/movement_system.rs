use crate::components::{Energy, Navigation, Position};
use crate::resources::WorldMap;
use legion::prelude::*;

pub fn build_movement_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("movement")
    .with_query(<(Write<Energy>, Write<Position>, Write<Navigation>)>::query())
    .write_resource::<WorldMap>()
    .build(|_, mut world, map, query| {
      for (entity, (mut energy, mut position, mut nav)) in query.iter_entities_mut(&mut world) {
        if !energy.can_act() || nav.goal.is_none() {
          continue;
        }

        energy.spend();

        if let Some(dest) = nav.next() {
          if map[dest].is_walkable() && !map[dest].is_occupied() {
            map[*position].occupant = None;
            *position = dest;
            map[dest].occupant = Some(entity);
          }
        }

        if nav.is_empty() {
          nav.goal = None;
        }
      }
    })
}
