// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Symbolic verification via canonical forms.

use mm_core::Expr;

/// Verify that two expressions are symbolically equivalent.
///
/// This works by converting both expressions to canonical form
/// and checking structural equality.
pub fn verify_equivalent(a: &Expr, b: &Expr) -> bool {
    let canon_a = a.canonicalize();
    let canon_b = b.canonicalize();
    canon_a == canon_b
}

/// Check if an expression is symbolically zero.
pub fn is_zero(expr: &Expr) -> bool {
    let canon = expr.canonicalize();
    canon.is_zero()
}

/// Check if an expression is symbolically one.
pub fn is_one(expr: &Expr) -> bool {
    let canon = expr.canonicalize();
    canon.is_one()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_symbolic_equivalence() {
        // 2 + 3 and 5 should be equivalent
        let a = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        let b = Expr::int(5);

        assert!(verify_equivalent(&a, &b));
    }

    #[test]
    fn test_commutative_equivalence() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // x * 2 and 2 * x should be equivalent
        let a = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));
        let b = Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)));

        assert!(verify_equivalent(&a, &b));
    }
}
