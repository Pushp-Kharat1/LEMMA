// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Inequality rules for IMO-level problem solving.
//! Includes AM-GM, Cauchy-Schwarz, Jensen's, triangle inequality, and more.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all inequality rules (50+).
pub fn inequality_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.extend(am_gm_rules());
    rules.extend(cauchy_schwarz_rules());
    rules.extend(triangle_inequality_rules());
    rules.extend(absolute_value_rules());
    rules.extend(square_inequality_rules());
    // Phase 3: Advanced inequalities
    rules.extend(advanced_inequality_rules());

    rules
}

// ============================================================================
// AM-GM Inequality (ID 300+)
// For non-negative reals: (a+b)/2 >= sqrt(ab), equality iff a=b
// ============================================================================

fn am_gm_rules() -> Vec<Rule> {
    vec![
        // (a + b)/2 >= sqrt(ab) can be rewritten
        Rule {
            id: RuleId(300),
            name: "am_gm_2",
            category: RuleCategory::AlgebraicSolving,
            description: "AM-GM: (a+b)/2 ≥ √(ab), so a+b ≥ 2√(ab)",
            is_applicable: |expr, _ctx| {
                // Match a + b pattern where we might apply AM-GM
                // BUT NOT for pure arithmetic (no variables)
                if let Expr::Add(a, b) = expr {
                    // Only apply if expression contains variables
                    fn has_var(e: &Expr) -> bool {
                        match e {
                            Expr::Var(_) => true,
                            Expr::Const(_) | Expr::Pi | Expr::E => false,
                            Expr::Neg(x)
                            | Expr::Sqrt(x)
                            | Expr::Sin(x)
                            | Expr::Cos(x)
                            | Expr::Tan(x)
                            | Expr::Ln(x)
                            | Expr::Exp(x)
                            | Expr::Abs(x)
                            | Expr::Floor(x)
                            | Expr::Ceiling(x)
                            | Expr::Factorial(x) => has_var(x),
                            Expr::Add(x, y)
                            | Expr::Sub(x, y)
                            | Expr::Mul(x, y)
                            | Expr::Div(x, y)
                            | Expr::Pow(x, y)
                            | Expr::GCD(x, y)
                            | Expr::LCM(x, y)
                            | Expr::Mod(x, y)
                            | Expr::Binomial(x, y) => has_var(x) || has_var(y),
                            _ => false,
                        }
                    }
                    return has_var(a) || has_var(b);
                }
                false
            },
            apply: |expr, _ctx| {
                // This is informational - in a full system we'd track inequalities
                if let Expr::Add(a, b) = expr {
                    // Suggest: a + b >= 2*sqrt(a*b)
                    let sqrt_ab = Expr::Sqrt(Box::new(Expr::Mul(a.clone(), b.clone())));
                    let two_sqrt_ab = Expr::Mul(Box::new(Expr::int(2)), Box::new(sqrt_ab));
                    return vec![RuleApplication {
                        result: two_sqrt_ab,
                        justification: "AM-GM: a + b ≥ 2√(ab), equality iff a = b".to_string(),
                    }];
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
        // a^2 + b^2 >= 2ab (derived from AM-GM)
        Rule {
            id: RuleId(301),
            name: "sum_squares_ge_product",
            category: RuleCategory::AlgebraicSolving,
            description: "a² + b² ≥ 2ab (from (a-b)² ≥ 0)",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    let a_is_sq = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    let b_is_sq = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    return a_is_sq && b_is_sq;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        // a² + b² >= 2ab, so can bound
                        let two_ab = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(base_a.clone(), base_b.clone())),
                        );
                        return vec![RuleApplication {
                            result: two_ab,
                            justification: "a² + b² ≥ 2ab (lower bound)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // a² + b² + c² >= ab + bc + ca
        Rule {
            id: RuleId(302),
            name: "sum_three_squares",
            category: RuleCategory::AlgebraicSolving,
            description: "a² + b² + c² ≥ ab + bc + ca",
            is_applicable: |expr, _ctx| {
                // Match (a² + b²) + c² or similar 3-term sum of squares
                fn is_square(e: &Expr) -> bool {
                    matches!(e, Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)))
                }
                if let Expr::Add(left, right) = expr {
                    if is_square(right.as_ref()) {
                        if let Expr::Add(a, b) = left.as_ref() {
                            return is_square(a.as_ref()) && is_square(b.as_ref());
                        }
                    }
                    if is_square(left.as_ref()) {
                        if let Expr::Add(a, b) = right.as_ref() {
                            return is_square(a.as_ref()) && is_square(b.as_ref());
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                fn extract_base(e: &Expr) -> Option<Box<Expr>> {
                    if let Expr::Pow(base, _) = e {
                        Some(base.clone())
                    } else {
                        None
                    }
                }
                if let Expr::Add(left, right) = expr {
                    let (a_sq, b_sq, c_sq) = if let Expr::Add(a, b) = left.as_ref() {
                        (a.as_ref(), b.as_ref(), right.as_ref())
                    } else if let Expr::Add(a, b) = right.as_ref() {
                        (a.as_ref(), b.as_ref(), left.as_ref())
                    } else {
                        return vec![];
                    };
                    if let (Some(a), Some(b), Some(c)) =
                        (extract_base(a_sq), extract_base(b_sq), extract_base(c_sq))
                    {
                        // a² + b² + c² >= ab + bc + ca
                        let ab = Expr::Mul(a.clone(), b.clone());
                        let bc = Expr::Mul(b.clone(), c.clone());
                        let ca = Expr::Mul(c.clone(), a.clone());
                        let sum = Expr::Add(
                            Box::new(Expr::Add(Box::new(ab), Box::new(bc))),
                            Box::new(ca),
                        );
                        return vec![RuleApplication {
                            result: sum,
                            justification: "a² + b² + c² ≥ ab + bc + ca (lower bound)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
        // For positive reals: a/b + b/a >= 2
        Rule {
            id: RuleId(303),
            name: "reciprocal_sum_ge_2",
            category: RuleCategory::AlgebraicSolving,
            description: "a/b + b/a ≥ 2 for positive a,b",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(left, right) = expr {
                    if let (Expr::Div(a1, b1), Expr::Div(b2, a2)) = (left.as_ref(), right.as_ref())
                    {
                        return a1 == a2 && b1 == b2;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(left, right) = expr {
                    if let (Expr::Div(a1, b1), Expr::Div(b2, a2)) = (left.as_ref(), right.as_ref())
                    {
                        if a1 == a2 && b1 == b2 {
                            return vec![RuleApplication {
                                result: Expr::int(2),
                                justification: "a/b + b/a ≥ 2 (minimum)".to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // (a+b+c)/3 >= cbrt(abc) - AM-GM for 3 terms
        Rule {
            id: RuleId(304),
            name: "am_gm_3",
            category: RuleCategory::AlgebraicSolving,
            description: "AM-GM for 3 terms: (a+b+c)/3 ≥ ∛(abc)",
            is_applicable: |expr, _ctx| {
                // Match (a + b + c) / 3 or ((a + b) + c) / 3
                if let Expr::Div(num, denom) = expr {
                    if let Expr::Const(d) = denom.as_ref() {
                        if *d == Rational::from_integer(3) {
                            // Check if numerator is a 3-term sum
                            if let Expr::Add(left, right) = num.as_ref() {
                                if let Expr::Add(_, _) = left.as_ref() {
                                    return true;
                                }
                                if let Expr::Add(_, _) = right.as_ref() {
                                    return true;
                                }
                            }
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(num, _) = expr {
                    // Extract the 3 terms from the sum
                    if let Expr::Add(left, right) = num.as_ref() {
                        let (a, b, c) = if let Expr::Add(a, b) = left.as_ref() {
                            (a.clone(), b.clone(), right.clone())
                        } else if let Expr::Add(a, b) = right.as_ref() {
                            (left.clone(), a.clone(), b.clone())
                        } else {
                            return vec![];
                        };
                        // AM-GM: (a+b+c)/3 >= (abc)^(1/3)
                        let abc = Expr::Mul(Box::new(Expr::Mul(a, b)), c);
                        let one_third = Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(3)));
                        let cube_root = Expr::Pow(Box::new(abc), Box::new(one_third));
                        return vec![RuleApplication {
                            result: cube_root,
                            justification: "AM-GM: (a+b+c)/3 ≥ ∛(abc), equality iff a=b=c"
                                .to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
    ]
}

// ============================================================================
// Cauchy-Schwarz Inequality (ID 320+)
// (Σaᵢ²)(Σbᵢ²) >= (Σaᵢbᵢ)²
// ============================================================================

fn cauchy_schwarz_rules() -> Vec<Rule> {
    vec![
        // (a² + b²)(c² + d²) >= (ac + bd)²
        Rule {
            id: RuleId(320),
            name: "cauchy_schwarz_2",
            category: RuleCategory::AlgebraicSolving,
            description: "Cauchy-Schwarz: (a²+b²)(c²+d²) ≥ (ac+bd)²",
            is_applicable: |expr, _ctx| {
                // Match (a² + b²)(c² + d²) pattern
                if let Expr::Mul(left, right) = expr {
                    if let (Expr::Add(_, _), Expr::Add(_, _)) = (left.as_ref(), right.as_ref()) {
                        // Check if both are sums of squares
                        return true; // Simplified check
                    }
                }
                false
            },
            apply: |_expr, _ctx| {
                // Would need to extract variables and construct lower bound
                vec![]
            },
            reversible: false,
            cost: 4,
        },
        // Engel form / Titu's Lemma: a²/x + b²/y >= (a+b)²/(x+y)
        Rule {
            id: RuleId(321),
            name: "titus_lemma",
            category: RuleCategory::AlgebraicSolving,
            description: "Titu's Lemma: a²/x + b²/y ≥ (a+b)²/(x+y)",
            is_applicable: |expr, _ctx| {
                // Match a²/x + b²/y pattern
                fn is_sq_over_var(e: &Expr) -> bool {
                    if let Expr::Div(num, _denom) = e {
                        matches!(num.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)))
                    } else {
                        false
                    }
                }
                if let Expr::Add(left, right) = expr {
                    return is_sq_over_var(left.as_ref()) && is_sq_over_var(right.as_ref());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(left, right) = expr {
                    if let (Expr::Div(num1, denom1), Expr::Div(num2, denom2)) =
                        (left.as_ref(), right.as_ref())
                    {
                        if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (num1.as_ref(), num2.as_ref()) {
                            // (a+b)² / (x+y)
                            let a_plus_b = Expr::Add(a.clone(), b.clone());
                            let a_plus_b_sq = Expr::Pow(Box::new(a_plus_b), Box::new(Expr::int(2)));
                            let x_plus_y = Expr::Add(denom1.clone(), denom2.clone());
                            let result = Expr::Div(Box::new(a_plus_b_sq), Box::new(x_plus_y));
                            return vec![RuleApplication {
                                result,
                                justification:
                                    "Titu's Lemma: a²/x + b²/y ≥ (a+b)²/(x+y) (lower bound)"
                                        .to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 4,
        },
    ]
}

// ============================================================================
// Triangle Inequality (ID 340+)
// |a + b| <= |a| + |b|, |a - b| >= ||a| - |b||
// ============================================================================

fn triangle_inequality_rules() -> Vec<Rule> {
    vec![
        // |a + b| <= |a| + |b|
        Rule {
            id: RuleId(340),
            name: "triangle_ineq",
            category: RuleCategory::AlgebraicSolving,
            description: "|a + b| ≤ |a| + |b|",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Add(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Add(a, b) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Add(
                                Box::new(Expr::Abs(a.clone())),
                                Box::new(Expr::Abs(b.clone())),
                            ),
                            justification: "|a + b| ≤ |a| + |b| (upper bound)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // |a - b| >= ||a| - |b||
        Rule {
            id: RuleId(341),
            name: "reverse_triangle",
            category: RuleCategory::AlgebraicSolving,
            description: "|a - b| ≥ ||a| - |b||",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Sub(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Sub(a, b) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Abs(Box::new(Expr::Sub(
                                Box::new(Expr::Abs(a.clone())),
                                Box::new(Expr::Abs(b.clone())),
                            ))),
                            justification: "|a - b| ≥ ||a| - |b|| (lower bound)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
    ]
}

// ============================================================================
// Absolute Value Rules (ID 360+)
// ============================================================================

fn absolute_value_rules() -> Vec<Rule> {
    vec![
        // |a| >= 0
        Rule {
            id: RuleId(360),
            name: "abs_nonneg",
            category: RuleCategory::Simplification,
            description: "|a| ≥ 0 for all a",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Abs(_)),
            apply: |expr, _ctx| {
                if let Expr::Abs(_) = expr {
                    return vec![RuleApplication {
                        result: Expr::Gte(Box::new(expr.clone()), Box::new(Expr::int(0))),
                        justification: "|a| ≥ 0 always holds".to_string(),
                    }];
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // |a*b| = |a|*|b|
        Rule {
            id: RuleId(361),
            name: "abs_product",
            category: RuleCategory::Simplification,
            description: "|a·b| = |a|·|b|",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Mul(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Mul(a, b) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Abs(a.clone())),
                                Box::new(Expr::Abs(b.clone())),
                            ),
                            justification: "|a·b| = |a|·|b|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // |a/b| = |a|/|b|
        Rule {
            id: RuleId(362),
            name: "abs_quotient",
            category: RuleCategory::Simplification,
            description: "|a/b| = |a|/|b|",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Div(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Div(a, b) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Abs(a.clone())),
                                Box::new(Expr::Abs(b.clone())),
                            ),
                            justification: "|a/b| = |a|/|b|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // |-a| = |a|
        Rule {
            id: RuleId(363),
            name: "abs_neg",
            category: RuleCategory::Simplification,
            description: "|-a| = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Neg(_));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Neg(a) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "|-a| = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // ||a|| = |a|
        Rule {
            id: RuleId(364),
            name: "abs_abs",
            category: RuleCategory::Simplification,
            description: "||a|| = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Abs(_));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Abs(inner) = expr {
                    if let Expr::Abs(a) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "||a|| = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // |a|² = a²
        Rule {
            id: RuleId(365),
            name: "abs_squared",
            category: RuleCategory::Simplification,
            description: "|a|² = a²",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)) {
                        return matches!(base.as_ref(), Expr::Abs(_));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Abs(inner) = base.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Pow(inner.clone(), exp.clone()),
                            justification: "|a|² = a²".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
    ]
}

// ============================================================================
// Square Non-negativity Rules (ID 380+)
// ============================================================================

fn square_inequality_rules() -> Vec<Rule> {
    vec![
        // a² >= 0
        Rule {
            id: RuleId(380),
            name: "square_nonneg",
            category: RuleCategory::AlgebraicSolving,
            description: "a² ≥ 0 for all real a",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(_, exp) = expr {
                    return matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(_, _) = expr {
                    return vec![RuleApplication {
                        result: Expr::Gte(Box::new(expr.clone()), Box::new(Expr::int(0))),
                        justification: "a² ≥ 0 for all real a (squares are non-negative)"
                            .to_string(),
                    }];
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // a² = 0 iff a = 0
        Rule {
            id: RuleId(381),
            name: "square_zero",
            category: RuleCategory::EquationSolving,
            description: "a² = 0 ⟺ a = 0",
            is_applicable: |expr, _ctx| {
                if let Expr::Equation { lhs, rhs } = expr {
                    if matches!(rhs.as_ref(), Expr::Const(c) if c.is_zero()) {
                        if let Expr::Pow(_, exp) = lhs.as_ref() {
                            return matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Equation { lhs, .. } = expr {
                    if let Expr::Pow(base, _) = lhs.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: base.clone(),
                                rhs: Box::new(Expr::int(0)),
                            },
                            justification: "a² = 0 ⟺ a = 0".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // (a-b)² >= 0, so a² + b² >= 2ab
        Rule {
            id: RuleId(382),
            name: "diff_squared_ge_zero",
            category: RuleCategory::AlgebraicSolving,
            description: "(a-b)² ≥ 0",
            is_applicable: |expr, _ctx| {
                // Match (a-b)² pattern
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Const(e) = exp.as_ref() {
                        if *e == Rational::from_integer(2) {
                            return matches!(base.as_ref(), Expr::Sub(_, _));
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    if let Expr::Sub(_, _) = base.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Gte(Box::new(expr.clone()), Box::new(Expr::int(0))),
                            justification: "(a-b)² ≥ 0 always holds (squares are non-negative)"
                                .to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
    ]
}

// ============================================================================
// Phase 3: Advanced Inequalities (ID 500+)
// ============================================================================

/// Get all advanced inequality rules
pub fn advanced_inequality_rules() -> Vec<Rule> {
    vec![
        // Bernoulli's inequality
        bernoulli_inequality(),
        // Power mean inequalities
        qm_am_inequality(),
        hm_gm_inequality(),
        // Basic comparison rules
        positive_square_root(),
        exp_positivity(),
        // More absolute value rules
        abs_product(),
        abs_quotient(),
        abs_power(),
        // Inequality manipulations
        add_to_both_sides(),
        mul_positive_both_sides(),
        // Square root comparisons
        sqrt_comparison(),
        ln_comparison(),
        // Exponential and log inequalities
        exp_monotonic(),
        ln_monotonic(),
    ]
}

// (1+x)^n >= 1 + nx for x >= -1, n >= 1
fn bernoulli_inequality() -> Rule {
    Rule {
        id: RuleId(500),
        name: "bernoulli_inequality",
        category: RuleCategory::AlgebraicSolving,
        description: "(1+x)^n >= 1 + nx for x >= -1, n >= 1",
        is_applicable: |expr, _ctx| {
            // Match (1+x)^n pattern
            if let Expr::Pow(base, _) = expr {
                if let Expr::Add(left, _) = base.as_ref() {
                    if let Expr::Const(c) = left.as_ref() {
                        return *c == Rational::from_integer(1);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Add(_, x) = base.as_ref() {
                    // Lower bound: 1 + nx
                    let nx = Expr::Mul(exp.clone(), x.clone());
                    let lower_bound = Expr::Add(Box::new(Expr::int(1)), Box::new(nx));
                    return vec![RuleApplication {
                        result: lower_bound,
                        justification: "(1+x)^n >= 1+nx (Bernoulli)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// QM >= AM: sqrt((a² + b²)/2) >= (a + b)/2
fn qm_am_inequality() -> Rule {
    Rule {
        id: RuleId(501),
        name: "qm_am_inequality",
        category: RuleCategory::AlgebraicSolving,
        description: "QM >= AM: √((a²+b²)/2) >= (a+b)/2",
        is_applicable: |expr, _ctx| {
            // Match sqrt((a² + b²)/2) pattern
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let Expr::Const(d) = denom.as_ref() {
                        if *d == Rational::from_integer(2) {
                            return matches!(num.as_ref(), Expr::Add(_, _));
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            // Transform to lower bound (a+b)/2
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let Expr::Add(a_sq, b_sq) = num.as_ref() {
                        // Try to extract bases from squares
                        fn extract_base(e: &Expr) -> Option<Box<Expr>> {
                            if let Expr::Pow(base, exp) = e {
                                if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2))
                                {
                                    return Some(base.clone());
                                }
                            }
                            None
                        }
                        if let (Some(a), Some(b)) =
                            (extract_base(a_sq.as_ref()), extract_base(b_sq.as_ref()))
                        {
                            let a_plus_b = Expr::Add(a, b);
                            let am = Expr::Div(Box::new(a_plus_b), denom.clone());
                            return vec![RuleApplication {
                                result: am,
                                justification: "QM >= AM: √((a²+b²)/2) >= (a+b)/2 (lower bound)"
                                    .to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// HM <= GM: 2ab/(a+b) <= sqrt(ab)
fn hm_gm_inequality() -> Rule {
    Rule {
        id: RuleId(502),
        name: "hm_gm_inequality",
        category: RuleCategory::AlgebraicSolving,
        description: "HM <= GM: 2ab/(a+b) <= √(ab)",
        is_applicable: |expr, _ctx| {
            // Match 2ab/(a+b) pattern
            if let Expr::Div(num, denom) = expr {
                if let Expr::Mul(two, ab) = num.as_ref() {
                    if matches!(two.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)) {
                        if matches!(ab.as_ref(), Expr::Mul(_, _)) {
                            return matches!(denom.as_ref(), Expr::Add(_, _));
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(num, _denom) = expr {
                if let Expr::Mul(_, ab) = num.as_ref() {
                    if let Expr::Mul(a, b) = ab.as_ref() {
                        // Upper bound: sqrt(ab)
                        let ab_prod = Expr::Mul(a.clone(), b.clone());
                        let gm = Expr::Sqrt(Box::new(ab_prod));
                        return vec![RuleApplication {
                            result: gm,
                            justification: "HM <= GM: 2ab/(a+b) <= √(ab) (upper bound)".to_string(),
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

// sqrt(a) is real for a >= 0
fn positive_square_root() -> Rule {
    Rule {
        id: RuleId(503),
        name: "positive_square_root",
        category: RuleCategory::Simplification,
        description: "√a >= 0 for a >= 0",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sqrt(_)),
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// e^x > 0 for all x
fn exp_positivity() -> Rule {
    Rule {
        id: RuleId(504),
        name: "exp_positivity",
        category: RuleCategory::Simplification,
        description: "e^x > 0 for all x",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Exp(_)),
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// |ab| = |a||b|
fn abs_product() -> Rule {
    Rule {
        id: RuleId(505),
        name: "abs_product",
        category: RuleCategory::Simplification,
        description: "|ab| = |a||b|",
        is_applicable: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                return matches!(inner.as_ref(), Expr::Mul(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Mul(
                            Box::new(Expr::Abs(a.clone())),
                            Box::new(Expr::Abs(b.clone())),
                        ),
                        justification: "|ab| = |a||b|".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// |a/b| = |a|/|b|
fn abs_quotient() -> Rule {
    Rule {
        id: RuleId(506),
        name: "abs_quotient",
        category: RuleCategory::Simplification,
        description: "|a/b| = |a|/|b|",
        is_applicable: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                return matches!(inner.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Div(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Abs(a.clone())),
                            Box::new(Expr::Abs(b.clone())),
                        ),
                        justification: "|a/b| = |a|/|b|".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// |a^n| = |a|^n for integer n
fn abs_power() -> Rule {
    Rule {
        id: RuleId(507),
        name: "abs_power",
        category: RuleCategory::Simplification,
        description: "|a^n| = |a|^n",
        is_applicable: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                return matches!(inner.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Pow(Box::new(Expr::Abs(base.clone())), exp.clone()),
                        justification: "|a^n| = |a|^n".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// a = b => a + c = b + c
fn add_to_both_sides() -> Rule {
    Rule {
        id: RuleId(508),
        name: "add_to_both_sides",
        category: RuleCategory::EquationSolving,
        description: "Add same expression to both sides of equation",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. }),
        apply: |_expr, _ctx| vec![], // Needs c from context
        reversible: true,
        cost: 1,
    }
}

// a = b => ac = bc (for c > 0)
fn mul_positive_both_sides() -> Rule {
    Rule {
        id: RuleId(509),
        name: "mul_positive_both_sides",
        category: RuleCategory::EquationSolving,
        description: "Multiply both sides by positive expression",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. }),
        apply: |_expr, _ctx| vec![], // Needs c from context
        reversible: true,
        cost: 1,
    }
}

// a >= b >= 0 => sqrt(a) >= sqrt(b)
fn sqrt_comparison() -> Rule {
    Rule {
        id: RuleId(510),
        name: "sqrt_comparison",
        category: RuleCategory::AlgebraicSolving,
        description: "For a,b >= 0: a >= b => √a >= √b",
        is_applicable: |expr, _ctx| {
            // Match a >= b where both could have sqrt
            matches!(expr, Expr::Gte(_, _) | Expr::Gt(_, _))
        },
        apply: |expr, _ctx| {
            if let Expr::Gte(a, b) = expr {
                let sqrt_a = Expr::Sqrt(a.clone());
                let sqrt_b = Expr::Sqrt(b.clone());
                return vec![RuleApplication {
                    result: Expr::Gte(Box::new(sqrt_a), Box::new(sqrt_b)),
                    justification: "For a,b >= 0: a >= b => √a >= √b (sqrt is increasing)"
                        .to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// a > b > 0 => ln(a) > ln(b)
fn ln_comparison() -> Rule {
    Rule {
        id: RuleId(511),
        name: "ln_comparison",
        category: RuleCategory::AlgebraicSolving,
        description: "For a,b > 0: a > b => ln(a) > ln(b)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Gt(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Gt(a, b) = expr {
                let ln_a = Expr::Ln(a.clone());
                let ln_b = Expr::Ln(b.clone());
                return vec![RuleApplication {
                    result: Expr::Gt(Box::new(ln_a), Box::new(ln_b)),
                    justification: "For a,b > 0: a > b => ln(a) > ln(b) (ln is increasing)"
                        .to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// a > b => e^a > e^b
fn exp_monotonic() -> Rule {
    Rule {
        id: RuleId(512),
        name: "exp_monotonic",
        category: RuleCategory::AlgebraicSolving,
        description: "a > b => e^a > e^b (exp is increasing)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Gt(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Gt(a, b) = expr {
                let exp_a = Expr::Exp(a.clone());
                let exp_b = Expr::Exp(b.clone());
                return vec![RuleApplication {
                    result: Expr::Gt(Box::new(exp_a), Box::new(exp_b)),
                    justification: "a > b => e^a > e^b (exp is strictly increasing)".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// a > b > 0 => ln(a) > ln(b)
fn ln_monotonic() -> Rule {
    Rule {
        id: RuleId(513),
        name: "ln_monotonic",
        category: RuleCategory::AlgebraicSolving,
        description: "a > b > 0 => ln(a) > ln(b) (ln is increasing)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Gt(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Gt(a, b) = expr {
                let ln_a = Expr::Ln(a.clone());
                let ln_b = Expr::Ln(b.clone());
                return vec![RuleApplication {
                    result: Expr::Gt(Box::new(ln_a), Box::new(ln_b)),
                    justification: "a > b > 0 => ln(a) > ln(b) (ln is strictly increasing on R+)"
                        .to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}
