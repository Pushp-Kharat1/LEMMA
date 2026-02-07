// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Advanced polynomial rules for IMO-level problem solving.
//! Includes Vieta's formulas, symmetric polynomials, partial fractions.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational, Symbol, SymbolTable};
use std::sync::{Mutex, OnceLock};

fn intern_symbol(name: &str) -> Symbol {
    static INTERNER: OnceLock<Mutex<SymbolTable>> = OnceLock::new();
    let m = INTERNER.get_or_init(|| Mutex::new(SymbolTable::new()));
    m.lock().expect("symbol interner poisoned").intern(name)
}

/// Collects the complete set of polynomial transformation and solving rules.
///
/// This aggregates rules from Vieta’s formulas, symmetric polynomial identities, factoring rules, rational-root criteria, and phase-3 advanced polynomial rules (IDs 500–527, 540–561, 800–818).
///
/// # Returns
///
/// A vector containing all defined `Rule` instances used for polynomial transformations, factorizations, and root analysis.
///
/// # Examples
///
/// ```
/// let rules = polynomial_rules();
/// assert_eq!(rules.len(), 54);
/// ```
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let a = intern_symbol("a");
                let b = intern_symbol("b");
                let r1 = intern_symbol("r1");
                let r2 = intern_symbol("r2");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Add(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                        rhs: Box::new(Expr::Div(
                            Box::new(Expr::Neg(Box::new(Expr::Var(b)))),
                            Box::new(Expr::Var(a)),
                        )),
                    },
                    justification: "Vieta (quadratic) r1+r2 = -b/a".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let a = intern_symbol("a");
                let c = intern_symbol("c");
                let r1 = intern_symbol("r1");
                let r2 = intern_symbol("r2");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Mul(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                        rhs: Box::new(Expr::Div(Box::new(Expr::Var(c)), Box::new(Expr::Var(a)))),
                    },
                    justification: "Vieta (quadratic) r1*r2 = c/a".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let a = intern_symbol("a");
                let b = intern_symbol("b");
                let r1 = intern_symbol("r1");
                let r2 = intern_symbol("r2");
                let r3 = intern_symbol("r3");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Add(
                            Box::new(Expr::Add(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                            Box::new(Expr::Var(r3)),
                        )),
                        rhs: Box::new(Expr::Div(
                            Box::new(Expr::Neg(Box::new(Expr::Var(b)))),
                            Box::new(Expr::Var(a)),
                        )),
                    },
                    justification: "Vieta (cubic) r1+r2+r3 = -b/a".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let a = intern_symbol("a");
                let c = intern_symbol("c");
                let r1 = intern_symbol("r1");
                let r2 = intern_symbol("r2");
                let r3 = intern_symbol("r3");
                let lhs = Expr::Add(
                    Box::new(Expr::Add(
                        Box::new(Expr::Mul(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                        Box::new(Expr::Mul(Box::new(Expr::Var(r2)), Box::new(Expr::Var(r3)))),
                    )),
                    Box::new(Expr::Mul(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r3)))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(lhs),
                        rhs: Box::new(Expr::Div(Box::new(Expr::Var(c)), Box::new(Expr::Var(a)))),
                    },
                    justification: "Vieta (cubic) r1r2 + r2r3 + r1r3 = c/a".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let a = intern_symbol("a");
                let d = intern_symbol("d");
                let r1 = intern_symbol("r1");
                let r2 = intern_symbol("r2");
                let r3 = intern_symbol("r3");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Mul(
                            Box::new(Expr::Mul(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                            Box::new(Expr::Var(r3)),
                        )),
                        rhs: Box::new(Expr::Div(
                            Box::new(Expr::Neg(Box::new(Expr::Var(d)))),
                            Box::new(Expr::Var(a)),
                        )),
                    },
                    justification: "Vieta (cubic) product = -d/a".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let e1 = intern_symbol("e1");
                let sum_x = intern_symbol("Σx_i");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(e1)),
                        rhs: Box::new(Expr::Var(sum_x)),
                    },
                    justification: "e1 equals sum of variables".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let e2 = intern_symbol("e2");
                let sum_pairs = intern_symbol("Σx_ix_j");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(e2)),
                        rhs: Box::new(Expr::Var(sum_pairs)),
                    },
                    justification: "e2 equals sum of pairwise products".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let p1 = intern_symbol("p1");
                let e1 = intern_symbol("e1");
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(p1)),
                        rhs: Box::new(Expr::Var(e1)),
                    },
                    justification: "Newton: p1 = e1".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let p2 = intern_symbol("p2");
                let e1 = intern_symbol("e1");
                let e2 = intern_symbol("e2");
                let rhs = Expr::Sub(
                    Box::new(Expr::Pow(Box::new(Expr::Var(e1)), Box::new(Expr::int(2)))),
                    Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(e2)))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(p2)),
                        rhs: Box::new(rhs),
                    },
                    justification: "Newton: p2 = e1^2 - 2e2".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let p3 = intern_symbol("p3");
                let e1 = intern_symbol("e1");
                let e2 = intern_symbol("e2");
                let e3 = intern_symbol("e3");
                let rhs = Expr::Add(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Pow(Box::new(Expr::Var(e1)), Box::new(Expr::int(3)))),
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(3)),
                            Box::new(Expr::Mul(Box::new(Expr::Var(e1)), Box::new(Expr::Var(e2)))),
                        )),
                    )),
                    Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(e3)))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(p3)),
                        rhs: Box::new(rhs),
                    },
                    justification: "Newton: p3 = e1^3 -3e1e2 +3e3".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                if let Expr::Add(x, y) = expr {
                    let sum = Expr::Add(x.clone(), y.clone());
                    let rhs = Expr::Sub(
                        Box::new(Expr::Pow(Box::new(sum.clone()), Box::new(Expr::int(3)))),
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(3)),
                            Box::new(Expr::Mul(x.clone(), Box::new(sum))),
                        )),
                    );
                    return vec![RuleApplication {
                        result: rhs,
                        justification: "x^3 + y^3 = (x+y)^3 - 3xy(x+y)".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    // Expect a = x^3 + y^3, b = z^3 - 3xyz or similar; provide factored form symbolically
                    let x = intern_symbol("x");
                    let y = intern_symbol("y");
                    let z = intern_symbol("z");
                    let lhs = Expr::Add(
                        Box::new(Expr::Add(
                            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
                            Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(3)))),
                        )),
                        Box::new(Expr::Add(
                            Box::new(Expr::Pow(Box::new(Expr::Var(z)), Box::new(Expr::int(3)))),
                            Box::new(Expr::Neg(Box::new(Expr::Mul(
                                Box::new(Expr::int(3)),
                                Box::new(Expr::Mul(
                                    Box::new(Expr::Var(x)),
                                    Box::new(Expr::Mul(
                                        Box::new(Expr::Var(y)),
                                        Box::new(Expr::Var(z)),
                                    )),
                                )),
                            )))),
                        )),
                    );
                    let factor1 = Expr::Add(
                        Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                        Box::new(Expr::Var(z)),
                    );
                    let factor2 = Expr::Add(
                        Box::new(Expr::Sub(
                            Box::new(Expr::Sub(
                                Box::new(Expr::Add(
                                    Box::new(Expr::Pow(
                                        Box::new(Expr::Var(x)),
                                        Box::new(Expr::int(2)),
                                    )),
                                    Box::new(Expr::Pow(
                                        Box::new(Expr::Var(y)),
                                        Box::new(Expr::int(2)),
                                    )),
                                )),
                                Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                            )),
                            Box::new(Expr::Mul(Box::new(Expr::Var(y)), Box::new(Expr::Var(z)))),
                        )),
                        Box::new(Expr::Sub(
                            Box::new(Expr::Pow(Box::new(Expr::Var(z)), Box::new(Expr::int(2)))),
                            Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(z)))),
                        )),
                    );
                    let rhs = Expr::Mul(Box::new(factor1), Box::new(factor2));
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        },
                        justification: "Sum of three cubes minus 3xyz factorization".to_string(),
                    }];
                }
                vec![]
            },
            reversible: true,
            cost: 4,
        },
    ]
}

