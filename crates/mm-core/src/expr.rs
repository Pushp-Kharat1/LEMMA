// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Mathematical expression representation.
//!
//! The [`Expr`] enum is the core data structure representing mathematical expressions
//! as an abstract syntax tree (AST).

use crate::{Rational, Symbol};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// A mathematical expression represented as an abstract syntax tree.
///
/// # Design Decisions
///
/// - **Exact arithmetic**: Uses [`Rational`] for constants to avoid floating-point errors
/// - **Interned symbols**: Uses [`Symbol`] for variables for fast comparison
/// - **Box for recursion**: Heap allocation for recursive structure
/// - **Hash + Eq**: Enables memoization and duplicate detection
#[derive(Clone, Debug)]
pub enum Expr {
    // ========== Atoms ==========
    /// A constant rational number (e.g., 3, 1/2, -7)
    Const(Rational),

    /// A variable (e.g., x, y, z)
    Var(Symbol),

    /// Mathematical constant π (pi)
    Pi,

    /// Mathematical constant e (Euler's number)
    E,

    // ========== Unary Operations ==========
    /// Negation: -a
    Neg(Box<Expr>),

    /// Square root: √a
    Sqrt(Box<Expr>),

    /// Sine: sin(a)
    Sin(Box<Expr>),

    /// Cosine: cos(a)
    Cos(Box<Expr>),

    /// Tangent: tan(a)
    Tan(Box<Expr>),

    /// Natural logarithm: ln(a)
    Ln(Box<Expr>),

    /// Exponential: e^a
    Exp(Box<Expr>),

    /// Absolute value: |a|
    Abs(Box<Expr>),

    // ========== Binary Operations ==========
    /// Addition: a + b
    Add(Box<Expr>, Box<Expr>),

    /// Subtraction: a - b
    Sub(Box<Expr>, Box<Expr>),

    /// Multiplication: a * b
    Mul(Box<Expr>, Box<Expr>),

    /// Division: a / b
    Div(Box<Expr>, Box<Expr>),

    /// Power: a^b
    Pow(Box<Expr>, Box<Expr>),

    // ========== N-ary Operations (for canonicalization) ==========
    /// Sum of terms: a₁ + a₂ + ... + aₙ
    ///
    /// Used in canonical form to represent flattened sums with collected like terms.
    Sum(Vec<Term>),

    /// Product of factors: a₁ * a₂ * ... * aₙ
    ///
    /// Used in canonical form to represent flattened products with combined powers.
    Product(Vec<Factor>),

    // ========== Calculus ==========
    /// Derivative: d/dx(expr)
    Derivative { expr: Box<Expr>, var: Symbol },

    /// Integral: ∫ expr dx
    Integral { expr: Box<Expr>, var: Symbol },

    // ========== Relations ==========
    /// Equation: lhs = rhs
    Equation { lhs: Box<Expr>, rhs: Box<Expr> },

    /// Greater than or equal: lhs ≥ rhs
    Gte(Box<Expr>, Box<Expr>),

    /// Greater than: lhs > rhs
    Gt(Box<Expr>, Box<Expr>),

    /// Less than or equal: lhs ≤ rhs
    Lte(Box<Expr>, Box<Expr>),

    /// Less than: lhs < rhs
    Lt(Box<Expr>, Box<Expr>),

    // ========== Number Theory ==========
    /// Greatest common divisor: gcd(a, b)
    GCD(Box<Expr>, Box<Expr>),

    /// Least common multiple: lcm(a, b)
    LCM(Box<Expr>, Box<Expr>),

    /// Modulo: a mod b
    Mod(Box<Expr>, Box<Expr>),

    /// Floor function: ⌊x⌋
    Floor(Box<Expr>),

    /// Ceiling function: ⌈x⌉
    Ceiling(Box<Expr>),

    /// Factorial: n!
    Factorial(Box<Expr>),

    /// Binomial coefficient: C(n, k) = n! / (k!(n-k)!)
    Binomial(Box<Expr>, Box<Expr>),

