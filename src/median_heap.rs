use std::cmp::{Ordering::*};
use std::fmt::{Debug, Formatter, Result};

use min_max_heap::MinMaxHeap;

use crate::average_with::AverageWith;

/// A median heap implemented with two binary heaps.
pub struct MedianHeap<T: Ord> {
  max_size: Option<usize>,
  left: MinMaxHeap<T>,
  right: MinMaxHeap<T>,
}

impl<T: Ord> Default for MedianHeap<T> {
  /// Creates an empty `MedianHeap<T>`.
  #[inline]
  fn default() -> Self {
    Self {
      max_size: Default::default(),
      left: Default::default(),
      right: Default::default(),
    }
  }
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

impl<T: Ord + AverageWith + Clone> MedianHeap<T> {
  /// Creates an empty `MedianHeap`.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::new();
  /// heap.push(4);
  /// ```
  #[inline]
  pub fn new() -> Self  {
    Default::default()
  }

  /// Creates an empty `MedianHeap` which can only grow to `max_size`.
  ///
  /// # Panics
  ///
  /// Panics if `max_size` is zero.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::with_max_size(42);
  /// heap.push(4);
  /// ```
  #[inline]
  pub fn with_max_size(max_size: usize) -> Self  {
    assert!(max_size > 0);

    let heap_size = (max_size + 3) / 2;

    Self {
      max_size: Some(max_size),
      left: MinMaxHeap::with_capacity(heap_size),
      right: MinMaxHeap::with_capacity(heap_size),
    }
  }

  /// Returns the maximum size the median heap can grow to.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let heap = MedianHeap::<i32>::with_max_size(42);
  /// assert_eq!(heap.max_size(), Some(42));
  /// ```
  pub fn max_size(&self) -> Option<usize> {
    self.max_size
  }

  /// Returns the length of the heap.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::new();
  /// assert_eq!(heap.len(), 0);
  ///
  /// heap.push(1);
  /// assert_eq!(heap.len(), 1);
  /// ```
  pub fn len(&self) -> usize {
    self.left.len() + self.right.len()
  }

  /// Returns `true` if there are no elements on the heap.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::<i32>::new();
  ///
  /// assert_eq!(heap.is_empty(), true);
  /// ```
  pub fn is_empty(&self) -> bool {
    self.left.is_empty() && self.right.is_empty()
  }

  fn is_full(&self) -> bool {
    if let Some(max_size) = self.max_size {
      self.left.len() + self.right.len() >= max_size
    } else {
      false
    }
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
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::new();
  ///
  /// assert_eq!(heap.median(), None);
  ///
  /// heap.push(1);
  /// assert_eq!(heap.median(), Some(1));
  ///
  /// heap.push(3);
  /// assert_eq!(heap.median(), Some(2));
  /// ```
  pub fn median(&self) -> Option<T> {
    match self.left.len().cmp(&self.right.len()) {
      Less    => self.right.peek_min().cloned(),
      Greater => self.left.peek_max().cloned(),
      Equal   => {
        self.left.peek_max().cloned().and_then(|left| {
          self.right.peek_min().cloned().and_then(|right| {
            Some(left.average_with(&right))
          })
        })
      },
    }
  }

  /// Pushes an item onto the median heap.
  ///
  /// When `max_size` is set and the heap is full, this will remove
  ///   - the smallest item, if the pushed item is greater than `>` the current median
  ///   - the largest item, if the pushed item is less than `<` the current median
  ///   - both the smallest and the largest item, if the pushed item is equal `==` to the current median
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::new();
  ///
  /// heap.push(1);
  /// heap.push(2);
  /// heap.push(3);
  ///
  /// assert_eq!(heap.len(), 3);
  /// ```
  ///
  /// Usage with `max_size`:
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// let mut heap = MedianHeap::with_max_size(2);
  ///
  /// heap.push(1);
  /// heap.push(1);
  /// assert_eq!(heap.len(), 2);
  /// ```
  ///
  /// When we now push another `1` it will be inserted in the middle, which leads to
  /// both the smallest and largest item being removed in order not to overflow the `max_size`.
  ///
  /// ```
  /// # extern crate medianheap;
  /// # use medianheap::MedianHeap;
  /// #
  /// # let mut heap = MedianHeap::with_max_size(2);
  /// #
  /// # heap.push(1);
  /// # heap.push(1);
  /// #
  /// heap.push(1);
  /// assert_eq!(heap.len(), 1);
  /// ```
  pub fn push(&mut self, item: T) {
    match self.median().map(|median| item.cmp(&median)).unwrap_or(Equal) {
      Less if self.is_full() => {
        self.left.push(item);

        if self.left.len() > self.right.len() {
          self.right.push(self.left.pop_max().unwrap());
        }

        self.right.pop_max();
      },
      Less => {
        self.left.push(item);

        if self.left.len() > self.right.len() + 1 {
          self.right.push(self.left.pop_max().unwrap());
        }
      },
      Greater if self.is_full() => {
        self.right.push(item);

        if self.right.len() > self.left.len() {
          self.left.push(self.right.pop_min().unwrap());
        }

        self.left.pop_min();
      },
      Greater => {
        self.right.push(item);

        if self.right.len() > self.left.len() + 1 {
          self.left.push(self.right.pop_min().unwrap());
        }
      },
      Equal => {
        if self.is_full() {
          self.left.pop_min();
          self.right.pop_max();
        }

        if self.left.len() > self.right.len() {
          self.right.push(item);
        } else {
          self.left.push(item);
        }
      },
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use ordered_float::NotNan;

  #[test]
  fn push() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));

    heap.push(2.0.into());
    assert_eq!(heap.median(), Some(1.5.into()));

    heap.push(3.0.into());
    assert_eq!(heap.median(), Some(2.0.into()));

    heap.push(4.0.into());
    assert_eq!(heap.median(), Some(2.5.into()));

    heap.push(5.0.into());
    assert_eq!(heap.median(), Some(3.0.into()));

    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(2.5.into()));
  }

