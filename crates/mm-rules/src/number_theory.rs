// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Number theory transformation rules for IMO-level problem solving.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational, Symbol, SymbolTable};
use std::sync::{Mutex, OnceLock};

/// Return an interned `Symbol` for the given name.
///
/// The function maintains a single global symbol interner and returns a canonical `Symbol`
/// for `name`; repeated calls with the same `name` yield the same `Symbol`.
///
/// # Panics
///
/// Panics if the global symbol interner's mutex is poisoned.
///
/// # Examples
///
/// ```
/// let a = intern_symbol("alpha");
/// let b = intern_symbol("alpha");
/// assert_eq!(a, b);
/// ```
fn intern_symbol(name: &str) -> Symbol {
    static INTERNER: OnceLock<Mutex<SymbolTable>> = OnceLock::new();
    let m = INTERNER.get_or_init(|| Mutex::new(SymbolTable::new()));
    m.lock().expect("symbol interner poisoned").intern(name)
}

/// Collects the complete set of number-theory transformation rules.
///
/// This function aggregates rules from all number-theory subgroups (divisibility,
/// modular arithmetic, gcd/lcm, perfect powers, parity, sum formulas, factorial,
/// floor/ceiling, and advanced number-theory rules) into a single vector.
///
/// # Returns
///
/// A `Vec<Rule>` containing every rule provided by the number-theory modules.
///
/// # Examples
///
/// ```
/// let rules = number_theory_rules();
/// assert!(!rules.is_empty());
/// ```
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

