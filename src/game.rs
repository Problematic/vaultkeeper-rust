use crate::components::{tags::Player, Input, *};
use crate::resources::{
  map::{generators::BSPMapGenerator, MapGenerator, WorldMap},
  GameTime, Keymap,
};
use crate::systems::*;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bracket_lib::prelude::*;
use legion::prelude::*;
use rand::{
  distributions::{Distribution, Uniform},
  rngs::StdRng,
  seq::{IteratorRandom, SliceRandom},
};
use std::collections::HashSet;

pub struct Game {
  pub resources: Resources,
  pub world: World,
  pub schedule: Option<Schedule>,
}

impl Game {
  pub fn init(&mut self) {
    let schedule = Schedule::builder()
      .add_system(build_input_intent_system())
      .add_system(build_energy_system())
      .add_system(build_wander_ai_system())
      .flush()
      .add_system(build_attack_system())
      .add_system(build_movement_system())
      .flush()
      .add_system(build_visibility_system())
      .build();
    self.schedule = Some(schedule);

    let map = BSPMapGenerator::new()
      .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
      .with_impassible_borders(true)
      .with_iterations(4)
      .with_partition_jitter(0.1)
      .with_room_size(0.5)
      .with_min_room_size((3, 3))
      .with_filled(true)
      .build(&mut *self.resources.get_mut::<StdRng>().unwrap());
    self.resources.insert(map);

    let map = &mut self.resources.get_mut::<WorldMap>().unwrap();
    let start_room = map
      .rooms
      .choose(&mut *self.resources.get_mut::<StdRng>().unwrap())
      .unwrap();
    let start_pos = start_room.center();

    self.world.insert(
      (tags::Mobile, tags::Player),
      vec![(
        start_pos,
        Appearance::new('@', WHITE, BLACK),
        Energy::new(Energy::ACTION_COST),
        Speed(60),
        Health::new(100),
        None::<Input>,
        Viewshed::new(8),
      )],
    );

    let monsters = [
      ('a', LIGHTBLUE, Speed(60), Health::new(10)),
      ('a', CORNSILK, Speed(80), Health::new(15)),
      ('c', DEEPSKYBLUE, Speed(20), Health::new(20)),
      ('d', VIOLETRED, Speed(40), Health::new(10)),
      ('t', SILVER, Speed(100), Health::new(5)),
    ];

    let count_dist = Uniform::from(0..=3);
    self.world.insert(
      (tags::Mobile, tags::Monster),
      map
        .rooms
        .iter()
        .filter(|room| *room != start_room)
        .flat_map(|room| {
          let rng = &mut *self.resources.get_mut::<StdRng>().unwrap();
          let monster_count = count_dist.sample(rng);

          (0..monster_count)
            .map(|_| {
              let mut pos = room.point_set().iter().choose(rng).copied().unwrap();

              if pos.x == 0 {
                pos.x += 1;
              } else if pos.x == map.width() - 1 {
                pos.x -= 1;
              }

              if pos.y == 0 {
                pos.y += 1;
              } else if pos.y == map.height() - 1 {
                pos.y -= 1;
              }

              let (glyph, fg, speed, health) = monsters.choose(rng).cloned().unwrap();

              (
                pos,
                Appearance::new(glyph, fg, BLACK),
                Energy::default(),
                speed,
                health,
              )
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>(),
    );

    let query = Read::<Position>::query();
    for (entity, pos) in query.iter_entities(&self.world) {
      map[*pos].occupant = Some(entity);
    }
  }

  fn render(&self, term: &mut BTerm) {
    term.cls();

    let map = self.resources.get::<WorldMap>().unwrap();

    map.render(term);

    let player_viewsheds = <Read<Viewshed>>::query().filter(tag::<Player>());
    let mut visible_tiles = HashSet::<Position>::new();
    for viewshed in player_viewsheds.iter(&self.world) {
      visible_tiles.extend(viewshed.visible_tiles.iter());
      viewshed.visible_tiles.iter().for_each(|pos| {
        let Appearance { glyph, fg, bg } = map[*pos].appearance();

        term.set(pos.x, pos.y, fg, bg, glyph);
      });
    }

    let query = <(Read<Position>, Read<Appearance>)>::query();
    for (position, appearance) in query.iter(&self.world) {
      if !visible_tiles.contains(&position) {
        continue;
      }

      let Position { x, y } = *position;
      let Appearance { glyph, fg, bg } = *appearance;

      term.set(x, y, fg, bg, glyph);
    }
  }
}

impl GameState for Game {
  fn tick(&mut self, term: &mut BTerm) {
    use std::time::Duration;

    {
      let mut time = self.resources.get_mut::<GameTime>().unwrap();
      time.capture_time(Duration::from_secs_f32(term.frame_time_ms / 1000.0));
    }

    let input = term.key.and_then(|key| {
      self
        .resources
        .get::<Keymap<Input>>()
        .unwrap()
        .get(&key)
        .copied()
    });

    // TODO: remove in favor of more robust quit workflow
    if matches!(input, Some(Input::Cancel)) {
      term.quit();
      return;
    }

    let query = Write::<Option<Input>>::query().filter(tag::<tags::Player>());
    // TODO: map input to a specific entity (for multi-unit/multiplayer)
    for mut player_input in query.iter_mut(&mut self.world) {
      // This is the player equivalent of determining intent, which will
      // then need to be translated to an action
      *player_input = input;
    }

    if let Some(schedule) = &mut self.schedule {
      schedule.execute(&mut self.world, &mut self.resources);
    }

    self.render(term);
  }
}
