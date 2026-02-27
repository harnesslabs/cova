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

criterion_group!(benches, bench_homology);
criterion_main!(benches);
