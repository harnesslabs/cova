use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_space::complexes::{Simplex, SimplicialComplex};
use cova_space::homology::Chain;
use cova_algebra::algebras::boolean::Boolean;

fn bench_homology(c: &mut Criterion) {
    let mut group = c.benchmark_group("Homology");

    let mut complex = SimplicialComplex::new();
    let triangle = Simplex::new(2, vec![0, 1, 2]);
    complex.join_element(triangle.clone());

    let chain = Chain::<SimplicialComplex, Boolean>::from_item_and_coeff(&complex, triangle, Boolean(true));

    group.bench_function("boundary", |b| b.iter(|| {
        black_box(chain.boundary())
    }));

    group.finish();
}

fn bench_homology_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Homology Large");

    // Tetrahedron boundary (3-simplex with 4 triangular faces)
    let mut complex_tet = SimplicialComplex::new();
    let tetrahedron = Simplex::new(3, vec![0, 1, 2, 3]);
    complex_tet.join_element(tetrahedron.clone());

    let chain_tet = Chain::<SimplicialComplex, Boolean>::from_item_and_coeff(
        &complex_tet, tetrahedron, Boolean(true)
    );

    group.bench_function("boundary_tetrahedron", |b| b.iter(|| {
        black_box(chain_tet.boundary())
    }));

    // Multi-term chain: sum of 5 triangles sharing edges
    let mut complex_multi = SimplicialComplex::new();
    let triangles = vec![
        Simplex::new(2, vec![0, 1, 2]),
        Simplex::new(2, vec![1, 2, 3]),
        Simplex::new(2, vec![2, 3, 4]),
        Simplex::new(2, vec![3, 4, 5]),
        Simplex::new(2, vec![4, 5, 6]),
    ];
    for tri in &triangles {
        complex_multi.join_element(tri.clone());
    }

    // Build a chain that is the sum of all 5 triangles
    let mut multi_chain = Chain::<SimplicialComplex, Boolean>::from_item_and_coeff(
        &complex_multi, triangles[0].clone(), Boolean(true)
    );
    for tri in &triangles[1..] {
        let single = Chain::<SimplicialComplex, Boolean>::from_item_and_coeff(
            &complex_multi, tri.clone(), Boolean(true)
        );
        multi_chain = multi_chain + single;
    }

    group.bench_function("boundary_multi_chain", |b| b.iter(|| {
        black_box(multi_chain.boundary())
    }));

    group.finish();
}

criterion_group!(benches, bench_homology, bench_homology_large);
criterion_main!(benches);
