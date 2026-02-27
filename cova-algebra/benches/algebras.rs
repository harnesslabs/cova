#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::algebras::clifford::{CliffordAlgebra, Euclidean3D};

fn bench_clifford(c: &mut Criterion) {
    let mut group = c.benchmark_group("Clifford Algebra");
    let algebra = CliffordAlgebra::<f64, 3, Euclidean3D>::new();
    let e1 = algebra.blade([0]);
    let e2 = algebra.blade([1]);
    let e1_e2 = e1.clone() * e2.clone();
    
    group.bench_function("add", |b| b.iter(|| black_box(e1.clone()) + black_box(e1_e2.clone())));
    group.bench_function("mul", |b| b.iter(|| black_box(e1.clone()) * black_box(e1_e2.clone())));
    group.finish();
}

criterion_group!(benches, bench_clifford);
criterion_main!(benches);