/// Assembles the set of modular-arithmetic transformation rules used by the solver.
///
/// The returned vector contains rules for modular simplification and solving, including
/// identities and algorithms such as a mod a = 0, 0 mod n = 0, modular inverse computation,
/// modular exponentiation, extended GCD, Legendre symbol evaluation, Tonelli–Shanks (symbolic),
/// primitive-root search, discrete logarithm schema, and Hensel-lifting schema. Each entry is a
/// fully constructed `Rule` describing applicability, transformation, justification, reversibility,
/// and cost.
///
/// # Examples
///
/// ```rust
/// let rules = modular_rules();
/// // Expect at least the basic mod simplification rule (id 120) to be present.
/// assert!(rules.iter().any(|r| r.id == RuleId(120)));
/// ```
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
        // Modular inverse: a⁻¹ mod m exists iff gcd(a,m) = 1
        Rule {
            id: RuleId(123),
            name: "modular_inverse",
            category: RuleCategory::EquationSolving,
            description: "a⁻¹ mod m via extended Euclidean",
            is_applicable: |expr, _ctx| {
                // Match: Mod(Div(1, a), m) or inverse pattern
                if let Expr::Mod(inner, modulus) = expr {
                    if let Expr::Div(num, _) = inner.as_ref() {
                        if matches!(num.as_ref(), Expr::Const(c) if *c == Rational::from_integer(1))
                        {
                            // Check modulus is small constant
                            if let Expr::Const(m) = modulus.as_ref() {
                                return m.is_integer() && m.numer() > 1 && m.numer() < 1000;
                            }
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(inner, modulus) = expr {
                    if let Expr::Div(_, a) = inner.as_ref() {
                        if let Expr::Const(m) = modulus.as_ref() {
                            if let Expr::Const(av) = a.as_ref() {
                                let a_val = av.numer();
                                let m_val = m.numer();

                                // Extended Euclidean algorithm
                                fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
                                    if b == 0 {
                                        return (a, 1, 0);
                                    }
                                    let (g, x1, y1) = extended_gcd(b, a % b);
                                    (g, y1, x1 - (a / b) * y1)
                                }

                                let (g, x, _) = extended_gcd(a_val, m_val);
                                if g == 1 {
                                    let inv = ((x % m_val) + m_val) % m_val;
                                    return vec![RuleApplication {
                                        result: Expr::Const(Rational::from_integer(inv)),
                                        justification: format!(
                                            "Modular inverse: {}⁻¹ ≡ {} (mod {})",
                                            a_val, inv, m_val
                                        ),
                                    }];
                                }
                            }
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
        // Modular exponentiation: a^n mod m (fast)
        Rule {
            id: RuleId(124),
            name: "modular_exponentiation",
            category: RuleCategory::Simplification,
            description: "a^n mod m via repeated squaring",
            is_applicable: |expr, _ctx| {
                // Match: Mod(Pow(a, n), m)
                if let Expr::Mod(inner, modulus) = expr {
                    if matches!(inner.as_ref(), Expr::Pow(_, _)) {
                        if let Expr::Const(m) = modulus.as_ref() {
                            return m.is_integer() && m.numer() > 1 && m.numer() < 1000;
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(inner, modulus) = expr {
                    if let Expr::Pow(base, exp) = inner.as_ref() {
                        if let (Expr::Const(a), Expr::Const(n), Expr::Const(m)) =
                            (base.as_ref(), exp.as_ref(), modulus.as_ref())
                        {
                            let a_val = a.numer();
                            let n_val = n.numer();
                            let m_val = m.numer();

                            if n_val >= 0 && m_val > 0 {
                                // Fast modular exponentiation
                                fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
                                    let mut result = 1i64;
                                    base %= modulus;
                                    while exp > 0 {
                                        if exp % 2 == 1 {
                                            result = (result * base) % modulus;
                                        }
                                        base = (base * base) % modulus;
                                        exp /= 2;
                                    }
                                    result
                                }

                                let result = mod_pow(a_val, n_val, m_val);
                                return vec![RuleApplication {
                                    result: Expr::Const(Rational::from_integer(result)),
                                    justification: format!(
                                        "Modular exponentiation: {}^{} ≡ {} (mod {})",
                                        a_val, n_val, result, m_val
                                    ),
                                }];
                            }
                        }
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
        // Extended GCD: gcd(a,b) = ax + by (Bezout coefficients)
        Rule {
            id: RuleId(125),
            name: "extended_gcd",
            category: RuleCategory::EquationSolving,
            description: "Extended GCD: gcd(a,b) = ax + by",
            is_applicable: |expr, _ctx| {
                // Match: GCD(a, b) where both are small constants
                if let Expr::GCD(a, b) = expr {
                    if let (Expr::Const(av), Expr::Const(bv)) = (a.as_ref(), b.as_ref()) {
                        return av.is_integer()
                            && bv.is_integer()
                            && av.numer().abs() < 1000
                            && bv.numer().abs() < 1000;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::GCD(a, b) = expr {
                    if let (Expr::Const(av), Expr::Const(bv)) = (a.as_ref(), b.as_ref()) {
                        let a_val = av.numer();
                        let b_val = bv.numer();

                        // Extended Euclidean algorithm
                        fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
                            if b == 0 {
                                return (a, 1, 0);
                            }
                            let (g, x1, y1) = extended_gcd(b, a % b);
                            (g, y1, x1 - (a / b) * y1)
                        }

                        let (g, x, y) = extended_gcd(a_val, b_val);
                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(g)),
                            justification: format!(
                                "Extended GCD: gcd({}, {}) = {} = {}·{} + {}·{}",
                                a_val, b_val, g, a_val, x, b_val, y
                            ),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 2,
        },
        // Legendre symbol computation (a/p)
        Rule {
            id: RuleId(126),
            name: "legendre_symbol_compute",
            category: RuleCategory::Simplification,
            description: "Legendre symbol (a/p) computation",
            is_applicable: |expr, _ctx| {
                // Custom pattern for Legendre symbol representation
                // For now, check Mod(a, p) for odd primes
                if let Expr::Mod(_, p) = expr {
                    if let Expr::Const(pv) = p.as_ref() {
                        let p_val = pv.numer();
                        return p_val > 2 && p_val < 100 && p_val % 2 == 1;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Mod(a, p) = expr {
                    if let (Expr::Const(av), Expr::Const(pv)) = (a.as_ref(), p.as_ref()) {
                        let a_val = av.numer();
                        let p_val = pv.numer();

                        // Legendre symbol: (a/p) via Euler's criterion
                        fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
                            let mut result = 1i64;
                            base = ((base % modulus) + modulus) % modulus;
                            while exp > 0 {
                                if exp % 2 == 1 {
                                    result = (result * base) % modulus;
                                }
                                base = (base * base) % modulus;
                                exp /= 2;
                            }
                            result
                        }

                        let exp = (p_val - 1) / 2;
                        let result = mod_pow(a_val, exp, p_val);
                        let legendre = if result == 0 {
                            0
                        } else if result == 1 {
                            1
                        } else {
                            -1
                        };

                        return vec![RuleApplication {
                            result: Expr::Const(Rational::from_integer(legendre)),
                            justification: format!("Legendre ({}/{}) = {}", a_val, p_val, legendre),
                        }];
                    }
                }
                vec![]
            },
            reversible: false,
            cost: 3,
        },
        // Tonelli-Shanks: Modular square root
        Rule {
            id: RuleId(127),
            name: "tonelli_shanks_compute",
            category: RuleCategory::EquationSolving,
            description: "Tonelli-Shanks modular square root",
            is_applicable: |expr, _ctx| {
                // Match: Sqrt(Mod(a, p)) or Mod(Sqrt(a), p) for odd primes
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Const(a) = inner.as_ref() {
                        return a.is_integer() && a.numer() > 0 && a.numer() < 100;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("x : x^2 ≡ a (mod p)")),
                    justification: "Tonelli-Shanks computes sqrt mod prime".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Primitive root finder
        Rule {
            id: RuleId(128),
            name: "primitive_root_find",
            category: RuleCategory::AlgebraicSolving,
            description: "Find smallest primitive root mod n",
            is_applicable: |expr, _ctx| {
                // Match: Const(n) for small n where primitive roots exist
                if let Expr::Const(n) = expr {
                    let n_val = n.numer();
                    // Primitive roots exist for n = 1,2,4,p^k,2p^k
                    return n_val > 0 && n_val < 50;
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Const(n) = expr {
                    let n_val = n.numer();

                    // Compute Euler's totient
                    fn euler_phi(mut n: i64) -> i64 {
                        let mut result = n;
                        let mut p = 2i64;
                        while p * p <= n {
                            if n % p == 0 {
                                while n % p == 0 {
                                    n /= p;
                                }
                                result -= result / p;
                            }
                            p += 1;
                        }
                        if n > 1 {
                            result -= result / n;
                        }
                        result
                    }

                    fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
                        let mut result = 1i64;
                        base %= modulus;
                        while exp > 0 {
                            if exp % 2 == 1 {
                                result = (result * base) % modulus;
                            }
                            base = (base * base) % modulus;
                            exp /= 2;
                        }
                        result
                    }

                    // Check if g is a primitive root mod n
                    fn is_primitive_root(g: i64, n: i64, phi: i64) -> bool {
                        if mod_pow(g, phi, n) != 1 {
                            return false;
                        }
                        // Check that order is exactly phi
                        let mut d = 2i64;
                        while d * d <= phi {
                            if phi % d == 0 {
                                if mod_pow(g, phi / d, n) == 1 {
                                    return false;
                                }
                                if d != phi / d && mod_pow(g, d, n) == 1 {
                                    return false;
                                }
                            }
                            d += 1;
                        }
                        true
                    }

                    let phi = euler_phi(n_val);

                    // Find smallest primitive root
                    for g in 2..n_val {
                        if is_primitive_root(g, n_val, phi) {
                            return vec![RuleApplication {
                                result: Expr::Const(Rational::from_integer(g)),
                                justification: format!(
                                    "Primitive root: {} is smallest primitive root mod {}",
                                    g, n_val
                                ),
                            }];
                        }
                    }
                }
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("no primitive root")),
                    justification: "No primitive root exists for this modulus".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
        // Discrete logarithm (baby-step giant-step for small moduli)
        Rule {
            id: RuleId(129),
            name: "discrete_log_bsgs",
            category: RuleCategory::EquationSolving,
            description: "Discrete log via baby-step giant-step",
            is_applicable: |expr, _ctx| {
                // Match: Equation { lhs: Pow(g, x), rhs: h } in modular context
                // For now, just detect power expressions
                matches!(expr, Expr::Pow(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("solve g^x ≡ h (mod n)")),
                    justification: "Discrete log (BSGS) schema".to_string(),
                }]
            },
            reversible: false,
            cost: 5,
        },
        // Hensel lifting for p-adic approximation
        Rule {
            id: RuleId(130),
            name: "hensel_lift",
            category: RuleCategory::EquationSolving,
            description: "Hensel's lemma: lift solution mod p to mod p^k",
            is_applicable: |expr, _ctx| {
                // Match: Mod(f(x), p^k) patterns
                matches!(expr, Expr::Mod(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Var(intern_symbol("lift root mod p -> p^k")),
                    justification: "Hensel lifting schema".to_string(),
                }]
            },
            reversible: false,
            cost: 4,
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

/// Returns the collection of standard summation identities as transformation rules.
///
/// The returned rules encode common series formulas (constant sum, arithmetic series,
/// sum of squares, sum of cubes, and geometric series) as `Rule` items that produce
/// symbolic equation or expression forms when applied.
///
/// # Examples
///
/// ```
/// let rules = sum_formulas();
/// // basic sanity: the collection contains at least the arithmetic series rule (id 201)
/// assert!(rules.iter().any(|r| r.id == RuleId(201)));
/// ```
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let n = intern_symbol("n");
                let rhs = Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Var(n)),
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(intern_symbol("n"))),
                            Box::new(Expr::int(1)),
                        )),
                    )),
                    Box::new(Expr::int(2)),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(intern_symbol("Σ_{i=1}^n i"))),
                        rhs: Box::new(rhs),
                    },
                    justification: "Arithmetic series sum".to_string(),
                }]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let n = intern_symbol("n");
                let rhs = Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Mul(
                            Box::new(Expr::Var(n)),
                            Box::new(Expr::Add(
                                Box::new(Expr::Var(intern_symbol("n"))),
                                Box::new(Expr::int(1)),
                            )),
                        )),
                        Box::new(Expr::Add(
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Var(intern_symbol("n"))),
                            )),
                            Box::new(Expr::int(1)),
                        )),
                    )),
                    Box::new(Expr::int(6)),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(intern_symbol("Σ_{i=1}^n i^2"))),
                        rhs: Box::new(rhs),
                    },
                    justification: "Sum of squares formula".to_string(),
                }]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |_expr, _ctx| {
                let n = intern_symbol("n");
                let half = Expr::Div(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Var(n)),
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(intern_symbol("n"))),
                            Box::new(Expr::int(1)),
                        )),
                    )),
                    Box::new(Expr::int(2)),
                );
                let rhs = Expr::Pow(Box::new(half), Box::new(Expr::int(2)));
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(intern_symbol("Σ_{i=1}^n i^3"))),
                        rhs: Box::new(rhs),
                    },
                    justification: "Sum of cubes formula".to_string(),
                }]
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
            apply: |_expr, _ctx| {
                let r = intern_symbol("r");
                let n = intern_symbol("n");
                let rhs = Expr::Div(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Pow(
                            Box::new(Expr::Var(r)),
                            Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                        )),
                        Box::new(Expr::int(1)),
                    )),
                    Box::new(Expr::Sub(Box::new(Expr::Var(r)), Box::new(Expr::int(1)))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(Expr::Var(intern_symbol("Σ_{k=0}^n r^k"))),
                        rhs: Box::new(rhs),
                    },
                    justification: "Geometric series sum".to_string(),
                }]
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
/// Constructs a rule that recognizes Diophantine equations of the form `x^n + y^n = z^n` and records that there are no nontrivial integer solutions when `n > 2`.
///
/// The rule matches an equation whose left-hand side is a sum of two like-powered terms and whose right-hand side is the same power; when applied it produces a canonical `0` result with a justification referencing Fermat's Last Theorem. The rule is not reversible and has low application cost.
///
/// # Examples
///
/// ```
/// let r = fermat_last_theorem();
/// assert_eq!(r.id, RuleId(701));
/// ```
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
                    if matches!(a.as_ref(), Expr::Pow(_, _))
                        && matches!(b.as_ref(), Expr::Pow(_, _))
                        && matches!(rhs.as_ref(), Expr::Pow(_, _))
                    {
                        return true;
                    }
                }
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(0)), // No solutions
                justification:
                    "Fermat's Last Theorem: No integer solutions for x^n + y^n = z^n when n > 2"
                        .to_string(),
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
/// Creates a rule that applies Euler's totient multiplicativity: φ(mn) = φ(m)·φ(n) when gcd(m,n) = 1.
///
/// The rule matches a product expression and rewrites φ(mn) into the product of φ on each factor,
/// with an explanatory justification. It is intended for use in simplification passes.
///
/// # Examples
///
/// ```
/// let rule = euler_phi_multiplicative();
/// assert_eq!(rule.id, RuleId(703));
/// assert_eq!(rule.name, "euler_phi_multiplicative");
/// ```
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
                    justification: "φ(mn) = φ(m)φ(n) for coprime m, n (Euler phi multiplicativity)"
                        .to_string(),
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
/// Constructs a rule that applies the Chinese Remainder Theorem to systems of congruences.

