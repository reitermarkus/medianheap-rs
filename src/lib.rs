#![deny(bad_style, future_incompatible, missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! A median heap for keeping track of a running median.

#[cfg(any(test, feature = "ordered-float"))]
extern crate ordered_float;

mod average_with;
pub use crate::average_with::AverageWith;

mod median_heap;
pub use crate::median_heap::MedianHeap;
