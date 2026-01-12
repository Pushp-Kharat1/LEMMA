// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Numerical evaluation of expressions.
//!
//! Evaluates expressions to floating-point values given variable bindings.

use crate::{Expr, Rational, Symbol};
use std::collections::HashMap;

/// Environment mapping variables to their values.
pub type Env = HashMap<Symbol, f64>;

/// Compute GCD using Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute factorial.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

impl Expr {
    /// Evaluate this expression numerically.
    ///
    /// # Arguments
    ///
    /// * `env` - A mapping from variable symbols to their f64 values
    ///
    /// # Returns
    ///
    /// The numerical result, or `None` if evaluation fails (e.g., division by zero,
    /// undefined variable, or domain error).
    ///
    /// # Example
    ///
    /// ```rust
    /// use mm_core::{Expr, SymbolTable, eval::Env};
    /// use std::collections::HashMap;
    ///
    /// let mut symbols = SymbolTable::new();
    /// let x = symbols.intern("x");
    ///
    /// // Create expression: x + 1
    /// let expr = Expr::Add(
    ///     Box::new(Expr::Var(x)),
    ///     Box::new(Expr::int(1)),
    /// );
    ///
    /// // Evaluate with x = 2
    /// let mut env = HashMap::new();
    /// env.insert(x, 2.0);
    ///
    /// assert_eq!(expr.evaluate(&env), Some(3.0));
    /// ```
    pub fn evaluate(&self, env: &Env) -> Option<f64> {
        match self {
            Expr::Const(r) => Some(r.to_f64()),
            Expr::Var(s) => env.get(s).copied(),
            Expr::Pi => Some(std::f64::consts::PI),
            Expr::E => Some(std::f64::consts::E),

            Expr::Neg(e) => e.evaluate(env).map(|x| -x),
            Expr::Sqrt(e) => {
                let val = e.evaluate(env)?;
                if val >= 0.0 {
                    Some(val.sqrt())
                } else {
                    None // Complex result
                }
            }
            Expr::Sin(e) => e.evaluate(env).map(|x| x.sin()),
            Expr::Cos(e) => e.evaluate(env).map(|x| x.cos()),
            Expr::Tan(e) => e.evaluate(env).map(|x| x.tan()),
            Expr::Ln(e) => {
                let val = e.evaluate(env)?;
                if val > 0.0 {
                    Some(val.ln())
                } else {
                    None // Domain error
                }
            }
            Expr::Exp(e) => e.evaluate(env).map(|x| x.exp()),
            Expr::Abs(e) => e.evaluate(env).map(|x| x.abs()),

            Expr::Add(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                Some(va + vb)
            }
            Expr::Sub(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                Some(va - vb)
            }
            Expr::Mul(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                Some(va * vb)
            }
            Expr::Div(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                if vb.abs() < 1e-15 {
                    None // Division by zero
                } else {
                    Some(va / vb)
                }
            }
            Expr::Pow(base, exp) => {
                let vb = base.evaluate(env)?;
                let ve = exp.evaluate(env)?;
                Some(vb.powf(ve))
            }

            Expr::Sum(terms) => {
                let mut sum = 0.0;
                for term in terms {
                    let val = term.expr.evaluate(env)?;
                    sum += term.coeff.to_f64() * val;
                }
                Some(sum)
            }
            Expr::Product(factors) => {
                let mut prod = 1.0;
                for factor in factors {
                    let base = factor.base.evaluate(env)?;
                    let power = factor.power.evaluate(env)?;
                    prod *= base.powf(power);
                }
                Some(prod)
            }

            // Calculus expressions can't be directly evaluated
            Expr::Derivative { .. } | Expr::Integral { .. } => None,

            // Equations return the difference (lhs - rhs)
            // Useful for checking if a solution satisfies the equation
            Expr::Equation { lhs, rhs } => {
                let vl = lhs.evaluate(env)?;
                let vr = rhs.evaluate(env)?;
                Some(vl - vr)
            }

            // Comparison operators - return 1.0 for true, 0.0 for false
            Expr::Gte(lhs, rhs) => {
                let vl = lhs.evaluate(env)?;
                let vr = rhs.evaluate(env)?;
                Some(if vl >= vr { 1.0 } else { 0.0 })
            }
            Expr::Gt(lhs, rhs) => {
                let vl = lhs.evaluate(env)?;
                let vr = rhs.evaluate(env)?;
                Some(if vl > vr { 1.0 } else { 0.0 })
            }
            Expr::Lte(lhs, rhs) => {
                let vl = lhs.evaluate(env)?;
                let vr = rhs.evaluate(env)?;
                Some(if vl <= vr { 1.0 } else { 0.0 })
            }
            Expr::Lt(lhs, rhs) => {
                let vl = lhs.evaluate(env)?;
                let vr = rhs.evaluate(env)?;
                Some(if vl < vr { 1.0 } else { 0.0 })
            }

            // Number theory operations
            Expr::GCD(a, b) => {
                let va = a.evaluate(env)? as i64;
                let vb = b.evaluate(env)? as i64;
                Some(gcd(va.abs(), vb.abs()) as f64)
            }
            Expr::LCM(a, b) => {
                let va = a.evaluate(env)? as i64;
                let vb = b.evaluate(env)? as i64;
                if va == 0 || vb == 0 {
                    Some(0.0)
                } else {
                    Some((va.abs() * vb.abs() / gcd(va.abs(), vb.abs())) as f64)
                }
            }
            Expr::Mod(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                if vb.abs() < 1e-15 {
                    None // Mod by zero
                } else {
                    Some(va % vb)
                }
            }
            Expr::Floor(e) => e.evaluate(env).map(|x| x.floor()),
            Expr::Ceiling(e) => e.evaluate(env).map(|x| x.ceil()),
            Expr::Factorial(e) => {
                let n = e.evaluate(env)? as u64;
                if n > 20 {
                    None // Overflow risk
                } else {
                    Some(factorial(n) as f64)
                }
            }
            Expr::Binomial(n_expr, k_expr) => {
                let n = n_expr.evaluate(env)? as u64;
                let k = k_expr.evaluate(env)? as u64;
                if k > n || n > 20 {
                    None
                } else {
                    Some((factorial(n) / (factorial(k) * factorial(n - k))) as f64)
                }
            }
            // Summation and Product - evaluate when bounds are constant integers
            Expr::Summation {
                var,
                from,
                to,
                body,
            } => {
                let from_val = from.evaluate(env)? as i64;
                let to_val = to.evaluate(env)? as i64;
                if (to_val - from_val).abs() > 1000 {
                    return None; // Prevent runaway
                }
                let mut sum = 0.0;
                let mut local_env = env.clone();
                for i in from_val..=to_val {
                    local_env.insert(*var, i as f64);
                    sum += body.evaluate(&local_env)?;
                }
                Some(sum)
            }
            Expr::BigProduct {
                var,
                from,
                to,
                body,
            } => {
                let from_val = from.evaluate(env)? as i64;
                let to_val = to.evaluate(env)? as i64;
                if (to_val - from_val).abs() > 100 {
                    return None; // Prevent overflow
                }
                let mut prod = 1.0;
                let mut local_env = env.clone();
                for i in from_val..=to_val {
                    local_env.insert(*var, i as f64);
                    prod *= body.evaluate(&local_env)?;
                }
                Some(prod)
            }

            // Quantifiers - cannot be directly evaluated numerically
            Expr::ForAll { .. } | Expr::Exists { .. } => None,

            // Logical connectives - return 1.0 for true, 0.0 for false
            Expr::And(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                Some(if va != 0.0 && vb != 0.0 { 1.0 } else { 0.0 })
            }
            Expr::Or(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                Some(if va != 0.0 || vb != 0.0 { 1.0 } else { 0.0 })
            }
            Expr::Not(e) => {
                let v = e.evaluate(env)?;
                Some(if v == 0.0 { 1.0 } else { 0.0 })
            }
            Expr::Implies(a, b) => {
                let va = a.evaluate(env)?;
                let vb = b.evaluate(env)?;
                // P → Q is equivalent to ¬P ∨ Q
                Some(if va == 0.0 || vb != 0.0 { 1.0 } else { 0.0 })
            }
        }
    }