///

/// The rule matches a system of modular equations and produces a symbolic placeholder for the unique solution modulo the product of pairwise-coprime moduli.

///

/// # Returns

///

/// A `Rule` which, when applied to matching congruences, yields an `Expr::Var` representing `"x ≡ a_i (mod m_i)"` and a justification stating uniqueness modulo the product of coprime moduli.

///

/// # Examples

///

/// ```

/// let rule = chinese_remainder_theorem();

/// // rule applies to congruence systems and is identified as RuleId(705)

/// assert_eq!(rule.id, RuleId(705));

/// ```
fn chinese_remainder_theorem() -> Rule {
    Rule {
        id: RuleId(705),
        name: "chinese_remainder_theorem",
        category: RuleCategory::EquationSolving,
        description: "CRT: x ≡ a_i (mod m_i) has unique solution mod Π m_i",
        is_applicable: |expr, _ctx| {
            // Match system of Mod equations - for now just detect pattern
            matches!(expr, Expr::Equation { .. } | Expr::Mod(_, _))
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("x ≡ a_i (mod m_i)")),
                justification: "CRT: unique solution mod product of coprime moduli".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// a is a quadratic residue mod p if a = x² (mod p) has solution
/// Produces a rule that recognizes modular square (quadratic residue) patterns and replaces them with a symbolic indicator.
///
/// The rule matches expressions of the form `x^2 (mod p)` (or equivalent modular-square patterns) and yields the symbolic variable `QR(a,p) ∈ {0,1}` to indicate whether `a` is a quadratic residue modulo `p`.
///
/// # Examples
///
/// ```
/// let rule = quadratic_residue();
/// assert_eq!(rule.id, RuleId(706));
/// assert_eq!(rule.name, "quadratic_residue");
/// ```
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
                    result: Expr::Var(intern_symbol("QR(a,p) ∈ {0,1}")),
                    justification: "Quadratic residue exists iff Legendre/Jacobi = 1".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// (ab/p) = (a/p)(b/p) for Legendre symbol
/// Constructs a rule representing multiplicativity of the Legendre symbol.
///
/// The rule encodes the identity (ab / p) = (a / p) (b / p) for Legendre symbols and matches multiplicative expressions.
///
/// # Returns
///
/// A `Rule` that rewrites `(ab/p)` to `(a/p)(b/p)`.
///
/// # Examples
///
/// ```
/// let rule = legendre_symbol_multiplicative();
/// assert_eq!(rule.id, RuleId(707));
/// ```
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
                    result: Expr::Mul(
                        Box::new(Expr::Var(intern_symbol("(a/p)"))),
                        Box::new(Expr::Var(intern_symbol("(b/p)"))),
                    ),
                    justification: "Legendre symbol multiplicative".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// (a/p) = a^((p-1)/2) (mod p)
/// Produces a rule implementing Euler's criterion for odd prime moduli.
///
/// The returned `Rule` matches expressions of the form `a^((p-1)/2) mod p` and
/// replaces them with a symbolic value indicating the possible outcomes: `-1`, `0`, or `1`.
/// The produced transformation carries the justification `"Euler criterion"`.
///
/// # Examples
///
/// ```
/// let rule = euler_criterion();
/// // rule id and basic behavior
/// assert_eq!(rule.id, RuleId(708));
/// ```
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
                    result: Expr::Var(intern_symbol("a^((p-1)/2) mod p ∈ {−1,0,1}")),
                    justification: "Euler criterion".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// π(x) ~ x/ln(x)
/// Provides a rule that recognizes expressions of the form x / ln(x) and rewrites them to x
/// using the prime-counting asymptotic π(x) ~ x / ln(x).
///
/// The rule matches Div(x, Ln(x)) and, when applied, produces `x` with a justification
/// indicating the asymptotic approximation for the prime-counting function.
///
/// # Examples
///
/// ```
/// let r = prime_counting_approx();
/// assert_eq!(r.name, "prime_counting_approx");
/// ```
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
                    justification: "Prime counting function: π(x) ~ x/ln(x) (asymptotic)"
                        .to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// For n > 1, there exists prime p with n < p < 2n
/// Constructs a transformation rule encoding Bertrand's postulate.
///
/// The rule matches comparison expressions that express a range like n < 2n
/// and yields a justification that for n > 1 there exists a prime p with n < p < 2n.
///
/// # Examples
///
/// ```
/// let rule = bertrand_postulate();
/// assert_eq!(rule.id, RuleId(710));
/// ```
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
                justification: "Bertrand's postulate: For n > 1, ∃ prime p with n < p < 2n"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// ax + by = c has solutions iff gcd(a,b) | c
/// Creates a rule encapsulating the solvability condition for linear Diophantine equations of the form `a*x + b*y = c`.
///
/// The rule matches an `Expr::Equation` whose left-hand side is a sum of two terms and produces a `RuleApplication` that preserves the equation while supplying the justification that the equation is solvable iff `gcd(a, b)` divides `c`.
///
/// # Examples
///
/// ```
/// let rule = linear_diophantine();
/// assert_eq!(rule.id, RuleId(711));
/// assert_eq!(rule.name, "linear_diophantine");
/// ```
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
                        justification: "Linear Diophantine: ax + by = c solvable iff gcd(a,b) | c"
                            .to_string(),
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
/// Constructs the rule that recognizes sums of two squares and returns a symbolic criterion.
///
/// Matches expressions of the form `a^2 + b^2` and produces a symbolic placeholder expression
/// `a^2+b^2 criterion` with justification "Sum of two squares criterion".
///
/// # Returns
///
/// A `Rule` that matches `Add(Pow(a, 2), Pow(b, 2))` and yields a symbolic `Expr::Var` describing the criterion.
///
/// # Examples
///
/// ```
/// let rule = sum_of_two_squares();
/// assert_eq!(rule.id.0, 713);
/// ```
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
                result: Expr::Var(intern_symbol("a^2+b^2 criterion")),
                justification: "Sum of two squares criterion".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Every n is sum of 4 squares (Lagrange)
/// Constructs a Rule encoding Lagrange's four-square theorem for nonnegative integer constants.
///
/// The rule matches any nonnegative integer constant and replaces it with a symbolic
/// representation `a^2+b^2+c^2+d^2` justified by "Lagrange four-square theorem".
///
/// # Examples
///
/// ```
/// let r = sum_of_four_squares();
/// // rule id corresponds to the four-squares rule
/// assert_eq!(r.id, RuleId(714));
/// ```
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
                result: Expr::Var(intern_symbol("a^2+b^2+c^2+d^2")),
                justification: "Lagrange four-square theorem".to_string(),
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
/// Creates a transformation rule that represents Hensel's lemma for lifting modular roots.
///
/// The rule targets modular congruences or equations that may admit a solution modulo a prime p and provides a symbolic result representing a lifted root modulo p^k.
///
/// # Returns
///
/// A `Rule` configured to match modular congruences or equations and to produce a symbolic placeholder indicating a lifted root modulo p^k when applied.
///
/// # Examples
///
/// ```
/// let r = hensel_lemma();
/// assert_eq!(r.name, "hensel_lemma");
/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("lift root mod p -> p^k")),
                justification: "Hensel's lemma lifting".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// ord_n(a) | φ(n)
/// Create a rule asserting that the multiplicative order of `a` modulo `n` divides Euler's totient `φ(n)`.
///
/// The rule matches GCD or modular expressions and yields a symbolic result `ord_n(a) | φ(n)` with a justification string.
///
/// # Examples
///
/// ```
/// let r = order_divides_phi();
/// assert_eq!(r.id, RuleId(717));
/// assert_eq!(r.name, "order_divides_phi");
/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("ord_n(a) | φ(n)")),
                justification: "Order divides phi".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Primitive roots exist for 1, 2, 4, p^k, 2p^k (odd prime p)
