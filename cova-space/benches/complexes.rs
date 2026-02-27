use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_space::complexes::{Simplex, SimplicialComplex, Cube, CubicalComplex};
use cova_algebra::algebras::boolean::Boolean;

fn bench_complexes(c: &mut Criterion) {
    let mut group = c.benchmark_group("Complexes");

    group.bench_function("simplicial_complex_instantiation", |b| b.iter(|| {
        let mut complex = SimplicialComplex::new();
        let triangle = Simplex::new(2, vec![0, 1, 2]);
        complex.join_element(black_box(triangle));
        black_box(complex)
    }));

    group.bench_function("cubical_complex_instantiation", |b| b.iter(|| {
        let mut complex = CubicalComplex::new();
        // A 2D cube using intervals. (e.g. [0,1] x [0,1])
        // Simplified constructor assuming standard binary representation for vertices
        let cube = Cube::new(2, vec![0, 1, 2, 3]);
        complex.join_element(black_box(cube));
        black_box(complex)
    }));

    let mut complex = SimplicialComplex::new();
    let tetrahedron = Simplex::new(3, vec![0, 1, 2, 3]);
    complex.join_element(tetrahedron);

    group.bench_function("simplicial_homology_computation", |b| b.iter(|| {
        // Compute homology over boolean coefficients for Betti numbers
        black_box(complex.homology::<Boolean>(black_box(0)));
        black_box(complex.homology::<Boolean>(black_box(1)));
        black_box(complex.homology::<Boolean>(black_box(2)));
    }));

    group.finish();
}

fn bench_complexes_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Complexes Large");

    // Build a triangulated surface: 8 triangles from 6 vertices (octahedron-like)
    group.bench_function("simplicial_build_surface", |b| b.iter(|| {
        let mut complex = SimplicialComplex::new();
        // Triangles forming a closed surface
        let triangles = vec![
            Simplex::new(2, vec![0, 1, 2]),
            Simplex::new(2, vec![0, 1, 3]),
            Simplex::new(2, vec![0, 2, 4]),
            Simplex::new(2, vec![0, 3, 4]),
            Simplex::new(2, vec![1, 2, 5]),
            Simplex::new(2, vec![1, 3, 5]),
            Simplex::new(2, vec![2, 4, 5]),
            Simplex::new(2, vec![3, 4, 5]),
        ];
        for tri in triangles {
            complex.join_element(black_box(tri));
        }
        black_box(complex)
    }));

    // 4-simplex (5 vertices) — larger boundary matrices for homology
    let mut complex_4 = SimplicialComplex::new();
    let four_simplex = Simplex::new(4, vec![0, 1, 2, 3, 4]);
    complex_4.join_element(four_simplex);

    group.bench_function("simplicial_homology_4simplex", |b| b.iter(|| {
        black_box(complex_4.homology::<Boolean>(black_box(0)));
        black_box(complex_4.homology::<Boolean>(black_box(1)));
        black_box(complex_4.homology::<Boolean>(black_box(2)));
        black_box(complex_4.homology::<Boolean>(black_box(3)));
    }));

    group.finish();
}

criterion_group!(benches, bench_complexes, bench_complexes_large);
criterion_main!(benches);