    // ========== Summation and Product Notation ==========
    /// Summation: Σ_{var=from}^{to} body
    /// Example: Σ_{i=1}^{n} i = 1 + 2 + ... + n
    Summation {
        var: Symbol,
        from: Box<Expr>,
        to: Box<Expr>,
        body: Box<Expr>,
    },

    /// Product notation: Π_{var=from}^{to} body
    /// Example: Π_{i=1}^{n} i = n!
    BigProduct {
        var: Symbol,
        from: Box<Expr>,
        to: Box<Expr>,
        body: Box<Expr>,
    },

    // ========== Quantifiers ==========
    /// Universal quantifier: ∀var. body
    /// Example: ∀n. n² ≥ 0 (for all n, n squared is nonnegative)
    ForAll {
        var: Symbol,
        /// Optional domain constraint (e.g., n ∈ ℕ)
        domain: Option<Box<Expr>>,
        body: Box<Expr>,
    },

    /// Existential quantifier: ∃var. body
    /// Example: ∃x. x² = 2 (there exists x such that x squared equals 2)
    Exists {
        var: Symbol,
        /// Optional domain constraint
        domain: Option<Box<Expr>>,
        body: Box<Expr>,
    },

    // ========== Logical Connectives ==========
    /// Logical AND: P ∧ Q
    And(Box<Expr>, Box<Expr>),

    /// Logical OR: P ∨ Q  
    Or(Box<Expr>, Box<Expr>),

    /// Logical NOT: ¬P
    Not(Box<Expr>),

    /// Implication: P → Q
    Implies(Box<Expr>, Box<Expr>),
}

/// A term in a sum: coefficient × expression
///
/// Example: In `3x² + 2x + 1`, the terms are:
/// - Term { coeff: 3, expr: x² }
/// - Term { coeff: 2, expr: x }
/// - Term { coeff: 1, expr: 1 }
#[derive(Clone, Debug)]
pub struct Term {
    /// The coefficient (rational number)
    pub coeff: Rational,
    /// The expression being multiplied
    pub expr: Expr,
}

/// A factor in a product: base ^ exponent
///
/// Example: In `x²y³`, the factors are:
/// - Factor { base: x, power: 2 }
/// - Factor { base: y, power: 3 }
#[derive(Clone, Debug)]
pub struct Factor {
    /// The base expression
    pub base: Expr,
    /// The exponent expression
    pub power: Expr,
}

