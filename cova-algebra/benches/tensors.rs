use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::tensors::{DMatrix, MatrixBuilder};
use cova_algebra::tensors::{rref_with_pivots, image, kernel};

fn bench_tensors(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tensors");

    let matrix: DMatrix<f64> = MatrixBuilder::new()
        .row([1.0, 2.0, 3.0])
        .row([4.0, 5.0, 6.0])
        .row([7.0, 8.0, 9.0])
        .build();

    group.bench_function("rref_with_pivots", |b| b.iter(|| {
        rref_with_pivots(black_box(&matrix))
    }));

    group.bench_function("image", |b| b.iter(|| {
        image(black_box(&matrix))
    }));

    group.bench_function("kernel", |b| b.iter(|| {
        kernel(black_box(&matrix))
    }));

    group.finish();
}

criterion_group!(benches, bench_tensors);
criterion_main!(benches);
