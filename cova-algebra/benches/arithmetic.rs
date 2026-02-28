use cova_algebra::{groups::Group, modular, prime_field};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

modular!(Mod7, u32, 7);
prime_field!(Mod7);

modular!(Mod65537, u32, 65537);
prime_field!(Mod65537);

fn bench_modular(c: &mut Criterion) {
  let mut group = c.benchmark_group("Modular Arithmetic");
  let a = Mod7::new(3);
  let b = Mod7::new(5);

  group.bench_function("add", |b_bench| b_bench.iter(|| black_box(a) + black_box(b)));
  group.bench_function("mul", |b_bench| b_bench.iter(|| black_box(a) * black_box(b)));
  group.bench_function("inverse", |b_bench| b_bench.iter(|| black_box(a).inverse()));
  group.finish();
}

fn bench_modular_large_prime(c: &mut Criterion) {
  let mut group = c.benchmark_group("Modular Arithmetic Large Prime");
  let a = Mod65537::new(12345);
  let b = Mod65537::new(54321);

  group.bench_function("add_large_prime", |b_bench| b_bench.iter(|| black_box(a) + black_box(b)));
  group.bench_function("mul_large_prime", |b_bench| b_bench.iter(|| black_box(a) * black_box(b)));
  group.bench_function("inverse_large_prime", |b_bench| b_bench.iter(|| black_box(a).inverse()));
  group.finish();
}

criterion_group!(benches, bench_modular, bench_modular_large_prime);
criterion_main!(benches);
