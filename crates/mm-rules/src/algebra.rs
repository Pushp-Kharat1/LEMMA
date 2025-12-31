// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Algebraic transformation rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleContext, RuleId};
use mm_core::{Expr, Rational};

/// Get all algebra rules.
pub fn algebra_rules() -> Vec<Rule> {
    vec![
        constant_fold(),
        identity_add_zero(),
        identity_mul_one(),
        zero_mul(),
        collect_like_terms(),
        distribute(),
        factor_common(),
        difference_of_squares(),
        perfect_square_sum(),
        perfect_square_diff(),
        power_of_one(),
        power_of_zero(),
        power_add(),
        power_mul(),
    ]
}

// ============================================================================
// Rule 1: Constant Folding
// ============================================================================

fn constant_fold() -> Rule {
    Rule {
        id: RuleId(1),
        name: "const_fold",
        category: RuleCategory::Simplification,
        description: "Evaluate constant expressions: 2 + 3 → 5",
        is_applicable: |expr, _ctx| match expr {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                matches!(a.as_ref(), Expr::Const(_)) && matches!(b.as_ref(), Expr::Const(_))
            }
            Expr::Pow(base, exp) => {
                if let (Expr::Const(_), Expr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                    e.is_integer() && e.numer().abs() <= 10
                } else {
                    false
                }
            }
            _ => false,
        },
        apply: |expr, _ctx| {
            match expr {
                Expr::Add(a, b) => {
                    if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Const(*r1 + *r2),
                            justification: format!("{} + {} = {}", r1, r2, *r1 + *r2),
                        }];
                    }
                }
                Expr::Sub(a, b) => {
                    if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Const(*r1 - *r2),
                            justification: format!("{} - {} = {}", r1, r2, *r1 - *r2),
                        }];
                    }
                }
                Expr::Mul(a, b) => {
                    if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Const(*r1 * *r2),
                            justification: format!("{} × {} = {}", r1, r2, *r1 * *r2),
                        }];
                    }
                }
                Expr::Div(a, b) => {
                    if let (Expr::Const(r1), Expr::Const(r2)) = (a.as_ref(), b.as_ref()) {
                        if !r2.is_zero() {
                            return vec![RuleApplication {
                                result: Expr::Const(*r1 / *r2),
                                justification: format!("{} ÷ {} = {}", r1, r2, *r1 / *r2),
                            }];
                        }
                    }
                }
                Expr::Pow(base, exp) => {
                    if let (Expr::Const(r), Expr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                        if e.is_integer() && e.numer().abs() <= 10 {
                            let result = r.pow(e.numer() as i32);
                            return vec![RuleApplication {
                                result: Expr::Const(result),
                                justification: format!("{}^{} = {}", r, e, result),
                            }];
                        }
                    }
                }
                _ => {}
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ============================================================================
// Rule 2: Identity Addition (x + 0 = x)
// ============================================================================

