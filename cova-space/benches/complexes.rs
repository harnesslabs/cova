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

criterion_group!(benches, bench_complexes);
criterion_main!(benches);
