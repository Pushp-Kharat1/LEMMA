// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Numerical verification via random sampling.

use mm_core::Expr;
use rand::Rng;
use std::collections::HashMap;

/// Verify that two expressions are equivalent by numerical sampling.
pub fn verify_equivalent(a: &Expr, b: &Expr, num_samples: usize, tolerance: f64) -> bool {
    a.approx_equals(b, num_samples, tolerance)
}

/// Check if an expression evaluates to zero.
pub fn is_zero(expr: &Expr, num_samples: usize, tolerance: f64) -> bool {
    let mut rng = rand::thread_rng();

    // Get all variables
    let vars = expr.free_vars();

    for _ in 0..num_samples {
        // Generate random environment
        let mut env = HashMap::new();
        for &var in &vars {
            let val: f64 = rng.gen_range(-10.0..10.0);
            // Avoid values close to zero to prevent domain issues
            let val = if val.abs() < 0.5 {
                val + if val >= 0.0 { 1.0 } else { -1.0 }
            } else {
                val
            };
            env.insert(var, val);
        }

        // Evaluate
        if let Some(val) = expr.evaluate(&env) {
            if val.abs() > tolerance {
                return false;
            }
        }
        // If evaluation fails, skip this sample
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_numerical_equivalence() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x + 1 and 1 + x should be equivalent
        let a = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));
        let b = Expr::Add(Box::new(Expr::int(1)), Box::new(Expr::Var(x)));

        assert!(verify_equivalent(&a, &b, 10, 1e-10));
    }

    #[test]
    fn test_is_zero() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x - x should be zero
        let expr = Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(x)));
        assert!(is_zero(&expr, 10, 1e-10));

        // x should not be zero
        let expr = Expr::Var(x);
        assert!(!is_zero(&expr, 10, 1e-10));
    }
}
