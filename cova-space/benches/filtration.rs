use cova_algebra::tensors::SVector;
#[cfg(feature = "parallel")]
use cova_space::filtration::ParallelFiltration;
use cova_space::{
  cloud::Cloud,
  filtration::{Filtration, vietoris_rips::VietorisRips},
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

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
  let vr: VietorisRips<2, cova_space::complexes::Complex<cova_space::complexes::Simplex>> =
    VietorisRips::new();

  let params: Vec<f64> = vec![0.1, 0.5, 0.8, 1.2, 1.5];
  let _max_dim = 2; // Up to 2-simplices

  group.bench_function("vietoris_rips_build", |b| {
    b.iter(|| black_box(vr.build(black_box(&cloud), black_box(1.0), black_box(&()))))
  });

  group.bench_function("vietoris_rips_build_serial", |b| {
    b.iter(|| {
      black_box(vr.build_serial(black_box(&cloud), black_box(params.clone()), black_box(&())))
    })
  });

  #[cfg(feature = "parallel")]
  group.bench_function("vietoris_rips_build_parallel", |b| {
    b.iter(|| {
      black_box(vr.build_parallel(black_box(&cloud), black_box(params.clone()), black_box(&())))
    })
  });

  group.finish();
}

/// Generate n points evenly spaced on a unit circle in R²
fn circle_points(n: usize) -> Vec<SVector<f64, 2>> {
  (0..n)
    .map(|i| {
      let theta = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
      SVector::from([theta.cos(), theta.sin()])
    })
    .collect()
}

fn bench_filtration_20pts(c: &mut Criterion) {
  let mut group = c.benchmark_group("Filtration 20pts");

  let cloud = Cloud::new(circle_points(20));
  let vr: VietorisRips<2, cova_space::complexes::Complex<cova_space::complexes::Simplex>> =
    VietorisRips::new();
  let params: Vec<f64> = vec![0.3, 0.6, 0.9, 1.2, 1.5];

  group.bench_function("vietoris_rips_build_20pts", |b| {
    b.iter(|| black_box(vr.build(black_box(&cloud), black_box(0.8), black_box(&()))))
  });

  group.bench_function("vietoris_rips_build_serial_20pts", |b| {
    b.iter(|| {
      black_box(vr.build_serial(black_box(&cloud), black_box(params.clone()), black_box(&())))
    })
  });

  group.finish();
}

fn bench_filtration_50pts(c: &mut Criterion) {
  let mut group = c.benchmark_group("Filtration 50pts");

  let cloud = Cloud::new(circle_points(50));
  let vr: VietorisRips<2, cova_space::complexes::Complex<cova_space::complexes::Simplex>> =
    VietorisRips::new();
  let params: Vec<f64> = vec![0.3, 0.6, 0.9, 1.2, 1.5];

  group.bench_function("vietoris_rips_build_50pts", |b| {
    b.iter(|| black_box(vr.build(black_box(&cloud), black_box(0.5), black_box(&()))))
  });

  group.bench_function("vietoris_rips_build_serial_50pts", |b| {
    b.iter(|| {
      black_box(vr.build_serial(black_box(&cloud), black_box(params.clone()), black_box(&())))
    })
  });

  group.finish();
}

criterion_group!(benches, bench_filtration, bench_filtration_20pts, bench_filtration_50pts);
criterion_main!(benches);
