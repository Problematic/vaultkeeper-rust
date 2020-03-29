use crate::components::{actions::WaitAction, markers::HasAction, Energy};
use legion::prelude::*;

pub fn build_wait_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("wait")
    .with_query(<Write<Energy>>::query().filter(component::<WaitAction>()))
    .build(|cmd, mut world, _, query| {
      for (entity, mut energy) in query.iter_entities_mut(&mut world) {
        if !energy.can_act() {
          continue;
        }

        energy.spend();

        cmd.remove_component::<WaitAction>(entity);
        cmd.remove_component::<HasAction>(entity);
      }
    })
}
