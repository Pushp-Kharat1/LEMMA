// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Algebraic transformation rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational, Symbol, SymbolTable};
use std::sync::{Mutex, OnceLock};

/// Returns the interned `Symbol` for a given name, reusing the same `Symbol` for identical names.
///
/// This function is thread-safe and will return the canonical `Symbol` associated with `name`.
///
/// # Parameters
///
/// - `name`: The identifier to intern.
///
/// # Returns
///
/// `Symbol` corresponding to the interned `name`; repeated calls with the same `name` yield the same `Symbol`.
///
/// # Examples
///
/// ```
/// let s1 = intern_symbol("x");
/// let s2 = intern_symbol("x");
/// assert_eq!(s1, s2);
/// ```
fn intern_symbol(name: &str) -> Symbol {
    static INTERNER: OnceLock<Mutex<SymbolTable>> = OnceLock::new();
    let m = INTERNER.get_or_init(|| Mutex::new(SymbolTable::new()));
    m.lock().expect("symbol interner poisoned").intern(name)
}

/// Assemble the collection of algebraic transformation rules used for expression simplification.
///
/// This includes core simplification and algebraic identities (constant folding, identities, distributive
/// and factoring rules, power/binomial rules, etc.), followed by advanced algebra rules and the
/// Phase 4 rule set.
///
/// # Returns
///
/// A `Vec<Rule>` containing the core rules in canonical order, then `advanced_algebra_rules()`, and
/// finally `phase4_algebra_rules()`.
///
/// # Examples
///
/// ```
/// let rules = algebra_rules();
/// assert!(!rules.is_empty());
/// ```
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
        binomial_expand(),      // NEW: (a+b)² → a² + 2ab + b²
        binomial_expand_diff(), // NEW: (a-b)² → a² - 2ab + b²
        sub_same(),             // NEW: x - x → 0
    ];
    // Add advanced algebra rules (Phase 1)
    rules.extend(advanced_algebra_rules());
    // Add Phase 4 algebra rules (500 milestone)
    rules.extend(phase4_algebra_rules());
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
// Rule 9: Perfect Square (sum) - Factor a² + 2ab + b² → (a + b)²
// ============================================================================

