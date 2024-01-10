use criterion::{criterion_group, criterion_main, Criterion};

use medianheap::MedianHeap;

fn benchmark(c: &mut Criterion) {
  c.bench_function("push with max_size", |b| {
    b.iter(|| {
      let mut heap = MedianHeap::with_max_size(512);

      for i in (0..8192).chain((0..8192).rev()) {
        heap.push(i);
      }
    })
  });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
