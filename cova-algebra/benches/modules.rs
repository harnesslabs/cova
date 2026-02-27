#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::modules::tropical::{BilinearForm, TropicalAlgebra, TropicalElement};

fn bench_tropical(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tropical Algebra");
    
    let a = TropicalElement::new(3.0);
    let b = TropicalElement::new(5.0);

    group.bench_function("element_add", |b_bench| b_bench.iter(|| black_box(a.clone()) + black_box(b.clone())));
    group.bench_function("element_mul", |b_bench| b_bench.iter(|| black_box(a.clone()) * black_box(b.clone())));

    let matrix = [
        [TropicalElement::new(1.0), TropicalElement::new(2.0)],
        [TropicalElement::new(2.0), TropicalElement::new(1.0)],
    ];
    let bilinear_form = BilinearForm::new(matrix);
    let algebra = TropicalAlgebra::new(bilinear_form);
    let x = [TropicalElement::new(3.0), TropicalElement::new(4.0)];
    let y = [TropicalElement::new(5.0), TropicalElement::new(6.0)];

    group.bench_function("bilinear_form_eval", |b_bench| b_bench.iter(|| {
        black_box(algebra.evaluate(black_box(&x), black_box(&y)))
    }));
    
    group.finish();
}

criterion_group!(benches, bench_tropical);
criterion_main!(benches);
