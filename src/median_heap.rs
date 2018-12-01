use std::mem;
use std::collections::BinaryHeap;
use std::cmp::{Ordering::*, Reverse};
use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div};

use num_traits::FromPrimitive;

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

impl<T: Ord + FromPrimitive + Add<Output = T> + Div<T, Output = T> + Debug + Copy> MedianHeap<T> {
  /// Creates an empty `MedianHeap`.
  pub fn new() -> Self {
    Self {
      max_size: None,
      left: BinaryHeap::new(),
      right: BinaryHeap::new(),
    }
  }

  /// Creates an empty `MedianHeap` which can only grow to `max_size`.
  pub fn with_max_size(max_size: usize) -> Self {
    let mut median_heap = Self::new();
    median_heap.max_size = Some(max_size);
    median_heap
  }

  /// This either returns
  ///   - `Some(T)` containing the median value if there are an odd number of elements
  ///   - `Some(T)` containing the arithmetic mean of the two middlemost values if there are an even number of elements
  ///   - `None` if the heap is empty
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate ordered_float;
  /// # extern crate medianheap;
  /// use ordered_float::NotNan;
  /// # use medianheap::MedianHeap;
  ///
  /// let mut heap = MedianHeap::<NotNan<f32>>::new();
  ///
  /// assert_eq!(heap.median(), None);
  ///
  /// heap.push(1.0);
  /// assert_eq!(heap.median(), Some(1.0.into()));
  ///
  /// heap.push(2.0);
  /// assert_eq!(heap.median(), Some(1.5.into()));
  /// ```
  pub fn median(&self) -> Option<T> {
    match self.left.len().cmp(&self.right.len()) {
      Less    => self.right.peek().map(|item| item.0),
      Greater => self.left.peek().cloned(),
      Equal   => {
        self.left.peek().cloned().and_then(|left| {
          self.right.peek().and_then(|right| {
            T::from_u8(2).map(|div| (left + right.0) / div)
          })
        })
      },
    }
  }

