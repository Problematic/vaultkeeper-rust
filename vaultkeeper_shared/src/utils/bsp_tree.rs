type Depth = usize;

pub trait Partition: Sized {
  fn partition(&self) -> Option<(Self, Self)>;
}

#[derive(Debug)]
pub enum BSPTree<T>
where
  T: Partition,
{
  Leaf(Depth, T),
  Node(Depth, Box<BSPTree<T>>, Box<BSPTree<T>>),
  None,
}

impl<T> BSPTree<T>
where
  T: Partition,
{
  pub fn new(data: T) -> Self {
    Self::Leaf(0, data)
  }

  pub fn partition(&mut self) {
    use BSPTree::{Leaf, Node};

    *self = match std::mem::replace(self, BSPTree::None) {
      Leaf(depth, data) => {
        if let Some((left, right)) = data.partition() {
          Node(
            depth,
            Box::new(Leaf(depth + 1, left)),
            Box::new(Leaf(depth + 1, right)),
          )
        } else {
          Leaf(depth, data)
        }
      }
      Node(depth, mut left, mut right) => {
        left.partition();
        right.partition();

        Node(depth, left, right)
      }
      otherwise => otherwise,
    }
  }

  pub fn for_each<F>(&self, f: &mut F)
  where
    F: FnMut(usize, &T),
  {
    use BSPTree::*;

    match self {
      Leaf(depth, data) => f(*depth, data),
      Node(_depth, left, right) => {
        left.for_each(f);
        right.for_each(f);
      }
      _ => (),
    }
  }
}
