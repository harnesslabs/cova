use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_algebra::tensors::{DMatrix, DVector};
use cova_solver::admm::AdmmSolver;

fn bench_admm(c: &mut Criterion) {
    let mut group = c.benchmark_group("Admm");

    group.bench_function("lasso_small", |b| b.iter(|| {
        let a = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 0.0, 1.0]);
        let b_vec = DVector::from_vec(vec![1.0, 2.0]);
        let lambda = 0.1;
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_lasso(black_box(&a), black_box(&b_vec), black_box(lambda)).unwrap())
    }));

    group.bench_function("lasso_medium", |b| b.iter_with_large_drop(|| {
        let n_vars = 10;
        let n_obs = 20;
        let mut a = DMatrix::zeros(n_obs, n_vars);
        for i in 0..n_obs {
            for j in 0..n_vars {
                a[(i, j)] = ((i + j) % 5) as f64 * 0.1;
            }
        }
        let b_vec = DVector::from_element(n_obs, 2.0);
        let lambda = 0.1;
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_lasso(black_box(&a), black_box(&b_vec), black_box(lambda)).unwrap())
    }));

    group.bench_function("basis_pursuit_small", |b| b.iter(|| {
        let a = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let b_vec = DVector::from_vec(vec![1.0]);
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_basis_pursuit(black_box(&a), black_box(&b_vec)).unwrap())
    }));

    group.bench_function("basis_pursuit_medium", |b| b.iter_with_large_drop(|| {
        let n_vars = 10;
        let n_eq = 5; // Under-determined: fewer equations than variables
        let mut a = DMatrix::zeros(n_eq, n_vars);
        for i in 0..n_eq {
            for j in 0..n_vars {
                a[(i, j)] = ((i * j) % 3) as f64 * 0.5; // somewhat sparse matrix
            }
        }
        // Ensure some variables can satisfy Ax = b
        let mut b_vec = DVector::zeros(n_eq);
        for i in 0..n_eq {
            b_vec[i] = a[(i, 0)] * 1.0; 
        }
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_basis_pursuit(black_box(&a), black_box(&b_vec)).unwrap())
    }));

    group.finish();
}

fn bench_admm_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Admm Large");

    group.bench_function("lasso_large", |b| b.iter_with_large_drop(|| {
        let n_vars = 50;
        let n_obs = 100;
        let mut a = DMatrix::zeros(n_obs, n_vars);
        for i in 0..n_obs {
            for j in 0..n_vars {
                a[(i, j)] = ((i * 7 + j * 13 + 3) % 97) as f64 * 0.01;
            }
        }
        let b_vec = DVector::from_element(n_obs, 2.0);
        let lambda = 0.1;
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_lasso(black_box(&a), black_box(&b_vec), black_box(lambda)).unwrap())
    }));

    group.bench_function("basis_pursuit_large", |b| b.iter_with_large_drop(|| {
        let n_vars = 50;
        let n_eq = 25; // Under-determined: fewer equations than variables
        let mut a = DMatrix::zeros(n_eq, n_vars);
        for i in 0..n_eq {
            for j in 0..n_vars {
                a[(i, j)] = ((i * 7 + j * 13 + 3) % 97) as f64 * 0.01;
            }
        }
        let mut b_vec = DVector::zeros(n_eq);
        for i in 0..n_eq {
            for j in 0..n_vars {
                b_vec[i] += a[(i, j)] * 1.0;
            }
        }
        
        let mut solver = AdmmSolver::new();
        black_box(solver.solve_basis_pursuit(black_box(&a), black_box(&b_vec)).unwrap())
    }));

    group.finish();
}

criterion_group!(benches, bench_admm, bench_admm_large);
criterion_main!(benches);
