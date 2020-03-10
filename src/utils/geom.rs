use crate::components::Position;

#[allow(clippy::cast_precision_loss)]
#[allow(dead_code)]
pub fn sqr_dist(a: Position, b: Position) -> i32 {
  (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)
}

pub fn chebyshev_dist(a: Position, b: Position) -> i32 {
  std::cmp::max((b.x - a.x).abs(), (b.y - a.y).abs())
}
