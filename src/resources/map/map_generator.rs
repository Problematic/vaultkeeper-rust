use super::WorldMap;
use rand::RngCore;

pub trait MapGenerator {
  fn build(&mut self, rng: &mut dyn RngCore) -> WorldMap;
}
