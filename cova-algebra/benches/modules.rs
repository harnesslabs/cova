#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use cova_algebra::modules::tropical::{BilinearForm, TropicalAlgebra, TropicalElement};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_tropical(c: &mut Criterion) {
  let mut group = c.benchmark_group("Tropical Algebra");

  let a = TropicalElement::new(3.0);
  let b = TropicalElement::new(5.0);

  group.bench_function("element_add", |b_bench| b_bench.iter(|| black_box(a) + black_box(b)));
  group.bench_function("element_mul", |b_bench| b_bench.iter(|| black_box(a) * black_box(b)));

  let matrix = [[TropicalElement::new(1.0), TropicalElement::new(2.0)], [
    TropicalElement::new(2.0),
    TropicalElement::new(1.0),
  ]];
  let bilinear_form = BilinearForm::new(matrix);
  let algebra = TropicalAlgebra::new(bilinear_form);
  let x = [TropicalElement::new(3.0), TropicalElement::new(4.0)];
  let y = [TropicalElement::new(5.0), TropicalElement::new(6.0)];

  group.bench_function("bilinear_form_eval", |b_bench| {
    b_bench.iter(|| black_box(algebra.evaluate(black_box(&x), black_box(&y))))
  });

  group.finish();
}

fn bench_tropical_large(c: &mut Criterion) {
  let mut group = c.benchmark_group("Tropical Algebra 8x8");

  // 8×8 symmetric bilinear form — O(N²) = 64 iterations vs. 4 for 2×2
  let te = |v: f64| TropicalElement::new(v);
  let matrix: [[TropicalElement<f64>; 8]; 8] = [
    [te(1.0), te(2.0), te(3.0), te(4.0), te(5.0), te(6.0), te(7.0), te(8.0)],
    [te(2.0), te(1.0), te(2.0), te(3.0), te(4.0), te(5.0), te(6.0), te(7.0)],
    [te(3.0), te(2.0), te(1.0), te(2.0), te(3.0), te(4.0), te(5.0), te(6.0)],
    [te(4.0), te(3.0), te(2.0), te(1.0), te(2.0), te(3.0), te(4.0), te(5.0)],
    [te(5.0), te(4.0), te(3.0), te(2.0), te(1.0), te(2.0), te(3.0), te(4.0)],
    [te(6.0), te(5.0), te(4.0), te(3.0), te(2.0), te(1.0), te(2.0), te(3.0)],
    [te(7.0), te(6.0), te(5.0), te(4.0), te(3.0), te(2.0), te(1.0), te(2.0)],
    [te(8.0), te(7.0), te(6.0), te(5.0), te(4.0), te(3.0), te(2.0), te(1.0)],
  ];
  let bilinear_form = BilinearForm::new(matrix);
  let algebra = TropicalAlgebra::new(bilinear_form);

  let x = [te(1.0), te(2.0), te(3.0), te(4.0), te(5.0), te(6.0), te(7.0), te(8.0)];
  let y = [te(8.0), te(7.0), te(6.0), te(5.0), te(4.0), te(3.0), te(2.0), te(1.0)];

  group.bench_function("bilinear_form_eval_8x8", |b_bench| {
    b_bench.iter(|| black_box(algebra.evaluate(black_box(&x), black_box(&y))))
  });

  group.finish();
}

criterion_group!(benches, bench_tropical, bench_tropical_large);
criterion_main!(benches);
