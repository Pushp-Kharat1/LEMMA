// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Pattern matching utilities for complex mathematical structures.
//!
//! This module provides functions to recognize high-level patterns in expressions,
//! such as standard integral forms, specific polynomial structures, or
//! trigonometric identities that are too complex for simple rule closures.

use mm_core::Expr;

/// Represents standard forms of integrals for pattern matching.
#[derive(Debug, Clone)]
pub enum IntegralForm {
    PowerRule(Expr, Expr), // base, exponent
    Logarithmic,           // 1/x
    Exponential(Expr),     // e^u
    TrigSin(Expr),         // sin(u)
    TrigCos(Expr),         // cos(u)
    Unknown,
}

/// Attempts to match an expression against standard integral patterns.
///
/// This is used by the integration rules to quickly identify which technique
/// or formula to apply.
pub fn match_integral_pattern(expr: &Expr) -> Option<IntegralForm> {
    match expr {
        // Power rule: x^n
        Expr::Pow(base, exp) => Some(IntegralForm::PowerRule(*base.clone(), *exp.clone())),

        // Logarithmic: 1/x or x^-1
        Expr::Div(num, _denom) if is_one(num) => Some(IntegralForm::Logarithmic),

        // Exponential: e^x
        // Note: In a real implementation this would check for Euler's number base
        Expr::Exp(arg) => Some(IntegralForm::Exponential(*arg.clone())),

        // Trig patterns
        Expr::Sin(arg) => Some(IntegralForm::TrigSin(*arg.clone())),
        Expr::Cos(arg) => Some(IntegralForm::TrigCos(*arg.clone())),

        _ => None,
    }
}

/// Helper to check if an expression is the constant 1.
fn is_one(expr: &Expr) -> bool {
    // This is a simplified check. In the full system, checking for 1
    // involves rational comparisons.
    matches!(expr, Expr::Const(c) if c.is_one())
}
