use crate::components::{actions::MoveAction, tags::Monster, Direction, Position};
use crate::resources::WorldMap;
use legion::prelude::*;
use rand::{rngs::StdRng, Rng};

pub fn build_wander_ai_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("wander_ai")
    .with_query(Read::<Position>::query().filter(tag::<Monster>() & !component::<MoveAction>()))
    .read_resource::<WorldMap>()
    .write_resource::<StdRng>()
    .build(|cmd, world, (map, rng), query| {
      for (entity, pos) in query.iter_entities(&world) {
        // TODO: find a nearby destination to meander toward
        let direction: Direction = rng.gen();
        let dest = *pos + direction.as_delta_pos();
        if map[dest].is_walkable() {
          cmd.add_component(entity, MoveAction::new(direction));
        }
      }
    })
}
