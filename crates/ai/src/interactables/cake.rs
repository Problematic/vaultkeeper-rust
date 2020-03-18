use legion::prelude::*;
use std::time::Duration;
use vaultkeeper_shared::Time;

#[derive(Debug, Clone, Copy)]
pub struct Cake {
  pub slices_remaining: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct EatCakeInteraction {
  interactable: Entity,
  agent: Entity,
  time_remaining: Duration,
}

pub fn build_system() -> Box<dyn Schedulable> {
  SystemBuilder::new("cake_interactable")
    .read_resource::<Time>()
    .with_query(<Write<Cake>>::query())
    .with_query(<Write<EatCakeInteraction>>::query())
    .write_component::<Cake>()
    .build(|cmd, mut world, time, queries| {
      for (entity, mut interaction) in queries.1.iter_entities_mut(&mut world) {
        if let Some(remaining) = interaction.time_remaining.checked_sub(time.dt) {
          interaction.time_remaining = remaining;
        } else {
          if let Some(mut cake) = world.get_component_mut::<Cake>(interaction.interactable) {
            cake.slices_remaining = cake.slices_remaining.saturating_sub(1);
          }
          cmd.delete(entity);
        }
      }
    })
}
