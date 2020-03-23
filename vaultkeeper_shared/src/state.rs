use crate::ui::Input;
use legion::prelude::{Resources, World};

pub struct StateContext<'a, TData> {
  pub world: &'a mut World,
  pub resources: &'a mut Resources,
  pub data: &'a mut TData,
}

impl<'a, TData> StateContext<'a, TData> {
  pub fn new(world: &'a mut World, resources: &'a mut Resources, data: &'a mut TData) -> Self {
    Self {
      world,
      resources,
      data,
    }
  }
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
  fn on_start(&mut self, _context: StateContext<TData>) {}

  fn on_stop(&mut self, _context: StateContext<TData>) {}

  fn on_pause(&mut self, _context: StateContext<TData>) {}

  fn on_resume(&mut self, _context: StateContext<TData>) {}

  fn update(&mut self, _context: StateContext<TData>) -> Transition<TData> {
    Transition::None
  }

  fn handle_input(&mut self, _context: StateContext<TData>, _input: Input) -> Transition<TData> {
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

  pub fn start(&mut self, context: StateContext<TData>) {
    self.running = true;
    self.state_stack.last_mut().unwrap().on_start(context);
  }

  pub fn handle_input(&mut self, context: StateContext<TData>, input: Input) {
    assert!(self.running);

    let StateContext {
      world,
      resources,
      data,
    } = context;

    let transition = match self.state_stack.last_mut() {
      Some(state) => state.handle_input(
        StateContext {
          world,
          resources,
          data,
        },
        input,
      ),
      None => Transition::None,
    };

    self.transition(
      StateContext {
        world,
        resources,
        data,
      },
      transition,
    );
  }

  pub fn update(&mut self, context: StateContext<TData>) {
    assert!(self.running);

    let StateContext {
      world,
      resources,
      data,
    } = context;

    if let Some(active) = self.state_stack.last_mut() {
      let transition = active.update(StateContext {
        world,
        resources,
        data,
      });

      self.transition(
        StateContext {
          world,
          resources,
          data,
        },
        transition,
      );
    }
  }

  fn push(&mut self, state: Box<dyn State<TData>>, context: StateContext<TData>) {
    assert!(self.running);

    let StateContext {
      world,
      resources,
      data,
    } = context;

    if let Some(state) = self.state_stack.last_mut() {
      state.on_pause(StateContext {
        world,
        resources,
        data,
      });
    }

    self.state_stack.push(state);
    self.state_stack.last_mut().unwrap().on_start(StateContext {
      world,
      resources,
      data,
    });
  }

  fn switch(&mut self, state: Box<dyn State<TData>>, context: StateContext<TData>) {
    assert!(self.running);

    let StateContext {
      world,
      resources,
      data,
    } = context;

    if let Some(mut state) = self.state_stack.pop() {
      state.on_stop(StateContext {
        world,
        resources,
        data,
      });
    }

    self.state_stack.push(state);
    self.state_stack.last_mut().unwrap().on_start(StateContext {
      world,
      resources,
      data,
    });
  }

  fn pop(&mut self, context: StateContext<TData>) {
    assert!(self.running);

    let StateContext {
      world,
      resources,
      data,
    } = context;

    if let Some(mut state) = self.state_stack.pop() {
      state.on_stop(StateContext {
        world,
        resources,
        data,
      });
    }

    if let Some(state) = self.state_stack.last_mut() {
      state.on_resume(StateContext {
        world,
        resources,
        data,
      });
    } else {
      self.running = false;
    }
  }

  pub fn transition(&mut self, context: StateContext<TData>, transition: Transition<TData>) {
    assert!(self.running);

    match transition {
      Transition::Push(state) => self.push(state, context),
      Transition::Switch(state) => self.switch(state, context),
      Transition::Pop => self.pop(context),
      Transition::None => (),
      Transition::Quit => self.running = false,
    }
  }
}
