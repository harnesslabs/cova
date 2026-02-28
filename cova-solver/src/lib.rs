//! Cova Solvers Library
//!
//! This library provides various optimization solvers for convex problems.

pub mod admm;
mod error;
pub mod linear_programming;
pub mod traits;

pub use error::{SolverError, SolverResult};
pub use traits::{OptimizationProblem, Solution};