  /// Pushes an item onto the binary heap.
  ///
  ///
  ///
  /// When `max_size` is set and the heap is full, this will remove
  ///   - the smallest item, if the pushed item is greater than `>` the current median
  ///   - the largest item, if the pushed item is less than `<` the current median
  ///   - if the pushed item is equal `==` to the current median
  ///     - the smallest item, if the item occurs more often on the right side side of median
  ///     - the largest item, if the item occurs more often on the left side side of median
  ///     - both the smallest and the largest item, if the item occurs equally as often on the left side and the right side of median
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate ordered_float;
  /// # extern crate medianheap;
  /// use ordered_float::NotNan;
  /// # use medianheap::MedianHeap;
  ///
  /// let mut heap = MedianHeap::<NotNan<f32>>::new();
  ///
  /// heap.push(1.0);
  /// heap.push(2.0);
  /// heap.push(3.0);
  ///
  /// assert_eq!(heap.len(), 3);
  /// ```
  pub fn push(&mut self, item: impl Into<T>) {
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

        if self.left.len() > self.right.len() + 1 {
          self.right.push(Reverse(self.left.pop().unwrap()));
        }
      },
      Greater => {
        if self.is_full() {
          self.pop_min();
        }

        self.right.push(Reverse(item));

        if self.right.len() > self.left.len() {
          self.left.push(self.right.pop().unwrap().0);
        }
      },
      Equal => {
        if self.is_full() {
          let mut left = Vec::new();
          let mut right = Vec::new();

          while self.left.peek() == Some(&item) {
            left.push(self.left.pop().unwrap());
          }

          while self.right.peek() == Some(&Reverse(item)) {
            right.push(self.right.pop().unwrap());
          }

          match left.len().cmp(&right.len()) {
            Less => {
              self.pop_min();
              self.left.push(item);
            },
            Greater => {
              self.pop_max();
              self.right.push(Reverse(item))
            },
            Equal => {
              self.pop_min();
              self.pop_max();
              self.left.push(item);
            }
          }

          for i in left { self.left.push(i); }
          for i in right { self.right.push(i); }
        } else if self.left.len() > self.right.len() {
          self.right.push(Reverse(item));
        } else {
          self.left.push(item);
        }
      },
    };
  }

  /// Returns `true` if there are no elements on the heap.
  pub fn is_empty(&self) -> bool {
    self.left.is_empty() && self.right.is_empty()
  }

  /// Returns the length of the heap.
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
  fn push() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(1.0);
    assert_eq!(heap.median(), Some(1.0.into()));

    heap.push(2.0);
    assert_eq!(heap.median(), Some(1.5.into()));

    heap.push(3.0);
    assert_eq!(heap.median(), Some(2.0.into()));

    heap.push(4.0);
    assert_eq!(heap.median(), Some(2.5.into()));

    heap.push(5.0);
    assert_eq!(heap.median(), Some(3.0.into()));

    heap.push(1.0);
    assert_eq!(heap.median(), Some(2.5.into()));
  }

  #[test]
  fn push_ascending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(1.0);
    heap.push(2.0);
    heap.push(3.0);
    heap.push(4.0);
    heap.push(5.0);

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  fn push_descending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(5.0);
    heap.push(4.0);
    heap.push(3.0);
    heap.push(2.0);
    heap.push(1.0);

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  fn max_size_0() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(0);

    heap.push(1.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.push(2.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
    heap.push(3.0);
    assert_eq!(heap.median(), None);
    assert_eq!(heap.len(), 0);
  }

  #[test]
  fn max_size_1() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(1);

    heap.push(1.0);
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(2.0);
    assert_eq!(heap.median(), Some(2.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(3.0);
    assert_eq!(heap.median(), Some(3.0.into()));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_8() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(8);

    for i in 0..100 {
      heap.push(i as f32);

      if i < 8 {
        assert_eq!(heap.len(), i + 1);
      } else {
        assert_eq!(heap.len(), 8);
      }
    }

    assert_eq!(heap.median(), Some(95.5.into()));
    assert_eq!(heap.len(), 8);
  }

  #[test]
  fn f32() {
    MedianHeap::<NotNan<f32>>::new();
  }

  #[test]
  fn f64() {
    MedianHeap::<NotNan<f64>>::new();
  }

  #[test]
  fn max_size_balancing() {
    let mut heap = MedianHeap::<NotNan<f64>>::with_max_size(8);

    for _ in 0..8 {
      heap.push(100.0);
    }

    assert_eq!(heap.left.clone().into_sorted_vec(), vec![100.0.into(); 4]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(100.0.into()); 4]);

    for _ in 0..8 {
      heap.push(2.0);
    }

    assert_eq!(heap.left.clone().into_sorted_vec(), vec![2.0.into(); 4]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(2.0.into()); 4]);

    heap.push(1.0);
    assert_eq!(heap.left.clone().into_sorted_vec(), vec![1.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(2.0.into()), Reverse(2.0.into()), Reverse(2.0.into()), Reverse(2.0.into())]);

    heap.push(1.0);
    assert_eq!(heap.left.clone().into_sorted_vec(), vec![1.0.into(), 1.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(2.0.into()), Reverse(2.0.into()), Reverse(2.0.into()), Reverse(2.0.into())]);

    heap.push(3.0);
    assert_eq!(heap.left.clone().into_sorted_vec(), vec![1.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(3.0.into()), Reverse(2.0.into()), Reverse(2.0.into()), Reverse(2.0.into())]);

    heap.push(2.0);
    assert_eq!(heap.left.clone().into_sorted_vec(), vec![2.0.into(); 4]);
    assert_eq!(heap.right.clone().into_sorted_vec(), vec![Reverse(2.0.into()); 3]);
  }
}
