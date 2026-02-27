//! Linear Programming using self-contained gradient descent
//!
//! This module implements linear programming solvers using a penalty-based
//! steepest descent method. The solver is self-contained and only depends
//! on `cova-algebra` types (`DVector`, `DMatrix`).

use cova_algebra::tensors::{DMatrix, DVector};

use crate::{
  traits::{OptimizationProblem, Solution},
  SolverError, SolverResult,
};

/// Linear programming problem: minimize c^T x subject to Ax <= b, x >= 0
#[derive(Debug, Clone)]
pub struct LinearProgram {
  /// Objective function coefficients
  pub c:              DVector<f64>,
  /// Constraint matrix A
  pub a:              DMatrix<f64>,
  /// Constraint bounds b
  pub b:              DVector<f64>,
  /// Solver tolerance
  pub tolerance:      f64,
  /// Maximum iterations
  pub max_iterations: u64,
}

impl LinearProgram {
  /// Create a new linear programming problem
  pub fn new(c: DVector<f64>, a: DMatrix<f64>, b: DVector<f64>) -> SolverResult<Self> {
    // Validate dimensions
    let n = c.len();
    let m = b.len();

    if a.nrows() != m {
      return Err(SolverError::DimensionMismatch {
        expected: format!("A matrix rows {}", m),
        actual:   format!("{}", a.nrows()),
      });
    }

    if a.ncols() != n {
      return Err(SolverError::DimensionMismatch {
        expected: format!("A matrix cols {}", n),
        actual:   format!("{}", a.ncols()),
      });
    }

    Ok(Self { c, a, b, tolerance: 1e-6, max_iterations: 1000 })
  }

  /// Set solver tolerance
  pub fn with_tolerance(mut self, tolerance: f64) -> Self {
    self.tolerance = tolerance;
    self
  }

  /// Set maximum iterations
  pub fn with_max_iterations(mut self, max_iterations: u64) -> Self {
    self.max_iterations = max_iterations;
    self
  }

  /// Evaluate the penalised cost function at `x`.
  ///
  /// cost(x) = c^T x + penalty * Σ max(0, (Ax-b)_i)² + penalty * Σ max(0, -x_i)²
  pub fn cost(&self, x: &DVector<f64>) -> f64 {
    let penalty = 1000.0;

    // Original objective
    let objective = self.c.dot(x);

    // Constraint violations: Ax <= b
    let constraint_violations = &self.a * x - &self.b;
    let constraint_penalty: f64 = constraint_violations.iter().map(|&v| v.max(0.0).powi(2)).sum();

    // Non-negativity violations: x >= 0
    let nonnegativity_penalty: f64 = x.iter().map(|&v| (-v).max(0.0).powi(2)).sum();

    objective + penalty * (constraint_penalty + nonnegativity_penalty)
  }

  /// Evaluate the gradient of the penalised cost function at `x`.
  pub fn gradient(&self, x: &DVector<f64>) -> DVector<f64> {
    let penalty = 1000.0;
    let mut grad = self.c.clone();

    // Gradient from constraint violations: Ax <= b
    let constraint_violations = &self.a * x - &self.b;
    for (i, &violation) in constraint_violations.iter().enumerate() {
      if violation > 0.0 {
        for j in 0..x.len() {
          grad[j] += 2.0 * penalty * violation * self.a[(i, j)];
        }
      }
    }

    // Gradient from non-negativity violations: x >= 0
    for (i, &xi) in x.iter().enumerate() {
      if xi < 0.0 {
        grad[i] += -2.0 * penalty * xi;
      }
    }

    grad
  }
}

impl OptimizationProblem for LinearProgram {
  fn dimension(&self) -> usize { self.c.len() }

  fn solve(&self) -> SolverResult<Solution> {
    let n = self.dimension();
    let mut x = DVector::zeros(n);
    let mut step_size = 1e-3;
    let mut prev_cost = self.cost(&x);

    for iteration in 0..self.max_iterations {
      let grad = self.gradient(&x);
      let grad_norm = grad.norm();

      if grad_norm < self.tolerance {
        let objective_value = self.c.dot(&x);
        return Ok(Solution {
          x,
          objective_value,
          iterations: iteration + 1,
          converged: true,
          termination: "GradientNormConverged".to_string(),
        });
      }

      // Backtracking line search
      let direction = -&grad / grad_norm;
      let mut alpha = step_size;
      let armijo_c = 1e-4;

      for _ in 0..20 {
        let x_new = &x + alpha * &direction;
        let new_cost = self.cost(&x_new);
        if new_cost <= prev_cost + armijo_c * alpha * grad.dot(&direction) {
          break;
        }
        alpha *= 0.5;
      }

      x = &x + alpha * &direction;
      let new_cost = self.cost(&x);

      // Adaptive step size
      if new_cost < prev_cost {
        step_size *= 1.1;
      } else {
        step_size *= 0.5;
      }
      prev_cost = new_cost;
    }

    let objective_value = self.c.dot(&x);
    Ok(Solution {
      x,
      objective_value,
      iterations: self.max_iterations,
      converged: false,
      termination: "MaxIterations".to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_lp() {
    // minimize -x1 - 2*x2
    // subject to x1 + x2 <= 3
    //           2*x1 + x2 <= 4
    //           x1, x2 >= 0

    let c = DVector::from_vec(vec![-1.0, -2.0]);
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 2.0, 1.0]);
    let b = DVector::from_vec(vec![3.0, 4.0]);

    let lp = LinearProgram::new(c, a, b).unwrap();
    let result = lp.solve().unwrap();

    // Check that solution is reasonable (may not be exact due to penalty method)
    assert!(result.x[0] >= -0.1); // x1 >= 0 (with tolerance)
    assert!(result.x[1] >= -0.1); // x2 >= 0 (with tolerance)

    // Check constraints are approximately satisfied
    let constraint1 = result.x[0] + result.x[1];
    let constraint2 = 2.0 * result.x[0] + result.x[1];
    assert!(constraint1 <= 3.1); // Allow some tolerance
    assert!(constraint2 <= 4.1);
  }
}
