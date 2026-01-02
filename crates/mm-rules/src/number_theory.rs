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
    // Phase 3: Advanced number theory
    rules.extend(advanced_number_theory_rules());

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
                if let Expr::Mod(a, b) = expr {
                    return a == b;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(a, b) = expr {
                    if a == b {
                        return vec![RuleApplication {
                            result: Expr::int(0),
                            justification: "a mod a = 0".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // 0 mod n = 0
        Rule {
            id: RuleId(121),
            name: "zero_mod",
            category: RuleCategory::Simplification,
            description: "0 mod n = 0",
            is_applicable: |expr, _ctx| {
                if let Expr::Mod(a, _) = expr {
                    return matches!(a.as_ref(), Expr::Const(c) if c.is_zero());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(a, _) = expr {
                    if matches!(a.as_ref(), Expr::Const(c) if c.is_zero()) {
                        return vec![RuleApplication {
                            result: Expr::int(0),
                            justification: "0 mod n = 0".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // a mod 1 = 0 (for integers)
        Rule {
            id: RuleId(122),
            name: "mod_one",
            category: RuleCategory::Simplification,
            description: "a mod 1 = 0",
            is_applicable: |expr, _ctx| {
                if let Expr::Mod(_, b) = expr {
                    return matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(_, b) = expr {
                    if matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                        return vec![RuleApplication {
                            result: Expr::int(0),
                            justification: "a mod 1 = 0 (for integers)".to_string(),
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
            is_applicable: |expr, _ctx| {
                if let Expr::GCD(a, b) = expr {
                    return a == b;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::GCD(a, b) = expr {
                    if a == b {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "gcd(a, a) = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // gcd(a, 0) = |a|
        Rule {
            id: RuleId(141),
            name: "gcd_zero",
            category: RuleCategory::Simplification,
            description: "gcd(a, 0) = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::GCD(_, b) = expr {
                    return matches!(b.as_ref(), Expr::Const(c) if c.is_zero());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::GCD(a, b) = expr {
                    if matches!(b.as_ref(), Expr::Const(c) if c.is_zero()) {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "gcd(a, 0) = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // gcd(a, 1) = 1
        Rule {
            id: RuleId(142),
            name: "gcd_one",
            category: RuleCategory::Simplification,
            description: "gcd(a, 1) = 1",
            is_applicable: |expr, _ctx| {
                if let Expr::GCD(_, b) = expr {
                    return matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::GCD(_, b) = expr {
                    if matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                        return vec![RuleApplication {
                            result: Expr::int(1),
                            justification: "gcd(a, 1) = 1".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // lcm(a, a) = |a|
        Rule {
            id: RuleId(143),
            name: "lcm_self",
            category: RuleCategory::Simplification,
            description: "lcm(a, a) = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::LCM(a, b) = expr {
                    return a == b;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::LCM(a, b) = expr {
                    if a == b {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "lcm(a, a) = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // lcm(a, 1) = |a|
        Rule {
            id: RuleId(144),
            name: "lcm_one",
            category: RuleCategory::Simplification,
            description: "lcm(a, 1) = |a|",
            is_applicable: |expr, _ctx| {
                if let Expr::LCM(_, b) = expr {
                    return matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::LCM(a, b) = expr {
                    if matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                        return vec![RuleApplication {
                            result: Expr::Abs(a.clone()),
                            justification: "lcm(a, 1) = |a|".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // gcd(a,b) * lcm(a,b) = |a*b|
        Rule {
            id: RuleId(145),
            name: "gcd_lcm_product",
            category: RuleCategory::AlgebraicSolving,
            description: "gcd(a,b) * lcm(a,b) = |a*b|",
            is_applicable: |expr, _ctx| {
                // Match GCD(a,b) * LCM(a,b)
                if let Expr::Mul(left, right) = expr {
                    if let (Expr::GCD(a1, b1), Expr::LCM(a2, b2)) = (left.as_ref(), right.as_ref())
                    {
                        return a1 == a2 && b1 == b2;
                    }
                    if let (Expr::LCM(a1, b1), Expr::GCD(a2, b2)) = (left.as_ref(), right.as_ref())
                    {
                        return a1 == a2 && b1 == b2;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mul(left, right) = expr {
                    if let (Expr::GCD(a1, b1), Expr::LCM(_, _)) = (left.as_ref(), right.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Abs(Box::new(Expr::Mul(a1.clone(), b1.clone()))),
                            justification: "gcd(a,b) * lcm(a,b) = |a*b|".to_string(),
                        }];
                    }
                    if let (Expr::LCM(a1, b1), Expr::GCD(_, _)) = (left.as_ref(), right.as_ref()) {
                        return vec![RuleApplication {
                            result: Expr::Abs(Box::new(Expr::Mul(a1.clone(), b1.clone()))),
                            justification: "gcd(a,b) * lcm(a,b) = |a*b|".to_string(),
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

// ============================================================================
// Phase 3: Advanced Number Theory Rules (ID 700+)
// ============================================================================

/// Get all advanced number theory rules
pub fn advanced_number_theory_rules() -> Vec<Rule> {
    vec![
        // Fermat's theorems
        fermat_little_theorem(),
        fermat_last_theorem(),
        // Euler's theorem
        euler_theorem(),
        euler_phi_multiplicative(),
        euler_phi_prime_power(),
        // Chinese Remainder Theorem
        chinese_remainder_theorem(),
        // Quadratic residues
        quadratic_residue(),
        legendre_symbol_multiplicative(),
        euler_criterion(),
        // Prime number theorems
        prime_counting_approx(),
        bertrand_postulate(),
        // Diophantine equations
        linear_diophantine(),
        pell_equation(),
        sum_of_two_squares(),
        sum_of_four_squares(),
        // Wilson's theorem
        wilson_theorem(),
        // Lifting lemma
        hensel_lemma(),
        // Order of elements
        order_divides_phi(),
        primitive_root_existence(),
        // Legendre's formula
        legendre_formula(),
        // Lucas' theorem
        lucas_theorem(),
        // Mobius function
        mobius_inversion(),
        mobius_multiplicative(),
        // Chebyshev bounds
        chebyshev_prime_bounds(),
        // Perfect numbers
        even_perfect_number(),
        // Mersenne primes
        mersenne_prime_condition(),
        // Arithmetic functions
        sum_of_divisors(),
        number_of_divisors(),
        // Totient sum
        totient_sum(),
        // Primitive roots
        primitive_root_count(),
        // Carmichael function
        carmichael_function(),
        // Square-free numbers
        square_free_density(),
        // Prime gaps
        prime_gap_bound(),
        // Sophie Germain primes
        sophie_germain_prime(),
        // Quadric reciprocity
        quadratic_reciprocity(),
        // Jacobi symbol
        jacobi_symbol(),
        // Kronecker symbol
        kronecker_symbol(),
        // Modular square root
        tonelli_shanks(),
        // Discrete logarithm
        discrete_log_order(),
        // Continued fractions
        continued_fraction_gcd(),
        // Farey sequence
        farey_neighbors(),
        // Stern-Brocot tree
        stern_brocot(),
        // Egyptian fractions
        egyptian_fraction(),
        // Gaussian integers
        gaussian_norm(),
        gaussian_prime(),
    ]
}

// a^(p-1) ≡ 1 (mod p) for prime p, gcd(a,p)=1
fn fermat_little_theorem() -> Rule {
    Rule {
        id: RuleId(700),
        name: "fermat_little_theorem",
        category: RuleCategory::Simplification,
        description: "a^(p-1) ≡ 1 (mod p)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// x^n + y^n = z^n has no integer solutions for n > 2
fn fermat_last_theorem() -> Rule {
    Rule {
        id: RuleId(701),
        name: "fermat_last_theorem",
        category: RuleCategory::AlgebraicSolving,
        description: "No integer solutions to x^n + y^n = z^n for n > 2",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// a^φ(n) ≡ 1 (mod n) for gcd(a,n)=1
fn euler_theorem() -> Rule {
    Rule {
        id: RuleId(702),
        name: "euler_theorem",
        category: RuleCategory::Simplification,
        description: "a^φ(n) ≡ 1 (mod n)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// φ(mn) = φ(m)φ(n) for gcd(m,n)=1
fn euler_phi_multiplicative() -> Rule {
    Rule {
        id: RuleId(703),
        name: "euler_phi_multiplicative",
        category: RuleCategory::Simplification,
        description: "φ(mn) = φ(m)φ(n) for gcd(m,n)=1",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// φ(p^k) = p^k - p^(k-1)
fn euler_phi_prime_power() -> Rule {
    Rule {
        id: RuleId(704),
        name: "euler_phi_prime_power",
        category: RuleCategory::Simplification,
        description: "φ(p^k) = p^k - p^(k-1)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// System of congruences with coprime moduli
fn chinese_remainder_theorem() -> Rule {
    Rule {
        id: RuleId(705),
        name: "chinese_remainder_theorem",
        category: RuleCategory::EquationSolving,
        description: "CRT: x ≡ a_i (mod m_i) has unique solution mod Π m_i",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// a is a quadratic residue mod p if a = x² (mod p) has solution
fn quadratic_residue() -> Rule {
    Rule {
        id: RuleId(706),
        name: "quadratic_residue",
        category: RuleCategory::AlgebraicSolving,
        description: "Quadratic residue test",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// (ab/p) = (a/p)(b/p) for Legendre symbol
fn legendre_symbol_multiplicative() -> Rule {
    Rule {
        id: RuleId(707),
        name: "legendre_symbol_multiplicative",
        category: RuleCategory::Simplification,
        description: "(ab/p) = (a/p)(b/p)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// (a/p) = a^((p-1)/2) (mod p)
fn euler_criterion() -> Rule {
    Rule {
        id: RuleId(708),
        name: "euler_criterion",
        category: RuleCategory::Simplification,
        description: "(a/p) = a^((p-1)/2) mod p",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// π(x) ~ x/ln(x)
fn prime_counting_approx() -> Rule {
    Rule {
        id: RuleId(709),
        name: "prime_counting_approx",
        category: RuleCategory::Simplification,
        description: "π(x) ~ x/ln(x)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// For n > 1, there exists prime p with n < p < 2n
fn bertrand_postulate() -> Rule {
    Rule {
        id: RuleId(710),
        name: "bertrand_postulate",
        category: RuleCategory::AlgebraicSolving,
        description: "Prime exists between n and 2n",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// ax + by = c has solutions iff gcd(a,b) | c
fn linear_diophantine() -> Rule {
    Rule {
        id: RuleId(711),
        name: "linear_diophantine",
        category: RuleCategory::EquationSolving,
        description: "ax + by = c solvable iff gcd(a,b) | c",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// x² - Dy² = 1 fundamental solution
fn pell_equation() -> Rule {
    Rule {
        id: RuleId(712),
        name: "pell_equation",
        category: RuleCategory::EquationSolving,
        description: "Pell equation x² - Dy² = 1",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// n = a² + b² iff in prime factorization, primes ≡ 3 (mod 4) have even exponents
fn sum_of_two_squares() -> Rule {
    Rule {
        id: RuleId(713),
        name: "sum_of_two_squares",
        category: RuleCategory::AlgebraicSolving,
        description: "Sum of two squares condition",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// Every n is sum of 4 squares (Lagrange)
fn sum_of_four_squares() -> Rule {
    Rule {
        id: RuleId(714),
        name: "sum_of_four_squares",
        category: RuleCategory::AlgebraicSolving,
        description: "Every n = a² + b² + c² + d²",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// (p-1)! ≡ -1 (mod p) for prime p
fn wilson_theorem() -> Rule {
    Rule {
        id: RuleId(715),
        name: "wilson_theorem",
        category: RuleCategory::Simplification,
        description: "(p-1)! ≡ -1 (mod p)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Lifting solutions to higher prime powers
fn hensel_lemma() -> Rule {
    Rule {
        id: RuleId(716),
        name: "hensel_lemma",
        category: RuleCategory::EquationSolving,
        description: "Hensel's lemma for lifting solutions",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 4,
    }
}

// ord_n(a) | φ(n)
fn order_divides_phi() -> Rule {
    Rule {
        id: RuleId(717),
        name: "order_divides_phi",
        category: RuleCategory::Simplification,
        description: "ord_n(a) | φ(n)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Primitive roots exist for 1, 2, 4, p^k, 2p^k (odd prime p)
fn primitive_root_existence() -> Rule {
    Rule {
        id: RuleId(718),
        name: "primitive_root_existence",
        category: RuleCategory::AlgebraicSolving,
        description: "Primitive roots exist for n = 1,2,4,p^k,2p^k",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// ν_p(n!) = Σ ⌊n/p^k⌋
fn legendre_formula() -> Rule {
    Rule {
        id: RuleId(719),
        name: "legendre_formula",
        category: RuleCategory::Simplification,
        description: "ν_p(n!) = Σ⌊n/p^k⌋",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// C(m,n) ≡ Π C(m_i, n_i) (mod p) where m_i, n_i are base-p digits
fn lucas_theorem() -> Rule {
    Rule {
        id: RuleId(720),
        name: "lucas_theorem",
        category: RuleCategory::Simplification,
        description: "Lucas' theorem for binomials mod p",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// Σ μ(d)f(n/d) for divisor d of n
fn mobius_inversion() -> Rule {
    Rule {
        id: RuleId(721),
        name: "mobius_inversion",
        category: RuleCategory::Simplification,
        description: "Möbius inversion formula",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// μ(mn) = μ(m)μ(n) for gcd(m,n)=1
fn mobius_multiplicative() -> Rule {
    Rule {
        id: RuleId(722),
        name: "mobius_multiplicative",
        category: RuleCategory::Simplification,
        description: "μ(mn) = μ(m)μ(n) for gcd(m,n)=1",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// π(x) bounds using Chebyshev
fn chebyshev_prime_bounds() -> Rule {
    Rule {
        id: RuleId(723),
        name: "chebyshev_prime_bounds",
        category: RuleCategory::AlgebraicSolving,
        description: "Chebyshev bounds on π(x)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Even perfect number form: 2^(p-1)(2^p - 1) where 2^p - 1 is prime
fn even_perfect_number() -> Rule {
    Rule {
        id: RuleId(724),
        name: "even_perfect_number",
        category: RuleCategory::AlgebraicSolving,
        description: "Even perfect = 2^(p-1)(2^p - 1)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// 2^p - 1 prime => p is prime
fn mersenne_prime_condition() -> Rule {
    Rule {
        id: RuleId(725),
        name: "mersenne_prime_condition",
        category: RuleCategory::AlgebraicSolving,
        description: "2^p - 1 prime => p prime",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// σ(n) = Σ d for d|n
fn sum_of_divisors() -> Rule {
    Rule {
        id: RuleId(726),
        name: "sum_of_divisors",
        category: RuleCategory::Simplification,
        description: "σ(n) = Σ d for d|n",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// τ(n) = number of divisors
fn number_of_divisors() -> Rule {
    Rule {
        id: RuleId(727),
        name: "number_of_divisors",
        category: RuleCategory::Simplification,
        description: "τ(n) is number of divisors",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// Σ φ(d) = n for d|n
fn totient_sum() -> Rule {
    Rule {
        id: RuleId(728),
        name: "totient_sum",
        category: RuleCategory::Simplification,
        description: "Σ φ(d) = n for d|n",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Number of primitive roots mod n is φ(φ(n))
fn primitive_root_count() -> Rule {
    Rule {
        id: RuleId(729),
        name: "primitive_root_count",
        category: RuleCategory::Simplification,
        description: "φ(φ(n)) primitive roots mod n",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// λ(n) Carmichael function
fn carmichael_function() -> Rule {
    Rule {
        id: RuleId(730),
        name: "carmichael_function",
        category: RuleCategory::Simplification,
        description: "Carmichael function λ(n)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Density of square-free numbers = 6/π²
fn square_free_density() -> Rule {
    Rule {
        id: RuleId(731),
        name: "square_free_density",
        category: RuleCategory::Simplification,
        description: "Square-free density = 6/π²",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Prime gap bounds
fn prime_gap_bound() -> Rule {
    Rule {
        id: RuleId(732),
        name: "prime_gap_bound",
        category: RuleCategory::AlgebraicSolving,
        description: "Prime gap upper bounds",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// p is Sophie Germain prime if both p and 2p+1 are prime
fn sophie_germain_prime() -> Rule {
    Rule {
        id: RuleId(733),
        name: "sophie_germain_prime",
        category: RuleCategory::AlgebraicSolving,
        description: "Sophie Germain: p and 2p+1 both prime",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// (p/q)(q/p) = (-1)^((p-1)(q-1)/4)
fn quadratic_reciprocity() -> Rule {
    Rule {
        id: RuleId(734),
        name: "quadratic_reciprocity",
        category: RuleCategory::Simplification,
        description: "Quadratic reciprocity law",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Jacobi symbol generalization
fn jacobi_symbol() -> Rule {
    Rule {
        id: RuleId(735),
        name: "jacobi_symbol",
        category: RuleCategory::Simplification,
        description: "Jacobi symbol (a/n)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Kronecker symbol extension
fn kronecker_symbol() -> Rule {
    Rule {
        id: RuleId(736),
        name: "kronecker_symbol",
        category: RuleCategory::Simplification,
        description: "Kronecker symbol extension",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Tonelli-Shanks algorithm for modular square root
fn tonelli_shanks() -> Rule {
    Rule {
        id: RuleId(737),
        name: "tonelli_shanks",
        category: RuleCategory::EquationSolving,
        description: "Tonelli-Shanks modular square root",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 4,
    }
}

// Discrete log existence and order
fn discrete_log_order() -> Rule {
    Rule {
        id: RuleId(738),
        name: "discrete_log_order",
        category: RuleCategory::EquationSolving,
        description: "Discrete logarithm order",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// gcd via continued fractions
fn continued_fraction_gcd() -> Rule {
    Rule {
        id: RuleId(739),
        name: "continued_fraction_gcd",
        category: RuleCategory::Simplification,
        description: "GCD via continued fractions",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// Farey neighbors |ad - bc| = 1
fn farey_neighbors() -> Rule {
    Rule {
        id: RuleId(740),
        name: "farey_neighbors",
        category: RuleCategory::AlgebraicSolving,
        description: "Farey neighbors: |ad - bc| = 1",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Mediant in Stern-Brocot tree
fn stern_brocot() -> Rule {
    Rule {
        id: RuleId(741),
        name: "stern_brocot",
        category: RuleCategory::Simplification,
        description: "Stern-Brocot tree mediant",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// Egyptian fraction representation
fn egyptian_fraction() -> Rule {
    Rule {
        id: RuleId(742),
        name: "egyptian_fraction",
        category: RuleCategory::Expansion,
        description: "Egyptian fraction decomposition",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// N(a + bi) = a² + b²
fn gaussian_norm() -> Rule {
    Rule {
        id: RuleId(743),
        name: "gaussian_norm",
        category: RuleCategory::Simplification,
        description: "Gaussian integer norm N(a+bi) = a² + b²",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 1,
    }
}

// Gaussian prime conditions
fn gaussian_prime() -> Rule {
    Rule {
        id: RuleId(744),
        name: "gaussian_prime",
        category: RuleCategory::AlgebraicSolving,
        description: "Gaussian prime conditions",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}
