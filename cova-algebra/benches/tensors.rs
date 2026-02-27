use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::tensors::{DMatrix, MatrixBuilder};
use cova_algebra::tensors::{rref_with_pivots, image, kernel};

/// Build an n×n matrix with deterministic non-trivial entries.
fn make_matrix(n: usize) -> DMatrix<f64> {
    let mut data = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            // Produces a full-rank-ish matrix with varied entries
            data.push(((i * 7 + j * 13 + 3) % 97) as f64);
        }
    }
    DMatrix::from_row_slice(n, n, &data)
}

fn bench_tensors(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tensors");

    let matrix_3: DMatrix<f64> = MatrixBuilder::new()
        .row([1.0, 2.0, 3.0])
        .row([4.0, 5.0, 6.0])
        .row([7.0, 8.0, 9.0])
        .build();

    group.bench_function("rref_with_pivots", |b| b.iter(|| {
        rref_with_pivots(black_box(&matrix_3))
    }));

    group.bench_function("image", |b| b.iter(|| {
        image(black_box(&matrix_3))
    }));

    group.bench_function("kernel", |b| b.iter(|| {
        kernel(black_box(&matrix_3))
    }));

    group.finish();
}

fn bench_tensors_16x16(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tensors 16x16");
    let matrix = make_matrix(16);

    group.bench_function("rref_with_pivots_16x16", |b| b.iter(|| {
        rref_with_pivots(black_box(&matrix))
    }));

    group.bench_function("image_16x16", |b| b.iter(|| {
        image(black_box(&matrix))
    }));

    group.bench_function("kernel_16x16", |b| b.iter(|| {
        kernel(black_box(&matrix))
    }));

    group.finish();
}

fn bench_tensors_50x50(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tensors 50x50");
    let matrix = make_matrix(50);

    group.bench_function("rref_with_pivots_50x50", |b| b.iter(|| {
        rref_with_pivots(black_box(&matrix))
    }));

    group.bench_function("image_50x50", |b| b.iter(|| {
        image(black_box(&matrix))
    }));

    group.bench_function("kernel_50x50", |b| b.iter(|| {
        kernel(black_box(&matrix))
    }));

    group.finish();
}

criterion_group!(benches, bench_tensors, bench_tensors_16x16, bench_tensors_50x50);
criterion_main!(benches);
