// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Equation solving rules and transformations.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational, Symbol};

/// Get all equation solving rules.
pub fn equation_rules() -> Vec<Rule> {
    vec![
        isolate_variable(),
        cancel_addition(),
        cancel_subtraction(),
        cancel_multiplication(),
        cancel_division(),
        linear_solve(),
        quadratic_formula(),
    ]
}

// ============================================================================
// Rule 21: Isolate Variable (move terms)
// ============================================================================

fn isolate_variable() -> Rule {
    Rule {
        id: RuleId(21),
        name: "isolate_variable",
        category: RuleCategory::EquationSolving,
        description: "Move terms to isolate variable: ax + b = c → ax = c - b",
        is_applicable: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs: _ } = expr {
                // Check if LHS has addition/subtraction with constant
                if let Expr::Add(_, b) = lhs.as_ref() {
                    return matches!(b.as_ref(), Expr::Const(_));
                }
                if let Expr::Sub(_, b) = lhs.as_ref() {
                    return matches!(b.as_ref(), Expr::Const(_));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                // ax + b = c → ax = c - b
                if let Expr::Add(a, b) = lhs.as_ref() {
                    if matches!(b.as_ref(), Expr::Const(_)) {
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: a.clone(),
                                rhs: Box::new(Expr::Sub(rhs.clone(), b.clone())),
                            },
                            justification: "Subtract constant from both sides".to_string(),
                        }];
                    }
                }
                // ax - b = c → ax = c + b
                if let Expr::Sub(a, b) = lhs.as_ref() {
                    if matches!(b.as_ref(), Expr::Const(_)) {
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: a.clone(),
                                rhs: Box::new(Expr::Add(rhs.clone(), b.clone())),
                            },
                            justification: "Add constant to both sides".to_string(),
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
// Rule 22: Cancel Addition
// ============================================================================

fn cancel_addition() -> Rule {
    Rule {
        id: RuleId(22),
        name: "cancel_addition",
        category: RuleCategory::EquationSolving,
        description: "Cancel addition: x + a = b → x = b - a",
        is_applicable: |expr, _ctx| {
            if let Expr::Equation { lhs, .. } = expr {
                return matches!(lhs.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Add(x, a) = lhs.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: x.clone(),
                            rhs: Box::new(Expr::Sub(rhs.clone(), a.clone())),
                        },
                        justification: "x + a = b → x = b - a".to_string(),
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
// Rule 23: Cancel Subtraction
// ============================================================================

fn cancel_subtraction() -> Rule {
    Rule {
        id: RuleId(23),
        name: "cancel_subtraction",
        category: RuleCategory::EquationSolving,
        description: "Cancel subtraction: x - a = b → x = b + a",
        is_applicable: |expr, _ctx| {
            if let Expr::Equation { lhs, .. } = expr {
                return matches!(lhs.as_ref(), Expr::Sub(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Sub(x, a) = lhs.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: x.clone(),
                            rhs: Box::new(Expr::Add(rhs.clone(), a.clone())),
                        },
                        justification: "x - a = b → x = b + a".to_string(),
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
// Rule 24: Cancel Multiplication
// ============================================================================

fn cancel_multiplication() -> Rule {
    Rule {
        id: RuleId(24),
        name: "cancel_multiplication",
        category: RuleCategory::EquationSolving,
        description: "Cancel multiplication: ax = b -> x = b/a (or xa = b -> x = b/a)",
        is_applicable: |expr, _ctx| {
            if let Expr::Equation { lhs, .. } = expr {
                if let Expr::Mul(a, b) = lhs.as_ref() {
                    // Check if either operand is a non-zero constant
                    if matches!(a.as_ref(), Expr::Const(c) if !c.is_zero()) {
                        return true;
                    }
                    if matches!(b.as_ref(), Expr::Const(c) if !c.is_zero()) {
                        return true;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Mul(a, b) = lhs.as_ref() {
                    // Case 1: c * x = rhs -> x = rhs / c
                    if let Expr::Const(c) = a.as_ref() {
                        if !c.is_zero() {
                            return vec![RuleApplication {
                                result: Expr::Equation {
                                    lhs: b.clone(),
                                    rhs: Box::new(Expr::Div(rhs.clone(), a.clone())),
                                },
                                justification: format!("Divide both sides by {}", c),
                            }];
                        }
                    }
                    // Case 2: x * c = rhs -> x = rhs / c
                    if let Expr::Const(c) = b.as_ref() {
                        if !c.is_zero() {
                            return vec![RuleApplication {
                                result: Expr::Equation {
                                    lhs: a.clone(),
                                    rhs: Box::new(Expr::Div(rhs.clone(), b.clone())),
                                },
                                justification: format!("Divide both sides by {}", c),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// ============================================================================
// Rule 25: Cancel Division
// ============================================================================

fn cancel_division() -> Rule {
    Rule {
        id: RuleId(25),
        name: "cancel_division",
        category: RuleCategory::EquationSolving,
        description: "Cancel division: x/a = b → x = ab",
        is_applicable: |expr, _ctx| {
            if let Expr::Equation { lhs, .. } = expr {
                if let Expr::Div(_, a) = lhs.as_ref() {
                    return matches!(a.as_ref(), Expr::Const(c) if !c.is_zero());
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Div(x, a) = lhs.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: x.clone(),
                            rhs: Box::new(Expr::Mul(rhs.clone(), a.clone())),
                        },
                        justification: "x/a = b → x = ab".to_string(),
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
// Rule 26: Linear Solve (ax + b = c → x = (c-b)/a)
// ============================================================================

fn linear_solve() -> Rule {
    Rule {
        id: RuleId(26),
        name: "linear_solve",
        category: RuleCategory::EquationSolving,
        description: "Solve linear equation: ax + b = c → x = (c-b)/a",
        is_applicable: |expr, _ctx| {
            // Check for ax + b = c pattern
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Add(term, b) = lhs.as_ref() {
                    if matches!(b.as_ref(), Expr::Const(_)) {
                        if let Expr::Mul(a, _x) = term.as_ref() {
                            return matches!(a.as_ref(), Expr::Const(_))
                                && matches!(rhs.as_ref(), Expr::Const(_));
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if let Expr::Add(term, b) = lhs.as_ref() {
                    if let (Expr::Mul(a, x), Expr::Const(b_val), Expr::Const(c_val)) =
                        (term.as_ref(), b.as_ref(), rhs.as_ref())
                    {
                        if let Expr::Const(a_val) = a.as_ref() {
                            if !a_val.is_zero() {
                                let numerator = *c_val - *b_val;
                                let solution = numerator / *a_val;
                                return vec![RuleApplication {
                                    result: Expr::Equation {
                                        lhs: x.clone(),
                                        rhs: Box::new(Expr::Const(solution)),
                                    },
                                    justification: format!(
                                        "x = ({} - {}) / {} = {}",
                                        c_val, b_val, a_val, solution
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
        cost: 1,
    }
}

// ============================================================================
// Rule 27: Quadratic Formula
// ============================================================================

fn quadratic_formula() -> Rule {
    Rule {
        id: RuleId(27),
        name: "quadratic_formula",
        category: RuleCategory::EquationSolving,
        description: "Quadratic formula: ax² + bx + c = 0 → x = (-b ± √(b²-4ac))/(2a)",
        is_applicable: |expr, _ctx| {
            // Check for ax^2 + bx + c = 0 pattern
            if let Expr::Equation { rhs, .. } = expr {
                return rhs.is_zero();
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Equation { lhs, rhs } = expr {
                if rhs.is_zero() {
                    // Try to extract a, b, c coefficients
                    if let Some((a, b, c, var)) = extract_quadratic_coefficients(lhs) {
                        let discriminant = b * b - Rational::from_integer(4) * a * c;

                        if discriminant.is_negative() {
                            return vec![RuleApplication {
                                result: expr.clone(),
                                justification: "No real solutions (discriminant < 0)".to_string(),
                            }];
                        }

                        let two_a = Rational::from_integer(2) * a;

                        // x = (-b ± √discriminant) / 2a
                        // For simplicity, return the formula form
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: Box::new(Expr::Var(var)),
                                rhs: Box::new(Expr::Div(
                                    Box::new(Expr::Add(
                                        Box::new(Expr::Neg(Box::new(Expr::Const(b)))),
                                        Box::new(Expr::Sqrt(Box::new(Expr::Const(discriminant)))),
                                    )),
                                    Box::new(Expr::Const(two_a)),
                                )),
                            },
                            justification: format!(
                                "Quadratic formula: x = (-{} + √{}) / {}",
                                b, discriminant, two_a
                            ),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 5,
    }
}

/// Extract quadratic coefficients from ax² + bx + c expression.
fn extract_quadratic_coefficients(expr: &Expr) -> Option<(Rational, Rational, Rational, Symbol)> {
    // This is a simplified implementation that handles basic cases
    // Full implementation would need term collection

    match expr {
        // ax² + bx + c pattern via Add
        Expr::Add(left, right) => {
            // Try to find x² term
            if let Some((a, var)) = extract_squared_term(left) {
                // right might be bx + c or just c
                let (b, c) = extract_linear_and_const(right, var);
                return Some((a, b, c, var));
            }
            if let Some((a, var)) = extract_squared_term(right) {
                let (b, c) = extract_linear_and_const(left, var);
                return Some((a, b, c, var));
            }
        }
        // Just ax² = 0
        Expr::Mul(_, _) | Expr::Pow(_, _) => {
            if let Some((a, var)) = extract_squared_term(expr) {
                return Some((a, Rational::from_integer(0), Rational::from_integer(0), var));
            }
        }
        _ => {}
    }

    None
}

fn extract_squared_term(expr: &Expr) -> Option<(Rational, Symbol)> {
    match expr {
        // x²
        Expr::Pow(base, exp) => {
            if let (Expr::Var(v), Expr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                if *e == Rational::from_integer(2) {
                    return Some((Rational::from_integer(1), *v));
                }
            }
        }
        // a * x²
        Expr::Mul(a, b) => {
            if let Expr::Const(coeff) = a.as_ref() {
                if let Some((_, var)) = extract_squared_term(b) {
                    return Some((*coeff, var));
                }
            }
            if let Expr::Const(coeff) = b.as_ref() {
                if let Some((_, var)) = extract_squared_term(a) {
                    return Some((*coeff, var));
                }
            }
        }
        _ => {}
    }
    None
}

fn extract_linear_and_const(expr: &Expr, _var: Symbol) -> (Rational, Rational) {
    match expr {
        Expr::Const(c) => (Rational::from_integer(0), *c),
        Expr::Add(a, b) => {
            let (b1, c1) = extract_linear_and_const(a, _var);
            let (b2, c2) = extract_linear_and_const(b, _var);
            (b1 + b2, c1 + c2)
        }
        Expr::Mul(a, b) => {
            // bx term
            if let Expr::Const(coeff) = a.as_ref() {
                if matches!(b.as_ref(), Expr::Var(_)) {
                    return (*coeff, Rational::from_integer(0));
                }
            }
            if let Expr::Const(coeff) = b.as_ref() {
                if matches!(a.as_ref(), Expr::Var(_)) {
                    return (*coeff, Rational::from_integer(0));
                }
            }
            (Rational::from_integer(0), Rational::from_integer(0))
        }
        Expr::Var(_) => (Rational::from_integer(1), Rational::from_integer(0)),
        _ => (Rational::from_integer(0), Rational::from_integer(0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_cancel_addition() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = cancel_addition();
        let ctx = RuleContext::default();

        // x + 3 = 7
        let expr = Expr::Equation {
            lhs: Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            rhs: Box::new(Expr::int(7)),
        };

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        // Result should be x = 7 - 3
    }

    #[test]
    fn test_cancel_multiplication() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = cancel_multiplication();
        let ctx = RuleContext::default();

        // 2x = 6
        let expr = Expr::Equation {
            lhs: Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
            rhs: Box::new(Expr::int(6)),
        };

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        // Result should be x = 6 / 2
    }

    #[test]
    fn test_linear_solve() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = linear_solve();
        let ctx = RuleContext::default();

        // 2x + 3 = 7
        let expr = Expr::Equation {
            lhs: Box::new(Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
                Box::new(Expr::int(3)),
            )),
            rhs: Box::new(Expr::int(7)),
        };

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        // Result should be x = 2
    }
}
