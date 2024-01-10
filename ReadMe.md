# Median Heap

[![Crates.io](https://img.shields.io/crates/v/medianheap.svg)](https://crates.io/crates/medianheap)
[![Documentation](https://docs.rs/medianheap/badge.svg)](https://docs.rs/medianheap)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
medianheap = "0.4"
```

## Example

Elements of a `MedianHeap` must be `Ord`. If you want to create a `MedianHeap` containing floating point numbers, you can use the [`ordered-float`](https://crates.io/crates/ordered-float) crate.

```rust
let mut heap = MedianHeap::new();

assert_eq!(heap.median(), None);

heap.push(1);
assert_eq!(heap.median(), Some(Median::Single(&1)));

heap.push(3);
assert_eq!(heap.median(), Some(Median::Pair(&1, &3)));
```
