use crate::components::Position;
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
  Northwest,
}

impl Direction {
  pub fn as_delta_pos(self) -> Position {
    use Direction::*;

    match self {
      North => Position::new(0, -1),
      Northeast => Position::new(1, -1),
      East => Position::new(1, 0),
      Southeast => Position::new(1, 1),
      South => Position::new(0, 1),
      Southwest => Position::new(-1, 1),
      West => Position::new(-1, 0),
      Northwest => Position::new(-1, -1),
    }
  }
}

impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    use Direction::*;

    match rng.next_u32() % 8 {
      0 => North,
      1 => Northeast,
      2 => East,
      3 => Southeast,
      4 => South,
      5 => Southwest,
      6 => West,
      7 => Northwest,
      _ => unreachable!(),
    }
  }
}
