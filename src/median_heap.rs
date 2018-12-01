use std::collections::BinaryHeap;
use std::cmp::Ordering::*;
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div};

use crate::max::Max;
use crate::min::Min;

pub struct MedianHeap<T: Ord> {
  max_size: Option<usize>,
  left: BinaryHeap<Max<T>>,
  right: BinaryHeap<Min<T>>,
}

impl<T: Ord + Debug> Debug for MedianHeap<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "MaxHeap {{ ")?;

    if let Some(max_size) = self.max_size {
      write!(f, "max_size: {}, ", max_size)?
    }

    write!(f, "left: {:?}, right: {:?} }}", self.left, self.right)
  }
}

impl<T: Ord + From<f32> + Add<Output = T> + Div<T, Output = T> + Copy> MedianHeap<T> {
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

  pub fn median(&self) -> Option<T> {
    match self.left.len().cmp(&self.right.len()) {
      Less    => self.right.peek().map(|item| item.0),
      Greater => self.left.peek().map(|item| item.0),
      Equal   => {
        self.left.peek().and_then(|left| {
          self.right.peek().map(|right| {
            (left.0 + right.0) / T::from(2f32)
          })
        })
      },
    }
  }

  pub fn insert(&mut self, item: impl Into<T>) {
    let item = item.into();

    if self.max_size == Some(0) {
      return
    }

    let ordering = match self.median() {
      Some(median) if &item < &median => Less,
      Some(median) if &item > &median => Greater,
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

  use ordered_float::NotNan;

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
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.insert(1.0);
    heap.insert(2.0);
    heap.insert(3.0);
    heap.insert(4.0);

    assert_eq!(heap.left.into_vec(), vec![Max(2.0.into()), Max(1.0.into())]);
    assert_eq!(heap.right.into_vec(), vec![Min(3.0.into()), Min(4.0.into())]);
  }

  #[test]
  fn insert() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.insert(1.0);
    assert_eq!(heap.median(), Some(1.0.into()));

    heap.insert(2.0);
    assert_eq!(heap.median(), Some(1.5.into()));

    heap.insert(3.0);
    assert_eq!(heap.median(), Some(2.0.into()));

    heap.insert(4.0);
    assert_eq!(heap.median(), Some(2.5.into()));

    heap.insert(5.0);
    assert_eq!(heap.median(), Some(3.0.into()));

    heap.insert(1.0);
    assert_eq!(heap.median(), Some(2.5.into()));
  }

  #[test]
  fn insert_ascending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.insert(1.0);
    heap.insert(2.0);
    heap.insert(3.0);
    heap.insert(4.0);
    heap.insert(5.0);

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  fn insert_descending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.insert(5.0);
    heap.insert(4.0);
    heap.insert(3.0);
    heap.insert(2.0);
    heap.insert(1.0);

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  fn max_size_0() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(0);

    heap.insert(1.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.insert(2.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.insert(3.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
  }

  #[test]
  fn max_size_1() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(1);

    heap.insert(1.0);
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
    heap.insert(2.0);
    assert_eq!(heap.median(), Some(2.0.into()));
    assert_eq!(heap.len(), 1);
    heap.insert(3.0);
    assert_eq!(heap.median(), Some(3.0.into()));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_8() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(8);

    for i in 0..100 {
      heap.insert(i as f32);

      if i < 8 {
        assert_eq!(heap.len(), i + 1);
      } else {
        assert_eq!(heap.len(), 8);
      }
    }

    assert_eq!(heap.median(), Some(95.5.into()));
    assert_eq!(heap.len(), 8);
  }
}
