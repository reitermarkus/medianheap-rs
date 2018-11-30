use std::collections::BinaryHeap;
use std::cmp::Ordering::*;
use std::fmt::{Debug, Formatter, Result};

use crate::max::Max;
use crate::min::Min;

pub struct MedianHeap<T>
where T: PartialEq + PartialOrd + Ord,
{
  max_size: Option<usize>,
  left: BinaryHeap<Max<T>>,
  right: BinaryHeap<Min<T>>,
}

impl<T> Debug for MedianHeap<T> where T: PartialOrd + Ord + Debug {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "MaxHeap {{ ")?;

    if let Some(max_size) = self.max_size {
      write!(f, "max_size: {}, ", max_size)?
    }

    write!(f, "left: {:?}, right: {:?} }}", self.left, self.right)
  }
}

impl<T> MedianHeap<T>
where T: PartialEq + PartialOrd + Ord,
{
  pub fn new() -> Self {
    Self {
      max_size: None,
      left: BinaryHeap::new(),
      right: BinaryHeap::new(),
    }
  }

  pub fn with_max_size(max_size: usize) -> Self {
    let mut median_heap = Self::new();
    median_heap.max_size = Some(max_size);
    median_heap
  }

  pub fn median(&self) -> Option<&T> {
    match self.left.len().cmp(&self.right.len()) {
      Less    => self.right.peek().map(|item| &item.0),
      Greater => self.left.peek().map(|item| &item.0),
      Equal   => self.left.peek().map(|item| &item.0),
    }
  }

  pub fn insert(&mut self, item: T) {
    if self.max_size == Some(0) {
      return
    }

    let ordering = match self.median() {
      Some(median) if &item < median => Less,
      Some(median) if &item > median => Greater,
      _ => Equal,
    };

    match ordering {
      Less => {
        if self.is_full() {
          self.drop_right();
        }

        self.left.push(Max(item));
      },
      Greater => {
        if self.is_full() {
          self.drop_left();
        }

        self.right.push(Min(item));
      },
      Equal if self.left.len() > self.right.len() => self.right.push(Min(item)),
      Equal => {
        if self.is_full() {
          self.drop_left();
        }

        self.left.push(Max(item));
      },
    };

    if self.right.len() > self.left.len() {
      self.left.push(Max(self.right.pop().unwrap().0));
    } else if self.left.len() > self.right.len() + 1 {
      self.right.push(Min(self.left.pop().unwrap().0));
    }
  }

  pub fn is_empty(&self) -> bool {
    self.left.is_empty() && self.right.is_empty()
  }

  pub fn len(&self) -> usize {
    self.left.len() + self.right.len()
  }

  fn is_full(&self) -> bool {
    if let Some(max_size) = self.max_size {
      self.left.len() + self.right.len() >= max_size
    } else {
      false
    }
  }

  fn drop_left(&mut self) {
    if self.left.is_empty() {
      return
    }

    let mut new_heap = BinaryHeap::with_capacity(0);

    std::mem::swap(&mut new_heap, &mut self.left);

    let mut vec = new_heap.into_sorted_vec();
    vec.remove(0);

    self.left = BinaryHeap::from(vec);
  }

  fn drop_right(&mut self) {
    if self.right.is_empty() {
      return
    }

    let mut new_heap = BinaryHeap::with_capacity(0);

    std::mem::swap(&mut new_heap, &mut self.right);

    let mut vec = new_heap.into_sorted_vec();
    vec.remove(0);

    self.right = BinaryHeap::from(vec);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BinaryHeap;

  #[test]
  fn max_heap() {
    let mut heap = BinaryHeap::new();

    heap.push(Max(1));
    heap.push(Max(2));

    assert_eq!(heap.pop(), Some(Max(2)));
  }

  #[test]
  fn min_heap() {
    let mut heap = BinaryHeap::new();

    heap.push(Min(1));
    heap.push(Min(2));

    assert_eq!(heap.pop(), Some(Min(1)));
  }

  #[test]
  fn binary_heap_into_vec() {
    let mut heap = MedianHeap::new();

    heap.insert(1);
    heap.insert(2);
    heap.insert(3);
    heap.insert(4);

    assert_eq!(heap.left.into_vec(), vec![Max(2), Max(1)]);
    assert_eq!(heap.right.into_vec(), vec![Min(3), Min(4)]);
  }

  #[test]
  fn insert() {
    let mut heap = MedianHeap::new();

    heap.insert(1);
    assert_eq!(heap.median(), Some(&1));

    heap.insert(2);
    assert_eq!(heap.median(), Some(&1));

    heap.insert(3);
    assert_eq!(heap.median(), Some(&2));

    heap.insert(4);
    assert_eq!(heap.median(), Some(&2));

    heap.insert(5);
    assert_eq!(heap.median(), Some(&3));

    heap.insert(1);
    assert_eq!(heap.median(), Some(&2));
  }

  #[test]
  fn insert_ascending() {
    let mut heap = MedianHeap::new();

    heap.insert(1);
    heap.insert(2);
    heap.insert(3);
    heap.insert(4);
    heap.insert(5);

    assert_eq!(heap.median(), Some(&3));
  }

  #[test]
  fn insert_descending() {
    let mut heap = MedianHeap::new();

    heap.insert(5);
    heap.insert(4);
    heap.insert(3);
    heap.insert(2);
    heap.insert(1);

    assert_eq!(heap.median(), Some(&3));
  }

  #[test]
  fn max_size_0() {
    let mut heap = MedianHeap::with_max_size(0);

    heap.insert(1);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.insert(2);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.insert(3);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
  }

  #[test]
  fn max_size_1() {
    let mut heap = MedianHeap::with_max_size(1);

    heap.insert(1);
    assert_eq!(heap.median(), Some(&1));
    assert_eq!(heap.len(), 1);
    heap.insert(2);
    assert_eq!(heap.median(), Some(&2));
    assert_eq!(heap.len(), 1);
    heap.insert(3);
    assert_eq!(heap.median(), Some(&3));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_8() {
    let mut heap = MedianHeap::with_max_size(8);

    for i in 0..100 {
      heap.insert(i);

      if i < 8 {
        assert_eq!(heap.len(), i + 1);
      } else {
        assert_eq!(heap.len(), 8);
      }
    }

    assert_eq!(heap.median(), Some(&95));
    assert_eq!(heap.len(), 8);
  }
}
