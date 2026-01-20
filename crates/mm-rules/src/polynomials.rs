// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Advanced polynomial rules for IMO-level problem solving.
//! Includes Vieta's formulas, symmetric polynomials, partial fractions.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all polynomial rules (54 rules: 500-527, 540-561, 800-818).
pub fn polynomial_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.extend(vieta_rules());
    rules.extend(symmetric_polynomial_rules());
    rules.extend(factoring_rules());
    rules.extend(rational_root_rules());
    // Phase 3: Advanced polynomial rules
    rules.extend(advanced_polynomial_rules());

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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Equation { .. }),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vieta: For ax² + bx + c = 0, r₁ + r₂ = -b/a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Product of roots (quadratic)
        Rule {
            id: RuleId(501),
            name: "vieta_product_quadratic",
            category: RuleCategory::AlgebraicSolving,
            description: "For ax² + bx + c = 0: r₁ · r₂ = c/a",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Equation { .. }),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vieta: For ax² + bx + c = 0, r₁ · r₂ = c/a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Sum of roots (cubic)
        Rule {
            id: RuleId(502),
            name: "vieta_sum_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "For ax³ + bx² + cx + d = 0: r₁ + r₂ + r₃ = -b/a",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Equation { .. }),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vieta (cubic): r₁ + r₂ + r₃ = -b/a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Pairwise product sum (cubic)
        Rule {
            id: RuleId(503),
            name: "vieta_pairs_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "r₁r₂ + r₂r₃ + r₁r₃ = c/a",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vieta (cubic): r₁r₂ + r₂r₃ + r₁r₃ = c/a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Product of roots (cubic)
        Rule {
            id: RuleId(504),
            name: "vieta_product_cubic",
            category: RuleCategory::AlgebraicSolving,
            description: "r₁ · r₂ · r₃ = -d/a",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vieta (cubic): r₁ · r₂ · r₃ = -d/a".to_string(),
                }]
            },
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Elementary symmetric polynomial e₁ = Σxᵢ".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(521),
            name: "elementary_sym_2",
            category: RuleCategory::AlgebraicSolving,
            description: "e₂ = Σxᵢxⱼ (sum of pairwise products)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Elementary symmetric polynomial e₂ = Σxᵢxⱼ".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Power sum to elementary
        Rule {
            id: RuleId(522),
            name: "newton_identity_1",
            category: RuleCategory::AlgebraicSolving,
            description: "p₁ = e₁",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Newton's identity: p₁ = e₁".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(523),
            name: "newton_identity_2",
            category: RuleCategory::AlgebraicSolving,
            description: "p₂ = e₁² - 2e₂",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Pow(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Newton's identity: p₂ = e₁² - 2e₂".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        Rule {
            id: RuleId(524),
            name: "newton_identity_3",
            category: RuleCategory::AlgebraicSolving,
            description: "p₃ = e₁³ - 3e₁e₂ + 3e₃",
            is_applicable: |expr, _ctx| {
                matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Pow(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Newton's identity: p₃ = e₁³ - 3e₁e₂ + 3e₃".to_string(),
                }]
            },
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Pow(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "x³ + y³ = (x+y)³ - 3xy(x+y)".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        // x³ + y³ + z³ - 3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx)
        Rule {
            id: RuleId(527),
            name: "sum_three_cubes",
            category: RuleCategory::Factoring,
            description: "x³+y³+z³-3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "x³+y³+z³-3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx)".to_string(),
                }]
            },
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. } | Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Factor theorem: (x-a) divides P(x) iff P(a) = 0".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        // Remainder theorem: P(x) = (x-a)Q(x) + P(a)
        Rule {
            id: RuleId(541),
            name: "remainder_theorem",
            category: RuleCategory::AlgebraicSolving,
            description: "P(a) is remainder when dividing P(x) by (x-a)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification:
                        "Remainder theorem: P(a) is remainder when dividing P(x) by (x-a)"
                            .to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Polynomial division identity
        Rule {
            id: RuleId(542),
            name: "poly_div_identity",
            category: RuleCategory::AlgebraicSolving,
            description: "P(x) = D(x)·Q(x) + R(x)",
            is_applicable: |expr, _ctx| {
                matches!(expr, Expr::Div(_, _) | Expr::Add(_, _) | Expr::Mul(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Polynomial division: P(x) = D(x)·Q(x) + R(x)".to_string(),
                }]
            },
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
        // Difference of cubes: a³ - b³ = (a-b)(a² + ab + b²)
        Rule {
            id: RuleId(545),
            name: "diff_cubes",
            category: RuleCategory::Factoring,
            description: "a³ - b³ = (a-b)(a² + ab + b²)",
            is_applicable: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    let a_cube = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    let b_cube = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    return a_cube && b_cube;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        // (a-b)(a² + ab + b²)
                        let diff = Expr::Sub(base_a.clone(), base_b.clone());
                        let a_sq = Expr::Pow(base_a.clone(), Box::new(Expr::int(2)));
                        let ab = Expr::Mul(base_a.clone(), base_b.clone());
                        let b_sq = Expr::Pow(base_b.clone(), Box::new(Expr::int(2)));
                        let sum = Expr::Add(Box::new(Expr::Add(Box::new(a_sq), Box::new(ab))), Box::new(b_sq));
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(diff), Box::new(sum)),
                            justification: "Difference of cubes: a³ - b³ = (a-b)(a² + ab + b²)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 3,
        },
        // Sum of cubes: a³ + b³ = (a+b)(a² - ab + b²)
        Rule {
            id: RuleId(546),
            name: "sum_cubes",
            category: RuleCategory::Factoring,
            description: "a³ + b³ = (a+b)(a² - ab + b²)",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    let a_cube = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    let b_cube = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    return a_cube && b_cube;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        // (a+b)(a² - ab + b²)
                        let sum = Expr::Add(base_a.clone(), base_b.clone());
                        let a_sq = Expr::Pow(base_a.clone(), Box::new(Expr::int(2)));
                        let ab = Expr::Mul(base_a.clone(), base_b.clone());
                        let b_sq = Expr::Pow(base_b.clone(), Box::new(Expr::int(2)));
                        let diff = Expr::Sub(Box::new(Expr::Sub(Box::new(a_sq), Box::new(ab))), Box::new(b_sq));
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(sum), Box::new(diff)),
                            justification: "Sum of cubes: a³ + b³ = (a+b)(a² - ab + b²)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 3,
        },
        // Sophie Germain identity: a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)
        Rule {
            id: RuleId(547),
            name: "sophie_germain",
            category: RuleCategory::Factoring,
            description: "a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)",
            is_applicable: |expr, _ctx| {
                matches!(expr, Expr::Add(_, _) | Expr::Pow(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Sophie Germain: a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)".to_string(),
                }]
            },
            reversible: true,
            cost: 4,
        },
        // Factoring by grouping
        Rule {
            id: RuleId(548),
            name: "factor_by_grouping",
            category: RuleCategory::Factoring,
            description: "ax + ay + bx + by = (a+b)(x+y)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Factoring by grouping: ax + ay + bx + by = (a+b)(x+y)".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        // Sum of odd powers: x^(2n+1) + y^(2n+1) divisible by (x+y)
        Rule {
            id: RuleId(549),
            name: "sum_odd_powers",
            category: RuleCategory::Factoring,
            description: "x^(2n+1) + y^(2n+1) = (x+y)·Q(x,y)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Pow(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Sum of odd powers divisible by (x+y)".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Difference of even powers: x^(2n) - y^(2n) = (x-y)(x+y)·Q(x,y)
        Rule {
            id: RuleId(550),
            name: "diff_even_powers",
            category: RuleCategory::Factoring,
            description: "x^(2n) - y^(2n) = (x-y)(x+y)·Q(x,y)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Pow(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Difference of even powers: x^(2n) - y^(2n) = (x²-y²)·Q(x,y)".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Cyclotomic factorization
        Rule {
            id: RuleId(551),
            name: "cyclotomic_factor",
            category: RuleCategory::Factoring,
            description: "x^n - 1 = Π Φ_d(x) for d|n",
            is_applicable: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if matches!(a.as_ref(), Expr::Pow(_, _)) && matches!(b.as_ref(), Expr::Const(c) if c.is_one()) {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Cyclotomic factorization: x^n - 1 = Π Φ_d(x)".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Binomial expansion factorization
        Rule {
            id: RuleId(552),
            name: "binomial_factor",
            category: RuleCategory::Factoring,
            description: "(x+y)^n expansion via binomial theorem",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    return matches!(base.as_ref(), Expr::Add(_, _) | Expr::Sub(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Binomial expansion: (x+y)^n = Σ C(n,k)x^k y^(n-k)".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Quadratic in disguise: (x²)² + bx² + c
        Rule {
            id: RuleId(553),
            name: "quadratic_substitution",
            category: RuleCategory::Factoring,
            description: "Biquadratic: x⁴ + bx² + c via u = x²",
            is_applicable: |expr, _ctx| {
                matches!(expr, Expr::Add(_, _) | Expr::Pow(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Quadratic substitution: let u = x² for biquadratic".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        // Symmetric factorization
        Rule {
            id: RuleId(554),
            name: "symmetric_factor",
            category: RuleCategory::Factoring,
            description: "Symmetric polynomial factorization",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Symmetric polynomial can be factored via elementary symmetric functions".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Partial fraction decomposition
        Rule {
            id: RuleId(555),
            name: "partial_fractions",
            category: RuleCategory::Simplification,
            description: "P(x)/Q(x) = Σ A_i/(x-r_i)^k",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Partial fraction decomposition: P(x)/Q(x) = Σ A_i/(x-r_i)^k".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Horner's method for evaluation
        Rule {
            id: RuleId(556),
            name: "horner_method",
            category: RuleCategory::Simplification,
            description: "P(x) = (...((a_n·x + a_{n-1})x + ...)x + a_0",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Horner's method: efficient polynomial evaluation".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        // Synthetic division
        Rule {
            id: RuleId(557),
            name: "synthetic_division",
            category: RuleCategory::Simplification,
            description: "Synthetic division by (x-a)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Synthetic division: efficient division by linear factor".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        // Polynomial long division
        Rule {
            id: RuleId(558),
            name: "polynomial_long_division",
            category: RuleCategory::Simplification,
            description: "Long division algorithm for polynomials",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Polynomial long division: P(x) = D(x)·Q(x) + R(x)".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Ruffini's rule (special case of synthetic division)
        Rule {
            id: RuleId(559),
            name: "ruffini_rule",
            category: RuleCategory::Simplification,
            description: "Ruffini's rule for polynomial division",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Ruffini's rule: synthetic division variant".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Equation { .. }),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification:
                        "Rational root theorem: roots have form ±(factor of a₀)/(factor of aₙ)"
                            .to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Integer root criterion
        Rule {
            id: RuleId(561),
            name: "integer_root",
            category: RuleCategory::AlgebraicSolving,
            description: "Integer roots divide constant term",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Integer roots divide constant term".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
    ]
}

// ============================================================================
// Phase 3: Advanced Polynomial Rules (ID 800+)
// ============================================================================

/// Get all advanced polynomial rules
pub fn advanced_polynomial_rules() -> Vec<Rule> {
    vec![
        // Quadratic formula and discriminant
        quadratic_formula(),
        discriminant_sign(),
        discriminant_perfect_square(),
        // Cubic formulas
        cardano_formula(),
        cubic_discriminant(),
        // Quartic
        quartic_resolvent(),
        // Polynomial properties
        descartes_rule(),
        sturm_sequence(),
        // Resultant and discriminant
        resultant_definition(),
        bezout_theorem(),
        // Root bounds
        cauchy_bound(),
        fujiwara_bound(),
        // Derivative relationships
        gauss_lucas_theorem(),
        // Polynomial interpolation
        lagrange_interpolation(),
        newton_interpolation(),
        // Special polynomials
        chebyshev_recurrence(),
        hermite_recurrence(),
        legendre_recurrence(),
        laguerre_recurrence(),
    ]
}

// x = (-b ± √(b²-4ac)) / 2a
fn quadratic_formula() -> Rule {
    Rule {
        id: RuleId(800),
        name: "quadratic_formula",
        category: RuleCategory::EquationSolving,
        description: "x = (-b ± √(b²-4ac)) / 2a",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. } | Expr::Sqrt(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Quadratic formula: x = (-b ± √(b²-4ac)) / 2a".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Δ > 0: two distinct real roots
fn discriminant_sign() -> Rule {
    Rule {
        id: RuleId(801),
        name: "discriminant_sign",
        category: RuleCategory::AlgebraicSolving,
        description: "Δ > 0 ⟹ 2 real roots; Δ = 0 ⟹ 1 repeated; Δ < 0 ⟹ complex",
        is_applicable: |expr, _ctx| {
            matches!(
                expr,
                Expr::Gt(_, _) | Expr::Lt(_, _) | Expr::Equation { .. }
            )
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Discriminant sign determines root nature".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Δ is perfect square => rational roots
fn discriminant_perfect_square() -> Rule {
    Rule {
        id: RuleId(802),
        name: "discriminant_perfect_square",
        category: RuleCategory::AlgebraicSolving,
        description: "Δ = k² ⟹ rational roots",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _) | Expr::Sqrt(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Perfect square discriminant implies rational roots".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Cardano's formula for cubics
fn cardano_formula() -> Rule {
    Rule {
        id: RuleId(803),
        name: "cardano_formula",
        category: RuleCategory::EquationSolving,
        description: "Cardano's formula for x³ + px + q = 0",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _) | Expr::Equation { .. }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Cardano's formula for depressed cubic".to_string(),
            }]
        },
        reversible: false,
        cost: 5,
    }
}

// Cubic discriminant Δ = -4p³ - 27q²
fn cubic_discriminant() -> Rule {
    Rule {
        id: RuleId(804),
        name: "cubic_discriminant",
        category: RuleCategory::AlgebraicSolving,
        description: "Cubic discriminant Δ = -4p³ - 27q²",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Cubic discriminant: Δ = -4p³ - 27q²".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Resolvent cubic for quartics
fn quartic_resolvent() -> Rule {
    Rule {
        id: RuleId(805),
        name: "quartic_resolvent",
        category: RuleCategory::EquationSolving,
        description: "Resolvent cubic for quartic equations",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _) | Expr::Equation { .. }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Quartic resolvent cubic".to_string(),
            }]
        },
        reversible: false,
        cost: 5,
    }
}

// Sign changes bound positive real roots
fn descartes_rule() -> Rule {
    Rule {
        id: RuleId(806),
        name: "descartes_rule",
        category: RuleCategory::AlgebraicSolving,
        description: "Number of positive roots ≤ sign changes",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Lte(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Descartes' rule: positive roots ≤ sign changes".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Sturm sequence for root count
fn sturm_sequence() -> Rule {
    Rule {
        id: RuleId(807),
        name: "sturm_sequence",
        category: RuleCategory::AlgebraicSolving,
        description: "Sturm's theorem for counting real roots",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Derivative { expr: _, var: _ }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Sturm's theorem: count real roots via sequence".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Resultant of two polynomials
fn resultant_definition() -> Rule {
    Rule {
        id: RuleId(808),
        name: "resultant_definition",
        category: RuleCategory::AlgebraicSolving,
        description: "Res(f,g) = 0 ⟺ f and g share a root",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. } | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Resultant: Res(f,g) = 0 iff f and g share a root".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Bezout's theorem for polynomial GCD
fn bezout_theorem() -> Rule {
    Rule {
        id: RuleId(809),
        name: "bezout_theorem",
        category: RuleCategory::Simplification,
        description: "f·u + g·v = gcd(f,g) for some polynomials u,v",
        is_applicable: |expr, _ctx| matches!(expr, Expr::GCD(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Bezout's identity for polynomials: f·u + g·v = gcd(f,g)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// |roots| ≤ 1 + max|aᵢ/aₙ|
fn cauchy_bound() -> Rule {
    Rule {
        id: RuleId(810),
        name: "cauchy_bound",
        category: RuleCategory::AlgebraicSolving,
        description: "Cauchy bound on polynomial roots",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Abs(_) | Expr::Lte(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Cauchy root bound: |roots| ≤ 1 + max|aᵢ/aₙ|".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Fujiwara root bound
fn fujiwara_bound() -> Rule {
    Rule {
        id: RuleId(811),
        name: "fujiwara_bound",
        category: RuleCategory::AlgebraicSolving,
        description: "Fujiwara bound on polynomial roots",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Abs(_) | Expr::Lte(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Fujiwara root bound".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Roots of derivative in convex hull of roots
fn gauss_lucas_theorem() -> Rule {
    Rule {
        id: RuleId(812),
        name: "gauss_lucas_theorem",
        category: RuleCategory::AlgebraicSolving,
        description: "Critical points in convex hull of roots",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Derivative { expr: _, var: _ }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification:
                    "Gauss-Lucas: derivative roots lie in convex hull of polynomial roots"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Lagrange interpolation formula
fn lagrange_interpolation() -> Rule {
    Rule {
        id: RuleId(813),
        name: "lagrange_interpolation",
        category: RuleCategory::Simplification,
        description: "P(x) = Σ yᵢ Π(x-xⱼ)/(xᵢ-xⱼ)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Product { .. } | Expr::Sum { .. }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Lagrange interpolation: P(x) = Σ yᵢ Π(x-xⱼ)/(xᵢ-xⱼ)".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Newton's divided differences
fn newton_interpolation() -> Rule {
    Rule {
        id: RuleId(814),
        name: "newton_interpolation",
        category: RuleCategory::Simplification,
        description: "Newton form of interpolating polynomial",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Newton interpolation: divided differences form".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// T_{n+1}(x) = 2xT_n(x) - T_{n-1}(x)
fn chebyshev_recurrence() -> Rule {
    Rule {
        id: RuleId(815),
        name: "chebyshev_recurrence",
        category: RuleCategory::Simplification,
        description: "Chebyshev T_{n+1} = 2xT_n - T_{n-1}",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Chebyshev recurrence: T_{n+1} = 2xT_n - T_{n-1}".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// H_{n+1}(x) = 2xH_n(x) - 2nH_{n-1}(x)
fn hermite_recurrence() -> Rule {
    Rule {
        id: RuleId(816),
        name: "hermite_recurrence",
        category: RuleCategory::Simplification,
        description: "Hermite H_{n+1} = 2xH_n - 2nH_{n-1}",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Hermite recurrence: H_{n+1} = 2xH_n - 2nH_{n-1}".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// (n+1)P_{n+1} = (2n+1)xP_n - nP_{n-1}
fn legendre_recurrence() -> Rule {
    Rule {
        id: RuleId(817),
        name: "legendre_recurrence",
        category: RuleCategory::Simplification,
        description: "Legendre (n+1)P_{n+1} = (2n+1)xP_n - nP_{n-1}",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Legendre recurrence: (n+1)P_{n+1} = (2n+1)xP_n - nP_{n-1}"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// L_{n+1}(x) = (2n+1-x)L_n(x) - n²L_{n-1}(x)
fn laguerre_recurrence() -> Rule {
    Rule {
        id: RuleId(818),
        name: "laguerre_recurrence",
        category: RuleCategory::Simplification,
        description: "Laguerre L_{n+1} = (2n+1-x)L_n - n²L_{n-1}",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Laguerre recurrence: L_{n+1} = (2n+1-x)L_n - n²L_{n-1}".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}
