// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-core
//!
//! Core expression types and canonicalization for the Math Monster system.
//!
//! This crate provides:
//! - [`Expr`] - The core mathematical expression enum
//! - [`Symbol`] - Interned variable symbols for fast comparison
//! - Canonicalization - Converting expressions to a unique normal form
//! - Evaluation - Numerical evaluation of expressions
//! - Parsing - String to expression conversion
//!
//! ## Example
//!
//! ```rust
//! use mm_core::{Expr, SymbolTable};
//!
//! let mut symbols = SymbolTable::new();
//! let x = symbols.intern("x");
//!
//! // Create expression: x² + 2x + 1
//! let expr = Expr::Add(
//!     Box::new(Expr::Pow(
//!         Box::new(Expr::Var(x)),
//!         Box::new(Expr::Const(2.into())),
//!     )),
//!     Box::new(Expr::Add(
//!         Box::new(Expr::Mul(
//!             Box::new(Expr::Const(2.into())),
//!             Box::new(Expr::Var(x)),
//!         )),
//!         Box::new(Expr::Const(1.into())),
//!     )),
//! );
//!
//! // Canonicalize for comparison
//! let canonical = expr.canonicalize();
//! ```

pub mod canon;
pub mod error;
pub mod eval;
pub mod expr;
pub mod parse;
pub mod proof;
pub mod rational;
pub mod search;
pub mod symbol;

pub use error::MathError;
pub use expr::{Expr, Factor, Term};
pub use proof::{
    Constraint, Domain, Goal, GoalId, GoalStatus, HypId, Hypothesis, HypothesisOrigin, Proof,
    ProofState, ProofStep, Variable,
};
pub use rational::Rational;
pub use search::{NeuralHint, ProofSearchEngine, SearchConfig, SearchStats};
pub use symbol::{Symbol, SymbolTable};
