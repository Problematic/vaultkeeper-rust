use crate::components::{Navigation, Position};
use crate::utils;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use pathfinding::prelude::astar;
use specs::prelude::*;
use std::time::Instant;

#[derive(Default)]
pub struct PathfinderSystem;

impl<'a> System<'a> for PathfinderSystem {
  type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Navigation>);

  fn run(&mut self, (positions, mut navigations): Self::SystemData) {
    for (position, navigation) in (&positions, &mut navigations).join() {
      if navigation.needs_path() {
        let Navigation { goal, .. } = navigation;

        let start = Instant::now();
        let result = astar(
          position,
          successors,
          |p| utils::geom::chebyshev_dist(*position, *p),
          |p| goal.contains(p),
        );

        if let Some((path, cost)) = result {
          log::debug!(
            "Found path from {:?} to {:?} in {}ms (len={}, cost={})",
            position,
            goal,
            start.elapsed().as_millis(),
            path.len(),
            cost
          );

          navigation.path = path;
        } else {
          log::warn!(
            "Couldn't find path from {:?} to {:?}, clearing goal",
            position,
            goal
          );
          navigation.goal.clear();
        }
      }
    }
  }
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
