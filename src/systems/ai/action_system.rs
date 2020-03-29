use crate::components::{Intent, Navigation, Target};
use legion::prelude::*;

pub fn build_action_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("ai_action")
    .with_query(<(Read<Intent>, Write<Navigation>)>::query())
    .build(|_, mut world, _, query| {
      for (intent, mut navigation) in query.iter_mut(&mut world) {
        match *intent {
          Intent::Attack(entity) => {
            if navigation.goal.is_none() {
              navigation.goal = Some(Target::Entity(entity));
            }
          }
          _ => todo!(),
        }
      }
    })
}
