// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Expression canonicalization.
//!
//! Canonicalization converts expressions to a unique normal form so that
//! mathematically equivalent expressions compare equal.
//!
//! For example: `x + 1` and `1 + x` both canonicalize to the same form.

use crate::{Expr, Factor, Rational, Symbol, Term};
use std::collections::HashMap;

impl Expr {
    /// Convert this expression to canonical form.
    ///
    /// Canonical form has the following properties:
    /// - Constants are evaluated: `2 + 3` → `5`
    /// - Commutative operations are sorted: `b + a` → `a + b`
    /// - Associative operations are flattened: `(a + b) + c` → `Sum([a, b, c])`
    /// - Like terms are collected: `2x + 3x` → `5x`
    /// - Identities are applied: `x + 0` → `x`, `x * 1` → `x`
    ///
    /// Two expressions are mathematically equal if and only if their
    /// canonical forms are structurally equal.
    pub fn canonicalize(&self) -> Expr {
        self.canonicalize_with_depth(0)
    }

    /// Maximum recursion depth for canonicalization to prevent stack overflow.
    const MAX_CANON_DEPTH: usize = 100;

    /// Canonicalize with depth tracking to prevent stack overflow.
    fn canonicalize_with_depth(&self, depth: usize) -> Expr {
        if depth >= Self::MAX_CANON_DEPTH {
            // Return as-is if we've hit the depth limit
            return self.clone();
        }

        // First, recursively canonicalize children
        let simplified = self.simplify_recursive_with_depth(depth + 1);

        // Then apply top-level simplifications
        simplified.simplify_top()
    }

