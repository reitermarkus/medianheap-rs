#[cfg(any(test, feature = "ordered-float"))]
use ordered_float::NotNan;

/// This trait is used for calculating the arithmetic mean of the two middlemost elements
/// of a median heap if it contains an even number of elements.
pub trait AverageWith {
  /// Returns the arithmetic mean between two values.
  fn average_with(&self, other: &Self) -> Self;
}

#[cfg(any(test, feature = "ordered-float"))]
impl AverageWith for NotNan<f32> {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2.0
  }
}

#[cfg(any(test, feature = "ordered-float"))]
impl AverageWith for NotNan<f64> {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2.0
  }
}

impl AverageWith for i8 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for i16 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for i32 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for i64 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for i128 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for u8 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for u16 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for u32 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for u64 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}

impl AverageWith for u128 {
  fn average_with(&self, other: &Self) -> Self {
    (*self + *other) / 2
  }
}
