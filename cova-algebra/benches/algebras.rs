#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::algebras::clifford::{CliffordAlgebra, Euclidean3D, Euclidean4D};
use cova_algebra::tensors::SVector;

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

fn bench_clifford_4d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Clifford Algebra 4D");
    let algebra = CliffordAlgebra::<f64, 4, Euclidean4D>::new();

    // Dense multivectors: all 16 components nonzero (2^4 = 16 basis blades)
    let coeffs_a: [f64; 16] = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let coeffs_b: [f64; 16] = [
        16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0,
        8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    ];
    let a = algebra.element(SVector::<f64, 16>::from_row_slice(&coeffs_a));
    let b = algebra.element(SVector::<f64, 16>::from_row_slice(&coeffs_b));

    group.bench_function("add_dense_4d", |bench| bench.iter(|| black_box(a.clone()) + black_box(b.clone())));
    group.bench_function("mul_dense_4d", |bench| bench.iter(|| black_box(a.clone()) * black_box(b.clone())));
    group.finish();
}

criterion_group!(benches, bench_clifford, bench_clifford_4d);
criterion_main!(benches);
