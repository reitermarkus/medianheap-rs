#![deny(bad_style, future_incompatible, missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! A median heap for keeping track of a running median.

mod median_heap;
pub use crate::median_heap::{Median, MedianHeap};
