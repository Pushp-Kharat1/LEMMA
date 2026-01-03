// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-verifier
//!
//! Verification system for the Math Monster.
//!
//! Provides multiple levels of verification to ensure mathematical correctness:
//! - **Numerical**: Fast spot-checking at random points
//! - **Symbolic**: Canonical form comparison
//! - **Formal**: SMT solver proof (future)

pub mod numerical;
pub mod symbolic;

use mm_core::{Expr, MathError};
use mm_rules::{Rule, RuleContext};

/// Verification confidence level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerificationLevel {
    /// Fast numerical spot-checking.
    Numerical,
    /// Symbolic canonical form comparison.
    Symbolic,
    /// Full formal proof (SMT solver).
    Formal,
}

/// Result of verification.
#[derive(Debug, Clone)]
pub enum VerifyResult {
    /// Step is valid with given confidence.
    Valid { confidence: f64 },
    /// Step is invalid with reason.
    Invalid { reason: String },
    /// Could not determine (timeout, complexity).
    Unknown { reason: String },
}

impl VerifyResult {
    /// Check if the result is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self, VerifyResult::Valid { .. })
    }

    /// Get confidence if valid.
    pub fn confidence(&self) -> Option<f64> {
        match self {
            VerifyResult::Valid { confidence } => Some(*confidence),
            _ => None,
        }
    }
}

/// Check if an expression contains calculus operations (derivatives/integrals)
/// that cannot be numerically evaluated.
fn is_calculus_expr(expr: &Expr) -> bool {
    match expr {
        Expr::Derivative { .. } | Expr::Integral { .. } => true,
        Expr::Neg(e)
        | Expr::Sqrt(e)
        | Expr::Sin(e)
        | Expr::Cos(e)
        | Expr::Tan(e)
        | Expr::Ln(e)
        | Expr::Exp(e)
        | Expr::Abs(e) => is_calculus_expr(e),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => {
            is_calculus_expr(a) || is_calculus_expr(b)
        }
        Expr::Sum(terms) => terms.iter().any(|t| is_calculus_expr(&t.expr)),
        Expr::Product(factors) => factors
            .iter()
            .any(|f| is_calculus_expr(&f.base) || is_calculus_expr(&f.power)),
        Expr::Equation { lhs, rhs }
        | Expr::GCD(lhs, rhs)
        | Expr::LCM(lhs, rhs)
        | Expr::Mod(lhs, rhs)
        | Expr::Binomial(lhs, rhs)
        | Expr::Gte(lhs, rhs)
        | Expr::Gt(lhs, rhs)
        | Expr::Lte(lhs, rhs)
        | Expr::Lt(lhs, rhs) => is_calculus_expr(lhs) || is_calculus_expr(rhs),
        Expr::Floor(e) | Expr::Ceiling(e) | Expr::Factorial(e) => is_calculus_expr(e),
        Expr::Summation { from, to, body, .. } | Expr::BigProduct { from, to, body, .. } => {
            is_calculus_expr(from) || is_calculus_expr(to) || is_calculus_expr(body)
        }
        Expr::Const(_) | Expr::Var(_) | Expr::Pi | Expr::E => false,
    }
}