    /// Check if this expression approximately equals another at random points.
    ///
    /// Useful for quick verification that two expressions are equivalent.
    pub fn approx_equals(&self, other: &Expr, num_tests: usize, tolerance: f64) -> bool {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Collect all variables
        let mut vars_self = Vec::new();
        let mut vars_other = Vec::new();
        self.collect_vars(&mut vars_self);
        other.collect_vars(&mut vars_other);

        // Combine variables
        let mut all_vars: Vec<Symbol> = vars_self;
        for v in vars_other {
            if !all_vars.contains(&v) {
                all_vars.push(v);
            }
        }

        for _ in 0..num_tests {
            // Generate random environment
            let mut env = Env::new();
            for &var in &all_vars {
                // Use values in range [-10, 10], avoiding near-zero
                let val: f64 = rng.gen_range(-10.0..10.0);
                let val = if val.abs() < 0.5 { val + 1.0 } else { val };
                env.insert(var, val);
            }

            // Evaluate both
            match (self.evaluate(&env), other.evaluate(&env)) {
                (Some(v1), Some(v2)) => {
                    if (v1 - v2).abs() > tolerance * (1.0 + v1.abs().max(v2.abs())) {
                        return false;
                    }
                }
                (None, None) => {
                    // Both undefined at this point - could be equivalent
                    continue;
                }
                _ => {
                    // One defined, one not - definitely not equivalent
                    return false;
                }
            }
        }

        true
    }