  #[test]
  fn push_ascending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(1.0.into());
    heap.push(2.0.into());
    heap.push(3.0.into());
    heap.push(4.0.into());
    heap.push(5.0.into());

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  fn push_descending() {
    let mut heap = MedianHeap::<NotNan<f32>>::new();

    heap.push(5.0.into());
    heap.push(4.0.into());
    heap.push(3.0.into());
    heap.push(2.0.into());
    heap.push(1.0.into());

    assert_eq!(heap.median(), Some(3.0.into()));
  }

  #[test]
  #[should_panic]
  fn max_size_0() {
    MedianHeap::<NotNan<f32>>::with_max_size(0);
  }

  #[test]
  fn max_size_1() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(1);

    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_1_asc() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(1);

    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(2.0.into());
    assert_eq!(heap.median(), Some(2.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(3.0.into());
    assert_eq!(heap.median(), Some(3.0.into()));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_1_desc() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(1);

    heap.push(3.0.into());
    assert_eq!(heap.median(), Some(3.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(2.0.into());
    assert_eq!(heap.median(), Some(2.0.into()));
    assert_eq!(heap.len(), 1);
    heap.push(1.0.into());
    assert_eq!(heap.median(), Some(1.0.into()));
    assert_eq!(heap.len(), 1);
  }

  #[test]
  fn max_size_8() {
    let mut heap = MedianHeap::<NotNan<f32>>::with_max_size(8);

    for i in 0..100 {
      heap.push((i as f32).into());

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
      heap.push(100.0.into());
    }

    assert_eq!(heap.left.clone().into_vec_asc(), vec![100.0.into(); 4]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![100.0.into(); 4]);

    for _ in 0..(8 * 3 / 2 + 1) {
      heap.push(2.0.into());
    }

    assert_eq!(heap.left.clone().into_vec_asc(), vec![2.0.into(); 4]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![2.0.into(); 4]);

    heap.push(1.0.into());
    assert_eq!(heap.left.clone().into_vec_asc(), vec![1.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![2.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);

    heap.push(1.0.into());
    assert_eq!(heap.left.clone().into_vec_asc(), vec![1.0.into(), 1.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![2.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);

    heap.push(3.0.into());
    assert_eq!(heap.left.clone().into_vec_asc(), vec![1.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![3.0.into(), 2.0.into(), 2.0.into(), 2.0.into()]);

    heap.push(2.0.into());
    assert_eq!(heap.left.clone().into_vec_asc(), vec![2.0.into(); 4]);
    assert_eq!(heap.right.clone().into_vec_desc(), vec![2.0.into(); 3]);
  }
}
