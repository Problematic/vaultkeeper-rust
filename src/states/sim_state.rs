use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use ai::*;
use bracket_lib::prelude::*;
use components::*;
use rand::Rng;
use vaultkeeper_shared::{states::PauseState, State, Transition, WorldContext};

#[derive(Default, Debug)]
pub struct SimState {}

impl State for SimState {
  fn on_start(&mut self, _term: &mut BTerm, context: &mut WorldContext) {
    let mut rng = rand::thread_rng();

    context.world.insert(
      (),
      vec![
        (
          Name::new("Watercooler"),
          Position::new(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2),
          Renderable {
            glyph: to_cp437('#'),
            colors: ColorPair {
              fg: RGBA::named(AQUA),
              bg: RGBA::named(BLACK),
            },
          },
          Some(PointOfInterest(Need::Social)),
          Interactable { actions: vec![] },
        ),
        (
          Name::new("Cake"),
          Position::new(70, 45),
          Renderable {
            glyph: to_cp437('O'),
            colors: ColorPair {
              fg: RGBA::named(PINK),
              bg: RGBA::named(BLACK),
            },
          },
          None::<PointOfInterest>,
          Interactable { actions: vec![] },
        ),
      ],
    );

    context.world.insert(
      (Character,),
      vec![Position::new(10, 10), Position::new(70, 50)]
        .into_iter()
        .enumerate()
        .map(|(idx, pos)| {
          (
            Name::new(format!("Vaultizen #{:0>3}", idx + 1)),
            pos,
            Renderable {
              glyph: to_cp437('☺'),
              colors: ColorPair {
                fg: RGBA::named(WHITE),
                bg: RGBA::named(BLACK),
              },
            },
            Needs::from(vec![
              (Need::Hunger, rng.gen_range(35.0, 75.0)),
              (Need::Social, rng.gen_range(35.0, 75.0)),
            ]),
            Navigation::default(),
            Viewshed::default(),
            None::<Action>,
          )
        }),
    );
  }

  fn update(&mut self, term: &mut BTerm, _context: &mut WorldContext) -> Transition {
    if let Some(VirtualKeyCode::Space) = term.key {
      return Transition::Push(Box::new(PauseState));
    }

    Transition::None
  }
}