// ============================================================================
// Implement PartialEq, Eq, Hash for Expr
// ============================================================================

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Const(a), Expr::Const(b)) => a == b,
            (Expr::Var(a), Expr::Var(b)) => a == b,
            (Expr::Pi, Expr::Pi) => true,
            (Expr::E, Expr::E) => true,
            (Expr::Neg(a), Expr::Neg(b)) => a == b,
            (Expr::Sqrt(a), Expr::Sqrt(b)) => a == b,
            (Expr::Sin(a), Expr::Sin(b)) => a == b,
            (Expr::Cos(a), Expr::Cos(b)) => a == b,
            (Expr::Tan(a), Expr::Tan(b)) => a == b,
            (Expr::Ln(a), Expr::Ln(b)) => a == b,
            (Expr::Exp(a), Expr::Exp(b)) => a == b,
            (Expr::Abs(a), Expr::Abs(b)) => a == b,
            (Expr::Add(a1, a2), Expr::Add(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Sub(a1, a2), Expr::Sub(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Mul(a1, a2), Expr::Mul(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Div(a1, a2), Expr::Div(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Pow(a1, a2), Expr::Pow(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Sum(a), Expr::Sum(b)) => a == b,
            (Expr::Product(a), Expr::Product(b)) => a == b,
            (Expr::Derivative { expr: e1, var: v1 }, Expr::Derivative { expr: e2, var: v2 }) => {
                e1 == e2 && v1 == v2
            }
            (Expr::Integral { expr: e1, var: v1 }, Expr::Integral { expr: e2, var: v2 }) => {
                e1 == e2 && v1 == v2
            }
            (Expr::Equation { lhs: l1, rhs: r1 }, Expr::Equation { lhs: l2, rhs: r2 }) => {
                l1 == l2 && r1 == r2
            }
            (Expr::Gte(a1, a2), Expr::Gte(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Gt(a1, a2), Expr::Gt(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Lte(a1, a2), Expr::Lte(b1, b2)) => a1 == b1 && a2 == b2,
            (Expr::Lt(a1, a2), Expr::Lt(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Expr::Const(r) => r.hash(state),
            Expr::Var(s) => s.hash(state),
            Expr::Pi | Expr::E => {} // discriminant already hashed
            Expr::Neg(e)
            | Expr::Sqrt(e)
            | Expr::Sin(e)
            | Expr::Cos(e)
            | Expr::Tan(e)
            | Expr::Ln(e)
            | Expr::Exp(e)
            | Expr::Abs(e) => e.hash(state),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Sum(terms) => terms.hash(state),
            Expr::Product(factors) => factors.hash(state),
            Expr::Derivative { expr, var } | Expr::Integral { expr, var } => {
                expr.hash(state);
                var.hash(state);
            }
            Expr::Equation { lhs, rhs }
            | Expr::GCD(lhs, rhs)
            | Expr::LCM(lhs, rhs)
            | Expr::Mod(lhs, rhs)
            | Expr::Binomial(lhs, rhs)
            | Expr::Gte(lhs, rhs)
            | Expr::Gt(lhs, rhs)
            | Expr::Lte(lhs, rhs)
            | Expr::Lt(lhs, rhs) => {
                lhs.hash(state);
                rhs.hash(state);
            }
            Expr::Floor(e) | Expr::Ceiling(e) | Expr::Factorial(e) => e.hash(state),
            Expr::Summation {
                var,
                from,
                to,
                body,
            }
            | Expr::BigProduct {
                var,
                from,
                to,
                body,
            } => {
                var.hash(state);
                from.hash(state);
                to.hash(state);
                body.hash(state);
            }
            Expr::ForAll { var, domain, body } | Expr::Exists { var, domain, body } => {
                var.hash(state);
                domain.hash(state);
                body.hash(state);
            }
            Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) => {
                a.hash(state);
                b.hash(state);
            }
            Expr::Not(e) => e.hash(state),
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.coeff == other.coeff && self.expr == other.expr
    }
}

impl Eq for Term {}

impl Hash for Term {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coeff.hash(state);
        self.expr.hash(state);
    }
}

impl PartialEq for Factor {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.power == other.power
    }
}

impl Eq for Factor {}

impl Hash for Factor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.power.hash(state);
    }
}

// ============================================================================
// Implement Ord for consistent ordering (needed for canonicalization)
// ============================================================================

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by discriminant (type of expression)
        let self_disc = std::mem::discriminant(self);
        let other_disc = std::mem::discriminant(other);

        // Use debug representation for discriminant ordering
        let disc_cmp = format!("{:?}", self_disc).cmp(&format!("{:?}", other_disc));
        if disc_cmp != Ordering::Equal {
            return disc_cmp;
        }

        // Same type - compare contents
        match (self, other) {
            (Expr::Const(a), Expr::Const(b)) => a.cmp(b),
            (Expr::Var(a), Expr::Var(b)) => a.cmp(b),
            (Expr::Neg(a), Expr::Neg(b)) => a.cmp(b),
            (Expr::Sqrt(a), Expr::Sqrt(b)) => a.cmp(b),
            (Expr::Sin(a), Expr::Sin(b)) => a.cmp(b),
            (Expr::Cos(a), Expr::Cos(b)) => a.cmp(b),
            (Expr::Tan(a), Expr::Tan(b)) => a.cmp(b),
            (Expr::Ln(a), Expr::Ln(b)) => a.cmp(b),
            (Expr::Exp(a), Expr::Exp(b)) => a.cmp(b),
            (Expr::Abs(a), Expr::Abs(b)) => a.cmp(b),
            (Expr::Add(a1, a2), Expr::Add(b1, b2))
            | (Expr::Sub(a1, a2), Expr::Sub(b1, b2))
            | (Expr::Mul(a1, a2), Expr::Mul(b1, b2))
            | (Expr::Div(a1, a2), Expr::Div(b1, b2))
            | (Expr::Pow(a1, a2), Expr::Pow(b1, b2)) => a1.cmp(b1).then_with(|| a2.cmp(b2)),
            (Expr::Sum(a), Expr::Sum(b)) => a.cmp(b),
            (Expr::Product(a), Expr::Product(b)) => a.cmp(b),
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> Ordering {
        self.expr
            .cmp(&other.expr)
            .then_with(|| self.coeff.cmp(&other.coeff))
    }
}

impl PartialOrd for Factor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Factor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base
            .cmp(&other.base)
            .then_with(|| self.power.cmp(&other.power))
    }
}