// ============================================================================
// Polynomial Factoring Rules (ID 540+)
// ============================================================================

/// Collects factoring-related rule definitions for polynomial manipulations.
///
/// Returns a vector of Rule instances implementing factoring identities, theorems,
/// and decomposition/algorithmic helpers (Rule IDs 540–559).
///
/// # Examples
///
/// ```
/// let rules = factoring_rules();
/// assert_eq!(rules.len(), 20);
/// ```
fn factoring_rules() -> Vec<Rule> {
    vec![
        // Factor theorem: (x-a) divides P(x) iff P(a) = 0
        Rule {
            id: RuleId(540),
            name: "factor_theorem",
            category: RuleCategory::Factoring,
            description: "(x-a) | P(x) ⟺ P(a) = 0",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let p = intern_symbol("P(x)");
                let a = intern_symbol("a");
                let result = Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("P(a)"))),
                    rhs: Box::new(Expr::int(0)),
                };
                vec![RuleApplication {
                    result,
                    justification: "Factor theorem: (x-a)|P(x) iff P(a)=0".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let rem = intern_symbol("P(a)");
                vec![RuleApplication {
                    result: Expr::Var(rem),
                    justification: "Remainder theorem: remainder is P(a)".to_string(),
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
            apply: |_expr, _ctx| {
                let p = intern_symbol("P(x)");
                let d = intern_symbol("D(x)");
                let q = intern_symbol("Q(x)");
                let r = intern_symbol("R(x)");
                let rhs = Expr::Add(
                    Box::new(Expr::Mul(Box::new(Expr::Var(d)), Box::new(Expr::Var(q)))),
                    Box::new(Expr::Var(r)),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(p)),
                        rhs: Box::new(rhs),
                    },
                    justification: "P = D·Q + R".to_string(),
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
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if let Expr::Pow(base, _) = a.as_ref() {
                        // x² + bx where b = coefficient * x
                        // Result: (x + b/2)² - (b/2)²
                        let half_b = Expr::Div(b.clone(), Box::new(Expr::int(2)));
                        let x_plus_half_b = Expr::Add(base.clone(), Box::new(half_b.clone()));
                        let squared = Expr::Pow(Box::new(x_plus_half_b), Box::new(Expr::int(2)));
                        let quarter_b_sq = Expr::Pow(Box::new(half_b), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Sub(Box::new(squared), Box::new(quarter_b_sq)),
                            justification: "Complete the square: x² + bx = (x + b/2)² - (b/2)²"
                                .to_string(),
                        }];
                    }
                }
                vec![]
            },
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
            apply: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        // xⁿ - yⁿ = (x-y)(sum of terms)
                        // For general case, just return the factored form indication
                        let diff = Expr::Sub(base_a.clone(), base_b.clone());
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(diff), Box::new(expr.clone())),
                            justification:
                                "Difference of powers: xⁿ - yⁿ = (x-y)(xⁿ⁻¹ + xⁿ⁻²y + ... + yⁿ⁻¹)"
                                    .to_string(),
                        }];
                    }
                }
                vec![]
            },
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
                        let sum = Expr::Add(
                            Box::new(Expr::Add(Box::new(a_sq), Box::new(ab))),
                            Box::new(b_sq),
                        );
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(diff), Box::new(sum)),
                            justification: "Difference of cubes: a³ - b³ = (a-b)(a² + ab + b²)"
                                .to_string(),
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
                        let diff = Expr::Sub(
                            Box::new(Expr::Sub(Box::new(a_sq), Box::new(ab))),
                            Box::new(b_sq),
                        );
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(sum), Box::new(diff)),
                            justification: "Sum of cubes: a³ + b³ = (a+b)(a² - ab + b²)"
                                .to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                if let Expr::Add(a_term, b_term) = expr {
                    // assume structure a^4 + 4b^4
                    let a = intern_symbol("a");
                    let b = intern_symbol("b");
                    let factor1 = Expr::Add(
                        Box::new(Expr::Add(
                            Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(2)))),
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
                            )),
                        )),
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                        )),
                    );
                    let factor2 = Expr::Add(
                        Box::new(Expr::Add(
                            Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(2)))),
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
                            )),
                        )),
                        Box::new(Expr::Neg(Box::new(Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                        )))),
                    );
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(factor1), Box::new(factor2)),
                        justification: "Sophie Germain factorization".to_string(),
                    }];
                }
                vec![]
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
                if let Expr::Add(a, b) = expr {
                    // treat as (a+b)(x+y) schematic
                    let a_sym = intern_symbol("a");
                    let b_sym = intern_symbol("b");
                    let x_sym = intern_symbol("x");
                    let y_sym = intern_symbol("y");
                    let lhs = Expr::Mul(
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(a_sym)),
                            Box::new(Expr::Var(b_sym)),
                        )),
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(x_sym)),
                            Box::new(Expr::Var(y_sym)),
                        )),
                    );
                    return vec![RuleApplication {
                        result: lhs,
                        justification: "Factor by grouping schema".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                let x = intern_symbol("x");
                let y = intern_symbol("y");
                let factor = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
                vec![RuleApplication {
                    result: Expr::Mul(Box::new(factor), Box::new(expr.clone())),
                    justification: "x^{2n+1}+y^{2n+1} is divisible by (x+y)".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _)),
            apply: |expr, _ctx| {
                let x = intern_symbol("x");
                let y = intern_symbol("y");
                let factor = Expr::Mul(
                    Box::new(Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                    Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                );
                vec![RuleApplication {
                    result: Expr::Mul(Box::new(factor), Box::new(expr.clone())),
                    justification: "x^{2n}-y^{2n} divisible by (x-y)(x+y)".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _)),
            apply: |_expr, _ctx| {
                let phi = intern_symbol("Π Φ_d(x)");
                vec![RuleApplication {
                    result: Expr::Var(phi),
                    justification: "Cyclotomic: x^n-1 factors into cyclotomic polynomials"
                        .to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Add(_, _) | Expr::Sub(_, _))),
            apply: |expr, _ctx| {
                let k = intern_symbol("k");
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Add(x, y) | Expr::Sub(x, y) = base.as_ref() {
                        let term = Expr::Mul(
                            Box::new(Expr::Binomial(exp.clone(), Box::new(Expr::Var(k)))),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Pow(x.clone(), Box::new(Expr::Var(k)))),
                                Box::new(Expr::Pow(
                                    y.clone(),
                                    Box::new(Expr::Sub(exp.clone(), Box::new(Expr::Var(k)))),
                                )),
                            )),
                        );
                        let sum = Expr::Summation {
                            var: k,
                            from: Box::new(Expr::int(0)),
                            to: exp.clone(),
                            body: Box::new(term),
                        };
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: Box::new(expr.clone()),
                                rhs: Box::new(sum),
                            },
                            justification: "Binomial expansion".to_string(),
                        }];
                    }
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                let u = intern_symbol("u");
                vec![RuleApplication {
                    result: Expr::Var(u),
                    justification: "Let u = x^2 to reduce biquadratic".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let e_vars = intern_symbol("in_terms_of_e1_e2_e3");
                vec![RuleApplication {
                    result: Expr::Var(e_vars),
                    justification:
                        "Express symmetric polynomial via elementary symmetric polynomials"
                            .to_string(),
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
                    result: Expr::Var(intern_symbol("Σ A_i/(x-r_i)^k")),
                    justification: "Partial fractions schematic".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("nested_linear_form")),
                    justification: "Horner form nested linear".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("synthetic_div_result")),
                    justification: "Synthetic division schema".to_string(),
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
                    result: Expr::Var(intern_symbol("Q(x),R(x)")),
                    justification: "Long division schema".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("ruffini_result")),
                    justification: "Ruffini synthetic division".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("± factors(a0)/factors(an)")),
                    justification: "Rational root theorem".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("divides_constant_term")),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let a = intern_symbol("a");
            let b = intern_symbol("b");
            let c = intern_symbol("c");
            let disc = Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
                Box::new(Expr::Mul(
                    Box::new(Expr::int(4)),
                    Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(c)))),
                )),
            );
            let numerator = Expr::Add(
                Box::new(Expr::Neg(Box::new(Expr::Var(b)))),
                Box::new(Expr::Sqrt(Box::new(disc))),
            );
            let rhs = Expr::Div(
                Box::new(numerator),
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(a)))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("x"))),
                    rhs: Box::new(rhs),
                },
                justification: "Quadratic formula".to_string(),
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
        apply: |_expr, _ctx| {
            let delta = intern_symbol("Δ");
            let cases = Expr::Var(intern_symbol("{Δ>0:2 real, Δ=0:double, Δ<0:complex}"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(delta)),
                    rhs: Box::new(cases),
                },
                justification: "Discriminant sign cases".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("Δ=k^2 ⇒ rational roots")),
                justification: "Δ perfect square → rational roots".to_string(),
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("Cardano_solution")),
                justification: "Cardano formula schematic".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let p = intern_symbol("p");
            let q = intern_symbol("q");
            let rhs = Expr::Sub(
                Box::new(Expr::Mul(
                    Box::new(Expr::int(4)),
                    Box::new(Expr::Pow(Box::new(Expr::Var(p)), Box::new(Expr::int(3)))),
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::int(27)),
                    Box::new(Expr::Pow(Box::new(Expr::Var(q)), Box::new(Expr::int(2)))),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("Δ"))),
                    rhs: Box::new(rhs),
                },
                justification: "Cubic discriminant".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("resolvent_cubic")),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("sign_changes_bound")),
                justification: "Descartes' rule".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("sturm_sequence")),
                justification: "Sturm sequence root count".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("Res(f,g)"))),
                    rhs: Box::new(Expr::int(0)),
                },
                justification: "Resultant zero implies shared root".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("f*u+g*v"))),
                    rhs: Box::new(Expr::Var(intern_symbol("gcd(f,g)"))),
                },
                justification: "Bezout identity".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("cauchy_bound")),
                justification: "Cauchy root bound".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("fujiwara_bound")),
                justification: "Fujiwara bound".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("convex_hull_of_roots")),
                justification: "Gauss-Lucas theorem".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("Σ y_i Π(x-x_j)/(x_i-x_j)")),
                justification: "Lagrange interpolation".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("Newton_divided_differences")),
                justification: "Newton interpolation".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("T_{n+1}"))),
                    rhs: Box::new(Expr::Sub(
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Var(intern_symbol("x"))),
                                Box::new(Expr::Var(intern_symbol("T_n"))),
                            )),
                        )),
                        Box::new(Expr::Var(intern_symbol("T_{n-1}"))),
                    )),
                },
                justification: "Chebyshev recurrence".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let n = intern_symbol("n");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("H_{n+1}"))),
                    rhs: Box::new(Expr::Sub(
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Var(intern_symbol("x"))),
                                Box::new(Expr::Var(intern_symbol("H_n"))),
                            )),
                        )),
                        Box::new(Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Var(n)),
                                Box::new(Expr::Var(intern_symbol("H_{n-1}"))),
                            )),
                        )),
                    )),
                },
                justification: "Hermite recurrence".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let n = intern_symbol("n");
            let x = intern_symbol("x");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("P_{n+1}"))),
                    rhs: Box::new(Expr::Sub(
                        Box::new(Expr::Mul(
                            Box::new(Expr::Add(
                                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(n)))),
                                Box::new(Expr::int(1)),
                            )),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Var(x)),
                                Box::new(Expr::Var(intern_symbol("P_n"))),
                            )),
                        )),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Var(n)),
                            Box::new(Expr::Var(intern_symbol("P_{n-1}"))),
                        )),
                    )),
                },
                justification: "Legendre recurrence".to_string(),
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let n = intern_symbol("n");
            let x = intern_symbol("x");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("L_{n+1}"))),
                    rhs: Box::new(Expr::Sub(
                        Box::new(Expr::Mul(
                            Box::new(Expr::Sub(
                                Box::new(Expr::Add(
                                    Box::new(Expr::Mul(
                                        Box::new(Expr::int(2)),
                                        Box::new(Expr::Var(n)),
                                    )),
                                    Box::new(Expr::int(1)),
                                )),
                                Box::new(Expr::Var(x)),
                            )),
                            Box::new(Expr::Var(intern_symbol("L_n"))),
                        )),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Pow(Box::new(Expr::Var(n)), Box::new(Expr::int(2)))),
                            Box::new(Expr::Var(intern_symbol("L_{n-1}"))),
                        )),
                    )),
                },
                justification: "Laguerre recurrence".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}
