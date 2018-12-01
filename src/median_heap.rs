use std::mem;
use std::collections::BinaryHeap;
use std::cmp::{Ordering::*, Reverse};
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div};

#[derive(Default)]
pub struct MedianHeap<T: Ord> {
  max_size: Option<usize>,
  left: BinaryHeap<T>,
  right: BinaryHeap<Reverse<T>>,
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
      Greater => self.left.peek().cloned(),
      Equal   => {
        self.left.peek().cloned().and_then(|left| {
          self.right.peek().map(|right| {
            (left + right.0) / T::from(2f32)
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
      Some(median) if item < median => Less,
      Some(median) if item > median => Greater,
      _ => Equal,
    };

    match ordering {
      Less => {
        if self.is_full() {
          self.pop_max();
        }

        self.left.push(item);
      },
      Greater => {
        if self.is_full() {
          self.pop_min();
        }

        self.right.push(Reverse(item));
      },
      Equal if self.left.len() > self.right.len() => self.right.push(Reverse(item)),
      Equal => {
        if self.is_full() {
          self.pop_min();
        }

        self.left.push(item);
      },
    };

    if self.right.len() > self.left.len() {
      self.left.push(self.right.pop().unwrap().0);
    } else if self.left.len() > self.right.len() + 1 {
      self.right.push(Reverse(self.left.pop().unwrap()));
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

  fn pop_min(&mut self) {
    if self.left.is_empty() {
      return
    }

    let heap = mem::replace(&mut self.left, BinaryHeap::with_capacity(0));
    let mut vec = heap.into_sorted_vec();
    vec.remove(0);

    self.left = BinaryHeap::from(vec);
  }

  fn pop_max(&mut self) {
    if self.right.is_empty() {
      return
    }

    let heap = mem::replace(&mut self.right, BinaryHeap::with_capacity(0));
    let mut vec = heap.into_sorted_vec();
    vec.remove(0);

    self.right = BinaryHeap::from(vec);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use ordered_float::NotNan;

  #[test]
  fn binary_heap_into_vec() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.insert(1.0);
    heap.insert(2.0);
    heap.insert(3.0);
    heap.insert(4.0);

    assert_eq!(heap.left.into_vec(), vec![2.0.into(), 1.0.into()]);
    assert_eq!(heap.right.into_vec(), vec![Reverse(3.0.into()), Reverse(4.0.into())]);
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