// ============================================================================
// Helper constructors
// ============================================================================

impl Expr {
    /// Create a constant expression from an integer.
    pub fn int(n: i64) -> Self {
        Expr::Const(Rational::from_integer(n))
    }

    /// Create a constant expression from a fraction.
    pub fn frac(numer: i64, denom: i64) -> Self {
        Expr::Const(Rational::new(numer, denom))
    }

    /// Check if this expression is a constant zero.
    pub fn is_zero(&self) -> bool {
        matches!(self, Expr::Const(r) if r.is_zero())
    }

    /// Check if this expression is a constant one.
    pub fn is_one(&self) -> bool {
        matches!(self, Expr::Const(r) if r.is_one())
    }

    /// Check if this expression is a constant.
    pub fn is_const(&self) -> bool {
        matches!(self, Expr::Const(_))
    }

    /// Check if this expression is a variable.
    pub fn is_var(&self) -> bool {
        matches!(self, Expr::Var(_))
    }

    /// Get the complexity (number of nodes) of this expression.
    pub fn complexity(&self) -> usize {
        match self {
            Expr::Const(_) | Expr::Var(_) | Expr::Pi | Expr::E => 1,
            Expr::Neg(e)
            | Expr::Sqrt(e)
            | Expr::Sin(e)
            | Expr::Cos(e)
            | Expr::Tan(e)
            | Expr::Ln(e)
            | Expr::Exp(e)
            | Expr::Abs(e) => 1 + e.complexity(),
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b) => 1 + a.complexity() + b.complexity(),
            Expr::Sum(terms) => 1 + terms.iter().map(|t| 1 + t.expr.complexity()).sum::<usize>(),
            Expr::Product(factors) => {
                1 + factors
                    .iter()
                    .map(|f| 1 + f.base.complexity() + f.power.complexity())
                    .sum::<usize>()
            }
            Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => 1 + expr.complexity(),
            Expr::Equation { lhs, rhs }
            | Expr::GCD(lhs, rhs)
            | Expr::LCM(lhs, rhs)
            | Expr::Mod(lhs, rhs)
            | Expr::Binomial(lhs, rhs)
            | Expr::Gte(lhs, rhs)
            | Expr::Gt(lhs, rhs)
            | Expr::Lte(lhs, rhs)
            | Expr::Lt(lhs, rhs) => 1 + lhs.complexity() + rhs.complexity(),
            Expr::Floor(e) | Expr::Ceiling(e) | Expr::Factorial(e) => 1 + e.complexity(),
            Expr::Summation { from, to, body, .. } | Expr::BigProduct { from, to, body, .. } => {
                1 + from.complexity() + to.complexity() + body.complexity()
            }
            Expr::ForAll { domain, body, .. } | Expr::Exists { domain, body, .. } => {
                1 + domain.as_ref().map(|d| d.complexity()).unwrap_or(0) + body.complexity()
            }
            Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) => {
                1 + a.complexity() + b.complexity()
            }
            Expr::Not(e) => 1 + e.complexity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymbolTable;

    #[test]
    fn test_expr_equality() {
        let a = Expr::int(5);
        let b = Expr::int(5);
        let c = Expr::int(3);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_expr_complexity() {
        // 5
        assert_eq!(Expr::int(5).complexity(), 1);

        // x + 1
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        assert_eq!(expr.complexity(), 3);
    }
}
