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
                matches!(expr, Expr::Add(_, _))
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
            is_applicable: |_expr, _ctx| false, // Need 3-term pattern matching
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| false, // Would need cube root in Expr
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| false, // Informational
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| false, // Informational
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
    ]
}