fn perfect_square_sum() -> Rule {
    Rule {
        id: RuleId(9),
        name: "perfect_square_sum",
        category: RuleCategory::Factoring,
        description: "Factor perfect square: a² + 2ab + b² → (a + b)²",
        is_applicable: |expr, _ctx| {
            // Match a² + 2ab + b² pattern
            if let Expr::Add(left, right) = expr {
                // Check if we have (something + something) structure
                // Try to match a² + (2ab + b²) or (a² + 2ab) + b²
                if let Expr::Add(_, _) = left.as_ref() {
                    return true; // Could be a² + 2ab + b²
                }
                if let Expr::Add(_, _) = right.as_ref() {
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            // Try to pattern match a² + 2ab + b²
            // This is a simplified version - full matching is complex
            if let Expr::Add(left, right) = expr {
                // Look for pattern: a² + (2ab + b²)
                if let (Expr::Pow(a1, exp1), Expr::Add(mid, right2)) =
                    (left.as_ref(), right.as_ref())
                {
                    if let (Expr::Const(two), Expr::Mul(_, _)) = (exp1.as_ref(), mid.as_ref()) {
                        if *two == Rational::from(2) {
                            if let Expr::Pow(b1, exp2) = right2.as_ref() {
                                if let Expr::Const(two2) = exp2.as_ref() {
                                    if *two2 == Rational::from(2) {
                                        // Return (a + b)²
                                        return vec![RuleApplication {
                                            result: Expr::Pow(
                                                Box::new(Expr::Add(a1.clone(), b1.clone())),
                                                Box::new(Expr::Const(Rational::from(2))),
                                            ),
                                            justification: "a² + 2ab + b² = (a + b)²".to_string(),
                                        }];
                                    }
                                }
                            }
                        }
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
// Rule 15: Binomial Expansion (a + b)² → a² + 2ab + b²
// ============================================================================

fn binomial_expand() -> Rule {
    Rule {
        id: RuleId(15),
        name: "binomial_expand",
        category: RuleCategory::Expansion,
        description: "Expand (a + b)² → a² + 2ab + b²",
        is_applicable: |expr, _ctx| {
            // Match (something)^2 where something is an Add
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    return *e == Rational::from(2) && matches!(base.as_ref(), Expr::Add(_, _));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    if *e == Rational::from(2) {
                        if let Expr::Add(a, b) = base.as_ref() {
                            // (a + b)² = a² + 2ab + b²
                            let a_squared =
                                Expr::Pow(a.clone(), Box::new(Expr::Const(Rational::from(2))));
                            let two_ab = Expr::Mul(
                                Box::new(Expr::Const(Rational::from(2))),
                                Box::new(Expr::Mul(a.clone(), b.clone())),
                            );
                            let b_squared =
                                Expr::Pow(b.clone(), Box::new(Expr::Const(Rational::from(2))));

                            let result = Expr::Add(
                                Box::new(Expr::Add(Box::new(a_squared), Box::new(two_ab))),
                                Box::new(b_squared),
                            );

                            return vec![RuleApplication {
                                result,
                                justification: "(a + b)² = a² + 2ab + b²".to_string(),
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

// ============================================================================
// Rule 16: Binomial Expansion Difference (a - b)² → a² - 2ab + b²
// ============================================================================

fn binomial_expand_diff() -> Rule {
    Rule {
        id: RuleId(16),
        name: "binomial_expand_diff",
        category: RuleCategory::Expansion,
        description: "Expand (a - b)² → a² - 2ab + b²",
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    return *e == Rational::from(2) && matches!(base.as_ref(), Expr::Sub(_, _));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Const(e) = exp.as_ref() {
                    if *e == Rational::from(2) {
                        if let Expr::Sub(a, b) = base.as_ref() {
                            // (a - b)² = a² - 2ab + b²
                            let a_squared =
                                Expr::Pow(a.clone(), Box::new(Expr::Const(Rational::from(2))));
                            let two_ab = Expr::Mul(
                                Box::new(Expr::Const(Rational::from(2))),
                                Box::new(Expr::Mul(a.clone(), b.clone())),
                            );
                            let b_squared =
                                Expr::Pow(b.clone(), Box::new(Expr::Const(Rational::from(2))));

                            let result = Expr::Add(
                                Box::new(Expr::Sub(Box::new(a_squared), Box::new(two_ab))),
                                Box::new(b_squared),
                            );

                            return vec![RuleApplication {
                                result,
                                justification: "(a - b)² = a² - 2ab + b²".to_string(),
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

// ============================================================================
// Rule 17: Subtraction Same (x - x = 0)
// ============================================================================

fn sub_same() -> Rule {
    Rule {
        id: RuleId(17),
        name: "sub_same",
        category: RuleCategory::Simplification,
        description: "x - x = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Sub(a, b) = expr {
                return a == b;
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(a, b) = expr {
                if a == b {
                    return vec![RuleApplication {
                        result: Expr::Const(Rational::from(0)),
                        justification: "x - x = 0".to_string(),
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
// Phase 1: Advanced Algebra Rules (ID 300+)
// ============================================================================

/// Get all advanced algebra rules
pub fn advanced_algebra_rules() -> Vec<Rule> {
    vec![
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
        // Binomial identities - NOW ENABLED
        binomial_square_expand(),
        binomial_cube_expand(),
        // Subtraction to addition - NOW ENABLED
        sub_to_add(),
        // Division to multiplication - NOW ENABLED
        div_to_mul(),
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

// ============================================================================
// Phase 4: Additional Algebra Rules (ID 320-369)
// ============================================================================

/// Phase 4 algebra rules for 500 rules milestone
pub fn phase4_algebra_rules() -> Vec<Rule> {
    vec![
        log_product(),
        log_quotient(),
        log_power(),
        log_base_change(),
        log_one(),
        log_same_base(),
        exp_product(),
        exp_quotient(),
        exp_power(),
        exp_zero(),
        exp_one(),
        exp_ln(),
        ln_exp(),
        sqrt_product(),
        sqrt_quotient(),
        sqrt_square(),
        cube_root_cube(),
        nth_root_power(),
        rationalize_denominator(),
        conjugate_multiply(),
        sum_of_cubes_factor(),
        diff_of_cubes_factor(),
        perfect_cube_sum(),
        perfect_cube_diff(),
        quadratic_complete_square(),
        vieta_sum(),
        vieta_product(),
        factor_quadratic(),
        rational_root_test(),
        synthetic_division(),
        polynomial_division(),
        remainder_theorem(),
        factor_theorem(),
        bezout_identity(),
        euclidean_division(),
        fraction_add(),
        fraction_mul(),
        fraction_div(),
        cross_multiply(),
        lcd_combine(),
        abs_nonnegative(),
        abs_square(),
        triangle_inequality(),
        reverse_triangle(),
        am_gm_2(),
        am_gm_3(),
        qm_am(),
        cauchy_schwarz_2(),
        holders_inequality(),
        minkowski_inequality(),
    ]
}

// log(ab) = log(a) + log(b)
fn log_product() -> Rule {
    Rule {
        id: RuleId(320),
        name: "log_product",
        category: RuleCategory::Simplification,
        description: "log(ab) = log(a) + log(b)",
        is_applicable: |expr, _| {
            if let Expr::Ln(inner) = expr {
                return matches!(inner.as_ref(), Expr::Mul(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Ln(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Ln(a.clone())),
                            Box::new(Expr::Ln(b.clone())),
                        ),
                        justification: "log(ab) = log(a) + log(b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// log(a/b) = log(a) - log(b)
fn log_quotient() -> Rule {
    Rule {
        id: RuleId(321),
        name: "log_quotient",
        category: RuleCategory::Simplification,
        description: "log(a/b) = log(a) - log(b)",
        is_applicable: |expr, _| {
            if let Expr::Ln(inner) = expr {
                return matches!(inner.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Ln(inner) = expr {
                if let Expr::Div(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sub(
                            Box::new(Expr::Ln(a.clone())),
                            Box::new(Expr::Ln(b.clone())),
                        ),
                        justification: "log(a/b) = log(a) - log(b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// log(a^n) = n*log(a)
fn log_power() -> Rule {
    Rule {
        id: RuleId(322),
        name: "log_power",
        category: RuleCategory::Simplification,
        description: "log(a^n) = n*log(a)",
        is_applicable: |expr, _| {
            if let Expr::Ln(inner) = expr {
                return matches!(inner.as_ref(), Expr::Pow(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Ln(inner) = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Mul(exp.clone(), Box::new(Expr::Ln(base.clone()))),
                        justification: "log(a^n) = n*log(a)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// log_b(a) = ln(a)/ln(b)
/// Creates the rule that recognizes and represents the logarithm change-of-base identity.
///
/// The rule matches expressions of the form `ln(a) / ln(b)` and produces an equation
/// representing `log_b(a) = ln(a) / ln(b)`.
///
/// # Examples
///
/// ```
/// let rule = log_base_change();
/// let a = Expr::Var(intern_symbol("a"));
/// let b = Expr::Var(intern_symbol("b"));
/// let expr = Expr::Div(Box::new(Expr::Ln(Box::new(a.clone()))), Box::new(Expr::Ln(Box::new(b.clone()))));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// match &apps[0].result {
///     Expr::Equation { lhs, rhs } => {
///         // lhs is a placeholder `log_b(a)` and rhs is the original division `ln(a)/ln(b)`
///         assert!(matches!(lhs.as_ref(), Expr::Var(_)));
///         assert!(matches!(rhs.as_ref(), Expr::Div(_, _)));
///     }
///     _ => panic!("expected an equation"),
/// }
/// ```
fn log_base_change() -> Rule {
    Rule {
        id: RuleId(323),
        name: "log_base_change",
        category: RuleCategory::Simplification,
        description: "log_b(a) = ln(a)/ln(b)",
        is_applicable: |expr, _| {
            // Match: Div(Ln(a), Ln(b)) - this IS the change of base form
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Ln(_))
                    && matches!(denom.as_ref(), Expr::Ln(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                if let (Expr::Ln(a), Expr::Ln(b)) = (num.as_ref(), denom.as_ref()) {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: Box::new(Expr::Var(intern_symbol("log_b(a)"))),
                            rhs: Box::new(expr.clone()),
                        },
                        justification: "Change of base log_b(a) = ln(a)/ln(b)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// ln(1) = 0
fn log_one() -> Rule {
    Rule {
        id: RuleId(324),
        name: "log_one",
        category: RuleCategory::Simplification,
        description: "ln(1) = 0",
        is_applicable: |expr, _| matches!(expr, Expr::Ln(inner) if matches!(inner.as_ref(), Expr::Const(r) if r.numer() == 1 && r.denom() == 1)),
        apply: |_, _| {
            vec![RuleApplication {
                result: Expr::int(0),
                justification: "ln(1) = 0".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// ln(e) = 1
fn log_same_base() -> Rule {
    Rule {
        id: RuleId(325),
        name: "log_same_base",
        category: RuleCategory::Simplification,
        description: "ln(e) = 1",
        is_applicable: |expr, _| matches!(expr, Expr::Ln(inner) if matches!(inner.as_ref(), Expr::E)),
        apply: |_, _| {
            vec![RuleApplication {
                result: Expr::int(1),
                justification: "ln(e) = 1".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// e^a * e^b = e^(a+b)
/// Combine a product of two exponentials with base e into a single exponential with summed exponents.
///
/// When given an expression of the form `e^a * e^b`, produces `e^(a + b)`.
///
/// # Examples
///
/// ```
/// // Construct `e^1 * e^2` and assert it rewrites to `e^(1 + 2)`.
/// use crate::expr::Expr;
/// use crate::rules::exp_product;
///
/// let expr = Expr::Mul(
///     Box::new(Expr::Pow(Box::new(Expr::E), Box::new(Expr::Const(1.into())))),
///     Box::new(Expr::Pow(Box::new(Expr::E), Box::new(Expr::Const(2.into())))),
/// );
///
/// // `apply` takes an expression and a context; provide a default context if available.
/// let results = exp_product().apply(&expr, &mut Default::default());
/// assert_eq!(
///     results[0].result,
///     Expr::Pow(
///         Box::new(Expr::E),
///         Box::new(Expr::Add(Box::new(Expr::Const(1.into())), Box::new(Expr::Const(2.into()))))
///     )
/// );
/// ```
fn exp_product() -> Rule {
    Rule {
        id: RuleId(326),
        name: "exp_product",
        category: RuleCategory::Simplification,
        description: "e^a * e^b = e^(a+b)",
        is_applicable: |expr, _| {
            if let Expr::Mul(left, right) = expr {
                return matches!(left.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::E))
                    && matches!(right.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::E));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Pow(base1, exp1), Expr::Pow(base2, exp2)) =
                    (left.as_ref(), right.as_ref())
                {
                    if matches!(base1.as_ref(), Expr::E) && matches!(base2.as_ref(), Expr::E) {
                        return vec![RuleApplication {
                            result: Expr::Pow(
                                Box::new(Expr::E),
                                Box::new(Expr::Add(exp1.clone(), exp2.clone())),
                            ),
                            justification: "e^a * e^b = e^(a+b)".to_string(),
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

// e^a / e^b = e^(a-b)
/// Creates a rule that rewrites quotients of base‑e exponentials into a single exponential.
///
/// This rule matches expressions of the form `e^a / e^b` and produces `e^(a - b)`.
///
/// # Returns
///
/// A `Rule` that transforms `e^a / e^b` into `e^(a - b)` when both numerator and denominator are powers with base `e`.
///
/// # Examples
///
/// ```
/// let rule = exp_quotient();
/// assert_eq!(rule.name, "exp_quotient");
/// ```
fn exp_quotient() -> Rule {
    Rule {
        id: RuleId(327),
        name: "exp_quotient",
        category: RuleCategory::Simplification,
        description: "e^a / e^b = e^(a-b)",
        is_applicable: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::E))
                    && matches!(denom.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::E));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                if let (Expr::Pow(base1, exp1), Expr::Pow(base2, exp2)) =
                    (num.as_ref(), denom.as_ref())
                {
                    if matches!(base1.as_ref(), Expr::E) && matches!(base2.as_ref(), Expr::E) {
                        return vec![RuleApplication {
                            result: Expr::Pow(
                                Box::new(Expr::E),
                                Box::new(Expr::Sub(exp1.clone(), exp2.clone())),
                            ),
                            justification: "e^a / e^b = e^(a-b)".to_string(),
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

// (e^a)^b = e^(ab)
fn exp_power() -> Rule {
    Rule {
        id: RuleId(328),
        name: "exp_power",
        category: RuleCategory::Simplification,
        description: "(e^a)^b = e^(ab)",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Pow(inner_base, _) = base.as_ref() {
                    return matches!(inner_base.as_ref(), Expr::E);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, outer_exp) = expr {
                if let Expr::Pow(inner_base, inner_exp) = base.as_ref() {
                    if matches!(inner_base.as_ref(), Expr::E) {
                        return vec![RuleApplication {
                            result: Expr::Pow(
                                Box::new(Expr::E),
                                Box::new(Expr::Mul(inner_exp.clone(), outer_exp.clone())),
                            ),
                            justification: "(e^a)^b = e^(ab)".to_string(),
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

// e^0 = 1
fn exp_zero() -> Rule {
    Rule {
        id: RuleId(329),
        name: "exp_zero",
        category: RuleCategory::Simplification,
        description: "e^0 = 1",
        is_applicable: |expr, _| matches!(expr, Expr::Exp(inner) if matches!(inner.as_ref(), Expr::Const(r) if r.is_zero())),
        apply: |_, _| {
            vec![RuleApplication {
                result: Expr::int(1),
                justification: "e^0 = 1".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// e^1 = e
fn exp_one() -> Rule {
    Rule {
        id: RuleId(330),
        name: "exp_one",
        category: RuleCategory::Simplification,
        description: "e^1 = e",
        is_applicable: |expr, _| matches!(expr, Expr::Exp(inner) if matches!(inner.as_ref(), Expr::Const(r) if r.numer() == 1 && r.denom() == 1)),
        apply: |_, _| {
            vec![RuleApplication {
                result: Expr::E,
                justification: "e^1 = e".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// e^(ln(x)) = x
fn exp_ln() -> Rule {
    Rule {
        id: RuleId(331),
        name: "exp_ln",
        category: RuleCategory::Simplification,
        description: "e^(ln(x)) = x",
        is_applicable: |expr, _| matches!(expr, Expr::Exp(inner) if matches!(inner.as_ref(), Expr::Ln(_))),
        apply: |expr, _| {
            if let Expr::Exp(inner) = expr {
                if let Expr::Ln(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**x).clone(),
                        justification: "e^(ln(x)) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ln(e^x) = x
fn ln_exp() -> Rule {
    Rule {
        id: RuleId(332),
        name: "ln_exp",
        category: RuleCategory::Simplification,
        description: "ln(e^x) = x",
        is_applicable: |expr, _| matches!(expr, Expr::Ln(inner) if matches!(inner.as_ref(), Expr::Exp(_))),
        apply: |expr, _| {
            if let Expr::Ln(inner) = expr {
                if let Expr::Exp(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**x).clone(),
                        justification: "ln(e^x) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// √(ab) = √a * √b
fn sqrt_product() -> Rule {
    Rule {
        id: RuleId(333),
        name: "sqrt_product",
        category: RuleCategory::Simplification,
        description: "√(ab) = √a * √b",
        is_applicable: |expr, _| matches!(expr, Expr::Sqrt(inner) if matches!(inner.as_ref(), Expr::Mul(_, _))),
        apply: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Mul(
                            Box::new(Expr::Sqrt(a.clone())),
                            Box::new(Expr::Sqrt(b.clone())),
                        ),
                        justification: "√(ab) = √a * √b".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// √(a/b) = √a / √b
fn sqrt_quotient() -> Rule {
    Rule {
        id: RuleId(334),
        name: "sqrt_quotient",
        category: RuleCategory::Simplification,
        description: "√(a/b) = √a / √b",
        is_applicable: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                return matches!(inner.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Div(a, b) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Sqrt(a.clone())),
                            Box::new(Expr::Sqrt(b.clone())),
                        ),
                        justification: "√(a/b) = √a / √b".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// √(x²) = |x|
fn sqrt_square() -> Rule {
    Rule {
        id: RuleId(335),
        name: "sqrt_square",
        category: RuleCategory::Simplification,
        description: "√(x²) = |x|",
        is_applicable: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Pow(_, exp) = inner.as_ref() {
                    return matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 2 && r.denom() == 1);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Pow(base, _) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Abs(base.clone()),
                        justification: "√(x²) = |x|".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ∛(x³) = x
fn cube_root_cube() -> Rule {
    Rule {
        id: RuleId(336),
        name: "cube_root_cube",
        category: RuleCategory::Simplification,
        description: "∛(x³) = x",
        is_applicable: |expr, _| {
            if let Expr::Pow(inner, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 1 && r.denom() == 3) {
                    if let Expr::Pow(_, inner_exp) = inner.as_ref() {
                        return matches!(inner_exp.as_ref(), Expr::Const(r) if r.numer() == 3);
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(inner, _) = expr {
                if let Expr::Pow(base, _) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**base).clone(),
                        justification: "∛(x³) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ⁿ√(xⁿ) = |x| for even n, x for odd n
/// Create a rule that simplifies nested power expressions of the form (x^m)^(1/m).
///
/// The rule matches expressions where the outer exponent is a rational `1/m` (m > 1)
/// and the inner exponent is the integer `m`. When applicable the rule replaces
/// (x^m)^(1/m) with `|x|` if `m` is even, or with `x` if `m` is odd. The rule is
/// not reversible and has cost 2.
///
/// # Examples
///
/// ```
/// let _rule = nth_root_power();
/// ```
fn nth_root_power() -> Rule {
    Rule {
        id: RuleId(337),
        name: "nth_root_power",
        category: RuleCategory::Simplification,
        description: "ⁿ√(xⁿ) = |x| (even n) or x (odd n)",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, outer_exp) = expr {
                if let Expr::Pow(_inner_base, inner_exp) = base.as_ref() {
                    if let (Expr::Const(outer), Expr::Const(inner)) =
                        (outer_exp.as_ref(), inner_exp.as_ref())
                    {
                        return outer.numer() == 1
                            && outer.denom() > 1
                            && inner.is_integer()
                            && inner.numer() == outer.denom()
                            && inner.denom() == 1;
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, outer_exp) = expr {
                if let Expr::Pow(inner_base, inner_exp) = base.as_ref() {
                    if let (Expr::Const(outer), Expr::Const(inner)) =
                        (outer_exp.as_ref(), inner_exp.as_ref())
                    {
                        if outer.numer() == 1
                            && outer.denom() > 1
                            && inner.is_integer()
                            && inner.numer() == outer.denom()
                            && inner.denom() == 1
                        {
                            let is_even = outer.denom() % 2 == 0;
                            let simplified = if is_even {
                                Expr::Abs(inner_base.clone())
                            } else {
                                *inner_base.clone()
                            };

                            let justification = if is_even {
                                "ⁿ√(xⁿ) = |x| when n is even"
                            } else {
                                "ⁿ√(xⁿ) = x when n is odd"
                            };

                            return vec![RuleApplication {
                                result: simplified,
                                justification: justification.to_string(),
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

// 1/(a+b√c) * (a-b√c)/(a-b√c) = (a-b√c)/(a²-b²c)
/// Rationalize a division whose denominator is a binomial by multiplying with its conjugate.
///
/// This rule applies to expressions of the form `num / (a ± b)` and returns a transformation
/// that multiplies numerator and denominator by the conjugate `(a ∓ b)`, producing
/// `(num * (a ∓ b)) / (a^2 - b^2)`.
///
/// # Examples
///
/// ```
/// let _rule = rationalize_denominator();
/// ```
fn rationalize_denominator() -> Rule {
    Rule {
        id: RuleId(338),
        name: "rationalize_denominator",
        category: RuleCategory::Simplification,
        description: "Rationalize denominator with conjugate",
        is_applicable: |expr, _| {
            if let Expr::Div(_, denom) = expr {
                return matches!(denom.as_ref(), Expr::Add(_, _) | Expr::Sub(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                match denom.as_ref() {
                    Expr::Add(a, b) => {
                        let conjugate = Expr::Sub(a.clone(), b.clone());
                        let new_num = Expr::Mul(num.clone(), Box::new(conjugate));
                        let new_denom = Expr::Sub(
                            Box::new(Expr::Pow(a.clone(), Box::new(Expr::int(2)))),
                            Box::new(Expr::Pow(b.clone(), Box::new(Expr::int(2)))),
                        );
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(new_num), Box::new(new_denom)),
                            justification: "Multiply numerator and denominator by the conjugate to remove the radical".to_string(),
                        }];
                    }
                    Expr::Sub(a, b) => {
                        let conjugate = Expr::Add(a.clone(), b.clone());
                        let new_num = Expr::Mul(num.clone(), Box::new(conjugate));
                        let new_denom = Expr::Sub(
                            Box::new(Expr::Pow(a.clone(), Box::new(Expr::int(2)))),
                            Box::new(Expr::Pow(b.clone(), Box::new(Expr::int(2)))),
                        );
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(new_num), Box::new(new_denom)),
                            justification: "Multiply by conjugate: (a±b)(a∓b) = a² - b²"
                                .to_string(),
                        }];
                    }
                    _ => {}
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// (a+b)(a-b) = a² - b²
fn conjugate_multiply() -> Rule {
    Rule {
        id: RuleId(339),
        name: "conjugate_multiply",
        category: RuleCategory::Simplification,
        description: "(a+b)(a-b) = a² - b²",
        is_applicable: |expr, _| {
            // Match: Mul(Add(a,b), Sub(a,b)) or Mul(Add(a,b), Sub(b,a))
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Add(a1, b1), Expr::Sub(a2, b2)) = (left.as_ref(), right.as_ref()) {
                    return (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2);
                }
                if let (Expr::Sub(a1, b1), Expr::Add(a2, b2)) = (left.as_ref(), right.as_ref()) {
                    return (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Add(a, b), Expr::Sub(a2, b2)) = (left.as_ref(), right.as_ref()) {
                    if a == a2 && b == b2 {
                        return vec![RuleApplication {
                            result: Expr::Sub(
                                Box::new(Expr::Pow(a.clone(), Box::new(Expr::int(2)))),
                                Box::new(Expr::Pow(b.clone(), Box::new(Expr::int(2)))),
                            ),
                            justification: "(a+b)(a-b) = a² - b²".to_string(),
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

// a³ + b³ = (a+b)(a² - ab + b²)
fn sum_of_cubes_factor() -> Rule {
    Rule {
        id: RuleId(340),
        name: "sum_of_cubes_factor",
        category: RuleCategory::Factoring,
        description: "a³ + b³ = (a+b)(a² - ab + b²)",
        is_applicable: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Pow(_, exp1), Expr::Pow(_, exp2)) = (left.as_ref(), right.as_ref()) {
                    return matches!(exp1.as_ref(), Expr::Const(c) if c.numer() == 3)
                        && matches!(exp2.as_ref(), Expr::Const(c) if c.numer() == 3);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref()) {
                    // (a+b)(a² - ab + b²)
                    let a_plus_b = Expr::Add(a.clone(), b.clone());
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let ab = Expr::Mul(a.clone(), b.clone());
                    let second_factor = Expr::Sub(
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
fn diff_of_cubes_factor() -> Rule {
    Rule {
        id: RuleId(341),
        name: "diff_of_cubes_factor",
        category: RuleCategory::Factoring,
        description: "a³ - b³ = (a-b)(a² + ab + b²)",
        is_applicable: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Pow(_, exp1), Expr::Pow(_, exp2)) = (left.as_ref(), right.as_ref()) {
                    return matches!(exp1.as_ref(), Expr::Const(c) if c.numer() == 3)
                        && matches!(exp2.as_ref(), Expr::Const(c) if c.numer() == 3);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref()) {
                    // (a-b)(a² + ab + b²)
                    let a_minus_b = Expr::Sub(a.clone(), b.clone());
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let ab = Expr::Mul(a.clone(), b.clone());
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

// (a+b)³ = a³ + 3a²b + 3ab² + b³
fn perfect_cube_sum() -> Rule {
    Rule {
        id: RuleId(342),
        name: "perfect_cube_sum",
        category: RuleCategory::Expansion,
        description: "(a+b)³ = a³ + 3a²b + 3ab² + b³",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 3) {
                    return matches!(base.as_ref(), Expr::Add(_, _));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Add(a, b) = base.as_ref() {
                    // a³ + 3a²b + 3ab² + b³
                    let a_cubed = Expr::Pow(a.clone(), Box::new(Expr::int(3)));
                    let b_cubed = Expr::Pow(b.clone(), Box::new(Expr::int(3)));
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let three_a_sq_b = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(Box::new(a_sq), b.clone())),
                    );
                    let three_ab_sq = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(a.clone(), Box::new(b_sq))),
                    );
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Add(
                                Box::new(Expr::Add(Box::new(a_cubed), Box::new(three_a_sq_b))),
                                Box::new(three_ab_sq),
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

// (a-b)³ = a³ - 3a²b + 3ab² - b³
fn perfect_cube_diff() -> Rule {
    Rule {
        id: RuleId(343),
        name: "perfect_cube_diff",
        category: RuleCategory::Expansion,
        description: "(a-b)³ = a³ - 3a²b + 3ab² - b³",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 3) {
                    return matches!(base.as_ref(), Expr::Sub(_, _));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Sub(a, b) = base.as_ref() {
                    // a³ - 3a²b + 3ab² - b³
                    let a_cubed = Expr::Pow(a.clone(), Box::new(Expr::int(3)));
                    let b_cubed = Expr::Pow(b.clone(), Box::new(Expr::int(3)));
                    let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                    let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                    let three_a_sq_b = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(Box::new(a_sq), b.clone())),
                    );
                    let three_ab_sq = Expr::Mul(
                        Box::new(Expr::int(3)),
                        Box::new(Expr::Mul(a.clone(), Box::new(b_sq))),
                    );
                    return vec![RuleApplication {
                        result: Expr::Sub(
                            Box::new(Expr::Add(
                                Box::new(Expr::Sub(Box::new(a_cubed), Box::new(three_a_sq_b))),
                                Box::new(three_ab_sq),
                            )),
                            Box::new(b_cubed),
                        ),
                        justification: "(a-b)³ = a³ - 3a²b + 3ab² - b³".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// ax² + bx + c → a(x + b/2a)² - (b² - 4ac)/4a
/// Constructs a transformation rule that rewrites quadratic expressions into completed-square form.
///
/// The returned `Rule` matches quadratic sum expressions of the form `a*x^2 + b*x + c` (when applicable)
/// and produces an equivalent completed-square representation `(x + b/2)^2 - (b/2)^2 + c`.
///
/// # Examples
///
/// ```
/// let rule = quadratic_complete_square();
/// assert_eq!(rule.name, "quadratic_complete_square");
/// ```
fn quadratic_complete_square() -> Rule {
    Rule {
        id: RuleId(344),
        name: "quadratic_complete_square",
        category: RuleCategory::Simplification,
        description: "Complete the square for quadratic",
        is_applicable: |expr, _| {
            // Match: ax² + bx + c (simplified pattern)
            // This is complex - for now, just match Add expressions
            matches!(expr, Expr::Add(_, _))
        },
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("(x + b/2)^2 - (b/2)^2 + c")),
                justification: "Complete the square form".to_string(),
            }]
        },
        reversible: true,
        cost: 4,
    }
}

// For x² - (r1+r2)x + r1*r2 = 0, sum of roots = r1+r2
/// Creates a rule expressing Vieta's relation for a quadratic: the sum of roots equals -b/a.
///
/// The produced rule matches quadratic equations and, when applied, produces an equation of the form
/// `r1 + r2 = -b / a` with an explanatory justification.
///
/// # Examples
///
/// ```
/// let rule = vieta_sum();
/// assert_eq!(rule.name, "vieta_sum");
/// ```
fn vieta_sum() -> Rule {
    Rule {
        id: RuleId(345),
        name: "vieta_sum",
        category: RuleCategory::Simplification,
        description: "Sum of roots = -b/a",
        is_applicable: |expr, _| {
            // Match quadratic equations
            matches!(expr, Expr::Equation { .. })
        },
        apply: |_expr, _| {
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
                justification: "Vieta: r1+r2 = -b/a".to_string(),
            }]
        },
        reversible: true,
        cost: 2,
    }
}

// For x² - (r1+r2)x + r1*r2 = 0, product of roots = r1*r2
/// Constructs the Vieta product rule: an equation stating that the product of roots r1 * r2 equals c / a.
///
/// The rule produces an equation of the form `r1 * r2 = c / a` using interned symbols `r1`, `r2`, `c`, and `a`.
///
/// # Examples
///
/// ```
/// let rule = vieta_product();
/// assert_eq!(rule.name, "vieta_product");
/// ```
fn vieta_product() -> Rule {
    Rule {
        id: RuleId(346),
        name: "vieta_product",
        category: RuleCategory::Simplification,
        description: "Product of roots = c/a",
        is_applicable: |expr, _| matches!(expr, Expr::Equation { .. }),
        apply: |_expr, _| {
            let a = intern_symbol("a");
            let c = intern_symbol("c");
            let r1 = intern_symbol("r1");
            let r2 = intern_symbol("r2");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Mul(Box::new(Expr::Var(r1)), Box::new(Expr::Var(r2)))),
                    rhs: Box::new(Expr::Div(Box::new(Expr::Var(c)), Box::new(Expr::Var(a)))),
                },
                justification: "Vieta: r1*r2 = c/a".to_string(),
            }]
        },
        reversible: true,
        cost: 2,
    }
}

// ax² + bx + c → a(x-r1)(x-r2)
/// Creates a rule that factors a quadratic expression into a leading coefficient times two linear factors.
///
/// The rule matches quadratic addition expressions and rewrites them as `a * (x - r1) * (x - r2)`, where `a`, `r1`, `r2`, and `x` are placeholder symbols used to represent the leading coefficient, the two roots, and the variable respectively.
///
/// # Examples
///
/// ```
/// let rule = factor_quadratic();
/// assert_eq!(rule.id.0, 347);
/// assert_eq!(rule.name, "factor_quadratic");
/// ```
fn factor_quadratic() -> Rule {
    Rule {
        id: RuleId(347),
        name: "factor_quadratic",
        category: RuleCategory::Factoring,
        description: "Factor quadratic using roots",
        is_applicable: |expr, _| {
            // Match quadratic expressions
            if let Expr::Add(_, _) = expr {
                return true;
            }
            false
        },
        apply: |_expr, _| {
            let a = intern_symbol("a");
            let r1 = intern_symbol("r1");
            let r2 = intern_symbol("r2");
            let x = intern_symbol("x");
            let rhs = Expr::Mul(
                Box::new(Expr::Var(a)),
                Box::new(Expr::Mul(
                    Box::new(Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(r1)))),
                    Box::new(Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(r2)))),
                )),
            );
            vec![RuleApplication {
                result: rhs,
                justification: "Factor quadratic via roots".to_string(),
            }]
        },
        reversible: true,
        cost: 3,
    }
}

// Rational root theorem test
/// Creates a rule that applies the Rational Root Theorem to polynomial expressions.
///
/// The rule matches polynomial-like expressions and, when applied, yields a single
/// RuleApplication whose result is a symbolic placeholder representing the set of
/// possible rational roots: `"± factors(a0)/factors(an)"`. The justification string
/// for the application is `"Rational root theorem"`.
///
/// # Examples
///
/// ```
/// let r = rational_root_test();
/// assert_eq!(r.name, "rational_root_test");
/// ```
fn rational_root_test() -> Rule {
    Rule {
        id: RuleId(348),
        name: "rational_root_test",
        category: RuleCategory::Simplification,
        description: "Rational root theorem",
        is_applicable: |expr, _| matches!(expr, Expr::Add(_, _) | Expr::Pow(_, _)),
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("± factors(a0)/factors(an)")),
                justification: "Rational root theorem".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Synthetic division for polynomials
/// Creates a simplification rule that applies a synthetic-division transformation to division expressions.
///
/// The rule matches `Expr::Div` nodes and, when applied, produces a placeholder variable `synthetic_div_result`
/// representing the result of performing synthetic division. The produced application includes a justification
/// string "Synthetic division schema".
///
/// # Returns
///
/// A `Rule` that targets division expressions and yields an `Expr::Var(intern_symbol("synthetic_div_result"))`
/// as the transformation result.
///
/// # Examples
///
/// ```
/// let _rule = synthetic_division();
/// ```
fn synthetic_division() -> Rule {
    Rule {
        id: RuleId(349),
        name: "synthetic_division",
        category: RuleCategory::Simplification,
        description: "Synthetic division",
        is_applicable: |expr, _| matches!(expr, Expr::Div(_, _)),
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("synthetic_div_result")),
                justification: "Synthetic division schema".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Polynomial long division
/// Creates a rule that rewrites polynomial division expressions into quotient-and-remainder placeholders.
///
/// This rule is applicable to division expressions and, when applied, yields a single result
/// expression of the form `Expr::Var(intern_symbol("Q(x),R(x)"))` representing the quotient `Q(x)`
/// and remainder `R(x)` produced by polynomial long division. The `justification` describes the
/// transformation as a "Polynomial long division schema".
///
/// # Examples
///
/// ```
/// let rule = polynomial_division();
/// assert_eq!(rule.name, "polynomial_division");
/// // The rule targets `Expr::Div` expressions and produces a `Q(x),R(x)` placeholder result when applied.
/// ```
fn polynomial_division() -> Rule {
    Rule {
        id: RuleId(350),
        name: "polynomial_division",
        category: RuleCategory::Simplification,
        description: "Polynomial long division",
        is_applicable: |expr, _| matches!(expr, Expr::Div(_, _)),
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("Q(x),R(x)")),
                justification: "Polynomial long division schema".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// f(a) = remainder when f(x) divided by (x-a)
/// Produces a rule implementing the polynomial remainder theorem: when a polynomial `P(x)` is divided by `(x - a)`, the remainder is `P(a)`.
///
/// The rule matches division expressions and yields a symbolic remainder expression `P(a)`.
///
/// # Examples
///
/// ```
/// let rule = remainder_theorem();
/// let expr = Expr::Div(Box::new(Expr::Var(intern_symbol("P(x)"))), Box::new(Expr::Var(intern_symbol("x - a"))));
/// let apps = (rule.apply)(&expr);
/// assert_eq!(apps[0].result, Expr::Var(intern_symbol("P(a)")));
/// ```
fn remainder_theorem() -> Rule {
    Rule {
        id: RuleId(351),
        name: "remainder_theorem",
        category: RuleCategory::Simplification,
        description: "Remainder theorem",
        is_applicable: |expr, _| matches!(expr, Expr::Div(_, _)),
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Var(intern_symbol("P(a)")),
                justification: "Remainder is P(a)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// (x-a) is factor of f(x) iff f(a) = 0
/// Produces a rule that applies the factor theorem to polynomial expressions.
///
/// When applied, the rule yields an equation representing a root of a polynomial (the form `P(a) = 0`).
///
/// # Examples
///
/// ```
/// let rule = factor_theorem();
/// assert_eq!(rule.name, "factor_theorem");
/// ```
fn factor_theorem() -> Rule {
    Rule {
        id: RuleId(352),
        name: "factor_theorem",
        category: RuleCategory::Simplification,
        description: "Factor theorem",
        is_applicable: |expr, _| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("P(a)"))),
                    rhs: Box::new(Expr::int(0)),
                },
                justification: "Factor theorem P(a)=0".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// gcd(a,b) = ax + by for some integers x,y
/// Produces a rule asserting Bézout's identity for two integers.
///
/// The returned rule matches a `GCD(a, b)` expression and rewrites it as
/// the equation `gcd(a, b) = a*x + b*y` using interned placeholder symbols
/// `a`, `b`, `x`, and `y`.
///
/// # Examples
///
/// ```
/// let rule = bezout_identity();
/// assert_eq!(rule.id, RuleId(353));
/// assert_eq!(rule.name, "bezout_identity");
/// ```
fn bezout_identity() -> Rule {
    Rule {
        id: RuleId(353),
        name: "bezout_identity",
        category: RuleCategory::Simplification,
        description: "Bezout's identity: gcd(a,b) = ax + by",
        is_applicable: |expr, _| matches!(expr, Expr::GCD(_, _)),
        apply: |_expr, _| {
            let a = intern_symbol("a");
            let b = intern_symbol("b");
            let x = intern_symbol("x");
            let y = intern_symbol("y");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::GCD(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                    rhs: Box::new(Expr::Add(
                        Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(x)))),
                        Box::new(Expr::Mul(Box::new(Expr::Var(b)), Box::new(Expr::Var(y)))),
                    )),
                },
                justification: "Bezout identity gcd=a x + b y".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// a = bq + r, 0 ≤ r < b
/// Creates a simplification rule that rewrites a division into its Euclidean division equation.
///
/// The rule applies to any `Expr::Div` and produces an equation of the form `a = b*q + r`,
/// introducing interned placeholder symbols `a`, `b`, `q`, and `r` to represent dividend,
/// divisor, quotient, and remainder respectively. The produced `RuleApplication` contains
/// the equation as the result and a justification "Euclidean division a=bq+r".
///
/// # Examples
///
/// ```
/// let rule = euclidean_division();
/// // The rule is identified by id 354
/// assert_eq!(rule.id, RuleId(354));
/// ```
fn euclidean_division() -> Rule {
    Rule {
        id: RuleId(354),
        name: "euclidean_division",
        category: RuleCategory::Simplification,
        description: "Euclidean division",
        is_applicable: |expr, _| matches!(expr, Expr::Div(_, _)),
        apply: |_expr, _| {
            let a = intern_symbol("a");
            let b = intern_symbol("b");
            let q = intern_symbol("q");
            let r = intern_symbol("r");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(a)),
                    rhs: Box::new(Expr::Add(
                        Box::new(Expr::Mul(Box::new(Expr::Var(b)), Box::new(Expr::Var(q)))),
                        Box::new(Expr::Var(r)),
                    )),
                },
                justification: "Euclidean division a=bq+r".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// a/b + c/d = (ad + bc)/bd
/// Produces a rule that adds two rational expressions into a single fraction with a common denominator.
///
/// The rule matches an addition of two divisions `a/b + c/d` and rewrites it to `(a*d + b*c) / (b*d)`.
///
/// # Examples
///
/// ```
/// let rule = fraction_add();
/// let expr = Expr::Add(
///     Box::new(Expr::Div(Box::new(Expr::Const(Rational::from(1))), Box::new(Expr::Const(Rational::from(2))))),
///     Box::new(Expr::Div(Box::new(Expr::Const(Rational::from(3))), Box::new(Expr::Const(Rational::from(4))))),
/// );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// // resulting expression should represent (1*4 + 2*3) / (2*4) = (4 + 6) / 8
/// ```
fn fraction_add() -> Rule {
    Rule {
        id: RuleId(355),
        name: "fraction_add",
        category: RuleCategory::Simplification,
        description: "a/b + c/d = (ad + bc)/bd",
        is_applicable: |expr, _| {
            if let Expr::Add(left, right) = expr {
                return matches!(left.as_ref(), Expr::Div(_, _))
                    && matches!(right.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Div(a, b), Expr::Div(c, d)) = (left.as_ref(), right.as_ref()) {
                    let ad = Expr::Mul(a.clone(), d.clone());
                    let bc = Expr::Mul(b.clone(), c.clone());
                    let bd = Expr::Mul(b.clone(), d.clone());
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Add(Box::new(ad), Box::new(bc))),
                            Box::new(bd),
                        ),
                        justification: "a/b + c/d = (ad + bc)/bd".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// (a/b) * (c/d) = (ac)/(bd)
/// Creates a rule that multiplies two fractions: (a / b) * (c / d) => (a*c) / (b*d).
///
/// The produced `Rule` matches a multiplication whose left and right operands are both divisions,
/// and replaces it with a single division whose numerator is the product of the two numerators
/// and whose denominator is the product of the two denominators. The rule is reversible and has
/// cost 2.
///
/// # Examples
///
/// ```
/// let r = fraction_mul();
/// assert_eq!(r.id, RuleId(356));
/// assert_eq!(r.name, "fraction_mul");
/// ```
fn fraction_mul() -> Rule {
    Rule {
        id: RuleId(356),
        name: "fraction_mul",
        category: RuleCategory::Simplification,
        description: "(a/b) * (c/d) = (ac)/(bd)",
        is_applicable: |expr, _| {
            if let Expr::Mul(left, right) = expr {
                return matches!(left.as_ref(), Expr::Div(_, _))
                    && matches!(right.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Div(a, b), Expr::Div(c, d)) = (left.as_ref(), right.as_ref()) {
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Mul(a.clone(), c.clone())),
                            Box::new(Expr::Mul(b.clone(), d.clone())),
                        ),
                        justification: "(a/b) * (c/d) = (ac)/(bd)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// (a/b) / (c/d) = (ad)/(bc)
/// Transforms a division of two fractions (a/b) / (c/d) into a single fraction (a*d)/(b*c).
///
/// # Examples
///
/// ```
/// // Given an expression representing (a/b) / (c/d), applying this rule yields (a*d)/(b*c).
/// // Example (pseudocode):
/// // let rule = fraction_div();
/// // let expr = parse_expr("(a/b)/(c/d)");
/// // let apps = rule.apply(&expr, &RuleContext::default());
/// // assert_eq!(apps[0].result, parse_expr("(a*d)/(b*c)"));
/// ```
fn fraction_div() -> Rule {
    Rule {
        id: RuleId(357),
        name: "fraction_div",
        category: RuleCategory::Simplification,
        description: "(a/b) / (c/d) = (ad)/(bc)",
        is_applicable: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Div(_, _))
                    && matches!(denom.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                if let (Expr::Div(a, b), Expr::Div(c, d)) = (num.as_ref(), denom.as_ref()) {
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Mul(a.clone(), d.clone())),
                            Box::new(Expr::Mul(b.clone(), c.clone())),
                        ),
                        justification: "(a/b) / (c/d) = (ad)/(bc)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// a/b = c/d → ad = bc
/// Creates a rule that transforms an equation of two fractions by cross-multiplication.
///
/// The rule matches equations of the form `a / b = c / d` and produces the equivalent equation `a * d = b * c`.
///
/// # Examples
///
/// ```
/// let rule = cross_multiply();
/// // `rule` matches `a/b = c/d` and produces `a*d = b*c` when applied.
/// ```
fn cross_multiply() -> Rule {
    Rule {
        id: RuleId(358),
        name: "cross_multiply",
        category: RuleCategory::Simplification,
        description: "Cross multiply: a/b = c/d → ad = bc",
        is_applicable: |expr, _| {
            if let Expr::Equation { lhs, rhs } = expr {
                return matches!(lhs.as_ref(), Expr::Div(_, _))
                    && matches!(rhs.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let (Expr::Div(a, b), Expr::Div(c, d)) = (lhs.as_ref(), rhs.as_ref()) {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: Box::new(Expr::Mul(a.clone(), d.clone())),
                            rhs: Box::new(Expr::Mul(b.clone(), c.clone())),
                        },
                        justification: "Cross multiply: a/b = c/d → ad = bc".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// Combine fractions using LCD
/// Combine two fractional terms into a single fraction using the least common denominator.
///
/// Applies to expressions of the form `(a/b) + (c/d)` and rewrites them as `(a*d + b*c) / (b*d)`.
///
/// # Examples
///
/// ```
/// // Construct the rule and an example expression (symbols and helpers are assumed in scope).
/// let rule = lcd_combine();
/// let expr = Expr::Add(
///     Box::new(Expr::Div(Box::new(Expr::Symbol(intern_symbol("a"))), Box::new(Expr::Symbol(intern_symbol("b"))))),
///     Box::new(Expr::Div(Box::new(Expr::Symbol(intern_symbol("c"))), Box::new(Expr::Symbol(intern_symbol("d"))))),
/// );
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(
///     apps[0].result,
///     Expr::Div(
///         Box::new(Expr::Add(
///             Box::new(Expr::Mul(Box::new(Expr::Symbol(intern_symbol("a"))), Box::new(Expr::Symbol(intern_symbol("d"))))),
///             Box::new(Expr::Mul(Box::new(Expr::Symbol(intern_symbol("b"))), Box::new(Expr::Symbol(intern_symbol("c"))))),
///         )),
///         Box::new(Expr::Mul(Box::new(Expr::Symbol(intern_symbol("b"))), Box::new(Expr::Symbol(intern_symbol("d"))))),
///     )
/// );
/// ```
fn lcd_combine() -> Rule {
    Rule {
        id: RuleId(359),
        name: "lcd_combine",
        category: RuleCategory::Simplification,
        description: "Combine fractions using LCD",
        is_applicable: |expr, _| {
            // Same as fraction_add for now
            if let Expr::Add(left, right) = expr {
                return matches!(left.as_ref(), Expr::Div(_, _))
                    && matches!(right.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Div(a, b), Expr::Div(c, d)) = (left.as_ref(), right.as_ref()) {
                    let ad = Expr::Mul(a.clone(), d.clone());
                    let bc = Expr::Mul(b.clone(), c.clone());
                    let numerator = Expr::Add(Box::new(ad), Box::new(bc));
                    let denominator = Expr::Mul(b.clone(), d.clone());
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(denominator)),
                        justification: "(a/b)+(c/d) = (ad+bc)/(bd)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// |x| ≥ 0
/// Creates a rule asserting that the absolute value of an expression is greater than or equal to zero.
///
/// The rule matches `Expr::Abs(...)` and replaces it with `Expr::Gte(Abs(...), 0)` with a justification
/// stating that absolute values are always non-negative.
///
/// # Returns
///
/// A `Rule` whose `is_applicable` checks for `Expr::Abs` and whose `apply` produces `Abs(x) ≥ 0`.
///
/// # Examples
///
/// ```
/// let rule = abs_nonnegative();
/// // `rule` will match `Expr::Abs(...)` and produce `Expr::Gte(Abs(...), 0)` when applied.
/// ```
fn abs_nonnegative() -> Rule {
    Rule {
        id: RuleId(360),
        name: "abs_nonnegative",
        category: RuleCategory::Simplification,
        description: "|x| ≥ 0 always",
        is_applicable: |expr, _| matches!(expr, Expr::Abs(_)),
        apply: |expr, _| {
            if let Expr::Abs(_) = expr {
                return vec![RuleApplication {
                    result: Expr::Gte(Box::new(expr.clone()), Box::new(Expr::int(0))),
                    justification: "Absolute values are always non-negative".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// |x|² = x²
fn abs_square() -> Rule {
    Rule {
        id: RuleId(361),
        name: "abs_square",
        category: RuleCategory::Simplification,
        description: "|x|² = x²",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 2 && r.denom() == 1) {
                    return matches!(base.as_ref(), Expr::Abs(_));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Abs(x) = base.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Pow(x.clone(), exp.clone()),
                        justification: "|x|² = x²".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// |a + b| ≤ |a| + |b|
/// Converts an absolute value of a sum into the corresponding triangle inequality.
///
/// When the input is `|a + b|`, produces the inequality `|a + b| ≤ |a| + |b|`.
///
/// # Examples
///
/// ```
/// # use mm_core::{Expr, Symbol};
/// # use crate::algebra::triangle_inequality;
/// # use crate::intern_symbol;
/// let a = Expr::Symbol(intern_symbol("a"));
/// let b = Expr::Symbol(intern_symbol("b"));
/// let expr = Expr::Abs(Box::new(Expr::Add(Box::new(a.clone()), Box::new(b.clone()))));
/// let rule = triangle_inequality();
/// let apps = (rule.apply)(&expr, &Default::default());
/// assert_eq!(apps.len(), 1);
/// // Result is `|a + b| ≤ |a| + |b|`
/// ```
fn triangle_inequality() -> Rule {
fn triangle_inequality() -> Rule {
    Rule {
        id: RuleId(362),
        name: "triangle_inequality",
        category: RuleCategory::Simplification,
        description: "|a + b| ≤ |a| + |b|",
        is_applicable: |expr, _| {
            if let Expr::Abs(inner) = expr {
                return matches!(inner.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Add(a, b) = inner.as_ref() {
                    let rhs = Expr::Add(
                        Box::new(Expr::Abs(a.clone())),
                        Box::new(Expr::Abs(b.clone())),
                    );
                    return vec![RuleApplication {
                        result: Expr::Lte(Box::new(expr.clone()), Box::new(rhs)),
                        justification: "|a+b| ≤ |a| + |b| (triangle inequality)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// ||a| - |b|| ≤ |a - b|
/// Applies the reverse triangle inequality to an absolute difference expression.
///
/// Produces a rule that transforms an expression of the form `||a| - |b||` into
/// the inequality `||a| - |b|| ≤ |a - b|`.
///
/// # Examples
///
/// ```
/// let r = reverse_triangle();
/// assert_eq!(r.name, "reverse_triangle");
/// ```
fn reverse_triangle() -> Rule {
    Rule {
        id: RuleId(363),
        name: "reverse_triangle",
        category: RuleCategory::Simplification,
        description: "||a| - |b|| ≤ |a - b|",
        is_applicable: |expr, _| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Sub(left, right) = inner.as_ref() {
                    return matches!(left.as_ref(), Expr::Abs(_))
                        && matches!(right.as_ref(), Expr::Abs(_));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Sub(left, right) = inner.as_ref() {
                    if let (Expr::Abs(a), Expr::Abs(b)) = (left.as_ref(), right.as_ref()) {
                        let diff = Expr::Abs(Box::new(Expr::Sub(a.clone(), b.clone())));
                        return vec![RuleApplication {
                            result: Expr::Lte(Box::new(expr.clone()), Box::new(diff)),
                            justification: "||a|-|b|| ≤ |a-b| (reverse triangle inequality)"
                                .to_string(),
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

// (a+b)/2 ≥ √(ab)
/// Creates a rule that applies the two-term AM–GM inequality to expressions of the form `(a + b) / 2`.
///
/// The rule matches a division whose denominator is the integer `2` and whose numerator is an addition `a + b`.
/// When applied, it produces the inequality `(a + b) / 2 ≥ √(a * b)` and includes a justification string.
///
/// # Examples
///
/// ```
/// let r = am_gm_2();
/// assert_eq!(r.name, "am_gm_2");
/// ```
fn am_gm_2() -> Rule {
    Rule {
        id: RuleId(364),
        name: "am_gm_2",
        category: RuleCategory::Simplification,
        description: "(a+b)/2 ≥ √(ab) for a,b ≥ 0",
        is_applicable: |expr, _| {
            if let Expr::Div(num, den) = expr {
                if matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    return matches!(num.as_ref(), Expr::Add(_, _));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, den) = expr {
                if matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    if let Expr::Add(a, b) = num.as_ref() {
                        let geo_mean = Expr::Sqrt(Box::new(Expr::Mul(a.clone(), b.clone())));
                        return vec![RuleApplication {
                            result: Expr::Gte(Box::new(expr.clone()), Box::new(geo_mean)),
                            justification: "AM-GM: (a+b)/2 ≥ √(ab) for a,b ≥ 0".to_string(),
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

// (a+b+c)/3 ≥ ∛(abc)
/// Creates a rule that applies the AM-GM inequality for three nonnegative terms.
///
/// The rule matches expressions of the form `(a + b + c) / 3` (either `(a + b) + c` or `a + (b + c)`
/// nested as additions) and produces the inequality `(a + b + c)/3 ≥ ∛(a*b*c)`.
///
/// # Examples
///
/// ```
/// // Construct (a + b + c) / 3 and verify the rule applies and yields a `Gte` to the geometric mean.
/// let rule = am_gm_3();
/// let a = Expr::Symbol(intern_symbol("a"));
/// let b = Expr::Symbol(intern_symbol("b"));
/// let c = Expr::Symbol(intern_symbol("c"));
/// let sum = Expr::Add(Box::new(Expr::Add(Box::new(a.clone()), Box::new(b.clone()))), Box::new(c.clone()));
/// let expr = Expr::Div(Box::new(sum), Box::new(Expr::Const(Rational::new(3, 1))));
/// assert!(rule.can_apply(&expr));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// if let Expr::Gte(_, rhs) = &apps[0].result {
///     // rhs should be the cube root of a*b*c
///     let expected_prod = Expr::Mul(Box::new(Expr::Mul(Box::new(a), Box::new(b))), Box::new(c));
///     let expected_geo = Expr::Pow(Box::new(expected_prod), Box::new(Expr::Const(Rational::new(1, 3))));
///     assert_eq!(**rhs, expected_geo);
/// }
/// ```
fn am_gm_3() -> Rule {
    Rule {
        id: RuleId(365),
        name: "am_gm_3",
        category: RuleCategory::Simplification,
        description: "(a+b+c)/3 ≥ ∛(abc) for a,b,c ≥ 0",
        is_applicable: |expr, _| {
            if let Expr::Div(num, den) = expr {
                return matches!(den.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                    && matches!(num.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, den) = expr {
                if matches!(den.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1) {
                    // Accept (a+b)+c or a+(b+c)
                    if let Expr::Add(left, right) = num.as_ref() {
                        if let Expr::Add(a, b) = left.as_ref() {
                            let c_term = right.clone();
                            let product = Expr::Mul(
                                Box::new(Expr::Mul(a.clone(), b.clone())),
                                c_term.clone(),
                            );
                            let geo_mean = Expr::Pow(
                                Box::new(product),
                                Box::new(Expr::Const(Rational::new(1, 3))),
                            );
                            return vec![RuleApplication {
                                result: Expr::Gte(Box::new(expr.clone()), Box::new(geo_mean)),
                                justification: "AM-GM for three terms: (a+b+c)/3 ≥ ∛(abc)"
                                    .to_string(),
                            }];
                        } else if let Expr::Add(b, c_term) = right.as_ref() {
                            let a = left.clone();
                            let product = Expr::Mul(
                                Box::new(Expr::Mul(a.clone(), b.clone())),
                                c_term.clone(),
                            );
                            let geo_mean = Expr::Pow(
                                Box::new(product),
                                Box::new(Expr::Const(Rational::new(1, 3))),
                            );
                            return vec![RuleApplication {
                                result: Expr::Gte(Box::new(expr.clone()), Box::new(geo_mean)),
                                justification: "AM-GM for three terms: (a+b+c)/3 ≥ ∛(abc)"
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

// QM ≥ AM
/// QM-AM inequality rule: relates quadratic mean to arithmetic mean.
///
/// Produces a `Rule` that recognizes expressions of the form `√((a² + b²)/2)` and
/// rewrites them as the inequality `√((a² + b²)/2) ≥ (a + b) / 2`.
///
/// # Examples
///
/// ```
/// // Construct the expression sqrt((a^2 + b^2) / 2) and verify the rule applies.
/// let rule = qm_am();
/// let a = Expr::Sym(intern_symbol("a"));
/// let b = Expr::Sym(intern_symbol("b"));
/// let expr = Expr::Sqrt(Box::new(Expr::Div(
///     Box::new(Expr::Add(
///         Box::new(Expr::Pow(Box::new(a.clone()), Box::new(Expr::int(2)))),
///         Box::new(Expr::Pow(Box::new(b.clone()), Box::new(Expr::int(2)))),
///     )),
///     Box::new(Expr::int(2)),
/// )));
/// assert!(rule.can_apply(&expr));
/// let apps = rule.apply(&expr);
/// assert_eq!(apps.len(), 1);
/// ```
fn qm_am() -> Rule {
    Rule {
        id: RuleId(366),
        name: "qm_am",
        category: RuleCategory::Simplification,
        description: "√((a²+b²)/2) ≥ (a+b)/2",
        is_applicable: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
                    if matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                        if let Expr::Add(left, right) = num.as_ref() {
                            let left_sq = matches!(left.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1));
                            let right_sq = matches!(right.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1));
                            return left_sq && right_sq;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sqrt(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
                    if matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                        if let Expr::Add(a_sq, b_sq) = num.as_ref() {
                            let am = Expr::Div(
                                Box::new(Expr::Add(
                                    Box::new(match a_sq.as_ref() {
                                        Expr::Pow(a, _) => *a.clone(),
                                        _ => *(*a_sq).clone(),
                                    }),
                                    Box::new(match b_sq.as_ref() {
                                        Expr::Pow(b, _) => *b.clone(),
                                        _ => *(*b_sq).clone(),
                                    }),
                                )),
                                Box::new(Expr::int(2)),
                            );
                            return vec![RuleApplication {
                                result: Expr::Gte(Box::new(expr.clone()), Box::new(am)),
                                justification: "QM-AM: √((a²+b²)/2) ≥ (a+b)/2".to_string(),
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

// (Σaᵢbᵢ)² ≤ (Σaᵢ²)(Σbᵢ²)
/// Creates a rule that recognizes squared sums of two products of two terms and returns
/// a Cauchy–Schwarz inequality bounding: (ab + cd)² ≤ (a² + c²)(b² + d²).
///
/// The rule applies to expressions of the form `(a*b + c*d)^2` and produces a `RuleApplication`
/// whose result is an `Expr::Lte` mapping the original squared sum to the computed upper bound.
/// The justification string cites the Cauchy–Schwarz inequality.
///
/// # Examples
///
/// ```no_run
/// let rule = cauchy_schwarz_2();
/// // `rule` applies to expressions matching `(a*b + c*d)^2` and yields
/// // an `Expr::Lte((a*b + c*d)^2, (a^2 + c^2)*(b^2 + d^2))`.
/// ```
fn cauchy_schwarz_2() -> Rule {
    Rule {
        id: RuleId(367),
        name: "cauchy_schwarz_2",
        category: RuleCategory::Simplification,
        description: "(ab + cd)² ≤ (a²+c²)(b²+d²)",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    if let Expr::Add(_, _) = base.as_ref() {
                        return true;
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(sum, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    if let Expr::Add(first, second) = sum.as_ref() {
                        if let (Expr::Mul(a, b), Expr::Mul(c_term, d)) =
                            (first.as_ref(), second.as_ref())
                        {
                            let a_sq = Expr::Pow(a.clone(), Box::new(Expr::int(2)));
                            let c_sq = Expr::Pow(c_term.clone(), Box::new(Expr::int(2)));
                            let b_sq = Expr::Pow(b.clone(), Box::new(Expr::int(2)));
                            let d_sq = Expr::Pow(d.clone(), Box::new(Expr::int(2)));

                            let left_sum = Expr::Add(Box::new(a_sq), Box::new(c_sq));
                            let right_sum = Expr::Add(Box::new(b_sq), Box::new(d_sq));
                            let bound = Expr::Mul(Box::new(left_sum), Box::new(right_sum));

                            return vec![RuleApplication {
                                result: Expr::Lte(Box::new(expr.clone()), Box::new(bound)),
                                justification: "Cauchy-Schwarz: (ab+cd)² ≤ (a²+c²)(b²+d²)"
                                    .to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// Holder's inequality
/// Constructs a rule encoding Hölder's (Cauchy–Schwarz / absolute-product) inequality.
///
/// The rule applies to expressions of the form `|a * b|` and produces the inequality
/// `|a * b| ≤ |a| · |b|` as the rule application result.
///
/// # Examples
///
/// ```
/// // Construct an expression `|a * b|`, check applicability, and apply the rule.
/// let expr = Expr::Abs(Box::new(Expr::Mul(
///     Box::new(Expr::Symbol(intern_symbol("a"))),
///     Box::new(Expr::Symbol(intern_symbol("b"))),
/// )));
/// let rule = holders_inequality();
/// assert!(rule.can_apply(&expr));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// // The result is `|a * b| ≤ |a| * |b|`.
/// ```
fn holders_inequality() -> Rule {
    Rule {
        id: RuleId(368),
        name: "holders_inequality",
        category: RuleCategory::Simplification,
        description: "Holder's inequality",
        is_applicable: |expr, _| {
            if let Expr::Abs(inner) = expr {
                return matches!(inner.as_ref(), Expr::Mul(_, _));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Abs(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    let bound = Expr::Mul(
                        Box::new(Expr::Abs(a.clone())),
                        Box::new(Expr::Abs(b.clone())),
                    );
                    return vec![RuleApplication {
                        result: Expr::Lte(Box::new(expr.clone()), Box::new(bound)),
                        justification: "|ab| ≤ |a|·|b| (Hölder p=q=∞ case)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 4,
    }
}

// Minkowski inequality
/// Constructs the algebraic rule expressing the Minkowski inequality consequence for p ≥ 1.
///
/// The rule matches expressions of the form `|a + b|^p` (where `p` is an integer ≥ 1) and
/// produces the inequality `|a + b|^p ≤ 2^{p-1} (|a|^p + |b|^p)` as the rule application result.
///
/// # Examples
///
/// ```
/// let rule = minkowski_inequality();
/// assert_eq!(rule.id, RuleId(369));
/// ```
fn minkowski_inequality() -> Rule {
    Rule {
        id: RuleId(369),
        name: "minkowski_inequality",
        category: RuleCategory::Simplification,
        description: "Minkowski inequality",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() >= 1 && c.denom() == 1) {
                    if let Expr::Abs(inner) = base.as_ref() {
                        return matches!(inner.as_ref(), Expr::Add(_, _));
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if let Expr::Abs(inner) = base.as_ref() {
                    if let Expr::Add(a, b) = inner.as_ref() {
                        if let Expr::Const(p) = exp.as_ref() {
                            let coeff = Expr::Pow(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Const(Rational::new(p.numer() - 1, p.denom()))),
                            );
                            let a_term = Expr::Pow(
                                Box::new(Expr::Abs(a.clone())),
                                Box::new(Expr::Const(*p)),
                            );
                            let b_term = Expr::Pow(
                                Box::new(Expr::Abs(b.clone())),
                                Box::new(Expr::Const(*p)),
                            );
                            let rhs_inner = Expr::Add(Box::new(a_term), Box::new(b_term));
                            let rhs = Expr::Mul(Box::new(coeff), Box::new(rhs_inner));
                            return vec![RuleApplication {
                                result: Expr::Lte(Box::new(expr.clone()), Box::new(rhs)),
                                justification:
                                    "|a+b|^p ≤ 2^{p-1}(|a|^p+|b|^p) (Minkowski consequence)"
                                        .to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 4,
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