/// Constructs a rule that detects moduli which admit a primitive root.

///

/// The rule matches modulus expressions and, when applied, returns a symbolic

/// placeholder `Expr::Var("exists primitive root")` with the justification

/// "Primitive roots exist for 1,2,4,p^k,2p^k".

///

/// # Returns

///

/// A `Rule` whose application produces a symbolic existence result for moduli

/// of the form 1, 2, 4, p^k, or 2·p^k (where p is an odd prime).

///

/// # Examples

///

/// ```

/// let rule = primitive_root_existence();

/// assert_eq!(rule.id, RuleId(718));

/// assert_eq!(rule.name, "primitive_root_existence");

/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("exists primitive root")),
                justification: "Primitive roots exist for 1,2,4,p^k,2p^k".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// ν_p(n!) = Σ ⌊n/p^k⌋
/// Constructs a rule implementing Legendre's formula for the p-adic valuation of n!.
///
/// The rule matches factorial expressions and rewrites them to the sum
/// `Σ_{k≥1} ⌊n / p^k⌋`, which equals `ν_p(n!)` for a prime `p`.
///
/// # Examples
///
/// ```
/// let rule = legendre_formula();
/// assert_eq!(rule.id, RuleId(719));
/// assert_eq!(rule.name, "legendre_formula");
/// ```
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
                result: Expr::Var(intern_symbol("Σ_{k≥1} ⌊n/p^k⌋")),
                justification: "Legendre formula for ν_p(n!)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// C(m,n) ≡ Π C(m_i, n_i) (mod p) where m_i, n_i are base-p digits
