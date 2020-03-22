use crate::ui::Input;
use legion::prelude::{Resources, World};

pub struct WorldContext {
  pub world: World,
  pub resources: Resources,
}

#[allow(dead_code)]
pub enum Transition<T> {
  None,
  Push(Box<dyn State<T>>),
  Switch(Box<dyn State<T>>),
  Pop,
  Quit,
}

impl<T> Transition<T> {
  pub fn or(self, alt: Transition<T>) -> Transition<T> {
    match self {
      Transition::None => alt,
      _ => self,
    }
  }
}

pub trait State<TContext> {
  fn on_start(&mut self, _context: &mut TContext) {}

  fn on_stop(&mut self, _context: &mut TContext) {}

  fn on_pause(&mut self, _context: &mut TContext) {}

  fn on_resume(&mut self, _context: &mut TContext) {}

  fn update(&mut self, _context: &mut TContext) -> Transition<TContext> {
    Transition::None
  }

  fn handle_input(&mut self, _context: &mut TContext, _input: Input) -> Transition<TContext> {
    Transition::None
  }
}

pub struct StateMachine<'a, TContext> {
  running: bool,
  state_stack: Vec<Box<dyn State<TContext> + 'a>>,
}

impl<'a, TContext> StateMachine<'a, TContext> {
  pub fn new<S: State<TContext> + 'a>(initial_state: S) -> Self {
    Self {
      running: false,
      state_stack: vec![Box::new(initial_state)],
    }
  }

  pub fn start(&mut self, context: &mut TContext) {
    self.running = true;
    self.state_stack.last_mut().unwrap().on_start(context);
  }

  pub fn handle_input(&mut self, context: &mut TContext, input: Input) {
    assert!(self.running);

    let transition = match self.state_stack.last_mut() {
      Some(state) => state.handle_input(context, input),
      None => Transition::None,
    };

    self.transition(context, transition);
  }

  pub fn update(&mut self, context: &mut TContext) {
    assert!(self.running);

    if let Some(active) = self.state_stack.last_mut() {
      let transition = active.update(context);

      self.transition(context, transition);
    }
  }

  pub fn transition(&mut self, context: &mut TContext, transition: Transition<TContext>) {
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
