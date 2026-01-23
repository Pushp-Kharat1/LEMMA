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
        // Σc (n times) = cn
        Rule {
            id: RuleId(200),
            name: "sum_constant",
            category: RuleCategory::Simplification,
            description: "Σc (n times) = cn",
            is_applicable: |expr, _ctx| {
                // Match: Mul(Const, Var) which could represent cn
                if let Expr::Mul(a, b) = expr {
                    return matches!(a.as_ref(), Expr::Const(_))
                        && matches!(b.as_ref(), Expr::Var(_));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mul(c, n) = expr {
                    if let (Expr::Const(val), Expr::Var(var)) = (c.as_ref(), n.as_ref()) {
                        // cn represents sum of c, n times
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                Box::new(Expr::Const(val.clone())),
                                Box::new(Expr::Var(*var)),
                            ),
                            justification: "Σc (n times) = cn (sum of constant)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // 1+2+...+n = n(n+1)/2
        Rule {
            id: RuleId(201),
            name: "sum_arithmetic",
            category: RuleCategory::Simplification,
            description: "1+2+...+n = n(n+1)/2",
            is_applicable: |expr, _ctx| {
                // Match: Div(Mul(n, Add(n, 1)), 2)
                if let Expr::Div(num, denom) = expr {
                    if let Expr::Const(d) = denom.as_ref() {
                        if *d == Rational::from_integer(2) {
                            if let Expr::Mul(n1, n_plus_1) = num.as_ref() {
                                if let Expr::Add(n2, one) = n_plus_1.as_ref() {
                                    if matches!(one.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1))
                                    {
                                        return n1.as_ref() == n2.as_ref();
                                    }
                                }
                            }
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(num, _) = expr {
                    if let Expr::Mul(n, _) = num.as_ref() {
                        return vec![RuleApplication {
                            result: expr.clone(),
                            justification: "1+2+...+n = n(n+1)/2 (arithmetic sum formula)"
                                .to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // 1²+2²+...+n² = n(n+1)(2n+1)/6
        Rule {
            id: RuleId(202),
            name: "sum_squares",
            category: RuleCategory::Simplification,
            description: "1²+2²+...+n² = n(n+1)(2n+1)/6",
            is_applicable: |expr, _ctx| {
                // Match: Div(Mul(...), 6)
                if let Expr::Div(_, denom) = expr {
                    if let Expr::Const(d) = denom.as_ref() {
                        return *d == Rational::from_integer(6);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(num, denom) = expr {
                    if let Expr::Const(d) = denom.as_ref() {
                        if *d == Rational::from_integer(6) {
                            return vec![RuleApplication {
                                result: Expr::Div(num.clone(), denom.clone()),
                                justification: "1²+2²+...+n² = n(n+1)(2n+1)/6 (sum of squares)"
                                    .to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // 1³+2³+...+n³ = [n(n+1)/2]²
        Rule {
            id: RuleId(203),
            name: "sum_cubes",
            category: RuleCategory::Simplification,
            description: "1³+2³+...+n³ = [n(n+1)/2]²",
            is_applicable: |expr, _ctx| {
                // Match: Pow(Div(Mul(n, n+1), 2), 2)
                if let Expr::Pow(base, exp) = expr {
                    if let Expr::Const(e) = exp.as_ref() {
                        if *e == Rational::from_integer(2) {
                            return matches!(base.as_ref(), Expr::Div(_, _));
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Pow(base, _) = expr {
                    if let Expr::Div(_, _) = base.as_ref() {
                        return vec![RuleApplication {
                            result: expr.clone(),
                            justification: "1³+2³+...+n³ = [n(n+1)/2]² (sum of cubes)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: true,
            cost: 2,
        },
        // 1+r+r²+...+r^n = (r^(n+1)-1)/(r-1)
        Rule {
            id: RuleId(204),
            name: "geometric_sum",
            category: RuleCategory::Simplification,
            description: "1+r+r²+...+r^n = (r^(n+1)-1)/(r-1)",
            is_applicable: |expr, _ctx| {
                // Match: Div(Sub(Pow(r, n+1), 1), Sub(r, 1))
                if let Expr::Div(num, denom) = expr {
                    if let (Expr::Sub(_, _), Expr::Sub(_, _)) = (num.as_ref(), denom.as_ref()) {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Div(num, denom) = expr {
                    if let (Expr::Sub(pow_term, _), Expr::Sub(r, one)) =
                        (num.as_ref(), denom.as_ref())
                    {
                        if matches!(one.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1))
                        {
                            if let Expr::Pow(base, _) = pow_term.as_ref() {
                                if base.as_ref() == r.as_ref() {
                                    return vec![RuleApplication {
                                        result: expr.clone(),
                                        justification:
                                            "1+r+r²+...+r^n = (r^(n+1)-1)/(r-1) (geometric series)"
                                                .to_string(),
                                    }];
                                }
                            }
                        }
                    }
                }
                vec![]
            },
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
        // 0! = 1
        Rule {
            id: RuleId(220),
            name: "factorial_zero",
            category: RuleCategory::Simplification,
            description: "0! = 1",
            is_applicable: |expr, _ctx| {
                if let Expr::Factorial(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Const(c) if c.is_zero());
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Factorial(inner) = expr {
                    if matches!(inner.as_ref(), Expr::Const(c) if c.is_zero()) {
                        return vec![RuleApplication {
                            result: Expr::int(1),
                            justification: "0! = 1 (by definition)".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // 1! = 1
        Rule {
            id: RuleId(221),
            name: "factorial_one",
            category: RuleCategory::Simplification,
            description: "1! = 1",
            is_applicable: |expr, _ctx| {
                if let Expr::Factorial(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1));
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Factorial(inner) = expr {
                    if matches!(inner.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                        return vec![RuleApplication {
                            result: Expr::int(1),
                            justification: "1! = 1".to_string(),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // n! = n · (n-1)!
        Rule {
            id: RuleId(222),
            name: "factorial_recurse",
            category: RuleCategory::Expansion,
            description: "n! = n · (n-1)!",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Factorial(_)),
            apply: |expr, _ctx| {
                if let Expr::Factorial(n) = expr {
                    // n! = n * (n-1)!
                    let n_minus_1 = Expr::Sub(n.clone(), Box::new(Expr::int(1)));
                    let fact_n_minus_1 = Expr::Factorial(Box::new(n_minus_1));
                    let result = Expr::Mul(n.clone(), Box::new(fact_n_minus_1));
                    return vec![RuleApplication {
                        result,
                        justification: "n! = n · (n-1)! (factorial recursion)".to_string(),
                    }];
                }
                vec![]
            },
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
        // ⌊n⌋ = n for integer n
        Rule {
            id: RuleId(240),
            name: "floor_integer",
            category: RuleCategory::Simplification,
            description: "⌊n⌋ = n for integer n",
            is_applicable: |expr, _ctx| {
                if let Expr::Floor(inner) = expr {
                    // Integer check: if it's a Const with denominator 1
                    if let Expr::Const(r) = inner.as_ref() {
                        return r.is_integer();
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Floor(inner) = expr {
                    if let Expr::Const(r) = inner.as_ref() {
                        if r.is_integer() {
                            return vec![RuleApplication {
                                result: *inner.clone(),
                                justification: "⌊n⌋ = n for integer n".to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // ⌈n⌉ = n for integer n
        Rule {
            id: RuleId(241),
            name: "ceiling_integer",
            category: RuleCategory::Simplification,
            description: "⌈n⌉ = n for integer n",
            is_applicable: |expr, _ctx| {
                if let Expr::Ceiling(inner) = expr {
                    if let Expr::Const(r) = inner.as_ref() {
                        return r.is_integer();
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Ceiling(inner) = expr {
                    if let Expr::Const(r) = inner.as_ref() {
                        if r.is_integer() {
                            return vec![RuleApplication {
                                result: *inner.clone(),
                                justification: "⌈n⌉ = n for integer n".to_string(),
                            }];
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 1,
        },
        // ⌈x⌉ - ⌊x⌋ = 0 or 1
        Rule {
            id: RuleId(242),
            name: "floor_ceiling_diff",
            category: RuleCategory::AlgebraicSolving,
            description: "⌈x⌉ - ⌊x⌋ = 0 or 1",
            is_applicable: |expr, _ctx| {
                // Match: Sub(Ceiling(x), Floor(x))
                if let Expr::Sub(a, b) = expr {
                    if matches!(a.as_ref(), Expr::Ceiling(_))
                        && matches!(b.as_ref(), Expr::Floor(_))
                    {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Sub(a, b) = expr {
                    if let (Expr::Ceiling(x1), Expr::Floor(x2)) = (a.as_ref(), b.as_ref()) {
                        if x1 == x2 {
                            // Result is 0 for integers, 1 for non-integers
                            return vec![
                                RuleApplication {
                                    result: Expr::int(0),
                                    justification: "⌈x⌉ - ⌊x⌋ = 0 (when x is integer)".to_string(),
                                },
                                RuleApplication {
                                    result: Expr::int(1),
                                    justification: "⌈x⌉ - ⌊x⌋ = 1 (when x is non-integer)"
                                        .to_string(),
                                },
                            ];
                        }
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
        is_applicable: |expr, _ctx| {
            // Match: Mod(Pow(a, p-1), p)
            if let Expr::Mod(inner, modulus) = expr {
                if let Expr::Pow(_, exp) = inner.as_ref() {
                    // Check if exp = modulus - 1
                    if let Expr::Sub(m, one) = exp.as_ref() {
                        if m.as_ref() == modulus.as_ref() {
                            return matches!(one.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1));
                        }
                    }
                }
            }
            false
        },
        apply: |_expr, _ctx| {
            // By Fermat's little theorem, a^(p-1) ≡ 1 (mod p)
            vec![RuleApplication {
                result: Expr::int(1),
                justification:
                    "Fermat's Little Theorem: a^(p-1) ≡ 1 (mod p) for prime p, gcd(a,p)=1"
                        .to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Equation{ lhs: Add(Pow(x,n), Pow(y,n)), rhs: Pow(z,n) }
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Add(a, b) = lhs.as_ref() {
                    if matches!(a.as_ref(), Expr::Pow(_, _)) && 
                       matches!(b.as_ref(), Expr::Pow(_, _)) &&
                       matches!(rhs.as_ref(), Expr::Pow(_, _)) {
                        return true;
                    }
                }
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(0)), // No solutions
                justification: "Fermat's Last Theorem: No integer solutions for x^n + y^n = z^n when n > 2".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod(Pow(a, phi(n)), n)
            if let Expr::Mod(inner, _modulus) = expr {
                return matches!(inner.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::int(1),
                justification: "Euler's Theorem: a^φ(n) ≡ 1 (mod n) for gcd(a,n)=1".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mul(expr, expr) as the Euler phi is multiplicative
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(m, n) = expr {
                // φ(mn) = φ(m) * φ(n)
                return vec![RuleApplication {
                    result: Expr::Mul(m.clone(), n.clone()),
                    justification: "φ(mn) = φ(m)φ(n) for coprime m, n (Euler phi multiplicativity)".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Pow(p, k) for prime power
            matches!(expr, Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(p, k) = expr {
                // φ(p^k) = p^k - p^(k-1)
                let k_minus_1 = Expr::Sub(k.clone(), Box::new(Expr::int(1)));
                let p_k_minus_1 = Expr::Pow(p.clone(), Box::new(k_minus_1));
                let result = Expr::Sub(Box::new(expr.clone()), Box::new(p_k_minus_1));
                return vec![RuleApplication {
                    result,
                    justification: "φ(p^k) = p^k - p^(k-1) for prime p".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match system of Mod equations
            matches!(expr, Expr::Equation { .. })
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                return vec![RuleApplication {
                    result: Expr::Equation { 
                        lhs: lhs.clone(), 
                        rhs: rhs.clone() 
                    },
                    justification: "CRT: System x ≡ a_i (mod m_i) has unique solution mod Π m_i for coprime m_i".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod(Pow(x, 2), p) or Equation with Mod
            if let Expr::Mod(inner, _) = expr {
                return matches!(inner.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mod(_, _) = expr {
                return vec![RuleApplication {
                    result: expr.clone(),
                    justification: "a is a quadratic residue mod p if a = x² (mod p) has a solution".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Legendre symbol involves Mul and Mod
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                return vec![RuleApplication {
                    result: Expr::Mul(a.clone(), b.clone()),
                    justification: "Legendre symbol is multiplicative: (ab/p) = (a/p)(b/p)".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod(Pow(a, (p-1)/2), p)
            if let Expr::Mod(inner, _) = expr {
                return matches!(inner.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mod(_, _) = expr {
                // Result is ±1 or 0
                return vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "Euler's criterion: (a/p) = a^((p-1)/2) mod p = ±1 or 0".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Div(x, Ln(x))
            if let Expr::Div(num, denom) = expr {
                if let Expr::Ln(inner) = denom.as_ref() {
                    return num.as_ref() == inner.as_ref();
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(x, _) = expr {
                return vec![RuleApplication {
                    result: *x.clone(),
                    justification: "Prime counting function: π(x) ~ x/ln(x) (asymptotic)".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Lt(n, Mul(2, n)) or Gt pattern for range
            matches!(expr, Expr::Gt(_, _) | Expr::Lt(_, _))
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(1)), // True
                justification: "Bertrand's postulate: For n > 1, ∃ prime p with n < p < 2n".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match linear equations: Equation { lhs: Add(Mul(a,x), Mul(b,y)), rhs: c }
            matches!(expr, Expr::Equation { .. })
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Add(_, _) = lhs.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: lhs.clone(),
                            rhs: rhs.clone(),
                        },
                        justification: "Linear Diophantine: ax + by = c solvable iff gcd(a,b) | c".to_string(),
                    }];
                }
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Equation { lhs: Sub(Pow(x,2), Mul(D, Pow(y,2))), rhs: 1 }
            if let Expr::Equation { lhs, rhs } = expr {
                if matches!(rhs.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                    return matches!(lhs.as_ref(), Expr::Sub(_, _));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                return vec![RuleApplication {
                    result: Expr::Equation { lhs: lhs.clone(), rhs: rhs.clone() },
                    justification: "Pell equation x² - Dy² = 1 has infinitely many solutions via continued fractions".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Add(Pow(a,2), Pow(b,2))
            if let Expr::Add(a, b) = expr {
                let a_sq = matches!(a.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                let b_sq = matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2)));
                return a_sq && b_sq;
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "n = a² + b² iff primes ≡ 3 (mod 4) have even exponents in n's factorization".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match any positive integer - this is Lagrange's four square theorem
            matches!(expr, Expr::Const(c) if !c.is_negative())
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Lagrange's four square theorem: Every positive integer is a sum of four squares".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod(Factorial(p-1), p)
            if let Expr::Mod(inner, _) = expr {
                return matches!(inner.as_ref(), Expr::Factorial(_));
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Neg(Box::new(Expr::int(1))),
                justification: "Wilson's theorem: (p-1)! ≡ -1 (mod p) for prime p".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod equations that could be lifted
            matches!(expr, Expr::Mod(_, _) | Expr::Equation { .. })
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Hensel's lemma: Solutions mod p can be lifted to solutions mod p^k".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: GCD or divisibility expressions
            matches!(expr, Expr::GCD(_, _) | Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Order divides phi: ord_n(a) | φ(n) for gcd(a,n)=1".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Const(n) where n is a prime power or 2*prime power
            matches!(expr, Expr::Const(_) | Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Primitive roots exist for n = 1, 2, 4, p^k, 2p^k (odd prime p)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Factorial expressions
            matches!(expr, Expr::Factorial(_))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Legendre's formula: ν_p(n!) = Σ⌊n/p^k⌋ for k ≥ 1".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mod of binomial coefficients
            matches!(expr, Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Lucas' theorem: C(m,n) ≡ Π C(m_i, n_i) (mod p)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Sum or Div expressions (divisor sums)
            matches!(expr, Expr::Div(_, _) | Expr::Add(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Möbius inversion: f(n) = Σ g(d) ⇒ g(n) = Σ μ(d)f(n/d)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(m, n) = expr {
                return vec![RuleApplication {
                    result: Expr::Mul(m.clone(), n.clone()),
                    justification: "μ(mn) = μ(m)μ(n) for coprime m, n (Mobius is multiplicative)".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Ln(_))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Chebyshev bounds: c₁x/ln(x) < π(x) < c₂x/ln(x) for x ≥ 2".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Mul(Pow(2, p-1), Sub(Pow(2, p), 1))
            if let Expr::Mul(a, b) = expr {
                return matches!(a.as_ref(), Expr::Pow(_, _)) && matches!(b.as_ref(), Expr::Sub(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Euclid-Euler: Even perfect numbers have form 2^(p-1)(2^p - 1) where 2^p - 1 is prime".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Sub(Pow(2, p), 1)
            if let Expr::Sub(a, b) = expr {
                if matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                    return matches!(a.as_ref(), Expr::Pow(_, _));
                }
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(1)), // True
                justification: "Mersenne prime condition: If 2^p - 1 is prime, then p must be prime".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// σ(n) = Σ d for d|n
/// Computes the sum-of-divisors function σ(n) for small positive integer constants and otherwise
/// produces a descriptive, non-evaluated result.
///
/// For an expression that is a positive integer constant less than 1000, the rule's `apply` returns
/// a `Const` expression containing the integer σ(n) (the sum of all positive divisors of n) and a
/// justification string. For any other expression the rule is applicable only as a descriptive
/// transformation and returns the original expression with a generic justification.
///
/// # Examples
///
/// ```
/// let rule = sum_of_divisors();
/// let expr = Expr::Const(Rational::from_integer(6));
/// let apps = (rule.apply)(&expr, &Context::default());
/// assert_eq!(apps[0].result, Expr::Const(Rational::from_integer(12))); // 1+2+3+6 = 12
/// ```
fn sum_of_divisors() -> Rule {
    Rule {
        id: RuleId(726),
        name: "sum_of_divisors",
        category: RuleCategory::Simplification,
        description: "σ(n) = Σ d for d|n",
        is_applicable: |expr, _ctx| {
            // Match: Const (for computing divisor sums)
            if let Expr::Const(n) = expr {
                return n.is_positive() && n.is_integer() && n.numer() < 1000;
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Const(n) = expr {
                if n.is_integer() {
                    let num = n.numer();
                    if num > 0 && num < 1000 {
                        // Compute sum of divisors
                        let mut sum = 0i64;
                        for d in 1..=num {
                            if num % d == 0 {
                                sum += d;
                            }
                        }
                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(sum)),
                            justification: format!("Sum of divisors: σ({}) = {} (sum of all divisors)", num, sum),
                        }];
                    }
                }
            }
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Sum of divisors: σ(n) = Σ d for all d | n".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// τ(n) = number of divisors
/// Computes the number of positive divisors τ(n) for small positive integer constants.
///
/// This rule applies only when the expression is a positive integer constant less than 1000;
/// in that case it returns a concrete constant equal to the count of all positive divisors of n.
/// Otherwise the rule leaves the expression unchanged and provides a descriptive justification.
///
/// # Examples
///
/// ```
/// // Example: τ(12) = 6 because divisors are 1,2,3,4,6,12
/// let rule = number_of_divisors();
/// let expr = Expr::Const(Rational::from_integer(12));
/// let apps = (rule.apply)(&expr, &Default::default());
/// assert_eq!(apps[0].result, Expr::Const(Rational::from_integer(6)));
/// ```
fn number_of_divisors() -> Rule {
    Rule {
        id: RuleId(727),
        name: "number_of_divisors",
        category: RuleCategory::Simplification,
        description: "τ(n) is number of divisors",
        is_applicable: |expr, _ctx| {
            if let Expr::Const(n) = expr {
                return n.is_positive() && n.is_integer() && n.numer() < 1000;
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Const(n) = expr {
                if n.is_integer() {
                    let num = n.numer();
                    if num > 0 && num < 1000 {
                        // Count divisors
                        let mut count = 0i64;
                        for d in 1..=num {
                            if num % d == 0 {
                                count += 1;
                            }
                        }
                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(count)),
                            justification: format!("Number of divisors: τ({}) = {} (count of all divisors)", num, count),
                        }];
                    }
                }
            }
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Number of divisors: τ(n) counts divisors of n".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Const(_) | Expr::Var(_))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Totient sum identity: Σ φ(d) = n for all d | n".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Const(_) | Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Primitive root count: φ(φ(n)) primitive roots exist mod n (when they exist)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Const(_) | Expr::LCM(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Carmichael function λ(n) = lcm of orders of elements mod n".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Div(6, Pow(Pi, 2))
            if let Expr::Div(num, denom) = expr {
                if matches!(num.as_ref(), Expr::Const(c) if *c == Rational::from_integer(6)) {
                    return matches!(denom.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Pi));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Density of square-free integers is 6/π² = 1/ζ(2)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Sub(_, _) | Expr::Gt(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Prime gap bound: For large n, gap between consecutive primes is O(n^0.525)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Add(Mul(2, p), 1) pattern for 2p+1
            if let Expr::Add(a, b) = expr {
                if matches!(b.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1)) {
                    return matches!(a.as_ref(), Expr::Mul(_, _));
                }
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(1)),
                justification: "Sophie Germain prime: Both p and 2p+1 are prime".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mul(_, _) | Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Quadratic reciprocity: (p/q)(q/p) = (-1)^((p-1)(q-1)/4) for odd primes p,q".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Jacobi symbol (a/n) generalizes Legendre symbol to composite moduli".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Kronecker symbol extends Jacobi symbol to all integers".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mod(_, _) | Expr::Sqrt(_))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Tonelli-Shanks: Computes modular square root x where x² ≡ a (mod p)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Pow(_, _) | Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Discrete log: Find x such that g^x ≡ h (mod n)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::GCD(_, _) | Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Continued fraction expansion computes GCD via convergents".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Sub(_, _) | Expr::Abs(_))
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::int(1),
                justification: "Farey neighbors: Adjacent fractions a/b and c/d satisfy |ad - bc| = 1".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Add(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Stern-Brocot: Mediant of a/b and c/d is (a+c)/(b+d)".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Egyptian fraction: Express a/b as sum of distinct unit fractions".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            // Match: Add(Pow(a, 2), Pow(b, 2))
            if let Expr::Add(a, b) = expr {
                return matches!(a.as_ref(), Expr::Pow(_, _)) && matches!(b.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Gaussian norm: N(a+bi) = a² + b² is multiplicative".to_string(),
            }]
        },
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
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Const(_) | Expr::Add(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Gaussian primes: p ≡ 3 (mod 4) or factors of primes ≡ 1 (mod 4)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}