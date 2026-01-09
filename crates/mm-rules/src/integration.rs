// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Integration transformation rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all integration rules.
pub fn integration_rules() -> Vec<Rule> {
    vec![
        power_integral(),
        constant_integral(),
        sum_integral(),
        difference_integral(),
        sin_integral(),
        cos_integral(),
        exp_integral(),
        one_over_x_integral(),
        constant_multiple_integral(),
    ]
}

// ============================================================================
// Helper: Check if expression contains variable
// ============================================================================

fn contains_var(expr: &Expr, var: mm_core::Symbol) -> bool {
    match expr {
        Expr::Var(v) => *v == var,
        Expr::Const(_) | Expr::Pi | Expr::E => false,
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => {
            contains_var(a, var) || contains_var(b, var)
        }
        Expr::Neg(a)
        | Expr::Sin(a)
        | Expr::Cos(a)
        | Expr::Tan(a)
        | Expr::Exp(a)
        | Expr::Ln(a)
        | Expr::Sqrt(a)
        | Expr::Abs(a) => contains_var(a, var),
        Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => contains_var(expr, var),
        _ => false,
    }
}

// ============================================================================
// Rule 30: Power Rule for Integration: integral(x^n dx) = x^(n+1)/(n+1)
// ============================================================================

fn power_integral() -> Rule {
    Rule {
        id: RuleId(30),
        name: "power_integral",
        category: RuleCategory::Integral,
        description: "Power rule: integral(x^n dx) = x^(n+1)/(n+1) for n != -1",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Case 1: Just x
                if let Expr::Var(v) = inner.as_ref() {
                    return v == var;
                }
                // Case 2: x^n where n is constant and n != -1
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        return v == var && *n != Rational::from_integer(-1);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // integral(x dx) = x^2/2
                if let Expr::Var(v) = inner.as_ref() {
                    if v == var {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Pow(
                                    Box::new(Expr::Var(*var)),
                                    Box::new(Expr::int(2)),
                                )),
                                Box::new(Expr::int(2)),
                            ),
                            justification: "integral(x dx) = x^2/2".to_string(),
                        }];
                    }
                }
                // integral(x^n dx) = x^(n+1)/(n+1)
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        if v == var && *n != Rational::from_integer(-1) {
                            let n_plus_1 = *n + Rational::from_integer(1);
                            return vec![RuleApplication {
                                result: Expr::Div(
                                    Box::new(Expr::Pow(
                                        base.clone(),
                                        Box::new(Expr::Const(n_plus_1)),
                                    )),
                                    Box::new(Expr::Const(n_plus_1)),
                                ),
                                justification: format!(
                                    "integral(x^{} dx) = x^{}/{}",
                                    n, n_plus_1, n_plus_1
                                ),
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

// ============================================================================
// Rule 31: Constant Rule: integral(c dx) = cx
// ============================================================================

fn constant_integral() -> Rule {
    Rule {
        id: RuleId(31),
        name: "constant_integral",
        category: RuleCategory::Integral,
        description: "Constant rule: integral(c dx) = cx",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                return !contains_var(inner, *var);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if !contains_var(inner, *var) {
                    return vec![RuleApplication {
                        result: Expr::Mul(inner.clone(), Box::new(Expr::Var(*var))),
                        justification: "integral(c dx) = cx".to_string(),
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
// Rule 32: Sum Rule: integral(f + g dx) = integral(f dx) + integral(g dx)
// ============================================================================

fn sum_integral() -> Rule {
    Rule {
        id: RuleId(32),
        name: "sum_integral",
        category: RuleCategory::Integral,
        description: "Sum rule: integral(f + g dx) = integral(f dx) + integral(g dx)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, .. } = expr {
                return matches!(inner.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Add(f, g) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Integral {
                                expr: f.clone(),
                                var: *var,
                            }),
                            Box::new(Expr::Integral {
                                expr: g.clone(),
                                var: *var,
                            }),
                        ),
                        justification: "integral(f + g) = integral(f) + integral(g)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}
// ============================================================================
// Rule 33: Difference Rule: integral(f - g dx) = integral(f dx) - integral(g dx)
// ============================================================================

fn difference_integral() -> Rule {
    Rule {
        id: RuleId(33),
        name: "difference_integral",
        category: RuleCategory::Integral,
        description: "Difference rule: integral(f - g dx) = integral(f dx) - integral(g dx)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, .. } = expr {
                return matches!(inner.as_ref(), Expr::Sub(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Sub(f, g) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sub(
                            Box::new(Expr::Integral {
                                expr: f.clone(),
                                var: *var,
                            }),
                            Box::new(Expr::Integral {
                                expr: g.clone(),
                                var: *var,
                            }),
                        ),
                        justification: "integral(f - g) = integral(f) - integral(g)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// ============================================================================
// Rule 34: Sine Integral: integral(sin(x) dx) = -cos(x)
// ============================================================================

fn sin_integral() -> Rule {
    Rule {
        id: RuleId(34),
        name: "sin_integral",
        category: RuleCategory::Integral,
        description: "Sine integral: integral(sin(x) dx) = -cos(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Sin(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        return v == var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Sin(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: Expr::Neg(Box::new(Expr::Cos(arg.clone()))),
                                justification: "integral(sin(x) dx) = -cos(x)".to_string(),
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

// ============================================================================
// Rule 35: Cosine Integral: integral(cos(x) dx) = sin(x)
// ============================================================================

fn cos_integral() -> Rule {
    Rule {
        id: RuleId(35),
        name: "cos_integral",
        category: RuleCategory::Integral,
        description: "Cosine integral: integral(cos(x) dx) = sin(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Cos(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        return v == var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Cos(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: Expr::Sin(arg.clone()),
                                justification: "integral(cos(x) dx) = sin(x)".to_string(),
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

// ============================================================================
// Rule 36: Exponential Integral: integral(e^x dx) = e^x
// ============================================================================

fn exp_integral() -> Rule {
    Rule {
        id: RuleId(36),
        name: "exp_integral",
        category: RuleCategory::Integral,
        description: "Exponential integral: integral(e^x dx) = e^x",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Exp(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        return v == var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Exp(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: inner.clone().as_ref().clone(),
                                justification: "integral(e^x dx) = e^x".to_string(),
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

// ============================================================================
// Rule 37: Reciprocal Integral: integral(1/x dx) = ln|x|
// ============================================================================

fn one_over_x_integral() -> Rule {
    Rule {
        id: RuleId(37),
        name: "one_over_x_integral",
        category: RuleCategory::Integral,
        description: "Reciprocal integral: integral(1/x dx) = ln(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Check for 1/x or x^(-1)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Var(v) = denom.as_ref() {
                            return v == var;
                        }
                    }
                }
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        return v == var && *n == Rational::from_integer(-1);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Var(v) = denom.as_ref() {
                            if v == var {
                                return vec![RuleApplication {
                                    result: Expr::Ln(Box::new(Expr::Abs(Box::new(Expr::Var(
                                        *var,
                                    ))))),
                                    justification: "integral(1/x dx) = ln|x|".to_string(),
                                }];
                            }
                        }
                    }
                }
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        if v == var && *n == Rational::from_integer(-1) {
                            return vec![RuleApplication {
                                result: Expr::Ln(Box::new(Expr::Abs(Box::new(Expr::Var(*var))))),
                                justification: "integral(x^(-1) dx) = ln|x|".to_string(),
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

// ============================================================================
// Rule 38: Constant Multiple: integral(c*f dx) = c * integral(f dx)
// ============================================================================

fn constant_multiple_integral() -> Rule {
    Rule {
        id: RuleId(38),
        name: "constant_multiple_integral",
        category: RuleCategory::Integral,
        description: "Constant multiple: integral(c*f dx) = c * integral(f dx)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    // Check if first factor is constant
                    if !contains_var(a, *var) && contains_var(b, *var) {
                        return true;
                    }
                    // Check if second factor is constant
                    if contains_var(a, *var) && !contains_var(b, *var) {
                        return true;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    if !contains_var(a, *var) && contains_var(b, *var) {
                        // c * f -> c * integral(f)
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                a.clone(),
                                Box::new(Expr::Integral {
                                    expr: b.clone(),
                                    var: *var,
                                }),
                            ),
                            justification: "integral(c*f dx) = c * integral(f dx)".to_string(),
                        }];
                    }
                    if contains_var(a, *var) && !contains_var(b, *var) {
                        // f * c -> c * integral(f)
                        return vec![RuleApplication {
                            result: Expr::Mul(
                                b.clone(),
                                Box::new(Expr::Integral {
                                    expr: a.clone(),
                                    var: *var,
                                }),
                            ),
                            justification: "integral(f*c dx) = c * integral(f dx)".to_string(),
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
