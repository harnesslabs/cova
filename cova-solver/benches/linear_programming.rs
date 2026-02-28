use cova_algebra::tensors::{DMatrix, DVector};
use cova_solver::{linear_programming::LinearProgram, traits::OptimizationProblem};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

// Creates a small 2-variable, 2-constraint LP (similar to test_simple_lp)
fn setup_small_lp() -> LinearProgram {
  let c = DVector::from_vec(vec![-1.0, -2.0]);
  let a = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 2.0, 1.0]);
  let b = DVector::from_vec(vec![3.0, 4.0]);
  LinearProgram::new(c, a, b).unwrap()
}

// Creates a medium 10-variable, 20-constraint LP
fn setup_medium_lp() -> LinearProgram {
  let n = 10;
  let m = 20;
  let c = DVector::from_element(n, -1.0);
  let mut a = DMatrix::zeros(m, n);
  for i in 0..m {
    for j in 0..n {
      a[(i, j)] = ((i + j) % 5) as f64 * 0.1;
    }
  }
  let b = DVector::from_element(m, 5.0);
  LinearProgram::new(c, a, b).unwrap()
}

// Creates a large 50-variable, 100-constraint LP
fn setup_large_lp() -> LinearProgram {
  let n = 50;
  let m = 100;
  let c = DVector::from_element(n, -1.0);
  let mut a = DMatrix::zeros(m, n);
  for i in 0..m {
    for j in 0..n {
      a[(i, j)] = ((i * 7 + j * 13 + 3) % 97) as f64 * 0.01;
    }
  }
  let b = DVector::from_element(m, 10.0);
  LinearProgram::new(c, a, b).unwrap()
}

// Creates an xlarge 200-variable, 500-constraint LP
fn setup_xlarge_lp() -> LinearProgram {
  let n = 200;
  let m = 500;
  let c = DVector::from_element(n, -1.0);
  let mut a = DMatrix::zeros(m, n);
  for i in 0..m {
    for j in 0..n {
      a[(i, j)] = ((i * 7 + j * 13 + 3) % 97) as f64 * 0.01;
    }
  }
  let b = DVector::from_element(m, 20.0);
  LinearProgram::new(c, a, b).unwrap()
}

fn bench_linear_programming(c: &mut Criterion) {
  let mut group = c.benchmark_group("LinearProgramming");

  let small_lp = setup_small_lp();
  let medium_lp = setup_medium_lp();
  let small_x = DVector::from_vec(vec![1.0, 1.0]);
  let medium_x = DVector::from_element(10, 1.0);

  // Bench Isolated Cost Evaluation
  group.bench_function("cost_evaluation_small", |b| {
    b.iter(|| black_box(small_lp.cost(black_box(&small_x))))
  });

  group.bench_function("cost_evaluation_medium", |b| {
    b.iter(|| black_box(medium_lp.cost(black_box(&medium_x))))
  });

  // Bench Isolated Gradient Evaluation
  group.bench_function("gradient_evaluation_small", |b| {
    b.iter(|| black_box(small_lp.gradient(black_box(&small_x))))
  });

  group.bench_function("gradient_evaluation_medium", |b| {
    b.iter(|| black_box(medium_lp.gradient(black_box(&medium_x))))
  });

  // Bench End-to-End Solve
  group.bench_function("solve_small", |b| b.iter(|| black_box(small_lp.solve().unwrap())));

  group.bench_function("solve_medium", |b| b.iter(|| black_box(medium_lp.solve().unwrap())));

  group.finish();
}

fn bench_linear_programming_large(c: &mut Criterion) {
  let mut group = c.benchmark_group("LinearProgramming Large");

  let large_lp = setup_large_lp();
  let large_x = DVector::from_element(50, 1.0);

  group.bench_function("cost_evaluation_large", |b| {
    b.iter(|| black_box(large_lp.cost(black_box(&large_x))))
  });

  group.bench_function("gradient_evaluation_large", |b| {
    b.iter(|| black_box(large_lp.gradient(black_box(&large_x))))
  });

  group.bench_function("solve_large", |b| b.iter(|| black_box(large_lp.solve().unwrap())));

  group.finish();
}

fn bench_linear_programming_xlarge(c: &mut Criterion) {
  let mut group = c.benchmark_group("LinearProgramming XLarge");

  let xlarge_lp = setup_xlarge_lp();

  group.bench_function("solve_xlarge", |b| b.iter(|| black_box(xlarge_lp.solve().unwrap())));

  group.finish();
}

criterion_group!(
  benches,
  bench_linear_programming,
  bench_linear_programming_large,
  bench_linear_programming_xlarge
);
criterion_main!(benches);