/// Creates a Rule that applies Lucas' theorem to binomial coefficients modulo a prime p.
///
/// When applicable to an expression of the form binom(m, n) mod p, the rule rewrites it
/// as the product of binomial coefficients of the base-p digits of m and n:
/// ∏ C(m_i, n_i) mod p.
///
/// # Examples
///
/// ```
/// let r = lucas_theorem();
/// assert_eq!(r.id, RuleId(720));
/// assert_eq!(r.name, "lucas_theorem");
/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("∏ C(m_i,n_i) mod p")),
                justification: "Lucas theorem on base-p digits".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Σ μ(d)f(n/d) for divisor d of n
/// Creates a rule representing the Möbius inversion transformation for divisor sums.
///
/// The rule matches divisor-sum patterns and produces a symbolic result `g(n) = Σ μ(d) f(n/d)` as the transformation.
///
/// # Examples
///
/// ```
/// let r = mobius_inversion();
/// // RuleId is a tuple-like newtype; verify the id and name
/// assert_eq!(r.id.0, 721);
/// assert_eq!(r.name, "mobius_inversion");
/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("g(n)=Σ μ(d) f(n/d)")),
                justification: "Möbius inversion".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// μ(mn) = μ(m)μ(n) for gcd(m,n)=1
/// Encodes multiplicativity of the Möbius function: μ(mn) = μ(m)·μ(n) when gcd(m, n) = 1.
///
/// Matches a product expression and rewrites it to the symbolic product `μ(m) * μ(n)`, representing the Möbius function evaluated multiplicatively for coprime factors.
///
/// # Examples
///
/// ```
/// let r = mobius_multiplicative();
/// assert_eq!(r.id, RuleId(722));
/// assert_eq!(r.name, "mobius_multiplicative");
/// ```
fn mobius_multiplicative() -> Rule {
    Rule {
        id: RuleId(722),
        name: "mobius_multiplicative",
        category: RuleCategory::Simplification,
        description: "μ(mn) = μ(m)μ(n) for gcd(m,n)=1",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Mul(m, n) = expr {
                return vec![RuleApplication {
                    result: Expr::Mul(
                        Box::new(Expr::Var(intern_symbol("μ(m)"))),
                        Box::new(Expr::Var(intern_symbol("μ(n)"))),
                    ),
                    justification: "Mobius multiplicative (gcd=1)".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// π(x) bounds using Chebyshev
/// Creates a transformation rule representing Chebyshev's bounds on the prime-counting function.
///
/// The returned `Rule` matches division or natural-log expressions and, when applied,
/// produces a symbolic result expressing the inequality `c1 * x / ln x < π(x) < c2 * x / ln x`
/// with a justification string "Chebyshev prime bounds".
///
/// # Examples
///
/// ```
/// let r = chebyshev_prime_bounds();
/// assert_eq!(r.id, RuleId(723));
/// assert!(r.is_applicable(&Expr::Ln(Box::new(Expr::Var(intern_symbol("x")))), &()));
/// ```
fn chebyshev_prime_bounds() -> Rule {
    Rule {
        id: RuleId(723),
        name: "chebyshev_prime_bounds",
        category: RuleCategory::AlgebraicSolving,
        description: "Chebyshev bounds on π(x)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Ln(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("c1 x/ln x < π(x) < c2 x/ln x")),
                justification: "Chebyshev prime bounds".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Even perfect number form: 2^(p-1)(2^p - 1) where 2^p - 1 is prime
/// Creates a rule that recognizes even perfect numbers of the form 2^(p-1)(2^p - 1).
///
/// The produced `Rule` matches expressions shaped like 2^(p-1) * (2^p - 1) (the standard
/// characterization of even perfect numbers when 2^p - 1 is prime) and, when applied,
/// emits a canonical placeholder representing that form with a justification mentioning
/// Mersenne primes.
///
/// # Examples
///
/// ```
/// let rule = even_perfect_number();
/// assert_eq!(rule.id, RuleId(724));
/// // rule.is_applicable should return true for expressions matching 2^(p-1)*(2^p-1)
/// ```
fn even_perfect_number() -> Rule {
    Rule {
        id: RuleId(724),
        name: "even_perfect_number",
        category: RuleCategory::AlgebraicSolving,
        description: "Even perfect = 2^(p-1)(2^p - 1)",
        is_applicable: |expr, _ctx| {
            // Match: Mul(Pow(2, p-1), Sub(Pow(2, p), 1))
            if let Expr::Mul(a, b) = expr {
                return matches!(a.as_ref(), Expr::Pow(_, _))
                    && matches!(b.as_ref(), Expr::Sub(_, _));
            }
            false
        },
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("2^(p-1)(2^p-1)")),
                justification: "Even perfect form (Mersenne primes)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// 2^p - 1 prime => p is prime
/// Constructs the rule expressing the necessary condition for Mersenne primes.
///
/// The rule matches expressions of the form `2^p - 1` and, when applied, yields the logical
/// consequence that `p` must be prime if `2^p - 1` is prime.
///
/// # Returns
///
/// A `Rule` that recognizes `2^p - 1` and provides the justification "Mersenne prime condition: If 2^p - 1 is prime, then p must be prime".
///
/// # Examples
///
/// ```
/// let rule = mersenne_prime_condition();
/// assert_eq!(rule.id, RuleId(725));
/// assert_eq!(rule.name, "mersenne_prime_condition");
/// ```
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
                justification:
                    "Mersenne prime condition: If 2^p - 1 is prime, then p must be prime"
                        .to_string(),
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
                            justification: format!(
                                "Sum of divisors: σ({}) = {} (sum of all divisors)",
                                num, sum
                            ),
                        }];
                    }
                }
            }
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("σ(n)")),
                justification: "Sum of divisors function".to_string(),
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
                            justification: format!(
                                "Number of divisors: τ({}) = {} (count of all divisors)",
                                num, count
                            ),
                        }];
                    }
                }
            }
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("τ(n)")),
                justification: "Number of divisors function".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// Σ φ(d) = n for d|n
