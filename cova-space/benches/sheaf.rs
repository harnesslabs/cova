use std::collections::HashMap;

use cova_algebra::tensors::{DMatrix, DVector, MatrixBuilder};
use cova_space::{
  complexes::{Simplex, SimplicialComplex},
  sheaf::Sheaf,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// Mirror of the `simplicial_complex_1d` helper from sheaf.rs tests.
fn make_1d_sheaf() -> (Sheaf<SimplicialComplex, DVector<f64>>, Simplex, Simplex, Simplex) {
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

/// 2D sheaf on a triangle complex (3 vertices, 3 edges, 1 face).
/// Stalks: vertices R^2, edges R^3, face R^3.
fn make_2d_sheaf() -> (Sheaf<SimplicialComplex, DVector<f64>>, Vec<Simplex>, Vec<Simplex>, Simplex)
{
  let mut cc = SimplicialComplex::new();
  let v0 = cc.join_element(Simplex::new(0, vec![0]));
  let v1 = cc.join_element(Simplex::new(0, vec![1]));
  let v2 = cc.join_element(Simplex::new(0, vec![2]));
  let e01 = cc.join_element(Simplex::new(1, vec![0, 1]));
  let e02 = cc.join_element(Simplex::new(1, vec![0, 2]));
  let e12 = cc.join_element(Simplex::new(1, vec![1, 2]));
  let f012 = cc.join_element(Simplex::new(2, vec![0, 1, 2]));

  // Restriction maps: vertex (R^2) -> edge (R^3), edge (R^3) -> face (R^3)
  let id3: DMatrix<f64> = DMatrix::identity(3, 3);
  let embed_2_to_3 = DMatrix::from_row_slice(3, 2, &[1.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

  let restrictions: HashMap<(Simplex, Simplex), DMatrix<f64>> = HashMap::from([
    // vertex -> edge restrictions (R^2 -> R^3)
    ((v0.clone(), e01.clone()), embed_2_to_3.clone()),
    ((v1.clone(), e01.clone()), embed_2_to_3.clone()),
    ((v0.clone(), e02.clone()), embed_2_to_3.clone()),
    ((v2.clone(), e02.clone()), embed_2_to_3.clone()),
    ((v1.clone(), e12.clone()), embed_2_to_3.clone()),
    ((v2.clone(), e12.clone()), embed_2_to_3.clone()),
    // edge -> face restrictions (R^3 -> R^3)
    ((e01.clone(), f012.clone()), id3.clone()),
    ((e02.clone(), f012.clone()), id3.clone()),
    ((e12.clone(), f012.clone()), id3.clone()),
  ]);

  let sheaf = Sheaf::<SimplicialComplex, DVector<f64>>::new(cc, restrictions);
  (sheaf, vec![v0, v1, v2], vec![e01, e02, e12], f012)
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

  group.bench_function("is_global_section", |b| {
    b.iter(|| black_box(sheaf.is_global_section(black_box(&section))))
  });

  group.bench_function("coboundary_dim0", |b| b.iter(|| black_box(sheaf.coboundary(black_box(0)))));

  group.bench_function("coboundary_dim1", |b| b.iter(|| black_box(sheaf.coboundary(black_box(1)))));

  group.finish();
}

fn bench_sheaf_2d(c: &mut Criterion) {
  let mut group = c.benchmark_group("Sheaf 2D");

  let (sheaf, vertices, edges, face) = make_2d_sheaf();

  // Build a section: vertices R^2, edges R^3, face R^3
  let v_stalk = DVector::from_row_slice(&[1.0, 1.0]);
  let e_stalk = DVector::from_row_slice(&[1.0, 1.0, 2.0]); // embed_2_to_3 * [1,1] = [1,1,2]
  let mut section = HashMap::new();
  for v in &vertices {
    section.insert(v.clone(), v_stalk.clone());
  }
  for e in &edges {
    section.insert(e.clone(), e_stalk.clone());
  }
  section.insert(face.clone(), e_stalk.clone()); // id3 * e_stalk = e_stalk

  group.bench_function("is_global_section_2d", |b| {
    b.iter(|| black_box(sheaf.is_global_section(black_box(&section))))
  });

  group
    .bench_function("coboundary_dim0_2d", |b| b.iter(|| black_box(sheaf.coboundary(black_box(0)))));

  group
    .bench_function("coboundary_dim1_2d", |b| b.iter(|| black_box(sheaf.coboundary(black_box(1)))));

  group.finish();
}

criterion_group!(benches, bench_sheaf, bench_sheaf_2d);
criterion_main!(benches);
