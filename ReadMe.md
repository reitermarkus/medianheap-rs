# Median Heap

[![Build Status](https://travis-ci.com/reitermarkus/medianheap-rs.svg?branch=master)](https://travis-ci.com/reitermarkus/medianheap-rs)
[![Crates.io](https://img.shields.io/crates/v/medianheap.svg)](https://crates.io/crates/medianheap)
[![Documentation](https://docs.rs/medianheap/badge.svg)](https://docs.rs/medianheap)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
medianheap = "0.1"
```

and this to your crate root:

```rust
extern crate medianheap;
```

## Example

Elements of a `MedianHeap` must be `Ord + AverageWith + Clone`.

The `AverageWith` trait is used to calculate the mean value for the two middlemost items if the number of items is even. This is implemented for all integer types and the `NotNan` type from the [`ordered-float`](https://github.com/reem/rust-ordered-float) crate.

```rust
let mut heap = MedianHeap::new();

heap.push(1);

assert_eq!(heap.median(), Some(1));

heap.push(3);

assert_eq!(heap.median(), Some(2));
```
