use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result};

#[derive(PartialEq, Eq)]
pub struct Min<T: Ord>(pub T);

impl<T> PartialOrd for Min<T>
where T: Ord
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for Min<T>
where T: Ord
{
  fn cmp(&self, other: &Self) -> Ordering {
    other.0.cmp(&self.0)
  }
}

impl<T> Debug for Min<T> where T: PartialOrd + Ord + Debug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{:?}", self.0)
  }
}
