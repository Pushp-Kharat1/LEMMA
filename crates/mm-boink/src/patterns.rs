// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Pattern matching and solving for standard mathematical forms.
//!
//! This module provides functions to recognize AND SOLVE high-level patterns,
//! such as standard integral forms. Unlike the stub version, this actually
//! computes results for recognized patterns.

use mm_core::{Expr, Rational};

/// Represents standard forms of integrals for pattern matching.
#[derive(Debug, Clone)]
pub enum IntegralForm {
    /// Power rule: ∫x^n dx = x^(n+1)/(n+1) for n ≠ -1
    PowerRule { base: Expr, exponent: Expr },
    /// Logarithmic: ∫1/x dx = ln|x|
    Logarithmic,
    /// Exponential: ∫e^x dx = e^x
    Exponential { argument: Expr },
    /// Trig sin: ∫sin(x) dx = -cos(x)
    TrigSin { argument: Expr },
    /// Trig cos: ∫cos(x) dx = sin(x)
    TrigCos { argument: Expr },
    /// Constant: ∫c dx = cx
    Constant(Rational),
    /// Variable: ∫x dx = x²/2
    Variable,
    /// Unknown form - needs more sophisticated methods
    Unknown,
}

/// Attempts to match an expression against standard integral patterns.
pub fn match_integral_pattern(expr: &Expr) -> Option<IntegralForm> {
    match expr {
        // Constant: just a number
        Expr::Const(c) => Some(IntegralForm::Constant(*c)),

        // Variable: x
        Expr::Var(_) => Some(IntegralForm::Variable),

        // Power rule: x^n
        Expr::Pow(base, exp) => Some(IntegralForm::PowerRule {
            base: *base.clone(),
            exponent: *exp.clone(),
        }),

        // Square root: √x = x^(1/2)
        Expr::Sqrt(inner) => {
            if matches!(inner.as_ref(), Expr::Var(_)) {
                Some(IntegralForm::PowerRule {
                    base: *inner.clone(),
                    exponent: Expr::frac(1, 2),
                })
            } else {
                None
            }
        }

        // Logarithmic: 1/x
        Expr::Div(num, denom) => {
            if is_one(num) && matches!(denom.as_ref(), Expr::Var(_)) {
                Some(IntegralForm::Logarithmic)
            } else {
                None
            }
        }

        // Exponential: e^x
        Expr::Exp(arg) => Some(IntegralForm::Exponential {
            argument: *arg.clone(),
        }),

        // Trig patterns
        Expr::Sin(arg) => Some(IntegralForm::TrigSin {
            argument: *arg.clone(),
        }),
        Expr::Cos(arg) => Some(IntegralForm::TrigCos {
            argument: *arg.clone(),
        }),

        _ => None,
    }
}

/// Solve an integral given its detected form.
///
/// Returns the antiderivative expression (without +C).
pub fn solve_integral(form: &IntegralForm, var: mm_core::Symbol) -> Option<Expr> {
    match form {
        IntegralForm::Constant(c) => {
            // ∫c dx = cx
            Some(Expr::Mul(
                Box::new(Expr::Const(*c)),
                Box::new(Expr::Var(var)),
            ))
        }

        IntegralForm::Variable => {
            // ∫x dx = x²/2
            Some(Expr::Div(
                Box::new(Expr::Pow(Box::new(Expr::Var(var)), Box::new(Expr::int(2)))),
                Box::new(Expr::int(2)),
            ))
        }

        IntegralForm::PowerRule { base, exponent } => {
            // ∫x^n dx = x^(n+1)/(n+1) for n ≠ -1
            if let Expr::Const(n) = exponent {
                if *n == Rational::from(-1) {
                    // This is ln|x| case
                    return Some(Expr::Ln(Box::new(Expr::Abs(Box::new(base.clone())))));
                }

                let n_plus_1 = *n + Rational::from(1);
                Some(Expr::Div(
                    Box::new(Expr::Pow(
                        Box::new(base.clone()),
                        Box::new(Expr::Const(n_plus_1)),
                    )),
                    Box::new(Expr::Const(n_plus_1)),
                ))
            } else {
                // Non-constant exponent - can't solve simply
                None
            }
        }

        IntegralForm::Logarithmic => {
            // ∫1/x dx = ln|x|
            Some(Expr::Ln(Box::new(Expr::Abs(Box::new(Expr::Var(var))))))
        }

        IntegralForm::Exponential { argument } => {
            // ∫e^x dx = e^x (assuming argument is just x)
            if matches!(argument, Expr::Var(_)) {
                Some(Expr::Exp(Box::new(argument.clone())))
            } else {
                None // Chain rule needed
            }
        }

        IntegralForm::TrigSin { argument } => {
            // ∫sin(x) dx = -cos(x)
            if matches!(argument, Expr::Var(_)) {
                Some(Expr::Neg(Box::new(Expr::Cos(Box::new(argument.clone())))))
            } else {
                None
            }
        }

        IntegralForm::TrigCos { argument } => {
            // ∫cos(x) dx = sin(x)
            if matches!(argument, Expr::Var(_)) {
                Some(Expr::Sin(Box::new(argument.clone())))
            } else {
                None
            }
        }

        IntegralForm::Unknown => None,
    }
}

/// Helper to check if an expression is the constant 1.
fn is_one(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(c) if c.is_one())
}

// Tests require a SymbolTable to create valid symbols
// TODO: Add integration tests in examples/
