use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Max<T: Ord>(pub T);

impl<T: Ord + Debug> Debug for Max<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.0)
  }
}
