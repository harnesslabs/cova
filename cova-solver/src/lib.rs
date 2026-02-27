//! Cova Solvers Library
//!
//! This library provides various optimization solvers for convex problems.

pub mod admm;
pub mod linear_programming;
mod error;
pub mod traits;

pub use error::{SolverError, SolverResult};
pub use traits::{OptimizationProblem, Solution};
