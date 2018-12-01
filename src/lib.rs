#![deny(bad_style, future_incompatible, missing_debug_implementations, rust_2018_idioms)]

extern crate num_traits;

#[cfg(test)]
extern crate ordered_float;

mod median_heap;
pub use crate::median_heap::MedianHeap;
