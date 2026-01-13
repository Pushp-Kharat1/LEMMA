// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Polynomial Normalization for Algebraic Equality
//!
//! Converts expressions to a canonical polynomial form to enable
//! true algebraic equality checking instead of numerical verification.
//!
//! Example:
//! ```text
//! k(k+1)/2 + (k+1)  →  (k² + 3k + 2) / 2
//! (k+1)(k+2)/2      →  (k² + 3k + 2) / 2
//! ```
//! These are structurally equal in normal form.

use mm_core::{Expr, Rational, Symbol};
use std::collections::BTreeMap;

/// A monomial: coefficient * x₁^e₁ * x₂^e₂ * ...
/// Using BTreeMap for consistent ordering (canonical form)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Monomial {
    /// Variable powers: symbol -> exponent
    pub powers: BTreeMap<Symbol, u32>,
}

impl Monomial {
    /// Constant monomial (no variables)
    pub fn constant() -> Self {
        Self {
            powers: BTreeMap::new(),
        }
    }

    /// Single variable: x^1
    pub fn var(v: Symbol) -> Self {
        let mut powers = BTreeMap::new();
        powers.insert(v, 1);
        Self { powers }
    }

    /// Variable with power: x^n
    #[allow(dead_code)]
    pub fn var_pow(v: Symbol, n: u32) -> Self {
        if n == 0 {
            return Self::constant();
        }
        let mut powers = BTreeMap::new();
        powers.insert(v, n);
        Self { powers }
    }

    /// Multiply two monomials: x^a * x^b = x^(a+b)
    pub fn mul(&self, other: &Monomial) -> Monomial {
        let mut result = self.powers.clone();
        for (var, exp) in &other.powers {
            *result.entry(*var).or_insert(0) += exp;
        }
        // Remove zero exponents
        result.retain(|_, exp| *exp > 0);
        Monomial { powers: result }
    }

    /// Total degree of monomial
    #[allow(dead_code)]
    pub fn degree(&self) -> u32 {
        self.powers.values().sum()
    }

    /// Check if this is a constant (no variables)
    pub fn is_constant(&self) -> bool {
        self.powers.is_empty()
    }
}

/// A polynomial in normal form: sum of (coefficient, monomial) pairs
/// Stores as numerator polynomial over common denominator
#[derive(Debug, Clone)]
pub struct PolynomialNF {
    /// Terms: monomial -> coefficient (in numerator)
    terms: BTreeMap<Monomial, Rational>,
    /// Common denominator
    denominator: Rational,
}

impl PolynomialNF {
    /// Zero polynomial
    pub fn zero() -> Self {
        Self {
            terms: BTreeMap::new(),
            denominator: Rational::from(1),
        }
    }

    /// Constant polynomial
    pub fn constant(c: Rational) -> Self {
        if c.is_zero() {
            return Self::zero();
        }
        let mut terms = BTreeMap::new();
        terms.insert(Monomial::constant(), c);
        Self {
            terms,
            denominator: Rational::from(1),
        }
    }