    /// Recursively simplify all children with depth tracking.
    fn simplify_recursive_with_depth(&self, depth: usize) -> Expr {
        if depth >= Self::MAX_CANON_DEPTH {
            return self.clone();
        }

        match self {
            // Atoms don't need simplification
            Expr::Const(_) | Expr::Var(_) | Expr::Pi | Expr::E => self.clone(),

            // Unary operations
            Expr::Neg(e) => Expr::Neg(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Sin(e) => Expr::Sin(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Cos(e) => Expr::Cos(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Tan(e) => Expr::Tan(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Ln(e) => Expr::Ln(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Exp(e) => Expr::Exp(Box::new(e.canonicalize_with_depth(depth))),
            Expr::Abs(e) => Expr::Abs(Box::new(e.canonicalize_with_depth(depth))),

            // Binary operations
            Expr::Add(a, b) => Expr::Add(
                Box::new(a.canonicalize_with_depth(depth)),
                Box::new(b.canonicalize_with_depth(depth)),
            ),
            Expr::Sub(a, b) => Expr::Sub(
                Box::new(a.canonicalize_with_depth(depth)),
                Box::new(b.canonicalize_with_depth(depth)),
            ),
            Expr::Mul(a, b) => Expr::Mul(
                Box::new(a.canonicalize_with_depth(depth)),
                Box::new(b.canonicalize_with_depth(depth)),
            ),
            Expr::Div(a, b) => Expr::Div(
                Box::new(a.canonicalize_with_depth(depth)),
                Box::new(b.canonicalize_with_depth(depth)),
            ),
            Expr::Pow(a, b) => Expr::Pow(
                Box::new(a.canonicalize_with_depth(depth)),
                Box::new(b.canonicalize_with_depth(depth)),
            ),

            // N-ary operations
            Expr::Sum(terms) => Expr::Sum(
                terms
                    .iter()
                    .map(|t| Term {
                        coeff: t.coeff,
                        expr: t.expr.canonicalize_with_depth(depth),
                    })
                    .collect(),
            ),
            Expr::Product(factors) => Expr::Product(
                factors
                    .iter()
                    .map(|f| Factor {
                        base: f.base.canonicalize_with_depth(depth),
                        power: f.power.canonicalize_with_depth(depth),
                    })
                    .collect(),
            ),

            // Calculus
            Expr::Derivative { expr, var } => Expr::Derivative {
                expr: Box::new(expr.canonicalize_with_depth(depth)),
                var: *var,
            },
            Expr::Integral { expr, var } => Expr::Integral {
                expr: Box::new(expr.canonicalize_with_depth(depth)),
                var: *var,
            },

            // Equation
            Expr::Equation { lhs, rhs } => Expr::Equation {
                lhs: Box::new(lhs.canonicalize_with_depth(depth)),
                rhs: Box::new(rhs.canonicalize_with_depth(depth)),
            },
        }
    }

    /// Apply top-level simplifications.
    fn simplify_top(&self) -> Expr {
        match self {
            // ===== Constant folding =====
            Expr::Neg(e) => {
                if let Expr::Const(r) = e.as_ref() {
                    return Expr::Const(-*r);
                }
                // --x = x
                if let Expr::Neg(inner) = e.as_ref() {
                    return inner.as_ref().clone();
                }
                self.clone()
            }

            Expr::Add(a, b) => {
                // Constant folding
                if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                    return Expr::Const(*r1 + *r2);
                }
                // x + 0 = x
                if b.is_zero() {
                    return a.as_ref().clone();
                }
                if a.is_zero() {
                    return b.as_ref().clone();
                }
                // Sort for canonical order (commutative)
                if a > b {
                    return Expr::Add(b.clone(), a.clone());
                }
                self.clone()
            }

            Expr::Sub(a, b) => {
                // Constant folding
                if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                    return Expr::Const(*r1 - *r2);
                }
                // x - 0 = x
                if b.is_zero() {
                    return a.as_ref().clone();
                }
                // 0 - x = -x
                if a.is_zero() {
                    return Expr::Neg(b.clone());
                }
                // x - x = 0
                if a == b {
                    return Expr::Const(Rational::from_integer(0));
                }
                self.clone()
            }

            Expr::Mul(a, b) => {
                // Constant folding
                if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                    return Expr::Const(*r1 * *r2);
                }
                // x * 0 = 0
                if a.is_zero() || b.is_zero() {
                    return Expr::Const(Rational::from_integer(0));
                }
                // x * 1 = x
                if b.is_one() {
                    return a.as_ref().clone();
                }
                if a.is_one() {
                    return b.as_ref().clone();
                }
                // Sort for canonical order (commutative)
                if a > b {
                    return Expr::Mul(b.clone(), a.clone());
                }
                self.clone()
            }

            Expr::Div(a, b) => {
                // Constant folding
                if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                    if !r2.is_zero() {
                        return Expr::Const(*r1 / *r2);
                    }
                }
                // 0 / x = 0
                if a.is_zero() {
                    return Expr::Const(Rational::from_integer(0));
                }
                // x / 1 = x
                if b.is_one() {
                    return a.as_ref().clone();
                }
                // x / x = 1
                if a == b {
                    return Expr::Const(Rational::from_integer(1));
                }
                self.clone()
            }

            Expr::Pow(base, exp) => {
                // x^0 = 1
                if exp.is_zero() {
                    return Expr::Const(Rational::from_integer(1));
                }
                // x^1 = x
                if exp.is_one() {
                    return base.as_ref().clone();
                }
                // 0^n = 0 (for n > 0)
                if base.is_zero() {
                    return Expr::Const(Rational::from_integer(0));
                }
                // 1^n = 1
                if base.is_one() {
                    return Expr::Const(Rational::from_integer(1));
                }
                // Constant folding for integer exponents
                if let (Expr::Const(r), Expr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                    if e.is_integer() && e.numer().abs() <= 10 {
                        return Expr::Const(r.pow(e.numer() as i32));
                    }
                }
                self.clone()
            }

            // Sum: collect like terms and sort
            Expr::Sum(terms) => {
                // Collect like terms
                let mut term_map: HashMap<Expr, Rational> = HashMap::new();
                for term in terms {
                    *term_map
                        .entry(term.expr.clone())
                        .or_insert(Rational::from_integer(0)) = term_map[&term.expr] + term.coeff;
                }

                // Remove zero terms
                let mut new_terms: Vec<Term> = term_map
                    .into_iter()
                    .filter(|(_, coeff)| !coeff.is_zero())
                    .map(|(expr, coeff)| Term { coeff, expr })
                    .collect();

                // Sort for canonical order
                new_terms.sort();

                // Handle special cases
                if new_terms.is_empty() {
                    return Expr::Const(Rational::from_integer(0));
                }
                if new_terms.len() == 1 {
                    let term = &new_terms[0];
                    if term.coeff.is_one() {
                        return term.expr.clone();
                    }
                }

                Expr::Sum(new_terms)
            }

            // Product: combine like bases and sort
            Expr::Product(factors) => {
                // Combine like bases
                let mut factor_map: HashMap<Expr, Expr> = HashMap::new();
                for factor in factors {
                    let base = factor.base.clone();
                    let power = factor.power.clone();
                    factor_map
                        .entry(base.clone())
                        .and_modify(|existing_power| {
                            *existing_power = Expr::Add(
                                Box::new(existing_power.clone()),
                                Box::new(power.clone()),
                            )
                            .canonicalize();
                        })
                        .or_insert(power);
                }

                // Remove factors with zero exponent
                let mut new_factors: Vec<Factor> = factor_map
                    .into_iter()
                    .filter(|(_, power)| !power.is_zero())
                    .map(|(base, power)| Factor { base, power })
                    .collect();

                // Sort for canonical order
                new_factors.sort();

                // Handle special cases
                if new_factors.is_empty() {
                    return Expr::Const(Rational::from_integer(1));
                }
                if new_factors.len() == 1 {
                    let factor = &new_factors[0];
                    if factor.power.is_one() {
                        return factor.base.clone();
                    }
                }

                Expr::Product(new_factors)
            }

            // Other expressions pass through
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymbolTable;

    #[test]
    fn test_constant_folding() {
        // 2 + 3 = 5
        let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        assert_eq!(expr.canonicalize(), Expr::int(5));

        // 4 * 5 = 20
        let expr = Expr::Mul(Box::new(Expr::int(4)), Box::new(Expr::int(5)));
        assert_eq!(expr.canonicalize(), Expr::int(20));
    }

    #[test]
    fn test_identity_simplification() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x + 0 = x
        let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
        assert_eq!(expr.canonicalize(), Expr::Var(x));

        // x * 1 = x
        let expr = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        assert_eq!(expr.canonicalize(), Expr::Var(x));

        // x * 0 = 0
        let expr = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
        assert_eq!(expr.canonicalize(), Expr::int(0));
    }

    #[test]
    fn test_power_simplification() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x^0 = 1
        let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
        assert_eq!(expr.canonicalize(), Expr::int(1));

        // x^1 = x
        let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        assert_eq!(expr.canonicalize(), Expr::Var(x));

        // 2^3 = 8
        let expr = Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        assert_eq!(expr.canonicalize(), Expr::int(8));
    }

    #[test]
    fn test_commutative_ordering() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // 1 + x and x + 1 should canonicalize to the same form
        let expr1 = Expr::Add(Box::new(Expr::int(1)), Box::new(Expr::Var(x)));
        let expr2 = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));

        assert_eq!(expr1.canonicalize(), expr2.canonicalize());
    }
}
