use components::*;
use legion::prelude::*;

pub fn build_movement_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("movement")
    .with_query(<(Write<Position>, Write<Navigation>)>::query())
    .build(|_, mut world, _, query| {
      for (mut position, mut nav) in query.iter_mut(&mut world) {
        if let Some(pos) = nav.next() {
          *position = pos;
        }
      }
    })
}
