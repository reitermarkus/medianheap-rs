#![deny(bad_style, future_incompatible, missing_debug_implementations, rust_2018_idioms)]

mod median_heap;
pub use crate::median_heap::MedianHeap;

#[cfg(test)]
extern crate ordered_float;
