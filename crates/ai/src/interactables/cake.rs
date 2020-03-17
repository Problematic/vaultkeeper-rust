use legion::prelude::*;
use resources::DeltaTime;
use std::time::Duration;

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
    .read_resource::<DeltaTime>()
    .with_query(<Write<Cake>>::query())
    .with_query(<Write<EatCakeInteraction>>::query())
    .write_component::<Cake>()
    .build(|cmd, mut world, delta_time, queries| {
      for (entity, mut interaction) in queries.1.iter_entities_mut(&mut world) {
        if let Some(remaining) = interaction
          .time_remaining
          .checked_sub(delta_time.as_duration())
        {
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
