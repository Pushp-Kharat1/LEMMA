// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Error types for the LEMMA system.

use thiserror::Error;

/// Errors that can occur in mathematical operations.
#[derive(Error, Debug, Clone)]
pub enum MathError {
    /// Error during parsing.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Division by zero.
    #[error("Division by zero")]
    DivisionByZero,

    /// Undefined variable.
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    /// Domain error (e.g., sqrt of negative, log of non-positive).
    #[error("Domain error: {0}")]
    DomainError(String),

    /// No applicable rule found.
    #[error("No applicable rule")]
    NoApplicableRule,

    /// Verification failed.
    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    /// Search exhausted without finding solution.
    #[error("No solution found")]
    NoSolutionFound,

    /// Maximum depth exceeded.
    #[error("Maximum depth exceeded")]
    MaxDepthExceeded,
}

/// Result type for math operations.
pub type MathResult<T> = Result<T, MathError>;