    /// Single variable polynomial: x
    pub fn var(v: Symbol) -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(Monomial::var(v), Rational::from(1));
        Self {
            terms,
            denominator: Rational::from(1),
        }
    }

    /// Convert from Expr to polynomial normal form
    pub fn from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::Const(r) => Some(Self::constant(*r)),

            Expr::Var(v) => Some(Self::var(*v)),

            Expr::Neg(e) => {
                let p = Self::from_expr(e)?;
                Some(p.neg())
            }

            Expr::Add(a, b) => {
                let pa = Self::from_expr(a)?;
                let pb = Self::from_expr(b)?;
                Some(pa.add(&pb))
            }

            Expr::Sub(a, b) => {
                let pa = Self::from_expr(a)?;
                let pb = Self::from_expr(b)?;
                Some(pa.add(&pb.neg()))
            }

            Expr::Mul(a, b) => {
                let pa = Self::from_expr(a)?;
                let pb = Self::from_expr(b)?;
                Some(pa.mul(&pb))
            }

            Expr::Div(a, b) => {
                let pa = Self::from_expr(a)?;
                let pb = Self::from_expr(b)?;
                // Only handle division by constants for now
                if pb.is_constant() {
                    let divisor = pb.constant_value()?;
                    Some(pa.div_constant(divisor))
                } else {
                    None // Can't normalize rational functions yet
                }
            }

            Expr::Pow(base, exp) => {
                let pb = Self::from_expr(base)?;
                // Only handle constant integer exponents
                if let Expr::Const(r) = exp.as_ref() {
                    if r.is_integer() && r.denom() == 1 {
                        let n = r.numer();
                        if n >= 0 && n <= 10 {
                            return Some(pb.pow(n as u32));
                        }
                    }
                }
                None
            }

            // For other expressions, return None (can't normalize)
            _ => None,
        }
    }

    /// Negate polynomial
    pub fn neg(&self) -> Self {
        let terms = self
            .terms
            .iter()
            .map(|(m, c)| (m.clone(), Rational::from(0) - *c))
            .collect();
        Self {
            terms,
            denominator: self.denominator,
        }
    }

    /// Add two polynomials
    pub fn add(&self, other: &PolynomialNF) -> Self {
        // Compute LCM of denominators manually: lcm(a,b) = a*b / gcd(a,b)
        // For simplicity, just multiply denominators and scale numerators
        let new_denom = self.denominator * other.denominator;
        let mult_self = other.denominator;
        let mult_other = self.denominator;

        let mut terms = BTreeMap::new();

        // Add terms from self (scaled)
        for (mono, coeff) in &self.terms {
            let scaled = *coeff * mult_self;
            let entry = terms.entry(mono.clone()).or_insert(Rational::from(0));
            *entry = *entry + scaled;
        }

        // Add terms from other (scaled)
        for (mono, coeff) in &other.terms {
            let scaled = *coeff * mult_other;
            let entry = terms.entry(mono.clone()).or_insert(Rational::from(0));
            *entry = *entry + scaled;
        }

        // Remove zero terms
        terms.retain(|_, c| !c.is_zero());

        let mut result = Self {
            terms,
            denominator: new_denom,
        };
        result.simplify();
        result
    }

    /// Multiply two polynomials
    pub fn mul(&self, other: &PolynomialNF) -> Self {
        let mut terms = BTreeMap::new();

        for (m1, c1) in &self.terms {
            for (m2, c2) in &other.terms {
                let mono = m1.mul(m2);
                let coeff = *c1 * *c2;
                let entry = terms.entry(mono).or_insert(Rational::from(0));
                *entry = *entry + coeff;
            }
        }

        // Remove zero terms
        terms.retain(|_, c| !c.is_zero());

        let mut result = Self {
            terms,
            denominator: self.denominator * other.denominator,
        };
        result.simplify();
        result
    }

    /// Divide by a constant
    pub fn div_constant(&self, divisor: Rational) -> Self {
        Self {
            terms: self.terms.clone(),
            denominator: self.denominator * divisor,
        }
    }

    /// Raise to a power
    pub fn pow(&self, n: u32) -> Self {
        if n == 0 {
            return Self::constant(Rational::from(1));
        }

        let mut result = self.clone();
        for _ in 1..n {
            result = result.mul(self);
        }
        result
    }

    /// Simplify the polynomial (reduce common factors)
    fn simplify(&mut self) {
        if self.terms.is_empty() {
            self.denominator = Rational::from(1);
            return;
        }

        // Find GCD of all coefficients and denominator using i64 GCD
        fn gcd_i64(a: i64, b: i64) -> i64 {
            let a = a.abs();
            let b = b.abs();
            if b == 0 {
                a
            } else {
                gcd_i64(b, a % b)
            }
        }

        // Collect all numerators and the denominator
        let denom_val = self.denominator.numer().abs() * self.denominator.denom().signum();
        let mut g = denom_val.abs();

        for coeff in self.terms.values() {
            // Each coefficient contributes numer/denom to the GCD calculation
            g = gcd_i64(g, coeff.numer().abs());
        }

        if g > 1 {
            // Divide all coefficients and denominator by g
            self.denominator =
                Rational::new(self.denominator.numer() / g, self.denominator.denom());
            for coeff in self.terms.values_mut() {
                *coeff = Rational::new(coeff.numer() / g, coeff.denom());
            }
        }

        // Ensure denominator is positive
        if self.denominator.is_negative() {
            self.denominator = Rational::from(0) - self.denominator;
            for coeff in self.terms.values_mut() {
                *coeff = Rational::from(0) - *coeff;
            }
        }
    }

    /// Check if this is a constant polynomial
    pub fn is_constant(&self) -> bool {
        self.terms.is_empty()
            || (self.terms.len() == 1
                && self.terms.keys().next().map_or(false, |m| m.is_constant()))
    }

    /// Get constant value if this is a constant polynomial
    pub fn constant_value(&self) -> Option<Rational> {
        if self.terms.is_empty() {
            return Some(Rational::from(0));
        }
        if self.terms.len() == 1 {
            if let Some((mono, coeff)) = self.terms.iter().next() {
                if mono.is_constant() {
                    return Some(*coeff / self.denominator);
                }
            }
        }
        None
    }

    /// Check algebraic equality with another polynomial
    pub fn equals(&self, other: &PolynomialNF) -> bool {
        // Subtract and check if zero
        let diff = self.add(&other.neg());
        diff.terms.is_empty()
    }

    /// Convert back to Expr (for display/debugging)
    #[allow(dead_code)]
    pub fn to_expr(&self) -> Expr {
        if self.terms.is_empty() {
            return Expr::int(0);
        }

        let mut result: Option<Expr> = None;

        for (mono, coeff) in &self.terms {
            // Build monomial expression
            let mut term_expr: Option<Expr> = None;

            for (var, exp) in &mono.powers {
                let var_expr = if *exp == 1 {
                    Expr::Var(*var)
                } else {
                    Expr::Pow(Box::new(Expr::Var(*var)), Box::new(Expr::int(*exp as i64)))
                };

                term_expr = Some(match term_expr {
                    None => var_expr,
                    Some(e) => Expr::Mul(Box::new(e), Box::new(var_expr)),
                });
            }

            // Apply coefficient
            let term_with_coeff = match term_expr {
                None => Expr::Const(*coeff), // Constant term
                Some(e) if *coeff == Rational::from(1) => e,
                Some(e) if *coeff == Rational::from(-1) => Expr::Neg(Box::new(e)),
                Some(e) => Expr::Mul(Box::new(Expr::Const(*coeff)), Box::new(e)),
            };

            result = Some(match result {
                None => term_with_coeff,
                Some(r) => Expr::Add(Box::new(r), Box::new(term_with_coeff)),
            });
        }

        let numerator = result.unwrap_or(Expr::int(0));

        // Apply denominator
        if self.denominator == Rational::from(1) {
            numerator
        } else {
            Expr::Div(Box::new(numerator), Box::new(Expr::Const(self.denominator)))
        }
    }
}

