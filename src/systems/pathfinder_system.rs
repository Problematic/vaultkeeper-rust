use crate::components::{Intent, Memory, Name, Navigation, Position, Target};
use crate::resources::WorldMap;
use legion::prelude::*;
use pathfinding::prelude::{absdiff, astar};

pub fn build_pathfinder_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("pathfinder")
    .read_resource::<WorldMap>()
    .with_query(<(
      Read<Name>,
      Read<Position>,
      Read<Memory>,
      Write<Navigation>,
    )>::query())
    .build(|cmd, mut world, map, query| {
      for (entity, (name, pos, memory, mut nav)) in query.iter_entities_mut(&mut world) {
        let goal = match &*nav {
          Navigation {
            goal: Some(Target::Entity(target)),
            path,
          } if !path.is_empty() => {
            if let Some(target_pos) = memory.encounters.get(target) {
              if target_pos == &*pos {
                log::warn!("{} lost your scent.", *name);
                nav.goal = None;
                cmd.remove_component::<Intent>(entity);
                None
              } else if path.back() != Some(target_pos) {
                // TODO: cooldown on how often we can re-path due to our target moving?
                Some(*target_pos)
              } else {
                None
              }
            } else {
              None
            }
          }
          Navigation {
            goal: Some(goal),
            path,
          } if path.is_empty() => match goal {
            Target::Position(position) => Some(*position),
            Target::Entity(entity) => world.get_component::<Position>(*entity).as_deref().copied(),
          },
          _ => None,
        };

        if let Some(dest) = goal {
          if dest == *pos {
            log::warn!("Already at {:?}; bailing before pathfinding", dest);
            continue;
          }
          let result = astar(
            &*pos,
            |&p| {
              map
                .get_neighbors(p)
                .iter()
                .map(|&n| {
                  let cost = if map[n].is_occupied() {
                    // if a tile is occupied, it will *probably* be
                    // empty by the time we get there, so we just
                    // treat it as costly
                    10
                  } else if is_orthogonal(p, n) {
                    3
                  } else {
                    4
                  };
                  (n, cost)
                })
                .collect::<Vec<(Position, i32)>>()
            },
            |&p| absdiff(p.x, dest.x) + absdiff(p.y, dest.y),
            |&p| dest == p,
          );

          if let Some((path, _)) = result {
            nav.path.clear();
            nav.path.extend(&path[1..]);
          } else {
            log::warn!("Couldn't find a path from {:?} to {:?}", *pos, dest);
          }
        }
      }
    })
}

fn is_orthogonal(from: Position, to: Position) -> bool {
  absdiff(from.x, to.x) == 0 || absdiff(from.y, to.y) == 0
}
