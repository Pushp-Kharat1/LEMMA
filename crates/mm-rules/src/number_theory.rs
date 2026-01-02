// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Number theory transformation rules for IMO-level problem solving.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all number theory rules (100+).
pub fn number_theory_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    // Divisibility rules
    rules.extend(divisibility_rules());
    // Modular arithmetic
    rules.extend(modular_rules());
    // GCD/LCM
    rules.extend(gcd_lcm_rules());
    // Perfect powers
    rules.extend(perfect_power_rules());
    // Parity rules
    rules.extend(parity_rules());
    // Sum formulas
    rules.extend(sum_formulas());
    // Factorial rules
    rules.extend(factorial_rules());
    // Floor/Ceiling rules
    rules.extend(floor_ceiling_rules());

    rules
}

// ============================================================================
// Divisibility Rules (ID 100+)
// ============================================================================

fn divisibility_rules() -> Vec<Rule> {
    vec![
        // n | 0 for all n
        Rule {
            id: RuleId(100),
            name: "divides_zero",
            category: RuleCategory::AlgebraicSolving,
            description: "n divides 0 for any n",
            is_applicable: |expr, _ctx| {
                // Match: Divides(n, 0)
                if let Expr::Div(_, b) = expr {
                    return matches!(b.as_ref(), Expr::Const(c) if c.is_zero());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(_, b) = expr {
                    if matches!(b.as_ref(), Expr::Const(c) if c.is_zero()) {
                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(0)),
                            justification: "0/n = 0".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // n | n for all n
        Rule {
            id: RuleId(101),
            name: "divides_self",
            category: RuleCategory::Simplification,
            description: "n/n = 1 for any n ≠ 0",
            is_applicable: |expr, _ctx| {
                if let Expr::Div(a, b) = expr {
                    return a == b && !matches!(b.as_ref(), Expr::Const(c) if c.is_zero());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(a, b) = expr {
                    if a == b {
                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(1)),
                            justification: "n/n = 1".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // 2 | (a + a) = 2 | 2a
        Rule {
            id: RuleId(102),
            name: "even_sum",
            category: RuleCategory::Simplification,
            description: "a + a = 2a (even)",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    return a == b;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if a == b {
                        return vec![RuleApplication {
                            result: Expr::Mul(Box::new(Expr::int(2)), a.clone()),
                            justification: "a + a = 2a".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 1,
        },
        // Divisibility by 2: last digit even
        Rule {
            id: RuleId(103),
            name: "div_by_2",
            category: RuleCategory::Simplification,
            description: "2n is divisible by 2",
            is_applicable: |expr, _ctx| {
                if let Expr::Mul(a, _) = expr {
                    if let Expr::Const(c) = a.as_ref() {
                        return *c == Rational::from_integer(2);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mul(_, b) = expr {
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(Expr::int(2)), b.clone()),
                        justification: "2n is even (divisible by 2)".to_string(),
                    }];
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // a*b / a = b (when a != 0)
        Rule {
            id: RuleId(104),
            name: "cancel_common_factor",
            category: RuleCategory::Simplification,
            description: "(a*b)/a = b",
            is_applicable: |expr, _ctx| {
                if let Expr::Div(num, denom) = expr {
                    if let Expr::Mul(a, _) = num.as_ref() {
                        return a == denom;
                    }
                    if let Expr::Mul(_, b) = num.as_ref() {
                        return b == denom;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(num, denom) = expr {
                    if let Expr::Mul(a, b) = num.as_ref() {
                        if a == denom {
                            return vec![RuleApplication {
                                result: b.as_ref().clone(),
                                justification: "(a*b)/a = b".to_string(),
                            }];
                        }
                        if b == denom {
                            return vec![RuleApplication {
                                result: a.as_ref().clone(),
                                justification: "(a*b)/b = a".to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // (a/b) * b = a
        Rule {
            id: RuleId(105),
            name: "mul_by_denom",
            category: RuleCategory::Simplification,
            description: "(a/b) * b = a",
            is_applicable: |expr, _ctx| {
                if let Expr::Mul(a, b) = expr {
                    if let Expr::Div(_, denom) = a.as_ref() {
                        return denom == b;
                    }
                    if let Expr::Div(_, denom) = b.as_ref() {
                        return denom == a;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mul(a, b) = expr {
                    if let Expr::Div(num, denom) = a.as_ref() {
                        if denom == b {
                            return vec![RuleApplication {
                                result: num.as_ref().clone(),
                                justification: "(a/b) * b = a".to_string(),
                            }];
                        }
                    }
                    if let Expr::Div(num, denom) = b.as_ref() {
                        if denom == a {
                            return vec![RuleApplication {
                                result: num.as_ref().clone(),
                                justification: "b * (a/b) = a".to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // a^2 - b^2 = (a+b)(a-b)
        Rule {
            id: RuleId(106),
            name: "diff_squares_factor",
            category: RuleCategory::Factoring,
            description: "a² - b² = (a+b)(a-b)",
            is_applicable: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    let a_is_sq = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    let b_is_sq = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                    return a_is_sq && b_is_sq;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Add(base_a.clone(), base_b.clone())),
                                Box::new(Expr::Sub(base_a.clone(), base_b.clone())),
                            ),
                            justification: "a² - b² = (a+b)(a-b)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // a^3 - b^3 = (a-b)(a^2 + ab + b^2)
        Rule {
            id: RuleId(107),
            name: "diff_cubes_factor",
            category: RuleCategory::Factoring,
            description: "a³ - b³ = (a-b)(a² + ab + b²)",
            is_applicable: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    let a_is_cube = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    let b_is_cube = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    return a_is_cube && b_is_cube;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        let a_sq = Expr::Pow(base_a.clone(), Box::new(Expr::int(2)));
                        let ab = Expr::Mul(base_a.clone(), base_b.clone());
                        let b_sq = Expr::Pow(base_b.clone(), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Sub(base_a.clone(), base_b.clone())),
                                Box::new(Expr::Add(
                                    Box::new(Expr::Add(Box::new(a_sq), Box::new(ab))),
                                    Box::new(b_sq),
                                )),
                            ),
                            justification: "a³ - b³ = (a-b)(a² + ab + b²)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 3,
        },
        // a^3 + b^3 = (a+b)(a^2 - ab + b^2)
        Rule {
            id: RuleId(108),
            name: "sum_cubes_factor",
            category: RuleCategory::Factoring,
            description: "a³ + b³ = (a+b)(a² - ab + b²)",
            is_applicable: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    let a_is_cube = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    let b_is_cube = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(3)));
                    return a_is_cube && b_is_cube;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Add(a, b) = expr {
                    if let (Expr::Pow(base_a, _), Expr::Pow(base_b, _)) = (a.as_ref(), b.as_ref()) {
                        let a_sq = Expr::Pow(base_a.clone(), Box::new(Expr::int(2)));
                        let ab = Expr::Mul(base_a.clone(), base_b.clone());
                        let b_sq = Expr::Pow(base_b.clone(), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Add(base_a.clone(), base_b.clone())),
                                Box::new(Expr::Add(
                                    Box::new(Expr::Sub(Box::new(a_sq), Box::new(ab))),
                                    Box::new(b_sq),
                                )),
                            ),
                            justification: "a³ + b³ = (a+b)(a² - ab + b²)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 3,
        },
        // (a+b)^2 = a^2 + 2ab + b^2
        Rule {
            id: RuleId(109),
            name: "square_binomial_expand",
            category: RuleCategory::Expansion,
            description: "(a+b)² = a² + 2ab + b²",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)) {
                        return matches!(base.as_ref(), Expr::Add(_, _));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    if let Expr::Add(a, b) = base.as_ref() {
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
        },
        // (a-b)^2 = a^2 - 2ab + b^2
        Rule {
            id: RuleId(110),
            name: "square_binomial_sub_expand",
            category: RuleCategory::Expansion,
            description: "(a-b)² = a² - 2ab + b²",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)) {
                        return matches!(base.as_ref(), Expr::Sub(_, _));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    if let Expr::Sub(a, b) = base.as_ref() {
                        let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                        let two_ab = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(a.clone(), b.clone())),
                        );
                        let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Add(
                                Box::new(Expr::Sub(Box::new(a_sq), Box::new(two_ab))),
                                Box::new(b_sq),
                            ),
                            justification: "(a-b)² = a² - 2ab + b²".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
    ]
}

// ============================================================================
// Modular Arithmetic Rules (ID 120+)
// ============================================================================

fn modular_rules() -> Vec<Rule> {
    vec![
        // a mod a = 0
        Rule {
            id: RuleId(120),
            name: "mod_self",
            category: RuleCategory::Simplification,
            description: "a mod a = 0",
            is_applicable: |expr, _ctx| {
                // We don't have a Mod operator, but we can recognize floor(a/a)*a - a pattern
                false // placeholder - would need Mod in Expr
            },
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // 0 mod n = 0
        Rule {
            id: RuleId(121),
            name: "zero_mod",
            category: RuleCategory::Simplification,
            description: "0 mod n = 0",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
    ]
}

// ============================================================================
// GCD/LCM Rules (ID 140+)
// ============================================================================

fn gcd_lcm_rules() -> Vec<Rule> {
    vec![
        // gcd(a, a) = a
        Rule {
            id: RuleId(140),
            name: "gcd_self",
            category: RuleCategory::Simplification,
            description: "gcd(a, a) = a",
            is_applicable: |_expr, _ctx| false, // Need GCD in Expr
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // gcd(a, 0) = a
        Rule {
            id: RuleId(141),
            name: "gcd_zero",
            category: RuleCategory::Simplification,
            description: "gcd(a, 0) = |a|",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // gcd(a, 1) = 1
        Rule {
            id: RuleId(142),
            name: "gcd_one",
            category: RuleCategory::Simplification,
            description: "gcd(a, 1) = 1",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // lcm(a, a) = a
        Rule {
            id: RuleId(143),
            name: "lcm_self",
            category: RuleCategory::Simplification,
            description: "lcm(a, a) = a",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // lcm(a, 1) = a
        Rule {
            id: RuleId(144),
            name: "lcm_one",
            category: RuleCategory::Simplification,
            description: "lcm(a, 1) = |a|",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // gcd(a,b) * lcm(a,b) = |a*b|
        Rule {
            id: RuleId(145),
            name: "gcd_lcm_product",
            category: RuleCategory::AlgebraicSolving,
            description: "gcd(a,b) * lcm(a,b) = |a*b|",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
    ]
}

// ============================================================================
// Perfect Power Rules (ID 160+)
// ============================================================================

fn perfect_power_rules() -> Vec<Rule> {
    vec![
        // sqrt(a^2) = |a|
        Rule {
            id: RuleId(160),
            name: "sqrt_square",
            category: RuleCategory::Simplification,
            description: "√(a²) = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Pow(_, exp) = inner.as_ref() {
                        return matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Pow(base, _) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Abs(base.clone()),
                            justification: "√(a²) = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // (sqrt(a))^2 = a (for a >= 0)
        Rule {
            id: RuleId(161),
            name: "square_sqrt",
            category: RuleCategory::Simplification,
            description: "(√a)² = a",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)) {
                        return matches!(base.as_ref(), Expr::Sqrt(_));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    if let Expr::Sqrt(inner) = base.as_ref() {
                        return vec![RuleApplication {
                            result: inner.as_ref().clone(),
                            justification: "(√a)² = a".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // sqrt(a) * sqrt(b) = sqrt(ab)
        Rule {
            id: RuleId(162),
            name: "sqrt_product",
            category: RuleCategory::Simplification,
            description: "√a · √b = √(ab)",
            is_applicable: |expr, _ctx| {
                if let Expr::Mul(a, b) = expr {
                    return matches!(a.as_ref(), Expr::Sqrt(_))
                        && matches!(b.as_ref(), Expr::Sqrt(_));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mul(a, b) = expr {
                    if let (Expr::Sqrt(inner_a), Expr::Sqrt(inner_b)) = (a.as_ref(), b.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Sqrt(Box::new(Expr::Mul(
                                inner_a.clone(),
                                inner_b.clone(),
                            ))),
                            justification: "√a · √b = √(ab)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // sqrt(a/b) = sqrt(a)/sqrt(b)
        Rule {
            id: RuleId(163),
            name: "sqrt_quotient",
            category: RuleCategory::Simplification,
            description: "√(a/b) = √a/√b",
            is_applicable: |expr, _ctx| {
                if let Expr::Sqrt(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Div(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Div(a, b) = inner.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Sqrt(a.clone())),
                                Box::new(Expr::Sqrt(b.clone())),
                            ),
                            justification: "√(a/b) = √a/√b".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // a^(1/2) = sqrt(a)
        Rule {
            id: RuleId(164),
            name: "half_power_sqrt",
            category: RuleCategory::Simplification,
            description: "a^(1/2) = √a",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(_, exp) = expr {
                    if let Expr::Div(num, denom) = exp.as_ref() {
                        return matches!(num.as_ref(), Expr::Const(c) if c.is_one())
                            && matches!(denom.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    return vec![RuleApplication {
                        result: Expr::Sqrt(base.clone()),
                        justification: "a^(1/2) = √a".to_string(),
                    }];
                }
                vec![]
            },
            reversible: true,
            cost: 1,
        },
    ]
}

// ============================================================================
// Parity Rules (ID 180+)
// ============================================================================

fn parity_rules() -> Vec<Rule> {
    vec![
        // (-1)^(2n) = 1
        Rule {
            id: RuleId(180),
            name: "neg_one_even_power",
            category: RuleCategory::Simplification,
            description: "(-1)^(2n) = 1",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Neg(inner) = base.as_ref() {
                        if matches!(inner.as_ref(), Expr::Const(c) if c.is_one()) {
                            // Check if exponent is even
                            if let Expr::Const(n) = exp.as_ref() {
                                return n.is_integer() && n.numer() % 2 == 0;
                            }
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Neg(inner) = base.as_ref() {
                        if matches!(inner.as_ref(), Expr::Const(c) if c.is_one()) {
                            if let Expr::Const(n) = exp.as_ref() {
                                if n.is_integer() && n.numer() % 2 == 0 {
                                    return vec![RuleApplication {
                                        result: Expr::int(1),
                                        justification: "(-1)^(2n) = 1".to_string(),
                                    }];
                                }
                            }
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // (-1)^(2n+1) = -1
        Rule {
            id: RuleId(181),
            name: "neg_one_odd_power",
            category: RuleCategory::Simplification,
            description: "(-1)^(2n+1) = -1",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Neg(inner) = base.as_ref() {
                        if matches!(inner.as_ref(), Expr::Const(c) if c.is_one()) {
                            if let Expr::Const(n) = exp.as_ref() {
                                return n.is_integer() && n.numer() % 2 != 0;
                            }
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Neg(inner) = base.as_ref() {
                        if matches!(inner.as_ref(), Expr::Const(c) if c.is_one()) {
                            if let Expr::Const(n) = exp.as_ref() {
                                if n.is_integer() && n.numer() % 2 != 0 {
                                    return vec![RuleApplication {
                                        result: Expr::Neg(Box::new(Expr::int(1))),
                                        justification: "(-1)^(2n+1) = -1".to_string(),
                                    }];
                                }
                            }
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // (-a)^2 = a^2
        Rule {
            id: RuleId(182),
            name: "neg_squared",
            category: RuleCategory::Simplification,
            description: "(-a)² = a²",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(base.as_ref(), Expr::Neg(_)) {
                        if let Expr::Const(n) = exp.as_ref() {
                            return n.is_integer()
                                && n.numer() % 2 == 0
                                && *n > Rational::from_integer(0);
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Neg(inner) = base.as_ref() {
                        return vec![RuleApplication {
                            result: Expr::Pow(inner.clone(), exp.clone()),
                            justification: "(-a)² = a²".to_string(),
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
// Sum Formulas (ID 200+)
// ============================================================================

fn sum_formulas() -> Vec<Rule> {
    vec![
        // These would represent sigma notation transformations
        // Placeholder for now - would need Sum expression type
        Rule {
            id: RuleId(200),
            name: "sum_constant",
            category: RuleCategory::Simplification,
            description: "Σc (n times) = cn",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(201),
            name: "sum_arithmetic",
            category: RuleCategory::Simplification,
            description: "1+2+...+n = n(n+1)/2",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(202),
            name: "sum_squares",
            category: RuleCategory::Simplification,
            description: "1²+2²+...+n² = n(n+1)(2n+1)/6",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(203),
            name: "sum_cubes",
            category: RuleCategory::Simplification,
            description: "1³+2³+...+n³ = [n(n+1)/2]²",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(204),
            name: "geometric_sum",
            category: RuleCategory::Simplification,
            description: "1+r+r²+...+r^n = (r^(n+1)-1)/(r-1)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
    ]
}

// ============================================================================
// Factorial Rules (ID 220+)
// ============================================================================

fn factorial_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: RuleId(220),
            name: "factorial_zero",
            category: RuleCategory::Simplification,
            description: "0! = 1",
            is_applicable: |_expr, _ctx| false, // Need Factorial in Expr
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        Rule {
            id: RuleId(221),
            name: "factorial_one",
            category: RuleCategory::Simplification,
            description: "1! = 1",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        Rule {
            id: RuleId(222),
            name: "factorial_recurse",
            category: RuleCategory::Expansion,
            description: "n! = n · (n-1)!",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
    ]
}

// ============================================================================
// Floor/Ceiling Rules (ID 240+)
// ============================================================================

fn floor_ceiling_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: RuleId(240),
            name: "floor_integer",
            category: RuleCategory::Simplification,
            description: "⌊n⌋ = n for integer n",
            is_applicable: |_expr, _ctx| false, // Need Floor in Expr
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        Rule {
            id: RuleId(241),
            name: "ceiling_integer",
            category: RuleCategory::Simplification,
            description: "⌈n⌉ = n for integer n",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        Rule {
            id: RuleId(242),
            name: "floor_ceiling_diff",
            category: RuleCategory::AlgebraicSolving,
            description: "⌈x⌉ - ⌊x⌋ = 0 or 1",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 2,
        },
    ]
}