fn identity_add_zero() -> Rule {
    Rule {
        id: RuleId(2),
        name: "identity_add_zero",
        category: RuleCategory::Simplification,
        description: "Remove additive identity: x + 0 → x",
        is_applicable: |expr, _ctx| {
            if let Expr::Add(a, b) = expr {
                return a.is_zero() || b.is_zero();
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(a, b) = expr {
                if a.is_zero() {
                    return vec![RuleApplication {
                        result: b.as_ref().clone(),
                        justification: "0 + x = x".to_string(),
                    }];
                }
                if b.is_zero() {
                    return vec![RuleApplication {
                        result: a.as_ref().clone(),
                        justification: "x + 0 = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ============================================================================
// Rule 3: Identity Multiplication (x * 1 = x)
// ============================================================================

fn identity_mul_one() -> Rule {
    Rule {
        id: RuleId(3),
        name: "identity_mul_one",
        category: RuleCategory::Simplification,
        description: "Remove multiplicative identity: x * 1 → x",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                return a.is_one() || b.is_one();
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                if a.is_one() {
                    return vec![RuleApplication {
                        result: b.as_ref().clone(),
                        justification: "1 × x = x".to_string(),
                    }];
                }
                if b.is_one() {
                    return vec![RuleApplication {
                        result: a.as_ref().clone(),
                        justification: "x × 1 = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ============================================================================
// Rule 4: Zero Multiplication (x * 0 = 0)
// ============================================================================

fn zero_mul() -> Rule {
    Rule {
        id: RuleId(4),
        name: "zero_mul",
        category: RuleCategory::Simplification,
        description: "Multiply by zero: x * 0 → 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                return a.is_zero() || b.is_zero();
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                if a.is_zero() || b.is_zero() {
                    return vec![RuleApplication {
                        result: Expr::int(0),
                        justification: "x × 0 = 0".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ============================================================================
// Rule 5: Collect Like Terms (ax + bx = (a+b)x)
// ============================================================================

fn collect_like_terms() -> Rule {
    Rule {
        id: RuleId(5),
        name: "collect_like_terms",
        category: RuleCategory::Simplification,
        description: "Collect like terms: ax + bx → (a+b)x",
        is_applicable: |expr, _ctx| {
            // Check for pattern: (c1 * x) + (c2 * x) or x + x
            if let Expr::Add(a, b) = expr {
                // Simple case: x + x
                if a == b {
                    return true;
                }
                // Check for coefficient * variable patterns
                let term_a = extract_term(a);
                let term_b = extract_term(b);
                if let (Some((_, base_a)), Some((_, base_b))) = (term_a, term_b) {
                    return base_a == base_b;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(a, b) = expr {
                // Simple case: x + x = 2x
                if a == b {
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(Expr::int(2)), a.clone()),
                        justification: "x + x = 2x".to_string(),
                    }];
                }
                // General case
                let term_a = extract_term(a);
                let term_b = extract_term(b);
                if let (Some((coeff_a, base_a)), Some((coeff_b, base_b))) = (term_a, term_b) {
                    if base_a == base_b {
                        let new_coeff = coeff_a + coeff_b;
                        if new_coeff.is_zero() {
                            return vec![RuleApplication {
                                result: Expr::int(0),
                                justification: format!("{}x + {}x = 0", coeff_a, coeff_b),
                            }];
                        }
                        if new_coeff.is_one() {
                            return vec![RuleApplication {
                                result: base_a.clone(),
                                justification: format!("{}x + {}x = x", coeff_a, coeff_b),
                            }];
                        }
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Const(new_coeff)),
                                Box::new(base_a.clone()),
                            ),
                            justification: format!("{}x + {}x = {}x", coeff_a, coeff_b, new_coeff),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

/// Extract coefficient and base from a term.
/// Returns (coefficient, base_expression)
fn extract_term(expr: &Expr) -> Option<(Rational, &Expr)> {
    match expr {
        Expr::Const(_) => Some((Rational::from_integer(1), expr)),
        Expr::Var(_) => Some((Rational::from_integer(1), expr)),
        Expr::Mul(a, b) => {
            if let Expr::Const(c) = a.as_ref() {
                Some((*c, b.as_ref()))
            } else if let Expr::Const(c) = b.as_ref() {
                Some((*c, a.as_ref()))
            } else {
                Some((Rational::from_integer(1), expr))
            }
        }
        Expr::Neg(e) => extract_term(e).map(|(c, base)| (-c, base)),
        _ => Some((Rational::from_integer(1), expr)),
    }
}

// ============================================================================
// Rule 6: Distributive Property (expand)
// ============================================================================

fn distribute() -> Rule {
    Rule {
        id: RuleId(6),
        name: "distribute",
        category: RuleCategory::Expansion,
        description: "Distribute: a(b + c) → ab + ac",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                // a * (b + c) or (a + b) * c
                return matches!(a.as_ref(), Expr::Add(_, _))
                    || matches!(b.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                // a * (b + c) = a*b + a*c
                if let Expr::Add(b1, b2) = b.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Mul(a.clone(), b1.clone())),
                            Box::new(Expr::Mul(a.clone(), b2.clone())),
                        ),
                        justification: "a(b + c) = ab + ac".to_string(),
                    }];
                }
                // (a + b) * c = a*c + b*c
                if let Expr::Add(a1, a2) = a.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Mul(a1.clone(), b.clone())),
                            Box::new(Expr::Mul(a2.clone(), b.clone())),
                        ),
                        justification: "(a + b)c = ac + bc".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// ============================================================================
// Rule 7: Factor Common (reverse distribute)
// ============================================================================

fn factor_common() -> Rule {
    Rule {
        id: RuleId(7),
        name: "factor_common",
        category: RuleCategory::Factoring,
        description: "Factor common: ab + ac → a(b + c)",
        is_applicable: |expr, _ctx| {
            // Check for ab + ac pattern
            if let Expr::Add(left, right) = expr {
                if let (Expr::Mul(a1, _), Expr::Mul(a2, _)) = (left.as_ref(), right.as_ref()) {
                    return a1 == a2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Mul(a1, b1), Expr::Mul(a2, b2)) = (left.as_ref(), right.as_ref()) {
                    if a1 == a2 {
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                a1.clone(),
                                Box::new(Expr::Add(b1.clone(), b2.clone())),
                            ),
                            justification: "ab + ac = a(b + c)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// ============================================================================
// Rule 8: Difference of Squares
// ============================================================================

fn difference_of_squares() -> Rule {
    Rule {
        id: RuleId(8),
        name: "difference_of_squares",
        category: RuleCategory::Factoring,
        description: "Factor difference of squares: a² - b² → (a+b)(a-b)",
        is_applicable: |expr, _ctx| {
            // Check for a² - b² pattern
            if let Expr::Sub(left, right) = expr {
                let left_is_square =
                    matches!(left.as_ref(), Expr::Pow(_, exp) if exp.as_ref() == &Expr::int(2));
                let right_is_square =
                    matches!(right.as_ref(), Expr::Pow(_, exp) if exp.as_ref() == &Expr::int(2));
                return left_is_square && right_is_square;
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref()) {
                    return vec![RuleApplication {
                        result: Expr::Mul(
                            Box::new(Expr::Add(a.clone(), b.clone())),
                            Box::new(Expr::Sub(a.clone(), b.clone())),
                        ),
                        justification: "a² - b² = (a + b)(a - b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// ============================================================================
// Rule 9: Perfect Square (sum)
// ============================================================================

fn perfect_square_sum() -> Rule {
    Rule {
        id: RuleId(9),
        name: "perfect_square_sum",
        category: RuleCategory::Factoring,
        description: "Factor perfect square: a² + 2ab + b² → (a + b)²",
        is_applicable: |_expr, _ctx| {
            // This requires more complex pattern matching
            // Simplified check for now
            false
        },
        apply: |_expr, _ctx| {
            // TODO: Implement pattern matching for a² + 2ab + b²
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// ============================================================================
// Rule 10: Perfect Square (difference)
// ============================================================================

fn perfect_square_diff() -> Rule {
    Rule {
        id: RuleId(10),
        name: "perfect_square_diff",
        category: RuleCategory::Factoring,
        description: "Factor perfect square: a² - 2ab + b² → (a - b)²",
        is_applicable: |_expr, _ctx| {
            // This requires more complex pattern matching
            false
        },
        apply: |_expr, _ctx| {
            // TODO: Implement pattern matching for a² - 2ab + b²
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// ============================================================================
// Rule 11: Power of One (x^1 = x)
// ============================================================================

fn power_of_one() -> Rule {
    Rule {
        id: RuleId(11),
        name: "power_of_one",
        category: RuleCategory::Simplification,
        description: "x^1 = x",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 1 && r.denom() == 1)),
        apply: |expr, _ctx| {
            if let Expr::Pow(base, _) = expr {
                vec![RuleApplication {
                    result: base.as_ref().clone(),
                    justification: "x^1 = x".to_string(),
                }]
            } else {
                vec![]
            }
        },
        reversible: true,
        cost: 1,
    }
}

// ============================================================================
// Rule 12: Power of Zero (x^0 = 1)
// ============================================================================

fn power_of_zero() -> Rule {
    Rule {
        id: RuleId(12),
        name: "power_of_zero",
        category: RuleCategory::Simplification,
        description: "x^0 = 1 (where x ≠ 0)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(r) if r.is_zero())),
        apply: |expr, _ctx| {
            if let Expr::Pow(_, _) = expr {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "x^0 = 1".to_string(),
                }]
            } else {
                vec![]
            }
        },
        reversible: false, // Not reversible - we lose base info
        cost: 1,
    }
}

// ============================================================================
// Rule 13: Power Add (x^a * x^b = x^(a+b))
// ============================================================================

fn power_add() -> Rule {
    Rule {
        id: RuleId(13),
        name: "power_add",
        category: RuleCategory::Simplification,
        description: "x^a * x^b = x^(a+b)",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                // Check if both are powers with same base
                if let (Expr::Pow(base1, _), Expr::Pow(base2, _)) = (left.as_ref(), right.as_ref())
                {
                    return base1 == base2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Pow(base1, exp1), Expr::Pow(base2, exp2)) =
                    (left.as_ref(), right.as_ref())
                {
                    if base1 == base2 {
                        let new_exp = Expr::Add(
                            Box::new(exp1.as_ref().clone()),
                            Box::new(exp2.as_ref().clone()),
                        );
                        return vec![RuleApplication {
                            result: Expr::Pow(base1.clone(), Box::new(new_exp)),
                            justification: "x^a * x^b = x^(a+b)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// ============================================================================
// Rule 14: Power Multiply ((x^a)^b = x^(a*b))
// ============================================================================

fn power_mul() -> Rule {
    Rule {
        id: RuleId(14),
        name: "power_mul",
        category: RuleCategory::Simplification,
        description: "(x^a)^b = x^(a*b)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(inner, _) if matches!(inner.as_ref(), Expr::Pow(_, _))),
        apply: |expr, _ctx| {
            if let Expr::Pow(inner, outer_exp) = expr {
                if let Expr::Pow(base, inner_exp) = inner.as_ref() {
                    let new_exp = Expr::Mul(
                        Box::new(inner_exp.as_ref().clone()),
                        Box::new(outer_exp.as_ref().clone()),
                    );
                    return vec![RuleApplication {
                        result: Expr::Pow(base.clone(), Box::new(new_exp)),
                        justification: "(x^a)^b = x^(a*b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_constant_fold() {
        let rule = constant_fold();
        let ctx = RuleContext::default();

        // 2 + 3
        let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        assert!(rule.can_apply(&expr, &ctx));

        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].result, Expr::int(5));
    }

    #[test]
    fn test_identity_add_zero() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = identity_add_zero();
        let ctx = RuleContext::default();

        // x + 0
        let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
        assert!(rule.can_apply(&expr, &ctx));

        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].result, Expr::Var(x));
    }

    #[test]
    fn test_distribute() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        let rule = distribute();
        let ctx = RuleContext::default();

        // 2 * (x + y)
        let expr = Expr::Mul(
            Box::new(Expr::int(2)),
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
        );
        assert!(rule.can_apply(&expr, &ctx));

        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        // Result should be 2*x + 2*y
    }
}
