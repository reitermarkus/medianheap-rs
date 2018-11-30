use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Max<T: PartialOrd + Ord>(pub T);

impl<T> Debug for Max<T> where T: PartialOrd + Ord + Debug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.0)
  }
}
