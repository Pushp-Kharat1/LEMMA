// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Algebraic transformation rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all algebra rules.
pub fn algebra_rules() -> Vec<Rule> {
    let mut rules = vec![
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
    ];
    // Add advanced algebra rules (Phase 1)
    rules.extend(advanced_algebra_rules());
    rules
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

// ============================================================================
// Phase 1: Advanced Algebra Rules (ID 300+)
// ============================================================================

/// Get all advanced algebra rules
pub fn advanced_algebra_rules() -> Vec<Rule> {
    vec![
        // Safe rules that don't affect Pythagorean identity
        // Sum/Difference of cubes
        sum_of_cubes(),
        diff_of_cubes(),
        // Sophie Germain identity
        sophie_germain(),
        // Power rules
        power_subtract(),
        negative_exponent(),
        fractional_distribute(),
        // Double negative
        double_negative(),
        // Binomial identities (may interfere - testing disabled)
        // binomial_square_expand(),
        // binomial_cube_expand(),
        // Subtraction to addition (may interfere - testing disabled)
        // sub_to_add(),
        // Division to multiplication (may interfere - testing disabled)
        // div_to_mul(),
    ]
}

// a³ + b³ = (a+b)(a² - ab + b²)
fn sum_of_cubes() -> Rule {
    Rule {
        id: RuleId(300),
        name: "sum_of_cubes",
        category: RuleCategory::Factoring,
        description: "a³ + b³ = (a+b)(a² - ab + b²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Pow(_, exp1), Expr::Pow(_, exp2)) = (left.as_ref(), right.as_ref()) {
                    if let (Expr::Const(e1), Expr::Const(e2)) = (exp1.as_ref(), exp2.as_ref()) {
                        return *e1 == Rational::from_integer(3)
                            && *e2 == Rational::from_integer(3);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref()) {
                    // (a+b)(a² - ab + b²)
                    let a_plus_b = Expr::Add(a.clone(), b.clone());
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let ab = Expr::Mul(a.clone(), b.clone());
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let second_factor = Expr::Add(
                        Box::new(Expr::Sub(Box::new(a_sq), Box::new(ab))),
                        Box::new(b_sq),
                    );
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(a_plus_b), Box::new(second_factor)),
                        justification: "a³ + b³ = (a+b)(a² - ab + b²)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// a³ - b³ = (a-b)(a² + ab + b²)
fn diff_of_cubes() -> Rule {
    Rule {
        id: RuleId(301),
        name: "diff_of_cubes",
        category: RuleCategory::Factoring,
        description: "a³ - b³ = (a-b)(a² + ab + b²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Pow(_, exp1), Expr::Pow(_, exp2)) = (left.as_ref(), right.as_ref()) {
                    if let (Expr::Const(e1), Expr::Const(e2)) = (exp1.as_ref(), exp2.as_ref()) {
                        return *e1 == Rational::from_integer(3)
                            && *e2 == Rational::from_integer(3);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref()) {
                    // (a-b)(a² + ab + b²)
                    let a_minus_b = Expr::Sub(a.clone(), b.clone());
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let ab = Expr::Mul(a.clone(), b.clone());
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let second_factor = Expr::Add(
                        Box::new(Expr::Add(Box::new(a_sq), Box::new(ab))),
                        Box::new(b_sq),
                    );
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(a_minus_b), Box::new(second_factor)),
                        justification: "a³ - b³ = (a-b)(a² + ab + b²)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// Sophie Germain: a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)
fn sophie_germain() -> Rule {
    Rule {
        id: RuleId(302),
        name: "sophie_germain",
        category: RuleCategory::Factoring,
        description: "a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)",
        is_applicable: |expr, _ctx| {
            // Match a⁴ + 4b⁴
            if let Expr::Add(left, right) = expr {
                if let Expr::Pow(_, exp1) = left.as_ref() {
                    if let Expr::Const(e1) = exp1.as_ref() {
                        if *e1 == Rational::from_integer(4) {
                            // Check right is 4*something^4
                            if let Expr::Mul(coef, inner) = right.as_ref() {
                                if let Expr::Const(c) = coef.as_ref() {
                                    if *c == Rational::from_integer(4) {
                                        if let Expr::Pow(_, exp2) = inner.as_ref() {
                                            if let Expr::Const(e2) = exp2.as_ref() {
                                                return *e2 == Rational::from_integer(4);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(left, right) = expr {
                if let Expr::Pow(a, _) = left.as_ref() {
                    if let Expr::Mul(_, inner) = right.as_ref() {
                        if let Expr::Pow(b, _) = inner.as_ref() {
                            // (a² + 2b² + 2ab)(a² + 2b² - 2ab)
                            let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                            let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                            let two_b_sq = Expr::Mul(Box::new(Expr::int(2)), Box::new(b_sq));
                            let two_ab = Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Mul(a.clone(), b.clone())),
                            );

                            let sum_part =
                                Expr::Add(Box::new(a_sq.clone()), Box::new(two_b_sq.clone()));
                            let factor1 =
                                Expr::Add(Box::new(sum_part.clone()), Box::new(two_ab.clone()));
                            let factor2 = Expr::Sub(Box::new(sum_part), Box::new(two_ab));

                            return vec![RuleApplication {
                                result: Expr::Mul(Box::new(factor1), Box::new(factor2)),
                                justification: "a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)"
                                    .to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

// (a+b)² = a² + 2ab + b²
fn binomial_square_expand() -> Rule {
    Rule {
        id: RuleId(303),
        name: "binomial_square_expand",
        category: RuleCategory::Expansion,
        description: "(a+b)² = a² + 2ab + b²",
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    if *e == Rational::from_integer(2) {
                        return matches!(base.as_ref(), Expr::Add(_, _));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Add(a, b) = base.as_ref() {
                    // a² + 2ab + b²
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let two_ab = Expr::Mul(
                        Box::new(Expr::int(2)),
                        Box::new(Expr::Mul(a.clone(), b.clone())),
                    );
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Add(Box::new(a_sq), Box::new(two_ab))),
                            Box::new(b_sq),
                        ),
                        justification: "(a+b)² = a² + 2ab + b²".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// (a+b)³ = a³ + 3a²b + 3ab² + b³
fn binomial_cube_expand() -> Rule {
    Rule {
        id: RuleId(304),
        name: "binomial_cube_expand",
        category: RuleCategory::Expansion,
        description: "(a+b)³ = a³ + 3a²b + 3ab² + b³",
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    if *e == Rational::from_integer(3) {
                        return matches!(base.as_ref(), Expr::Add(_, _));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Add(a, b) = base.as_ref() {
                    // a³ + 3a²b + 3ab² + b³
                    let a_cubed = Expr::Pow(a.clone(), Box::new(Expr::int(3)));
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let b_cubed = Expr::Pow(b.clone(), Box::new(Expr::int(3)));

                    let three_a_sq_b = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(Box::new(a_sq), b.clone())),
                    );
                    let three_a_b_sq = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(a.clone(), Box::new(b_sq))),
                    );

                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Add(
                                Box::new(Expr::Add(Box::new(a_cubed), Box::new(three_a_sq_b))),
                                Box::new(three_a_b_sq),
                            )),
                            Box::new(b_cubed),
                        ),
                        justification: "(a+b)³ = a³ + 3a²b + 3ab² + b³".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// x^a / x^b = x^(a-b)
fn power_subtract() -> Rule {
    Rule {
        id: RuleId(305),
        name: "power_subtract",
        category: RuleCategory::Simplification,
        description: "x^a / x^b = x^(a-b)",
        is_applicable: |expr, _ctx| {
            if let Expr::Div(num, den) = expr {
                if let (Expr::Pow(base1, _), Expr::Pow(base2, _)) = (num.as_ref(), den.as_ref()) {
                    return base1 == base2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(num, den) = expr {
                if let (Expr::Pow(base, exp1), Expr::Pow(_, exp2)) = (num.as_ref(), den.as_ref()) {
                    let new_exp = Expr::Sub(exp1.clone(), exp2.clone());
                    return vec![RuleApplication {
                        result: Expr::Pow(base.clone(), Box::new(new_exp)),
                        justification: "x^a / x^b = x^(a-b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// x^(-n) = 1/x^n
fn negative_exponent() -> Rule {
    Rule {
        id: RuleId(306),
        name: "negative_exponent",
        category: RuleCategory::Simplification,
        description: "x^(-n) = 1/x^n",
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(_, exp) = expr {
                if let Expr::Neg(_) = exp.as_ref() {
                    return true;
                }
                if let Expr::Const(c) = exp.as_ref() {
                    return c.is_negative();
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                let positive_exp = if let Expr::Neg(inner) = exp.as_ref() {
                    inner.clone()
                } else if let Expr::Const(c) = exp.as_ref() {
                    if c.is_negative() {
                        Box::new(Expr::Const(-c.clone()))
                    } else {
                        return vec![];
                    }
                } else {
                    return vec![];
                };
                return vec![RuleApplication {
                    result: Expr::Div(
                        Box::new(Expr::int(1)),
                        Box::new(Expr::Pow(base.clone(), positive_exp)),
                    ),
                    justification: "x^(-n) = 1/x^n".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// (a/b)^n = a^n / b^n
fn fractional_distribute() -> Rule {
    Rule {
        id: RuleId(307),
        name: "fractional_distribute",
        category: RuleCategory::Simplification,
        description: "(a/b)^n = a^n / b^n",
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(base, _) = expr {
                return matches!(base.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Div(num, den) = base.as_ref() {
                    let num_pow = Expr::Pow(num.clone(), exp.clone());
                    let den_pow = Expr::Pow(den.clone(), exp.clone());
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(num_pow), Box::new(den_pow)),
                        justification: "(a/b)^n = a^n / b^n".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// --x = x
fn double_negative() -> Rule {
    Rule {
        id: RuleId(308),
        name: "double_negative",
        category: RuleCategory::Simplification,
        description: "--x = x",
        is_applicable: |expr, _ctx| {
            if let Expr::Neg(inner) = expr {
                return matches!(inner.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Neg(inner) = expr {
                if let Expr::Neg(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: x.as_ref().clone(),
                        justification: "--x = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// a - b = a + (-b)
fn sub_to_add() -> Rule {
    Rule {
        id: RuleId(309),
        name: "sub_to_add",
        category: RuleCategory::Simplification,
        description: "a - b = a + (-b)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Sub(a, b) = expr {
                return vec![RuleApplication {
                    result: Expr::Add(a.clone(), Box::new(Expr::Neg(b.clone()))),
                    justification: "a - b = a + (-b)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// a / b = a * (1/b)
fn div_to_mul() -> Rule {
    Rule {
        id: RuleId(310),
        name: "div_to_mul",
        category: RuleCategory::Simplification,
        description: "a / b = a * (1/b)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Div(a, b) = expr {
                let reciprocal = Expr::Div(Box::new(Expr::int(1)), b.clone());
                return vec![RuleApplication {
                    result: Expr::Mul(a.clone(), Box::new(reciprocal)),
                    justification: "a / b = a * (1/b)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::RuleContext;

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