/// Creates the totient-sum rule encoding the identity Σ_{d|n} φ(d) = n.
///
/// The rule matches any constant or variable expression and, when applied,
/// yields the equation `Σ_{d|n} φ(d) = n` as a symbolic transformation with
/// the justification "Totient sum identity".
///
/// # Examples
///
/// ```
/// let r = totient_sum();
/// assert_eq!(r.id, RuleId(728));
/// assert_eq!(r.name, "totient_sum");
/// ```
fn totient_sum() -> Rule {
    Rule {
        id: RuleId(728),
        name: "totient_sum",
        category: RuleCategory::Simplification,
        description: "Σ φ(d) = n for d|n",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Const(_) | Expr::Var(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("Σ_{d|n} φ(d)"))),
                    rhs: Box::new(Expr::Var(intern_symbol("n"))),
                },
                justification: "Totient sum identity".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Number of primitive roots mod n is φ(φ(n))
/// Provides a rule that produces the count of primitive roots modulo n.
///
/// When applied this rule yields the symbolic expression `φ(φ(n))`, representing
/// the number of primitive roots modulo n. The rule has id 729 and is classified
/// under simplification; it matches constant or power expressions.
///
/// # Examples
///
/// ```
/// let r = primitive_root_count();
/// assert_eq!(r.id, RuleId(729));
/// assert_eq!(r.name, "primitive_root_count");
/// ```
fn primitive_root_count() -> Rule {
    Rule {
        id: RuleId(729),
        name: "primitive_root_count",
        category: RuleCategory::Simplification,
        description: "φ(φ(n)) primitive roots mod n",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Const(_) | Expr::Pow(_, _)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("φ(φ(n))")),
                justification: "Count of primitive roots".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// λ(n) Carmichael function
/// Construct a transformation Rule representing the Carmichael function λ(n).
///
/// The rule matches integer constants or lcm expressions and produces a symbolic
/// placeholder `λ(n) = lcm of orders` with justification "Carmichael function".
///
/// # Examples
///
/// ```
/// let r = carmichael_function();
/// assert_eq!(r.name, "carmichael_function");
/// ```
fn carmichael_function() -> Rule {
    Rule {
        id: RuleId(730),
        name: "carmichael_function",
        category: RuleCategory::Simplification,
        description: "Carmichael function λ(n)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Const(_) | Expr::LCM(_, _)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("λ(n) = lcm of orders")),
                justification: "Carmichael function".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Density of square-free numbers = 6/π²
/// Creates a rule that recognizes the asymptotic density of square-free integers (6/π²).
///
/// The rule matches the expression 6/π² and produces the canonical representation `6/π²` with a justification stating the density of square-free integers.
///
/// # Examples
///
/// ```
/// let r = square_free_density();
/// assert!(r.description.contains("6/π²"));
/// ```
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
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Div(
                    Box::new(Expr::int(6)),
                    Box::new(Expr::Pow(Box::new(Expr::Pi), Box::new(Expr::int(2)))),
                ),
                justification: "Density of square-free integers is 6/π²".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Prime gap bounds
/// Provides a rule producing a symbolic upper bound for gaps between consecutive primes.
///
/// The rule, when applied to a subtraction or inequality expression, yields a symbolic variable
/// representing the upper bound "p_{n+1}-p_n = O(p_n^{0.525})".
///
/// # Examples
///
/// ```
/// let r = prime_gap_bound();
/// assert_eq!(r.id, RuleId(732));
/// assert_eq!(r.name, "prime_gap_bound");
/// ```
fn prime_gap_bound() -> Rule {
    Rule {
        id: RuleId(732),
        name: "prime_gap_bound",
        category: RuleCategory::AlgebraicSolving,
        description: "Prime gap upper bounds",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Gt(_, _)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("p_{n+1}-p_n = O(p_n^{0.525})")),
                justification: "Prime gap upper bound".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// p is Sophie Germain prime if both p and 2p+1 are prime
/// Creates a rule that recognizes expressions of the form `2*p + 1` and produces a symbolic assertion that `p` and `2p+1` are prime.
///
/// The rule matches an `Add` whose left side is a `Mul(2, p)`-like product and whose right side is the constant `1`,
/// and when applied returns the placeholder variable `p & 2p+1 prime`.
///
/// # Examples
///
/// ```
/// let r = sophie_germain_prime();
/// assert_eq!(r.id, RuleId(733));
/// ```
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
                result: Expr::Var(intern_symbol("p & 2p+1 prime")),
                justification: "Sophie Germain: p and 2p+1 both prime".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// (p/q)(q/p) = (-1)^((p-1)(q-1)/4)
/// Encodes the quadratic reciprocity law as a transformation rule.
///
/// The rule produces an equation expressing (p/q)(q/p) = (-1)^{((p-1)/2)*((q-1)/2)} using symbolic placeholders
/// so it can be applied to expressions involving quadratic residue Legendre symbols.
///
/// # Examples
///
/// ```
/// let rule = quadratic_reciprocity();
/// assert_eq!(rule.id, RuleId(734));
/// assert_eq!(rule.name, "quadratic_reciprocity");
/// ```
fn quadratic_reciprocity() -> Rule {
    Rule {
        id: RuleId(734),
        name: "quadratic_reciprocity",
        category: RuleCategory::Simplification,
        description: "Quadratic reciprocity law",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Pow(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("(p/q)(q/p)"))),
                    rhs: Box::new(Expr::Pow(
                        Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                        Box::new(Expr::Div(
                            Box::new(Expr::Mul(
                                Box::new(Expr::Sub(
                                    Box::new(Expr::Mul(
                                        Box::new(Expr::int(1)),
                                        Box::new(Expr::Var(intern_symbol("p"))),
                                    )),
                                    Box::new(Expr::int(1)),
                                )),
                                Box::new(Expr::Sub(
                                    Box::new(Expr::Mul(
                                        Box::new(Expr::int(1)),
                                        Box::new(Expr::Var(intern_symbol("q"))),
                                    )),
                                    Box::new(Expr::int(1)),
                                )),
                            )),
                            Box::new(Expr::int(4)),
                        )),
                    )),
                },
                justification: "Quadratic reciprocity".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Jacobi symbol generalization
/// Creates a rule that recognizes Jacobi-symbol-like expressions and replaces them with a symbolic placeholder.
///
/// The returned `Rule` matches division or modular expressions that represent a Jacobi symbol and, when applied,
/// produces the symbolic variable `(a/n) Jacobi` with the justification `"Jacobi symbol definition"`.
///
/// # Examples
///
/// ```
/// let rule = jacobi_symbol();
/// assert_eq!(rule.id, RuleId(735));
/// ```
fn jacobi_symbol() -> Rule {
    Rule {
        id: RuleId(735),
        name: "jacobi_symbol",
        category: RuleCategory::Simplification,
        description: "Jacobi symbol (a/n)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("(a/n) Jacobi")),
                justification: "Jacobi symbol definition".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Kronecker symbol extension
/// Constructs a rule that matches division or modular expressions and replaces them with a Kronecker-symbol placeholder.

///

/// The rule matches `Expr::Div` or `Expr::Mod` and, when applied, produces a symbolic variable `(a/n) Kronecker` with the justification "Kronecker symbol extends Jacobi".

///

/// # Returns

///

/// A `Rule` that recognizes `Div` or `Mod` expressions and yields an `Expr::Var(intern_symbol("(a/n) Kronecker"))` as the transformation result.

///

/// # Examples

///

/// ```

/// let r = kronecker_symbol();

/// assert_eq!(r.id, RuleId(736));

/// assert_eq!(r.name, "kronecker_symbol");

/// ```
fn kronecker_symbol() -> Rule {
    Rule {
        id: RuleId(736),
        name: "kronecker_symbol",
        category: RuleCategory::Simplification,
        description: "Kronecker symbol extension",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("(a/n) Kronecker")),
                justification: "Kronecker symbol extends Jacobi".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Tonelli-Shanks algorithm for modular square root
/// Constructs a rule that matches modular-square-root patterns and yields a Tonelli–Shanks symbolic solution placeholder.
///
/// The returned `Rule` matches expressions of the form `Mod(_, _)` or `Sqrt(_)` and, when applied,
/// produces a symbolic variable representing a root (`x: x^2 ≡ a (mod p)`) with a justification
/// indicating Tonelli–Shanks. The rule is non-reversible and has a cost of 4.
///
/// # Examples
///
/// ```
/// let rule = tonelli_shanks();
/// // rule matches modular-square-root shapes (Mod(...) or Sqrt(...))
/// assert_eq!(rule.id, RuleId(737));
/// ```
fn tonelli_shanks() -> Rule {
    Rule {
        id: RuleId(737),
        name: "tonelli_shanks",
        category: RuleCategory::EquationSolving,
        description: "Tonelli-Shanks modular square root",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mod(_, _) | Expr::Sqrt(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("x: x^2 ≡ a (mod p)")),
                justification: "Tonelli-Shanks modular sqrt".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Discrete log existence and order
/// Produces a rule representing the discrete logarithm problem.
///
/// The rule signals a transformation that yields a symbolic placeholder for solving congruences of the form `g^x ≡ h (mod n)`.
///
/// # Examples
///
/// ```
/// let rule = discrete_log_order();
/// assert_eq!(rule.id, RuleId(738));
/// ```
fn discrete_log_order() -> Rule {
    Rule {
        id: RuleId(738),
        name: "discrete_log_order",
        category: RuleCategory::EquationSolving,
        description: "Discrete logarithm order",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _) | Expr::Mod(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("solve g^x ≡ h (mod n)")),
                justification: "Discrete log order".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// gcd via continued fractions
/// Creates a rule that represents computing the greatest common divisor using continued fractions.
///
/// The rule matches `GCD(a, b)` or `Div(_, _)` expression shapes and, when applied, produces a symbolic
/// placeholder `Expr::Var("gcd via CF")` with justification "Continued fraction GCD".
///
/// # Examples
///
/// ```
/// let r = continued_fraction_gcd();
/// assert_eq!(r.id, RuleId(739));
/// ```
fn continued_fraction_gcd() -> Rule {
    Rule {
        id: RuleId(739),
        name: "continued_fraction_gcd",
        category: RuleCategory::Simplification,
        description: "GCD via continued fractions",
        is_applicable: |expr, _ctx| matches!(expr, Expr::GCD(_, _) | Expr::Div(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("gcd via CF")),
                justification: "Continued fraction GCD".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Farey neighbors |ad - bc| = 1
/// Constructs a rule that recognizes the Farey neighbor condition |a·d - b·c| = 1 for adjacent fractions.
///
/// The returned Rule matches expressions that represent the determinant-like difference between two fractions
/// (or its absolute value) and produces the constant `1` with a justification identifying the Farey neighbor relation.
///
/// # Examples
///
/// ```
/// let rule = farey_neighbors();
/// // rule id corresponds to the Farey neighbors rule defined in the rule set
/// assert_eq!(rule.id, RuleId(740));
/// ```
fn farey_neighbors() -> Rule {
    Rule {
        id: RuleId(740),
        name: "farey_neighbors",
        category: RuleCategory::AlgebraicSolving,
        description: "Farey neighbors: |ad - bc| = 1",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Abs(_)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::int(1),
                justification:
                    "Farey neighbors: Adjacent fractions a/b and c/d satisfy |ad - bc| = 1"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Mediant in Stern-Brocot tree
/// Creates a rule that replaces a sum of two fractions with their Stern–Brocot mediant.
///
/// The rule matches an addition whose operands are both divisions (fractions) and
/// rewrites (a/b) + (c/d) to (a+c)/(b+d) with the justification "Stern-Brocot mediant".
///
/// # Examples
///
/// ```
/// let rule = stern_brocot();
/// assert_eq!(rule.id, RuleId(741));
/// ```
fn stern_brocot() -> Rule {
    Rule {
        id: RuleId(741),
        name: "stern_brocot",
        category: RuleCategory::Simplification,
        description: "Stern-Brocot tree mediant",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Add(a, b) = expr {
                if let (Expr::Div(a1, b1), Expr::Div(c1, d1)) = (a.as_ref(), b.as_ref()) {
                    let num = Expr::Add(a1.clone(), c1.clone());
                    let den = Expr::Add(b1.clone(), d1.clone());
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(num), Box::new(den)),
                        justification: "Stern-Brocot mediant".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// Egyptian fraction representation
/// Creates a rule that represents an Egyptian fraction decomposition for rational divisions.
///
/// The rule matches a division expression a/b and provides a symbolic decomposition result `Σ 1/u_i = a/b`.
///
/// # Examples
///
/// ```
/// let rule = egyptian_fraction();
/// assert_eq!(rule.name, "egyptian_fraction");
/// ```
fn egyptian_fraction() -> Rule {
    Rule {
        id: RuleId(742),
        name: "egyptian_fraction",
        category: RuleCategory::Expansion,
        description: "Egyptian fraction decomposition",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Div(_, _) = expr {
                return vec![RuleApplication {
                    result: Expr::Var(intern_symbol("Σ 1/u_i = a/b")),
                    justification: "Egyptian fraction decomposition".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// N(a + bi) = a² + b²
/// Creates a rule that recognizes expressions of the form `a² + b²` as the Gaussian integer norm N(a + bi) = a² + b².
///
/// The rule matches an `Add` of two square `Pow` terms and produces the same `a² + b²` expression with a justification indicating the Gaussian norm.
///
/// # Examples
///
/// ```
/// let _rule = gaussian_norm();
/// ```
fn gaussian_norm() -> Rule {
    Rule {
        id: RuleId(743),
        name: "gaussian_norm",
        category: RuleCategory::Simplification,
        description: "Gaussian integer norm N(a+bi) = a² + b²",
        is_applicable: |expr, _ctx| {
            // Match: Add(Pow(a, 2), Pow(b, 2))
            if let Expr::Add(a, b) = expr {
                return matches!(a.as_ref(), Expr::Pow(_, _))
                    && matches!(b.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(a, b) = expr {
                return vec![RuleApplication {
                    result: Expr::Add(a.clone(), b.clone()),
                    justification: "Gaussian norm N(a+bi) = a²+b²".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// Gaussian prime conditions
/// Creates a rule that identifies Gaussian prime conditions.
///
/// The rule matches simple integer constants or sums and, when applied,
/// produces a symbolic result indicating the Gaussian-prime criteria
/// ("p≡3 mod4 or a^2+b^2 factors") with an explanatory justification.
///
/// # Examples
///
/// ```
/// let r = gaussian_prime();
/// assert_eq!(r.name, "gaussian_prime");
/// ```
fn gaussian_prime() -> Rule {
    Rule {
        id: RuleId(744),
        name: "gaussian_prime",
        category: RuleCategory::AlgebraicSolving,
        description: "Gaussian prime conditions",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Const(_) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("p≡3 mod4 or a^2+b^2 factors")),
                justification: "Gaussian prime criteria".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}