/// Check if two expressions are algebraically equal
pub fn algebraically_equal(a: &Expr, b: &Expr) -> Option<bool> {
    let pa = PolynomialNF::from_expr(a)?;
    let pb = PolynomialNF::from_expr(b)?;
    Some(pa.equals(&pb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_constant_equality() {
        let a = Expr::int(5);
        let b = Expr::int(5);
        assert_eq!(algebraically_equal(&a, &b), Some(true));

        let c = Expr::int(3);
        assert_eq!(algebraically_equal(&a, &c), Some(false));
    }

    #[test]
    fn test_variable_equality() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let a = Expr::Var(x);
        let b = Expr::Var(x);
        assert_eq!(algebraically_equal(&a, &b), Some(true));
    }

    #[test]
    fn test_polynomial_equality() {
        let mut symbols = SymbolTable::new();
        let k = symbols.intern("k");

        // (k+1)(k+2) = k² + 3k + 2
        let lhs = Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
            Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(2)))),
        );

        let rhs = Expr::Add(
            Box::new(Expr::Add(
                Box::new(Expr::Pow(Box::new(Expr::Var(k)), Box::new(Expr::int(2)))),
                Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(k)))),
            )),
            Box::new(Expr::int(2)),
        );

        assert_eq!(algebraically_equal(&lhs, &rhs), Some(true));
    }

    #[test]
    fn test_sum_formula_step() {
        let mut symbols = SymbolTable::new();
        let k = symbols.intern("k");

        // k(k+1)/2 + (k+1) = (k+1)(k+2)/2

        // LHS: k(k+1)/2 + (k+1)
        let k_times_k_plus_1 = Expr::Mul(
            Box::new(Expr::Var(k)),
            Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
        );
        let k_times_k_plus_1_div_2 = Expr::Div(Box::new(k_times_k_plus_1), Box::new(Expr::int(2)));
        let lhs = Expr::Add(
            Box::new(k_times_k_plus_1_div_2),
            Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
        );

        // RHS: (k+1)(k+2)/2
        let rhs = Expr::Div(
            Box::new(Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
                Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(2)))),
            )),
            Box::new(Expr::int(2)),
        );

        assert_eq!(algebraically_equal(&lhs, &rhs), Some(true));
    }
}
