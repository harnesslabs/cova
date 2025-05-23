//! # Boolean Algebra Module
//!
//! This module provides an implementation of the Boolean field GF(2) through a wrapper
//! around Rust's `bool` type.
//!
//! ## Mathematical Structure
//!
//! The Boolean field consists of two elements:
//! - `false` (0): The additive identity
//! - `true` (1): The multiplicative identity
//!
//! Operations are defined as:
//! - Addition: Exclusive OR (XOR) operation
//! - Multiplication: Logical AND operation
//! - Negation: Identity operation (x = -x in GF(2))
//! - Multiplicative inverse: Identity for non-zero elements
//!
//! ## Algebraic Properties
//!
//! The implementation satisfies multiple algebraic structures:
//! - Field: A complete algebraic field with addition and multiplication
//! - Abelian Group: Under addition with identity element `false`
//! - Vector Space: Over itself as the scalar field
//!
//! ## Applications
//!
//! Boolean algebra has numerous applications in:
//! - Digital circuit design
//! - Logic operations
//! - Cryptography (especially in finite field arithmetic)
//! - Error correction codes
//!
//! ## Example
//!
//! ```
//! use cova_algebra::algebras::boolean::Boolean;
//!
//! // Create Boolean values
//! let a = Boolean(true); // 1
//! let b = Boolean(false); // 0
//!
//! // Addition (XOR)
//! assert_eq!(a + a, Boolean(false)); // 1 + 1 = 0
//! assert_eq!(a + b, Boolean(true)); // 1 + 0 = 1
//!
//! // Multiplication (AND)
//! assert_eq!(a * b, Boolean(false)); // 1 * 0 = 0
//! assert_eq!(a * a, Boolean(true)); // 1 * 1 = 1
//! ```

use super::*;

/// A wrapper around `bool` that implements algebraic operations.
///
/// This type implements both [`Additive`] and [`Multiplicative`] traits using
/// bitwise operations:
/// - Addition is implemented as XOR (`^`)
/// - Multiplication is implemented as AND (`&`)
///
/// This makes `Boolean` a field with two elements, where:
/// - `false` is the additive identity (0)
/// - `true` is the multiplicative identity (1)
///
/// # Examples
///
/// ```
/// use cova_algebra::algebras::boolean::Boolean;
///
/// let a = Boolean(true);
/// let b = Boolean(false);
///
/// // Addition (XOR)
/// assert_eq!(a + b, Boolean(true));
/// assert_eq!(a + a, Boolean(false)); // a + a = 0
///
/// // Multiplication (AND)
/// assert_eq!(a * b, Boolean(false));
/// assert_eq!(a * a, Boolean(true)); // a * a = a
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
  fn from(b: bool) -> Self { Self(b) }
}

impl From<Boolean> for bool {
  fn from(b: Boolean) -> Self { b.0 }
}

impl One for Boolean {
  fn one() -> Self { Self(true) }
}

impl Zero for Boolean {
  fn zero() -> Self { Self(false) }

  fn is_zero(&self) -> bool { !self.0 }
}

impl Add for Boolean {
  type Output = Self;

  /// Implements addition as XOR operation.
  ///
  /// This corresponds to the addition operation in the field GF(2).
  #[allow(clippy::suspicious_arithmetic_impl)]
  fn add(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }
}

impl Sub for Boolean {
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn sub(self, rhs: Self) -> Self::Output { self + rhs }
}

impl Neg for Boolean {
  type Output = Self;

  fn neg(self) -> Self::Output { self }
}

impl SubAssign for Boolean {
  /// Implements subtraction assignment as XOR operation.
  #[allow(clippy::suspicious_op_assign_impl)]
  fn sub_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

impl AddAssign for Boolean {
  /// Implements addition assignment as XOR operation.
  #[allow(clippy::suspicious_op_assign_impl)]
  fn add_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

impl Mul for Boolean {
  type Output = Self;

  /// Implements multiplication as AND operation.
  ///
  /// This corresponds to the multiplication operation in the field GF(2).
  fn mul(self, rhs: Self) -> Self::Output { Self(self.0 && rhs.0) }
}

impl MulAssign for Boolean {
  /// Implements multiplication assignment as AND operation.
  #[allow(clippy::suspicious_op_assign_impl)]
  fn mul_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl Div for Boolean {
  type Output = Self;

  #[allow(clippy::suspicious_arithmetic_impl)]
  fn div(self, rhs: Self) -> Self::Output { self * rhs }
}

impl DivAssign for Boolean {
  /// Implements division assignment as AND operation.
  #[allow(clippy::suspicious_op_assign_impl)]
  fn div_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl Additive for Boolean {}
impl Multiplicative for Boolean {}

impl groups::Group for Boolean {
  fn identity() -> Self { Self(false) }

  fn inverse(&self) -> Self { Self(!self.0) }
}

impl groups::AbelianGroup for Boolean {}

impl rings::Ring for Boolean {}

impl rings::Field for Boolean {
  fn multiplicative_inverse(&self) -> Self { *self }
}

impl modules::LeftModule for Boolean {
  type Ring = Self;
}

impl modules::RightModule for Boolean {
  type Ring = Self;
}

impl modules::TwoSidedModule for Boolean {
  type Ring = Self;
}

impl modules::VectorSpace for Boolean {}
