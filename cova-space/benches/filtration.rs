use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_space::cloud::Cloud;
use cova_space::filtration::{Filtration, vietoris_rips::VietorisRips};
use cova_algebra::tensors::SVector;

#[cfg(feature = "parallel")]
use cova_space::filtration::ParallelFiltration;

fn bench_filtration(c: &mut Criterion) {
    let mut group = c.benchmark_group("Filtration");

    // Generate a simple point cloud (e.g., vertices of a square + noise)
    let points = vec![
        SVector::from([0.0, 0.0]),
        SVector::from([1.0, 0.0]),
        SVector::from([0.0, 1.0]),
        SVector::from([1.0, 1.0]),
        SVector::from([0.5, 0.5]),
    ];
    let cloud = Cloud::new(points);
    let vr: VietorisRips<2, cova_space::complexes::Complex<cova_space::complexes::Simplex>> = VietorisRips::new();

    let params: Vec<f64> = vec![0.1, 0.5, 0.8, 1.2, 1.5];
    let max_dim = 2; // Up to 2-simplices

    group.bench_function("vietoris_rips_build", |b| b.iter(|| {
        black_box(vr.build(black_box(&cloud), black_box(1.0), black_box(&())))
    }));

    group.bench_function("vietoris_rips_build_serial", |b| b.iter(|| {
        black_box(vr.build_serial(black_box(&cloud), black_box(params.clone()), black_box(&())))
    }));

    #[cfg(feature = "parallel")]
    group.bench_function("vietoris_rips_build_parallel", |b| b.iter(|| {
        black_box(vr.build_parallel(black_box(&cloud), black_box(params.clone()), black_box(&())))
    }));

    group.finish();
}

criterion_group!(benches, bench_filtration);
criterion_main!(benches);