    /// Collect all variable symbols in this expression.
    fn collect_vars(&self, vars: &mut Vec<Symbol>) {
        match self {
            Expr::Var(s) => {
                if !vars.contains(s) {
                    vars.push(*s);
                }
            }
            Expr::Const(_) | Expr::Pi | Expr::E => {}
            Expr::Neg(e)
            | Expr::Sqrt(e)
            | Expr::Sin(e)
            | Expr::Cos(e)
            | Expr::Tan(e)
            | Expr::Ln(e)
            | Expr::Exp(e)
            | Expr::Abs(e) => {
                e.collect_vars(vars);
            }
            Expr::Add(a, b)
            | Expr::Sub(a, b)
            | Expr::Mul(a, b)
            | Expr::Div(a, b)
            | Expr::Pow(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            Expr::Sum(terms) => {
                for term in terms {
                    term.expr.collect_vars(vars);
                }
            }
            Expr::Product(factors) => {
                for factor in factors {
                    factor.base.collect_vars(vars);
                    factor.power.collect_vars(vars);
                }
            }
            Expr::Derivative { expr, var } | Expr::Integral { expr, var } => {
                expr.collect_vars(vars);
                if !vars.contains(var) {
                    vars.push(*var);
                }
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
                lhs.collect_vars(vars);
                rhs.collect_vars(vars);
            }
            Expr::Floor(e) | Expr::Ceiling(e) | Expr::Factorial(e) => {
                e.collect_vars(vars);
            }
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
                from.collect_vars(vars);
                to.collect_vars(vars);
                body.collect_vars(vars);
                // The bound variable is not free
                vars.retain(|v| v != var);
            }
            Expr::ForAll { var, domain, body } | Expr::Exists { var, domain, body } => {
                if let Some(d) = domain {
                    d.collect_vars(vars);
                }
                body.collect_vars(vars);
                // The bound variable is not free
                vars.retain(|v| v != var);
            }
            Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) => {
                a.collect_vars(vars);
                b.collect_vars(vars);
            }
            Expr::Not(e) => {
                e.collect_vars(vars);
            }
        }
    }

    /// Get all free variables in this expression.
    pub fn free_vars(&self) -> Vec<Symbol> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SymbolTable;

    #[test]
    fn test_constant_evaluation() {
        let expr = Expr::int(5);
        let env = Env::new();
        assert_eq!(expr.evaluate(&env), Some(5.0));
    }

    #[test]
    fn test_variable_evaluation() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let expr = Expr::Var(x);
        let mut env = Env::new();
        env.insert(x, 3.0);

        assert_eq!(expr.evaluate(&env), Some(3.0));
    }

    #[test]
    fn test_arithmetic_evaluation() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x^2 + 2x + 1 at x=3 should be 16
        let expr = Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
                Box::new(Expr::int(1)),
            )),
        );

        let mut env = Env::new();
        env.insert(x, 3.0);

        assert_eq!(expr.evaluate(&env), Some(16.0));
    }

    #[test]
    fn test_trig_evaluation() {
        let expr = Expr::Sin(Box::new(Expr::int(0)));
        let env = Env::new();
        assert!((expr.evaluate(&env).unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_approx_equals() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x + 1 and 1 + x should be approximately equal
        let expr1 = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        let expr2 = Expr::Add(Box::new(Expr::int(1)), Box::new(Expr::Var(x)));

        assert!(expr1.approx_equals(&expr2, 10, 1e-10));
    }
}
