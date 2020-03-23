use crate::ui::Input;
use legion::prelude::{Resources, World};

pub struct StateContext<TData> {
  pub world: World,
  pub resources: Resources,
  pub data: TData,
}

#[allow(dead_code)]
pub enum Transition<TData> {
  None,
  Push(Box<dyn State<TData>>),
  Switch(Box<dyn State<TData>>),
  Pop,
  Quit,
}

impl<TData> Transition<TData> {
  pub fn or(self, alt: Transition<TData>) -> Transition<TData> {
    match self {
      Transition::None => alt,
      _ => self,
    }
  }
}

pub trait State<TData> {
  fn on_start(&mut self, _context: &mut StateContext<TData>) {}

  fn on_stop(&mut self, _context: &mut StateContext<TData>) {}

  fn on_pause(&mut self, _context: &mut StateContext<TData>) {}

  fn on_resume(&mut self, _context: &mut StateContext<TData>) {}

  fn update(&mut self, _context: &mut StateContext<TData>) -> Transition<TData> {
    Transition::None
  }

  fn handle_input(
    &mut self,
    _context: &mut StateContext<TData>,
    _input: Input,
  ) -> Transition<TData> {
    Transition::None
  }
}

pub struct StateMachine<'a, TData> {
  running: bool,
  state_stack: Vec<Box<dyn State<TData> + 'a>>,
}

impl<'a, TData> StateMachine<'a, TData> {
  pub fn new<S: State<TData> + 'a>(initial_state: S) -> Self {
    Self {
      running: false,
      state_stack: vec![Box::new(initial_state)],
    }
  }

  pub fn start(&mut self, context: &mut StateContext<TData>) {
    self.running = true;
    self.state_stack.last_mut().unwrap().on_start(context);
  }

  pub fn handle_input(&mut self, context: &mut StateContext<TData>, input: Input) {
    assert!(self.running);

    let transition = match self.state_stack.last_mut() {
      Some(state) => state.handle_input(context, input),
      None => Transition::None,
    };

    self.transition(context, transition);
  }

  pub fn update(&mut self, context: &mut StateContext<TData>) {
    assert!(self.running);

    if let Some(active) = self.state_stack.last_mut() {
      let transition = active.update(context);

      self.transition(context, transition);
    }
  }

  pub fn transition(&mut self, context: &mut StateContext<TData>, transition: Transition<TData>) {
    assert!(self.running);

    if let Some(mut active) = self.state_stack.pop() {
      match transition {
        Transition::Push(mut state) => {
          active.on_pause(context);
          self.state_stack.push(active);
          state.on_start(context);
          self.state_stack.push(state);
        }
        Transition::Switch(mut state) => {
          active.on_stop(context);
          state.on_start(context);
          self.state_stack.push(state);
        }
        Transition::Pop => {
          active.on_stop(context);
          if let Some(next) = self.state_stack.last_mut() {
            next.on_resume(context);
          }
        }
        Transition::None => {
          self.state_stack.push(active);
        }
        Transition::Quit => self.running = false,
      }
    }
  }
}
