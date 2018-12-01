use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq)]
pub struct Min<T: Ord>(pub T);

impl<T: Ord> PartialOrd for Min<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: Ord> Ord for Min<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    other.0.cmp(&self.0)
  }
}

impl<T: Ord + Debug> Debug for Min<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.0)
  }
}
