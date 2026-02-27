use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::tensors::{DMatrix, DVector, MatrixBuilder};
use cova_space::complexes::{Simplex, SimplicialComplex};
use cova_space::sheaf::Sheaf;

/// Mirror of the `simplicial_complex_1d` helper from sheaf.rs tests.
fn make_1d_sheaf() -> (
    Sheaf<SimplicialComplex, DVector<f64>>,
    Simplex,
    Simplex,
    Simplex,
) {
    let mut cc = SimplicialComplex::new();
    // join_element assigns IDs; MUST use the returned values as restriction keys
    let v0 = cc.join_element(Simplex::new(0, vec![0]));
    let v1 = cc.join_element(Simplex::new(0, vec![1]));
    let e01 = cc.join_element(Simplex::new(1, vec![0, 1]));

    // Key: (parent, child) where parent <= child.
    // Matrix maps F(parent) -> F(child), matching `is_global_section` usage.
    // v0 stalk: R^1, v1 stalk: R^2, e01 stalk: R^2
    let restrictions: HashMap<(Simplex, Simplex), DMatrix<f64>> = HashMap::from([
        ((v0.clone(), e01.clone()), MatrixBuilder::new().column([1.0, 2.0]).build()), // 2x1
        ((v1.clone(), e01.clone()), {
            MatrixBuilder::new().column([2.0, 0.0]).column([0.0, 2.0]).build() // 2x2
        }),
    ]);

    let sheaf = Sheaf::<SimplicialComplex, DVector<f64>>::new(cc, restrictions);
    (sheaf, v0, v1, e01)
}

fn bench_sheaf(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sheaf");

    let (sheaf, v0, v1, e01) = make_1d_sheaf();

    // A valid global section (matching the test assertions in sheaf.rs):
    //   v0(R^1): [2.0], v1(R^2): [1.0, 2.0], e01(R^2): [2.0, 4.0]
    //   Check: matrix(v0->e01) * [2.0] = [1.0,2.0] * [2.0] = [2.0,4.0] ✓
    let section = HashMap::from([
        (v0.clone(), DVector::from_row_slice(&[2.0])),
        (v1.clone(), DVector::from_row_slice(&[1.0, 2.0])),
        (e01.clone(), DVector::from_row_slice(&[2.0, 4.0])),
    ]);

    group.bench_function("is_global_section", |b| b.iter(|| {
        black_box(sheaf.is_global_section(black_box(&section)))
    }));

    group.bench_function("coboundary_dim0", |b| b.iter(|| {
        black_box(sheaf.coboundary(black_box(0)))
    }));

    group.bench_function("coboundary_dim1", |b| b.iter(|| {
        black_box(sheaf.coboundary(black_box(1)))
    }));

    group.finish();
}

criterion_group!(benches, bench_sheaf);
criterion_main!(benches);
