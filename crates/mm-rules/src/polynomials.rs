// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Advanced polynomial rules for IMO-level problem solving.
//! Includes Vieta's formulas, symmetric polynomials, partial fractions.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all polynomial rules (60+).
pub fn polynomial_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.extend(vieta_rules());
    rules.extend(symmetric_polynomial_rules());
    rules.extend(factoring_rules());
    rules.extend(rational_root_rules());

    rules
}

// ============================================================================
// Vieta's Formulas (ID 500+)
// For x² + bx + c = 0 with roots r,s: r+s = -b, rs = c
// ============================================================================

fn vieta_rules() -> Vec<Rule> {
    vec![
        // Sum of roots (quadratic)
        Rule {
            id: RuleId(500),
            name: "vieta_sum_quadratic",
            category: RuleCategory::AlgebraicSolving,
            description: "For ax² + bx + c = 0: r₁ + r₂ = -b/a",
            is_applicable: |_expr, _ctx| false, // Need polynomial representation
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Product of roots (quadratic)
        Rule {
            id: RuleId(501),
            name: "vieta_product_quadratic",
            category: RuleCategory::AlgebraicSolving,
            description: "For ax² + bx + c = 0: r₁ · r₂ = c/a",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Sum of roots (cubic)
        Rule {
            id: RuleId(502),
            name: "vieta_sum_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "For ax³ + bx² + cx + d = 0: r₁ + r₂ + r₃ = -b/a",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Pairwise product sum (cubic)
        Rule {
            id: RuleId(503),
            name: "vieta_pairs_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "r₁r₂ + r₂r₃ + r₁r₃ = c/a",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Product of roots (cubic)
        Rule {
            id: RuleId(504),
            name: "vieta_product_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "r₁ · r₂ · r₃ = -d/a",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
    ]
}

// ============================================================================
// Symmetric Polynomial Rules (ID 520+)
// Newton's identities connecting power sums and elementary symmetric polynomials
// ============================================================================

fn symmetric_polynomial_rules() -> Vec<Rule> {
    vec![
        // Elementary symmetric polynomials
        Rule {
            id: RuleId(520),
            name: "elementary_sym_1",
            category: RuleCategory::AlgebraicSolving,
            description: "e₁ = Σxᵢ (sum of variables)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(521),
            name: "elementary_sym_2",
            category: RuleCategory::AlgebraicSolving,
            description: "e₂ = Σxᵢxⱼ (sum of pairwise products)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Power sum to elementary
        Rule {
            id: RuleId(522),
            name: "newton_identity_1",
            category: RuleCategory::AlgebraicSolving,
            description: "p₁ = e₁",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(523),
            name: "newton_identity_2",
            category: RuleCategory::AlgebraicSolving,
            description: "p₂ = e₁² - 2e₂",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        Rule {
            id: RuleId(524),
            name: "newton_identity_3",
            category: RuleCategory::AlgebraicSolving,
            description: "p₃ = e₁³ - 3e₁e₂ + 3e₃",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // x² + y² = (x+y)² - 2xy
        Rule {
            id: RuleId(525),
            name: "sum_squares_sym",
            category: RuleCategory::Simplification,
            description: "x² + y² = (x+y)² - 2xy",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    let a_sq = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    let b_sq = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    return a_sq && b_sq;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        let sum_sq = Expr::Pow(
                            Box::new(Expr::Add(base_a.clone(), base_b.clone())),
                            Box::new(Expr::int(2)),
                        );
                        let two_prod = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(base_a.clone(), base_b.clone())),
                        );
                        return vec![RuleApplication {
                            result: Expr::Sub(Box::new(sum_sq), Box::new(two_prod)),
                            justification: "x² + y² = (x+y)² - 2xy".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // x³ + y³ = (x+y)³ - 3xy(x+y)
        Rule {
            id: RuleId(526),
            name: "sum_cubes_sym",
            category: RuleCategory::Simplification,
            description: "x³ + y³ = (x+y)³ - 3xy(x+y)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // x³ + y³ + z³ - 3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx)
        Rule {
            id: RuleId(527),
            name: "sum_three_cubes",
            category: RuleCategory::Factoring,
            description: "x³+y³+z³-3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
    ]
}

// ============================================================================
// Polynomial Factoring Rules (ID 540+)
// ============================================================================

fn factoring_rules() -> Vec<Rule> {
    vec![
        // Factor theorem: (x-a) divides P(x) iff P(a) = 0
        Rule {
            id: RuleId(540),
            name: "factor_theorem",
            category: RuleCategory::Factoring,
            description: "(x-a) | P(x) ⟺ P(a) = 0",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // Remainder theorem: P(x) = (x-a)Q(x) + P(a)
        Rule {
            id: RuleId(541),
            name: "remainder_theorem",
            category: RuleCategory::AlgebraicSolving,
            description: "P(a) is remainder when dividing P(x) by (x-a)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
        // Polynomial division identity
        Rule {
            id: RuleId(542),
            name: "poly_div_identity",
            category: RuleCategory::AlgebraicSolving,
            description: "P(x) = D(x)·Q(x) + R(x)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
        // Complete the square: x² + bx = (x + b/2)² - b²/4
        Rule {
            id: RuleId(543),
            name: "complete_square",
            category: RuleCategory::Simplification,
            description: "x² + bx = (x + b/2)² - b²/4",
            is_applicable: |expr, _ctx| {
                // Match pattern x² + bx
                if let Expr::Add(a, b) = expr {
                    if let Expr::Pow(_, exp) = a.as_ref() {
                        if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2))
                        {
                            // Check b is multiple of x
                            return matches!(b.as_ref(), Expr::Mul(_, _));
                        }
                    }
                }
                false
            },
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // Difference of nth powers
        Rule {
            id: RuleId(544),
            name: "diff_nth_power",
            category: RuleCategory::Factoring,
            description: "xⁿ - yⁿ = (x-y)(xⁿ⁻¹ + xⁿ⁻²y + ... + yⁿ⁻¹)",
            is_applicable: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Pow(_, exp_a), Expr::Pow(_, exp_b)) = (a.as_ref(), b.as_ref()) {
                        return exp_a == exp_b;
                    }
                }
                false
            },
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
    ]
}

// ============================================================================
// Rational Root Theorem Rules (ID 560+)
// ============================================================================

fn rational_root_rules() -> Vec<Rule> {
    vec![
        // Rational root theorem
        Rule {
            id: RuleId(560),
            name: "rational_root_theorem",
            category: RuleCategory::AlgebraicSolving,
            description:
                "Rational roots of aₙxⁿ + ... + a₀ have form ±(factor of a₀)/(factor of aₙ)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 4,
        },
        // Integer root criterion
        Rule {
            id: RuleId(561),
            name: "integer_root",
            category: RuleCategory::AlgebraicSolving,
            description: "Integer roots divide constant term",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
    ]
}
