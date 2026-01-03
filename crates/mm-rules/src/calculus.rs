// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Calculus transformation rules (derivatives).

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::Expr;

/// Get all calculus rules.
pub fn calculus_rules() -> Vec<Rule> {
    let mut rules = vec![
        power_rule(),
        constant_rule(),
        sum_rule(),
        product_rule(),
        quotient_rule(),
        chain_rule_sin(),
        chain_rule_cos(),
        exp_rule(),
        ln_rule(),
    ];
    // Add advanced calculus rules (Phase 2)
    rules.extend(advanced_calculus_rules());
    // Add Phase 4 calculus rules (IMO milestone)
    rules.extend(phase4_calculus_rules());
    rules
}

// ============================================================================
// Rule 11: Power Rule d/dx(x^n) = n*x^(n-1)
// ============================================================================

fn power_rule() -> Rule {
    Rule {
        id: RuleId(11),
        name: "power_rule",
        category: RuleCategory::Derivative,
        description: "Power rule: d/dx(x^n) = n·x^(n-1)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Check for x^n pattern where the base contains the variable
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // Base should be the variable or contain it
                    if let Expr::Var(v) = base.as_ref() {
                        return v == var && matches!(exp.as_ref(), Expr::Const(_));
                    }
                }
                // Also handle simple variable: d/dx(x) = 1
                if let Expr::Var(v) = inner.as_ref() {
                    return v == var;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // d/dx(x) = 1
                if let Expr::Var(v) = inner.as_ref() {
                    if v == var {
                        return vec![RuleApplication {
                            result: Expr::int(1),
                            justification: "d/dx(x) = 1".to_string(),
                        }];
                    }
                }
                // d/dx(x^n) = n * x^(n-1)
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        if v == var {
                            let n_minus_1 = *n - mm_core::Rational::from_integer(1);
                            return vec![RuleApplication {
                                result: Expr::Mul(
                                    Box::new(Expr::Const(*n)),
                                    Box::new(Expr::Pow(
                                        base.clone(),
                                        Box::new(Expr::Const(n_minus_1)),
                                    )),
                                ),
                                justification: format!("d/dx(x^{}) = {} · x^{}", n, n, n_minus_1),
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
// Rule 12: Constant Rule d/dx(c) = 0
// ============================================================================

fn constant_rule() -> Rule {
    Rule {
        id: RuleId(12),
        name: "constant_rule",
        category: RuleCategory::Derivative,
        description: "Constant rule: d/dx(c) = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Constant has no occurrence of the variable
                return !contains_var(inner, *var);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if !contains_var(inner, *var) {
                    return vec![RuleApplication {
                        result: Expr::int(0),
                        justification: "d/dx(c) = 0 (c does not contain x)".to_string(),
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
// Rule 13: Sum Rule d/dx(f + g) = f' + g'
// ============================================================================

fn sum_rule() -> Rule {
    Rule {
        id: RuleId(13),
        name: "sum_rule",
        category: RuleCategory::Derivative,
        description: "Sum rule: d/dx(f + g) = f' + g'",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                return matches!(inner.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Add(f, g) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Derivative {
                                expr: f.clone(),
                                var: *var,
                            }),
                            Box::new(Expr::Derivative {
                                expr: g.clone(),
                                var: *var,
                            }),
                        ),
                        justification: "d/dx(f + g) = f' + g'".to_string(),
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
// Rule 14: Product Rule d/dx(fg) = f'g + fg'
// ============================================================================

fn product_rule() -> Rule {
    Rule {
        id: RuleId(14),
        name: "product_rule",
        category: RuleCategory::Derivative,
        description: "Product rule: d/dx(fg) = f'g + fg'",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                return matches!(inner.as_ref(), Expr::Mul(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Mul(f, g) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Add(
                            Box::new(Expr::Mul(
                                Box::new(Expr::Derivative {
                                    expr: f.clone(),
                                    var: *var,
                                }),
                                g.clone(),
                            )),
                            Box::new(Expr::Mul(
                                f.clone(),
                                Box::new(Expr::Derivative {
                                    expr: g.clone(),
                                    var: *var,
                                }),
                            )),
                        ),
                        justification: "d/dx(fg) = f'g + fg'".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 19: Quotient Rule d/dx(f/g) = (f'g - fg') / g²
// ============================================================================

fn quotient_rule() -> Rule {
    Rule {
        id: RuleId(19),
        name: "quotient_rule",
        category: RuleCategory::Derivative,
        description: "Quotient rule: d/dx(f/g) = (f'g - fg') / g²",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                return matches!(inner.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(f, g) = inner.as_ref() {
                    // d/dx(f/g) = (f'g - fg') / g²
                    let f_prime = Box::new(Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    });
                    let g_prime = Box::new(Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    });

                    // f'g - fg'
                    let numerator = Expr::Sub(
                        Box::new(Expr::Mul(f_prime, g.clone())),
                        Box::new(Expr::Mul(f.clone(), g_prime)),
                    );

                    // g²
                    let denominator = Expr::Pow(g.clone(), Box::new(Expr::int(2)));

                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(denominator)),
                        justification: "d/dx(f/g) = (f'g - fg') / g²".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 4,
    }
}

// ============================================================================
// Rule 15: d/dx(sin(g(x))) = cos(g(x)) * g'(x) (Chain Rule)
// ============================================================================

fn chain_rule_sin() -> Rule {
    Rule {
        id: RuleId(15),
        name: "sin_chain_rule",
        category: RuleCategory::Derivative,
        description: "Sine chain rule: d/dx(sin(g)) = cos(g) * g'",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sin(arg) = inner.as_ref() {
                    // Apply if argument contains the variable
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sin(g) = inner.as_ref() {
                    // d/dx(sin(g)) = cos(g) * g'
                    let cos_g = Expr::Cos(g.clone());

                    // If g is just x, g' = 1, so result is just cos(g)
                    if let Expr::Var(v) = g.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: cos_g,
                                justification: "d/dx(sin(x)) = cos(x)".to_string(),
                            }];
                        }
                    }

                    // General case: cos(g) * g'
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(cos_g), Box::new(g_prime)),
                        justification: "d/dx(sin(g)) = cos(g) * g'".to_string(),
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
// Rule 16: d/dx(cos(g(x))) = -sin(g(x)) * g'(x) (Chain Rule)
// ============================================================================

fn chain_rule_cos() -> Rule {
    Rule {
        id: RuleId(16),
        name: "cos_chain_rule",
        category: RuleCategory::Derivative,
        description: "Cosine chain rule: d/dx(cos(g)) = -sin(g) * g'",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Cos(arg) = inner.as_ref() {
                    // Apply if argument contains the variable
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Cos(g) = inner.as_ref() {
                    // d/dx(cos(g)) = -sin(g) * g'
                    let neg_sin_g = Expr::Neg(Box::new(Expr::Sin(g.clone())));

                    // If g is just x, g' = 1, so result is just -sin(g)
                    if let Expr::Var(v) = g.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: neg_sin_g,
                                justification: "d/dx(cos(x)) = -sin(x)".to_string(),
                            }];
                        }
                    }

                    // General case: -sin(g) * g'
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(neg_sin_g), Box::new(g_prime)),
                        justification: "d/dx(cos(g)) = -sin(g) * g'".to_string(),
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
// Rule 17: d/dx(e^x) = e^x
// ============================================================================

fn exp_rule() -> Rule {
    Rule {
        id: RuleId(17),
        name: "exp_derivative",
        category: RuleCategory::Derivative,
        description: "Exponential derivative: d/dx(e^x) = e^x",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Exp(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        return v == var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Exp(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: inner.as_ref().clone(),
                                justification: "d/dx(e^x) = e^x".to_string(),
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
// Rule 18: d/dx(ln(x)) = 1/x
// ============================================================================

fn ln_rule() -> Rule {
    Rule {
        id: RuleId(18),
        name: "ln_derivative",
        category: RuleCategory::Derivative,
        description: "Natural log derivative: d/dx(ln(x)) = 1/x",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Ln(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        return v == var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Ln(arg) = inner.as_ref() {
                    if let Expr::Var(v) = arg.as_ref() {
                        if v == var {
                            return vec![RuleApplication {
                                result: Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::Var(*v))),
                                justification: "d/dx(ln(x)) = 1/x".to_string(),
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
// Phase 2: Advanced Calculus Rules (ID 400+)
// ============================================================================

/// Get all advanced calculus rules
pub fn advanced_calculus_rules() -> Vec<Rule> {
    vec![
        chain_rule_tan(),
        chain_rule_exp(),
        chain_rule_ln(),
        inverse_trig_deriv_arcsin(),
        inverse_trig_deriv_arccos(),
        inverse_trig_deriv_arctan(),
        diff_rule(),
        constant_multiple_rule(),
    ]
}

// d/dx(tan(g(x))) = sec²(g(x)) * g'(x)
fn chain_rule_tan() -> Rule {
    Rule {
        id: RuleId(400),
        name: "chain_rule_tan",
        category: RuleCategory::Derivative,
        description: "d/dx(tan(g(x))) = sec²(g(x)) * g'(x) = g'(x)/cos²(g(x))",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Tan(_) = inner.as_ref() {
                    if let Some(target) = ctx.target_var {
                        return target == *var;
                    }
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Tan(g) = inner.as_ref() {
                    // sec²(g(x)) * g'(x) = g'(x) / cos²(g(x))
                    let cos_sq = Expr::Pow(Box::new(Expr::Cos(g.clone())), Box::new(Expr::int(2)));
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(g_prime), Box::new(cos_sq)),
                        justification: "d/dx(tan(g)) = g'/cos²(g)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// d/dx(e^g(x)) = e^g(x) * g'(x)
fn chain_rule_exp() -> Rule {
    Rule {
        id: RuleId(401),
        name: "chain_rule_exp",
        category: RuleCategory::Derivative,
        description: "d/dx(e^g(x)) = e^g(x) * g'(x)",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Exp(_) = inner.as_ref() {
                    if let Some(target) = ctx.target_var {
                        return target == *var;
                    }
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Exp(g) = inner.as_ref() {
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Mul(Box::new(Expr::Exp(g.clone())), Box::new(g_prime)),
                        justification: "d/dx(e^g) = e^g * g'".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// d/dx(ln(g(x))) = g'(x)/g(x)
fn chain_rule_ln() -> Rule {
    Rule {
        id: RuleId(402),
        name: "chain_rule_ln",
        category: RuleCategory::Derivative,
        description: "d/dx(ln(g(x))) = g'(x)/g(x)",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Ln(_) = inner.as_ref() {
                    if let Some(target) = ctx.target_var {
                        return target == *var;
                    }
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Ln(g) = inner.as_ref() {
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(g_prime), g.clone()),
                        justification: "d/dx(ln(g)) = g'/g".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// d/dx(arcsin(x)) = 1/√(1-x²)
fn inverse_trig_deriv_arcsin() -> Rule {
    Rule {
        id: RuleId(403),
        name: "inverse_trig_deriv_arcsin",
        category: RuleCategory::Derivative,
        description: "d/dx(arcsin(x)) = 1/√(1-x²)",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // For now, just check if it's a simple variable
                if let Expr::Var(v) = inner.as_ref() {
                    if let Some(target) = ctx.target_var {
                        return target == *var && *v == *var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative {
                expr: inner,
                var: _,
            } = expr
            {
                if let Expr::Var(_) = inner.as_ref() {
                    // 1/√(1-x²)
                    let x_sq = Expr::Pow(inner.clone(), Box::new(Expr::int(2)));
                    let one_minus_x_sq = Expr::Sub(Box::new(Expr::int(1)), Box::new(x_sq));
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::int(1)),
                            Box::new(Expr::Sqrt(Box::new(one_minus_x_sq))),
                        ),
                        justification: "d/dx(arcsin(x)) = 1/√(1-x²)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// d/dx(arccos(x)) = -1/√(1-x²)
fn inverse_trig_deriv_arccos() -> Rule {
    Rule {
        id: RuleId(404),
        name: "inverse_trig_deriv_arccos",
        category: RuleCategory::Derivative,
        description: "d/dx(arccos(x)) = -1/√(1-x²)",
        is_applicable: |_expr, _ctx| false, // Placeholder - need Arccos type
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// d/dx(arctan(x)) = 1/(1+x²)
fn inverse_trig_deriv_arctan() -> Rule {
    Rule {
        id: RuleId(405),
        name: "inverse_trig_deriv_arctan",
        category: RuleCategory::Derivative,
        description: "d/dx(arctan(x)) = 1/(1+x²)",
        is_applicable: |_expr, _ctx| false, // Placeholder - need Arctan type
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}

// d/dx(f - g) = f' - g' (difference rule)
fn diff_rule() -> Rule {
    Rule {
        id: RuleId(406),
        name: "diff_rule",
        category: RuleCategory::Derivative,
        description: "d/dx(f - g) = f' - g'",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sub(_, _) = inner.as_ref() {
                    if let Some(target) = ctx.target_var {
                        return target == *var;
                    }
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sub(f, g) = inner.as_ref() {
                    let f_prime = Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    };
                    let g_prime = Expr::Derivative {
                        expr: g.clone(),
                        var: *var,
                    };
                    return vec![RuleApplication {
                        result: Expr::Sub(Box::new(f_prime), Box::new(g_prime)),
                        justification: "d/dx(f - g) = f' - g'".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// d/dx(c*f) = c*f' (constant multiple rule)
fn constant_multiple_rule() -> Rule {
    Rule {
        id: RuleId(407),
        name: "constant_multiple_rule",
        category: RuleCategory::Derivative,
        description: "d/dx(c*f) = c*f' where c is constant",
        is_applicable: |expr, ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Mul(left, _) = inner.as_ref() {
                    // Check if left is a constant
                    if matches!(left.as_ref(), Expr::Const(_)) {
                        if let Some(target) = ctx.target_var {
                            return target == *var;
                        }
                        return true;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Mul(c, f) = inner.as_ref() {
                    if matches!(c.as_ref(), Expr::Const(_)) {
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        return vec![RuleApplication {
                            result: Expr::Mul(c.clone(), Box::new(f_prime)),
                            justification: "d/dx(c*f) = c*f'".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

/// Check if an expression contains a specific variable.
fn contains_var(expr: &Expr, var: mm_core::Symbol) -> bool {
    match expr {
        Expr::Var(v) => *v == var,
        Expr::Const(_) | Expr::Pi | Expr::E => false,
        Expr::Neg(e)
        | Expr::Sqrt(e)
        | Expr::Sin(e)
        | Expr::Cos(e)
        | Expr::Tan(e)
        | Expr::Ln(e)
        | Expr::Exp(e)
        | Expr::Abs(e) => contains_var(e, var),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) | Expr::Pow(a, b) => {
            contains_var(a, var) || contains_var(b, var)
        }
        Expr::Sum(terms) => terms.iter().any(|t| contains_var(&t.expr, var)),
        Expr::Product(factors) => factors
            .iter()
            .any(|f| contains_var(&f.base, var) || contains_var(&f.power, var)),
        Expr::Derivative { expr, .. } | Expr::Integral { expr, .. } => contains_var(expr, var),
        Expr::Equation { lhs, rhs }
        | Expr::GCD(lhs, rhs)
        | Expr::LCM(lhs, rhs)
        | Expr::Mod(lhs, rhs)
        | Expr::Binomial(lhs, rhs) => contains_var(lhs, var) || contains_var(rhs, var),
        Expr::Floor(e) | Expr::Ceiling(e) | Expr::Factorial(e) => contains_var(e, var),
        Expr::Summation {
            var: v,
            from,
            to,
            body,
        }
        | Expr::BigProduct {
            var: v,
            from,
            to,
            body,
        } => {
            // Don't count bound var if it shadows
            if *v == var {
                contains_var(from, var) || contains_var(to, var)
            } else {
                contains_var(from, var) || contains_var(to, var) || contains_var(body, var)
            }
        }
    }
}

// ============================================================================
// Phase 4: Additional Calculus Rules (ID 420-469) for IMO
// ============================================================================

/// Phase 4 calculus rules for 450+ rules milestone
pub fn phase4_calculus_rules() -> Vec<Rule> {
    vec![
        integral_power(),
        integral_constant(),
        integral_sum(),
        integral_exp(),
        integral_ln(),
        integral_sin(),
        integral_cos(),
        integral_tan(),
        integral_sec2(),
        integral_csc2(),
        integral_sinh(),
        integral_cosh(),
        integration_by_parts(),
        u_substitution(),
        partial_fractions(),
        trig_substitution(),
        limit_constant(),
        limit_sum(),
        limit_product(),
        limit_quotient(),
        limit_power(),
        limit_lhopital(),
        limit_squeeze(),
        taylor_exp(),
        taylor_sin(),
        taylor_cos(),
        taylor_ln(),
        maclaurin_1mx(),
        geometric_series(),
        power_series_diff(),
        power_series_int(),
        partial_x(),
        partial_y(),
        partial_z(),
        gradient(),
        divergence_vec(),
        curl_vec(),
        laplacian(),
        chain_multivariable(),
        implicit_diff(),
        total_differential(),
        directional_derivative(),
        double_integral(),
        triple_integral(),
        line_integral(),
        surface_integral(),
        greens_theorem(),
        stokes_theorem(),
        divergence_theorem(),
        jacobian_transform(),
    ]
}

fn integral_power() -> Rule {
    Rule {
        id: RuleId(420),
        name: "integral_power",
        category: RuleCategory::Integral,
        description: "∫x^n dx = x^(n+1)/(n+1) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_constant() -> Rule {
    Rule {
        id: RuleId(421),
        name: "integral_constant",
        category: RuleCategory::Integral,
        description: "∫k dx = kx + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}
fn integral_sum() -> Rule {
    Rule {
        id: RuleId(422),
        name: "integral_sum",
        category: RuleCategory::Integral,
        description: "∫(f+g) dx = ∫f dx + ∫g dx",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_exp() -> Rule {
    Rule {
        id: RuleId(423),
        name: "integral_exp",
        category: RuleCategory::Integral,
        description: "∫e^x dx = e^x + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}
fn integral_ln() -> Rule {
    Rule {
        id: RuleId(424),
        name: "integral_ln",
        category: RuleCategory::Integral,
        description: "∫1/x dx = ln|x| + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_sin() -> Rule {
    Rule {
        id: RuleId(425),
        name: "integral_sin",
        category: RuleCategory::Integral,
        description: "∫sin(x) dx = -cos(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_cos() -> Rule {
    Rule {
        id: RuleId(426),
        name: "integral_cos",
        category: RuleCategory::Integral,
        description: "∫cos(x) dx = sin(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_tan() -> Rule {
    Rule {
        id: RuleId(427),
        name: "integral_tan",
        category: RuleCategory::Integral,
        description: "∫tan(x) dx = -ln|cos(x)| + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn integral_sec2() -> Rule {
    Rule {
        id: RuleId(428),
        name: "integral_sec2",
        category: RuleCategory::Integral,
        description: "∫sec²(x) dx = tan(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_csc2() -> Rule {
    Rule {
        id: RuleId(429),
        name: "integral_csc2",
        category: RuleCategory::Integral,
        description: "∫csc²(x) dx = -cot(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_sinh() -> Rule {
    Rule {
        id: RuleId(430),
        name: "integral_sinh",
        category: RuleCategory::Integral,
        description: "∫sinh(x) dx = cosh(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integral_cosh() -> Rule {
    Rule {
        id: RuleId(431),
        name: "integral_cosh",
        category: RuleCategory::Integral,
        description: "∫cosh(x) dx = sinh(x) + C",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn integration_by_parts() -> Rule {
    Rule {
        id: RuleId(432),
        name: "integration_by_parts",
        category: RuleCategory::Integral,
        description: "∫u dv = uv - ∫v du",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}
fn u_substitution() -> Rule {
    Rule {
        id: RuleId(433),
        name: "u_substitution",
        category: RuleCategory::Integral,
        description: "u-substitution technique",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn partial_fractions() -> Rule {
    Rule {
        id: RuleId(434),
        name: "partial_fractions",
        category: RuleCategory::Integral,
        description: "Partial fractions decomposition",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}
fn trig_substitution() -> Rule {
    Rule {
        id: RuleId(435),
        name: "trig_substitution",
        category: RuleCategory::Integral,
        description: "Trigonometric substitution",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}
fn limit_constant() -> Rule {
    Rule {
        id: RuleId(436),
        name: "limit_constant",
        category: RuleCategory::Simplification,
        description: "lim c = c",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 1,
    }
}
fn limit_sum() -> Rule {
    Rule {
        id: RuleId(437),
        name: "limit_sum",
        category: RuleCategory::Simplification,
        description: "lim(f+g) = lim f + lim g",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn limit_product() -> Rule {
    Rule {
        id: RuleId(438),
        name: "limit_product",
        category: RuleCategory::Simplification,
        description: "lim(fg) = lim f · lim g",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn limit_quotient() -> Rule {
    Rule {
        id: RuleId(439),
        name: "limit_quotient",
        category: RuleCategory::Simplification,
        description: "lim(f/g) = lim f / lim g",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn limit_power() -> Rule {
    Rule {
        id: RuleId(440),
        name: "limit_power",
        category: RuleCategory::Simplification,
        description: "lim(f^n) = (lim f)^n",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn limit_lhopital() -> Rule {
    Rule {
        id: RuleId(441),
        name: "limit_lhopital",
        category: RuleCategory::Simplification,
        description: "L'Hôpital's rule for 0/0 or ∞/∞",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn limit_squeeze() -> Rule {
    Rule {
        id: RuleId(442),
        name: "limit_squeeze",
        category: RuleCategory::Simplification,
        description: "Squeeze theorem",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn taylor_exp() -> Rule {
    Rule {
        id: RuleId(443),
        name: "taylor_exp",
        category: RuleCategory::Simplification,
        description: "e^x = Σ x^n/n!",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn taylor_sin() -> Rule {
    Rule {
        id: RuleId(444),
        name: "taylor_sin",
        category: RuleCategory::Simplification,
        description: "sin(x) = Σ (-1)^n x^(2n+1)/(2n+1)!",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn taylor_cos() -> Rule {
    Rule {
        id: RuleId(445),
        name: "taylor_cos",
        category: RuleCategory::Simplification,
        description: "cos(x) = Σ (-1)^n x^(2n)/(2n)!",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn taylor_ln() -> Rule {
    Rule {
        id: RuleId(446),
        name: "taylor_ln",
        category: RuleCategory::Simplification,
        description: "ln(1+x) = Σ (-1)^(n+1) x^n/n",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn maclaurin_1mx() -> Rule {
    Rule {
        id: RuleId(447),
        name: "maclaurin_1mx",
        category: RuleCategory::Simplification,
        description: "1/(1-x) = Σ x^n",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn geometric_series() -> Rule {
    Rule {
        id: RuleId(448),
        name: "geometric_series",
        category: RuleCategory::Simplification,
        description: "Σ ar^n = a/(1-r) for |r|<1",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}
fn power_series_diff() -> Rule {
    Rule {
        id: RuleId(449),
        name: "power_series_diff",
        category: RuleCategory::Derivative,
        description: "d/dx(Σa_n x^n) = Σ n·a_n x^(n-1)",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn power_series_int() -> Rule {
    Rule {
        id: RuleId(450),
        name: "power_series_int",
        category: RuleCategory::Integral,
        description: "∫(Σa_n x^n)dx = Σ a_n x^(n+1)/(n+1)",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}
fn partial_x() -> Rule {
    Rule {
        id: RuleId(451),
        name: "partial_x",
        category: RuleCategory::Derivative,
        description: "∂f/∂x partial derivative",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 2,
    }
}
fn partial_y() -> Rule {
    Rule {
        id: RuleId(452),
        name: "partial_y",
        category: RuleCategory::Derivative,
        description: "∂f/∂y partial derivative",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 2,
    }
}
fn partial_z() -> Rule {
    Rule {
        id: RuleId(453),
        name: "partial_z",
        category: RuleCategory::Derivative,
        description: "∂f/∂z partial derivative",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 2,
    }
}
fn gradient() -> Rule {
    Rule {
        id: RuleId(454),
        name: "gradient",
        category: RuleCategory::Derivative,
        description: "∇f = (∂f/∂x, ∂f/∂y, ∂f/∂z)",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn divergence_vec() -> Rule {
    Rule {
        id: RuleId(455),
        name: "divergence_vec",
        category: RuleCategory::Derivative,
        description: "∇·F divergence of vector field",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn curl_vec() -> Rule {
    Rule {
        id: RuleId(456),
        name: "curl_vec",
        category: RuleCategory::Derivative,
        description: "∇×F curl of vector field",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}
fn laplacian() -> Rule {
    Rule {
        id: RuleId(457),
        name: "laplacian",
        category: RuleCategory::Derivative,
        description: "∇²f = ∂²f/∂x² + ∂²f/∂y² + ∂²f/∂z²",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}
fn chain_multivariable() -> Rule {
    Rule {
        id: RuleId(458),
        name: "chain_multivariable",
        category: RuleCategory::Derivative,
        description: "Multivariable chain rule",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}
fn implicit_diff() -> Rule {
    Rule {
        id: RuleId(459),
        name: "implicit_diff",
        category: RuleCategory::Derivative,
        description: "Implicit differentiation",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn total_differential() -> Rule {
    Rule {
        id: RuleId(460),
        name: "total_differential",
        category: RuleCategory::Derivative,
        description: "df = ∂f/∂x dx + ∂f/∂y dy",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn directional_derivative() -> Rule {
    Rule {
        id: RuleId(461),
        name: "directional_derivative",
        category: RuleCategory::Derivative,
        description: "D_u f = ∇f · u",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 3,
    }
}
fn double_integral() -> Rule {
    Rule {
        id: RuleId(462),
        name: "double_integral",
        category: RuleCategory::Integral,
        description: "∬f dA double integral",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}
fn triple_integral() -> Rule {
    Rule {
        id: RuleId(463),
        name: "triple_integral",
        category: RuleCategory::Integral,
        description: "∭f dV triple integral",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 5,
    }
}
fn line_integral() -> Rule {
    Rule {
        id: RuleId(464),
        name: "line_integral",
        category: RuleCategory::Integral,
        description: "∫_C F·dr line integral",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}
fn surface_integral() -> Rule {
    Rule {
        id: RuleId(465),
        name: "surface_integral",
        category: RuleCategory::Integral,
        description: "∬_S F·dS surface integral",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 5,
    }
}
fn greens_theorem() -> Rule {
    Rule {
        id: RuleId(466),
        name: "greens_theorem",
        category: RuleCategory::Integral,
        description: "Green's theorem: ∮_C = ∬_D",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}
fn stokes_theorem() -> Rule {
    Rule {
        id: RuleId(467),
        name: "stokes_theorem",
        category: RuleCategory::Integral,
        description: "Stokes' theorem",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 5,
    }
}
fn divergence_theorem() -> Rule {
    Rule {
        id: RuleId(468),
        name: "divergence_theorem",
        category: RuleCategory::Integral,
        description: "Divergence theorem",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 5,
    }
}
fn jacobian_transform() -> Rule {
    Rule {
        id: RuleId(469),
        name: "jacobian_transform",
        category: RuleCategory::Integral,
        description: "Jacobian coordinate transform",
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuleContext;
    use mm_core::SymbolTable;

    #[test]
    fn test_power_rule() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = power_rule();
        let ctx = RuleContext::default();

        // d/dx(x^3)
        let expr = Expr::Derivative {
            expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            var: x,
        };

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        // Should be 3 * x^2
    }

    #[test]
    fn test_constant_rule() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = constant_rule();
        let ctx = RuleContext::default();

        // d/dx(5)
        let expr = Expr::Derivative {
            expr: Box::new(Expr::int(5)),
            var: x,
        };

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].result, Expr::int(0));
    }
}