/// Verifier for mathematical steps.
pub struct Verifier {
    level: VerificationLevel,
    num_samples: usize,
    tolerance: f64,
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier {
    /// Create a new verifier with default settings.
    pub fn new() -> Self {
        Self {
            level: VerificationLevel::Symbolic,
            num_samples: 10,
            tolerance: 1e-10,
        }
    }

    /// Set the verification level.
    pub fn with_level(mut self, level: VerificationLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the number of numerical samples.
    pub fn with_samples(mut self, n: usize) -> Self {
        self.num_samples = n;
        self
    }

    /// Verify a transformation step.
    ///
    /// Checks that applying the rule to `before` produces `after`.
    pub fn verify_step(
        &self,
        before: &Expr,
        after: &Expr,
        rule: &Rule,
        ctx: &RuleContext,
    ) -> VerifyResult {
        // 1. Check rule is applicable
        if !rule.can_apply(before, ctx) {
            return VerifyResult::Invalid {
                reason: format!("Rule '{}' is not applicable to this expression", rule.name),
            };
        }

        // 2. Check if result is in possible outputs
        let possible_results = rule.apply(before, ctx);
        let result_matches = possible_results
            .iter()
            .any(|r| self.expressions_equal(&r.result, after));

        if !result_matches {
            return VerifyResult::Invalid {
                reason: format!("Rule '{}' does not produce the claimed result", rule.name),
            };
        }

        // 3. Additional verification based on level
        // For calculus expressions (derivatives/integrals), skip numerical verification
        // since they cannot be numerically evaluated - trust the rule application
        if is_calculus_expr(before) || is_calculus_expr(after) {
            return VerifyResult::Valid { confidence: 0.95 };
        }

        match self.level {
            VerificationLevel::Numerical => {
                if numerical::verify_equivalent(before, after, self.num_samples, self.tolerance) {
                    VerifyResult::Valid { confidence: 0.999 }
                } else {
                    VerifyResult::Invalid {
                        reason: "Numerical verification failed".to_string(),
                    }
                }
            }
            VerificationLevel::Symbolic => {
                if symbolic::verify_equivalent(before, after) {
                    VerifyResult::Valid { confidence: 1.0 }
                } else {
                    // Fall back to numerical
                    if numerical::verify_equivalent(before, after, self.num_samples, self.tolerance)
                    {
                        VerifyResult::Valid { confidence: 0.999 }
                    } else {
                        VerifyResult::Invalid {
                            reason: "Symbolic verification failed".to_string(),
                        }
                    }
                }
            }
            VerificationLevel::Formal => {
                // TODO: Implement Z3 integration
                VerifyResult::Unknown {
                    reason: "Formal verification not yet implemented".to_string(),
                }
            }
        }
    }

    /// Verify that a solution satisfies an equation.
    pub fn verify_solution(
        &self,
        equation: &Expr,
        variable: mm_core::Symbol,
        solution: &Expr,
    ) -> VerifyResult {
        // Substitute solution into equation and check if lhs == rhs
        if let Expr::Equation { lhs, rhs } = equation {
            let lhs_subst = substitute(lhs, variable, solution);
            let rhs_subst = substitute(rhs, variable, solution);

            // After substitution, lhs should equal rhs
            if self.expressions_equal(&lhs_subst, &rhs_subst) {
                return VerifyResult::Valid { confidence: 1.0 };
            }

            // Try numerical verification
            let diff = Expr::Sub(Box::new(lhs_subst.clone()), Box::new(rhs_subst.clone()));
            if numerical::is_zero(&diff, self.num_samples, self.tolerance) {
                return VerifyResult::Valid { confidence: 0.999 };
            }

            return VerifyResult::Invalid {
                reason: "Solution does not satisfy the equation".to_string(),
            };
        }

        VerifyResult::Invalid {
            reason: "Expected an equation".to_string(),
        }
    }

    /// Check if two expressions are equal.
    fn expressions_equal(&self, a: &Expr, b: &Expr) -> bool {
        // First try structural equality
        if a == b {
            return true;
        }

        // Then try canonical form
        let canon_a = a.canonicalize();
        let canon_b = b.canonicalize();
        if canon_a == canon_b {
            return true;
        }

        // Finally try numerical
        a.approx_equals(b, self.num_samples, self.tolerance)
    }
}

/// Substitute a variable with an expression.
fn substitute(expr: &Expr, var: mm_core::Symbol, value: &Expr) -> Expr {
    match expr {
        Expr::Var(v) if *v == var => value.clone(),
        Expr::Var(_) | Expr::Const(_) | Expr::Pi | Expr::E => expr.clone(),
        Expr::Neg(e) => Expr::Neg(Box::new(substitute(e, var, value))),
        Expr::Sqrt(e) => Expr::Sqrt(Box::new(substitute(e, var, value))),
        Expr::Sin(e) => Expr::Sin(Box::new(substitute(e, var, value))),
        Expr::Cos(e) => Expr::Cos(Box::new(substitute(e, var, value))),
        Expr::Tan(e) => Expr::Tan(Box::new(substitute(e, var, value))),
        Expr::Ln(e) => Expr::Ln(Box::new(substitute(e, var, value))),
        Expr::Exp(e) => Expr::Exp(Box::new(substitute(e, var, value))),
        Expr::Abs(e) => Expr::Abs(Box::new(substitute(e, var, value))),
        Expr::Add(a, b) => Expr::Add(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Mul(a, b) => Expr::Mul(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Div(a, b) => Expr::Div(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Pow(a, b) => Expr::Pow(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Sum(terms) => Expr::Sum(
            terms
                .iter()
                .map(|t| mm_core::Term {
                    coeff: t.coeff,
                    expr: substitute(&t.expr, var, value),
                })
                .collect(),
        ),
        Expr::Product(factors) => Expr::Product(
            factors
                .iter()
                .map(|f| mm_core::Factor {
                    base: substitute(&f.base, var, value),
                    power: substitute(&f.power, var, value),
                })
                .collect(),
        ),
        Expr::Derivative { expr: e, var: v } => Expr::Derivative {
            expr: Box::new(substitute(e, var, value)),
            var: *v,
        },
        Expr::Integral { expr: e, var: v } => Expr::Integral {
            expr: Box::new(substitute(e, var, value)),
            var: *v,
        },
        Expr::Equation { lhs, rhs } => Expr::Equation {
            lhs: Box::new(substitute(lhs, var, value)),
            rhs: Box::new(substitute(rhs, var, value)),
        },
        // Number theory
        Expr::GCD(a, b) => Expr::GCD(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::LCM(a, b) => Expr::LCM(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Mod(a, b) => Expr::Mod(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Binomial(n, k) => Expr::Binomial(
            Box::new(substitute(n, var, value)),
            Box::new(substitute(k, var, value)),
        ),
        Expr::Gte(a, b) => Expr::Gte(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Gt(a, b) => Expr::Gt(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Lte(a, b) => Expr::Lte(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Lt(a, b) => Expr::Lt(
            Box::new(substitute(a, var, value)),
            Box::new(substitute(b, var, value)),
        ),
        Expr::Floor(e) => Expr::Floor(Box::new(substitute(e, var, value))),
        Expr::Ceiling(e) => Expr::Ceiling(Box::new(substitute(e, var, value))),
        Expr::Factorial(e) => Expr::Factorial(Box::new(substitute(e, var, value))),
        Expr::Summation {
            var: v,
            from,
            to,
            body,
        } => {
            // Don't substitute bound variable in body if it shadows
            if *v == var {
                Expr::Summation {
                    var: *v,
                    from: Box::new(substitute(from, var, value)),
                    to: Box::new(substitute(to, var, value)),
                    body: body.clone(),
                }
            } else {
                Expr::Summation {
                    var: *v,
                    from: Box::new(substitute(from, var, value)),
                    to: Box::new(substitute(to, var, value)),
                    body: Box::new(substitute(body, var, value)),
                }
            }
        }
        Expr::BigProduct {
            var: v,
            from,
            to,
            body,
        } => {
            if *v == var {
                Expr::BigProduct {
                    var: *v,
                    from: Box::new(substitute(from, var, value)),
                    to: Box::new(substitute(to, var, value)),
                    body: body.clone(),
                }
            } else {
                Expr::BigProduct {
                    var: *v,
                    from: Box::new(substitute(from, var, value)),
                    to: Box::new(substitute(to, var, value)),
                    body: Box::new(substitute(body, var, value)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_verifier_creation() {
        let verifier = Verifier::new();
        assert_eq!(verifier.level, VerificationLevel::Symbolic);
    }

    #[test]
    fn test_substitution() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // Substitute x = 3 into x + 1
        let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        let result = substitute(&expr, x, &Expr::int(3));

        // Should get 3 + 1
        assert_eq!(
            result,
            Expr::Add(Box::new(Expr::int(3)), Box::new(Expr::int(1)))
        );
    }
}
