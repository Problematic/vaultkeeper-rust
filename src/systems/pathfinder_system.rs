use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use components::utils::chebyshev_dist;
use components::*;
use legion::prelude::*;
use pathfinding::prelude::astar;
use std::time::Instant;

pub fn build_pathfinder_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("pathfinder")
    .with_query(<(Read<Position>, Write<Navigation>)>::query())
    .build(|_, mut world, _, query| {
      for (position, mut navigation) in query.iter_mut(&mut world) {
        match &*navigation {
          Navigation {
            goal: Some(goal),
            path,
          } if path.is_empty() => {
            let start = Instant::now();
            let result = astar(
              &*position,
              successors,
              |p| chebyshev_dist(*position, *p),
              |p| goal.contains(*p),
            );
            if let Some((path, cost)) = result {
              log::debug!(
                "Found path from {:?} to {:?} in {}\u{3bc}s (len={}, cost={})",
                position,
                goal,
                start.elapsed().as_micros(),
                path.len(),
                cost
              );
              navigation.path.clear();
              navigation.path.extend(path);
            } else {
              log::warn!(
                "Couldn't find path from {:?} to {:?}, clearing goal",
                position,
                goal
              );
              navigation.goal = None;
            }
          }
          _ => {}
        }
      }
    })
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn successors(pos: &Position) -> Vec<(Position, i32)> {
  let mut neighborhood = Vec::new();

  for dx in -1..=1 {
    for dy in -1..=1 {
      if dx == 0 && dy == 0 {
        continue;
      }

      let x = pos.x + dx;
      let y = pos.y + dy;

      if x < 0 || y < 0 || x >= WINDOW_WIDTH || y >= WINDOW_HEIGHT {
        continue;
      }

      let cost = if dx == 0 || dy == 0 { 2 } else { 3 };

      neighborhood.push((Position::new(x, y), cost));
    }
  }

  neighborhood
}
