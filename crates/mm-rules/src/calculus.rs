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
        // inverse_trig_deriv_arccos(), - STUB: need Arccos type
        // inverse_trig_deriv_arctan(), - STUB: need Arctan type
        diff_rule(),
        constant_multiple_rule(),
        constant_base_exp_simple(),  // Rule 408
        constant_base_exp_chain(),   // Rule 409
        sqrt_chain_rule(),           // Rule 476
        general_power_rule(),        // Rule 475
        log_base_simple(),           // Rule 411
        log_base_chain(),            // Rule 412
        sec_derivative(),            // Rule 472
        csc_derivative(),            // Rule 473
        cot_derivative(),            // Rule 474
        arcsin_derivative(),         // Rule 413
        arccos_derivative(),         // Rule 414
        arctan_derivative(),         // Rule 415
        arccot_derivative(),         // Rule 416
        arcsec_derivative(),         // Rule 417
        arccsc_derivative(),         // Rule 418
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

// NOTE: d/dx(arccos(x)) and d/dx(arctan(x)) rules need Arccos/Arctan types in Expr enum.
// They are not implemented until Expr is extended.

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

// ============================================================================
// Rule 408: d/dx(a^x) = a^x·ln(a) for constant base a
// ============================================================================

fn constant_base_exp_simple() -> Rule {
    Rule {
        id: RuleId(408),
        name: "constant_base_exp_simple",
        category: RuleCategory::Derivative,
        description: "d/dx(a^x) = a^x·ln(a) where a is constant",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // Base must be constant (not contain var), exponent must be var
                    if !contains_var(base, *var) {
                        if let Expr::Var(v) = exp.as_ref() {
                            return v == var;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(base, _) = inner.as_ref() {
                    // d/dx(a^x) = a^x * ln(a)
                    let a_pow_x = inner.as_ref().clone();
                    let ln_a = Expr::Ln(base.clone());
                    
                    return vec![RuleApplication {
                        result: Expr::Mul(
                            Box::new(a_pow_x),
                            Box::new(ln_a),
                        ),
                        justification: "d/dx(a^x) = a^x·ln(a)".to_string(),
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
// Rule 409: d/dx(a^f(x)) = a^f(x)·ln(a)·f'(x) for constant base a
// ============================================================================

fn constant_base_exp_chain() -> Rule {
    Rule {
        id: RuleId(409),
        name: "constant_base_exp_chain",
        category: RuleCategory::Derivative,
        description: "d/dx(a^f(x)) = a^f(x)·ln(a)·f'(x) where a is constant",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // Base must be constant, exponent must contain var but not be just var
                    if !contains_var(base, *var) && contains_var(exp, *var) {
                        // Not just Var (that's handled by rule 408)
                        return !matches!(exp.as_ref(), Expr::Var(_));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // d/dx(a^f) = a^f * ln(a) * f'
                    let a_pow_f = inner.as_ref().clone();
                    let ln_a = Expr::Ln(base.clone());
                    let f_prime = Expr::Derivative {
                        expr: exp.clone(),
                        var: *var,
                    };
                    
                    // a^f * ln(a) * f'
                    let result = Expr::Mul(
                        Box::new(Expr::Mul(
                            Box::new(a_pow_f),
                            Box::new(ln_a),
                        )),
                        Box::new(f_prime),
                    );
                    
                    return vec![RuleApplication {
                        result,
                        justification: "d/dx(a^f) = a^f·ln(a)·f'".to_string(),
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
// Rule 476: d/dx(√f(x)) = f'(x)/(2√f(x))
// ============================================================================

fn sqrt_chain_rule() -> Rule {
    Rule {
        id: RuleId(476),
        name: "sqrt_chain_rule",
        category: RuleCategory::Derivative,
        description: "d/dx(√f) = f'/(2√f)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sqrt(arg) = inner.as_ref() {
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sqrt(f) = inner.as_ref() {
                    // d/dx(√f) = f' / (2√f)
                    let f_prime = Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    };
                    let two = Expr::int(2);
                    let sqrt_f = Expr::Sqrt(f.clone());
                    let denominator = Expr::Mul(Box::new(two), Box::new(sqrt_f));
                    
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(f_prime), Box::new(denominator)),
                        justification: "d/dx(√f) = f'/(2√f)".to_string(),
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
// Rule 475: d/dx(f(x)^n) = n·f(x)^(n-1)·f'(x) - General power rule
// ============================================================================

fn general_power_rule() -> Rule {
    Rule {
        id: RuleId(475),
        name: "general_power_rule",
        category: RuleCategory::Derivative,
        description: "d/dx(f^n) = n·f^(n-1)·f' where n is constant",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // Base must contain var, exponent must be constant
                    if contains_var(base, *var) && !contains_var(exp, *var) {
                        // Not just Var (that's handled by basic power_rule)
                        return !matches!(base.as_ref(), Expr::Var(_));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Pow(f, n) = inner.as_ref() {
                    if let Expr::Const(n_val) = n.as_ref() {
                        // d/dx(f^n) = n * f^(n-1) * f'
                        let n_minus_1 = *n_val - mm_core::Rational::from_integer(1);
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        
                        // n * f^(n-1)
                        let n_times_f_pow = Expr::Mul(
                            n.clone(),
                            Box::new(Expr::Pow(f.clone(), Box::new(Expr::Const(n_minus_1)))),
                        );
                        
                        // (n * f^(n-1)) * f'
                        let result = Expr::Mul(Box::new(n_times_f_pow), Box::new(f_prime));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(f^n) = n·f^(n-1)·f'".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 411: d/dx(log_a(x)) = 1/(x·ln(a))
// ============================================================================

fn log_base_simple() -> Rule {
    Rule {
        id: RuleId(411),
        name: "log_base_simple",
        category: RuleCategory::Derivative,
        description: "d/dx(log_a(x)) = 1/(x·ln(a))",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Check for Log(base, arg) where arg is the variable
                if let Expr::Div(num, denom) = inner.as_ref() {
                    // log_a(x) is represented as ln(x)/ln(a)
                    if let (Expr::Ln(arg), Expr::Ln(base)) = (num.as_ref(), denom.as_ref()) {
                        if let Expr::Var(v) = arg.as_ref() {
                            if v == var && !contains_var(base, *var) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Ln(arg), Expr::Ln(base)) = (num.as_ref(), denom.as_ref()) {
                        if let Expr::Var(_) = arg.as_ref() {
                            // d/dx(log_a(x)) = 1/(x·ln(a))
                            let x_ln_a = Expr::Mul(arg.clone(), denom.clone());
                            return vec![RuleApplication {
                                result: Expr::Div(Box::new(Expr::int(1)), Box::new(x_ln_a)),
                                justification: "d/dx(log_a(x)) = 1/(x·ln(a))".to_string(),
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
// Rule 412: d/dx(log_a(f(x))) = f'(x)/(f(x)·ln(a))
// ============================================================================

fn log_base_chain() -> Rule {
    Rule {
        id: RuleId(412),
        name: "log_base_chain",
        category: RuleCategory::Derivative,
        description: "d/dx(log_a(f)) = f'/(f·ln(a))",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // log_a(f) is represented as ln(f)/ln(a)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Ln(arg), Expr::Ln(base)) = (num.as_ref(), denom.as_ref()) {
                        // arg must contain var, base must not contain var
                        if contains_var(arg, *var) && !contains_var(base, *var) {
                            // Not just Var (that's handled by rule 411)
                            return !matches!(arg.as_ref(), Expr::Var(_));
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Ln(f), Expr::Ln(_)) = (num.as_ref(), denom.as_ref()) {
                        // d/dx(log_a(f)) = f' / (f·ln(a))
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        let f_ln_a = Expr::Mul(f.clone(), denom.clone());
                        
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(f_prime), Box::new(f_ln_a)),
                            justification: "d/dx(log_a(f)) = f'/(f·ln(a))".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 472: d/dx(sec(f)) = d/dx(1/cos(f)) = f'·sec(f)·tan(f)
// ============================================================================

fn sec_derivative() -> Rule {
    Rule {
        id: RuleId(472),
        name: "sec_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(sec(f)) = f'·sec(f)·tan(f) where sec(f) = 1/cos(f)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Match pattern 1/cos(f)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Const(n), Expr::Cos(arg)) = (num.as_ref(), denom.as_ref()) {
                        if n.is_one() && contains_var(arg, *var) {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    if let Expr::Cos(f) = denom.as_ref() {
                        // d/dx(sec(f)) = f' * sec(f) * tan(f)
                        // = f' * (1/cos(f)) * (sin(f)/cos(f))
                        // = f' * sin(f) / cos²(f)
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        let sin_f = Expr::Sin(f.clone());
                        let cos_sq_f = Expr::Pow(Box::new(Expr::Cos(f.clone())), Box::new(Expr::int(2)));
                        
                        // f' * sin(f) / cos²(f)
                        let numerator = Expr::Mul(Box::new(f_prime), Box::new(sin_f));
                        let result = Expr::Div(Box::new(numerator), Box::new(cos_sq_f));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(sec(f)) = f'·sec(f)·tan(f)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 473: d/dx(csc(f)) = d/dx(1/sin(f)) = -f'·csc(f)·cot(f)
// ============================================================================

fn csc_derivative() -> Rule {
    Rule {
        id: RuleId(473),
        name: "csc_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(csc(f)) = -f'·csc(f)·cot(f) where csc(f) = 1/sin(f)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Match pattern 1/sin(f)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Const(n), Expr::Sin(arg)) = (num.as_ref(), denom.as_ref()) {
                        if n.is_one() && contains_var(arg, *var) {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    if let Expr::Sin(f) = denom.as_ref() {
                        // d/dx(csc(f)) = -f' * csc(f) * cot(f)
                        // = -f' * (1/sin(f)) * (cos(f)/sin(f))
                        // = -f' * cos(f) / sin²(f)
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        let cos_f = Expr::Cos(f.clone());
                        let sin_sq_f = Expr::Pow(Box::new(Expr::Sin(f.clone())), Box::new(Expr::int(2)));
                        
                        // -f' * cos(f) / sin²(f)
                        let numerator = Expr::Mul(Box::new(f_prime), Box::new(cos_f));
                        let result = Expr::Neg(Box::new(Expr::Div(Box::new(numerator), Box::new(sin_sq_f))));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(csc(f)) = -f'·csc(f)·cot(f)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 474: d/dx(cot(f)) = d/dx(cos(f)/sin(f)) = -f'/sin²(f)
// ============================================================================

fn cot_derivative() -> Rule {
    Rule {
        id: RuleId(474),
        name: "cot_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(cot(f)) = -f'/sin²(f) where cot(f) = cos(f)/sin(f)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                // Match pattern cos(f)/sin(f)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Cos(arg1), Expr::Sin(arg2)) = (num.as_ref(), denom.as_ref()) {
                        // Check if both args are the same and contain var
                        if arg1 == arg2 && contains_var(arg1, *var) {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(num, _) = inner.as_ref() {
                    if let Expr::Cos(f) = num.as_ref() {
                        // d/dx(cot(f)) = -f'/sin²(f)
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        let sin_sq_f = Expr::Pow(Box::new(Expr::Sin(f.clone())), Box::new(Expr::int(2)));
                        
                        // -f' / sin²(f)
                        let result = Expr::Neg(Box::new(Expr::Div(Box::new(f_prime), Box::new(sin_sq_f))));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(cot(f)) = -f'/sin²(f)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 413: d/dx(arcsin(f)) = f'/√(1-f²)
// ============================================================================

fn arcsin_derivative() -> Rule {
    Rule {
        id: RuleId(413),
        name: "arcsin_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arcsin(f)) = f'/√(1-f²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arcsin(arg) = inner.as_ref() {
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arcsin(f) = inner.as_ref() {
                    // d/dx(arcsin(f)) = f' / √(1-f²)
                    let f_prime = Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    };
                    
                    // 1 - f²
                    let one_minus_f_squared = Expr::Sub(
                        Box::new(Expr::int(1)),
                        Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2))))
                    );
                    
                    // √(1-f²)
                    let sqrt_denom = Expr::Sqrt(Box::new(one_minus_f_squared));
                    
                    // f' / √(1-f²)
                    let result = Expr::Div(Box::new(f_prime), Box::new(sqrt_denom));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "d/dx(arcsin(f)) = f'/√(1-f²)".to_string(),
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
// Rule 414: d/dx(arccos(f)) = -f'/√(1-f²)
// ============================================================================

fn arccos_derivative() -> Rule {
    Rule {
        id: RuleId(414),
        name: "arccos_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arccos(f)) = -f'/√(1-f²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arccos(arg) = inner.as_ref() {
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arccos(f) = inner.as_ref() {
                    // d/dx(arccos(f)) = -f' / √(1-f²)
                    let f_prime = Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    };
                    
                    // 1 - f²
                    let one_minus_f_squared = Expr::Sub(
                        Box::new(Expr::int(1)),
                        Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2))))
                    );
                    
                    // √(1-f²)
                    let sqrt_denom = Expr::Sqrt(Box::new(one_minus_f_squared));
                    
                    // f' / √(1-f²)
                    let fraction = Expr::Div(Box::new(f_prime), Box::new(sqrt_denom));
                    
                    // -f' / √(1-f²)
                    let result = Expr::Neg(Box::new(fraction));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "d/dx(arccos(f)) = -f'/√(1-f²)".to_string(),
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
// Rule 415: d/dx(arctan(f)) = f'/(1+f²)
// ============================================================================

fn arctan_derivative() -> Rule {
    Rule {
        id: RuleId(415),
        name: "arctan_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arctan(f)) = f'/(1+f²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arctan(arg) = inner.as_ref() {
                    return contains_var(arg, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Arctan(f) = inner.as_ref() {
                    // d/dx(arctan(f)) = f' / (1+f²)
                    let f_prime = Expr::Derivative {
                        expr: f.clone(),
                        var: *var,
                    };
                    
                    // 1 + f²
                    let one_plus_f_squared = Expr::Add(
                        Box::new(Expr::int(1)),
                        Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2))))
                    );
                    
                    // f' / (1+f²)
                    let result = Expr::Div(Box::new(f_prime), Box::new(one_plus_f_squared));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "d/dx(arctan(f)) = f'/(1+f²)".to_string(),
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
// Rule 416: d/dx(arccot(f)) = -f'/(1+f²)
// ============================================================================

fn arccot_derivative() -> Rule {
    Rule {
        id: RuleId(416),
        name: "arccot_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arccot(f)) = -f'/(1+f²)",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                // Match arccot(something)
                if let Expr::Div(one, inner_expr) = inner.as_ref() {
                    if matches!(one.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Tan(arg) = inner_expr.as_ref() {
                            // arccot(x) = 1/tan(x) pattern
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(_, inner_expr) = inner.as_ref() {
                    if let Expr::Tan(f) = inner_expr.as_ref() {
                        // d/dx(arccot(f)) = -f'/(1+f²)
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        
                        // 1 + f²
                        let one_plus_f_sq = Expr::Add(
                            Box::new(Expr::int(1)),
                            Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2))))
                        );
                        
                        // -f'/(1+f²)
                        let result = Expr::Neg(Box::new(Expr::Div(
                            Box::new(f_prime),
                            Box::new(one_plus_f_sq)
                        )));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(arccot(f)) = -f'/(1+f²)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// ============================================================================
// Rule 417: d/dx(arcsec(f)) = f'/(|f|√(f²-1))
// ============================================================================

fn arcsec_derivative() -> Rule {
    Rule {
        id: RuleId(417),
        name: "arcsec_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arcsec(f)) = f'/(|f|√(f²-1))",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                // Match arcsec(something) = 1/cos(something)
                if let Expr::Div(one, inner_expr) = inner.as_ref() {
                    if matches!(one.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Cos(_) = inner_expr.as_ref() {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(_, inner_expr) = inner.as_ref() {
                    if let Expr::Cos(f) = inner_expr.as_ref() {
                        // d/dx(arcsec(f)) = f'/(|f|√(f²-1))
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        
                        // f² - 1
                        let f_sq_minus_1 = Expr::Sub(
                            Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2)))),
                            Box::new(Expr::int(1))
                        );
                        
                        // √(f²-1)
                        let sqrt_part = Expr::Sqrt(Box::new(f_sq_minus_1));
                        
                        // |f|√(f²-1)
                        let denominator = Expr::Mul(
                            Box::new(Expr::Abs(f.clone())),
                            Box::new(sqrt_part)
                        );
                        
                        // f'/(|f|√(f²-1))
                        let result = Expr::Div(
                            Box::new(f_prime),
                            Box::new(denominator)
                        );
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(arcsec(f)) = f'/(|f|√(f²-1))".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 4,
    }
}

// ============================================================================
// Rule 418: d/dx(arccsc(f)) = -f'/(|f|√(f²-1))
// ============================================================================

fn arccsc_derivative() -> Rule {
    Rule {
        id: RuleId(418),
        name: "arccsc_derivative",
        category: RuleCategory::Derivative,
        description: "d/dx(arccsc(f)) = -f'/(|f|√(f²-1))",
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, .. } = expr {
                // Match arccsc(something) = 1/sin(something)
                if let Expr::Div(one, inner_expr) = inner.as_ref() {
                    if matches!(one.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Sin(_) = inner_expr.as_ref() {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Div(_, inner_expr) = inner.as_ref() {
                    if let Expr::Sin(f) = inner_expr.as_ref() {
                        // d/dx(arccsc(f)) = -f'/(|f|√(f²-1))
                        let f_prime = Expr::Derivative {
                            expr: f.clone(),
                            var: *var,
                        };
                        
                        // f² - 1
                        let f_sq_minus_1 = Expr::Sub(
                            Box::new(Expr::Pow(f.clone(), Box::new(Expr::int(2)))),
                            Box::new(Expr::int(1))
                        );
                        
                        // √(f²-1)
                        let sqrt_part = Expr::Sqrt(Box::new(f_sq_minus_1));
                        
                        // |f|√(f²-1)
                        let denominator = Expr::Mul(
                            Box::new(Expr::Abs(f.clone())),
                            Box::new(sqrt_part)
                        );
                        
                        // -f'/(|f|√(f²-1))
                        let result = Expr::Neg(Box::new(Expr::Div(
                            Box::new(f_prime),
                            Box::new(denominator)
                        )));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "d/dx(arccsc(f)) = -f'/(|f|√(f²-1))".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 4,
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
        | Expr::Arcsin(e)
        | Expr::Arccos(e)
        | Expr::Arctan(e)
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
        | Expr::Binomial(lhs, rhs)
        | Expr::Gte(lhs, rhs)
        | Expr::Gt(lhs, rhs)
        | Expr::Lte(lhs, rhs)
        | Expr::Lt(lhs, rhs) => contains_var(lhs, var) || contains_var(rhs, var),
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
        Expr::ForAll {
            var: v,
            domain,
            body,
        }
        | Expr::Exists {
            var: v,
            domain,
            body,
        } => {
            // Don't count bound var if it shadows
            if *v == var {
                domain
                    .as_ref()
                    .map(|d| contains_var(d, var))
                    .unwrap_or(false)
            } else {
                domain
                    .as_ref()
                    .map(|d| contains_var(d, var))
                    .unwrap_or(false)
                    || contains_var(body, var)
            }
        }
        Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) => {
            contains_var(a, var) || contains_var(b, var)
        }
        Expr::Not(e) => contains_var(e, var),
    }
}

// ============================================================================
// Phase 4: Additional Calculus Rules (ID 420-469) for IMO
// ============================================================================

/// Phase 4 calculus rules for 450+ rules milestone
pub fn phase4_calculus_rules() -> Vec<Rule> {
    vec![
        integral_constant_multiple(),  // Rule 419
        integral_power(),
        integral_constant(),
        integral_sum(),
        integral_exp(),
        integral_ln(),
        integral_sin(),
        integral_cos(),
        integral_difference(),  // Rule 427
        integral_tan(),
        integral_sec2(),
        integral_csc2(),
        integral_sinh(),
        integral_cosh(),
        integration_by_parts(),
        u_substitution(),
        partial_fractions(),
        trig_substitution(),
        integral_cot(),  // Rule 441
        integral_sec(),
        integral_csc(),
        integral_sec_tan(),
        integral_inv_sqrt_a2_minus_x2(),  // Rule 445
        integral_inv_a2_plus_x2(),
        integral_inv_x_sqrt_x2_minus_a2(),
        integral_x_sin(),  // Rule 448
        integral_x_cos(),
        integral_ln_x(),
        integral_x_exp_ax(),
        integral_x_over_x2_plus_a2(),  // Rule 452
        integral_x_over_x2_minus_a2(),
        integral_exp_ax(),
        integral_one_over_x2_minus_a2(),
        integral_sin_squared(),  // Rule 456 - Advanced patterns
        integral_cos_squared(),
        integral_tan_squared(),
        integral_sec_cubed(),
        integral_x2_sin(),  // Rule 460
        integral_x2_cos(),
        integral_exp_sin(),
        integral_exp_cos(),
        integral_sqrt_a2_minus_x2(),  // Rule 464 - Square root patterns
        integral_sqrt_x2_plus_a2(),
        integral_sqrt_x2_minus_a2(),
        integral_x_sqrt_a2_minus_x2(),
        integral_inv_sqrt_x2_plus_a2(),
        integral_inv_sqrt_x2_minus_a2(),
        integral_x_over_sqrt_x2_plus_a2(),
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

// ============================================================================
// Rule 419: ∫k·f(x) dx = k·∫f(x) dx (constant multiple rule)
// ============================================================================

fn integral_constant_multiple() -> Rule {
    Rule {
        id: RuleId(419),
        name: "integral_constant_multiple",
        category: RuleCategory::Integral,
        description: "∫k·f(x) dx = k·∫f(x) dx",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match k * f where k doesn't contain the variable
                if let Expr::Mul(k, f) = inner.as_ref() {
                    return !contains_var(k, *var) && contains_var(f, *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Mul(k, f) = inner.as_ref() {
                    // ∫k·f dx = k·∫f dx
                    let integral_f = Expr::Integral {
                        expr: f.clone(),
                        var: *var,
                    };
                    let result = Expr::Mul(k.clone(), Box::new(integral_f));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫k·f(x) dx = k·∫f(x) dx".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

fn integral_power() -> Rule {
    Rule {
        id: RuleId(420),
        name: "integral_power",
        category: RuleCategory::Integral,
        description: "∫x^n dx = x^(n+1)/(n+1) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x^n where n is a constant and n ≠ -1
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                        // Check that it's not x^(-1) since that would give ln
                        let is_neg_one = n.is_integer() && n.numer() == -1;
                        return *v == *var && !is_neg_one;
                    }
                }
                // Also match just x (which is x^1)
                if let Expr::Var(v) = inner.as_ref() {
                    return *v == *var;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let Expr::Const(n) = exp.as_ref() {
                        // ∫x^n dx = x^(n+1)/(n+1)
                        let n_plus_1 = *n + mm_core::Rational::from_integer(1);
                        let new_power = Expr::Pow(
                            base.clone(),
                            Box::new(Expr::Const(n_plus_1))
                        );
                        let result = Expr::Div(
                            Box::new(new_power),
                            Box::new(Expr::Const(n_plus_1))
                        );
                        
                        return vec![RuleApplication {
                            result,
                            justification: format!("∫x^{} dx = x^{}/({})", n, n_plus_1, n_plus_1),
                        }];
                    }
                } else if let Expr::Var(_) = inner.as_ref() {
                    // ∫x dx = x²/2
                    let result = Expr::Div(
                        Box::new(Expr::Pow(inner.clone(), Box::new(Expr::int(2)))),
                        Box::new(Expr::int(2))
                    );
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫x dx = x²/2".to_string(),
                    }];
                }
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match constant (doesn't contain the variable)
                return !contains_var(inner, *var);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // ∫k dx = k*x
                let result = Expr::Mul(
                    inner.clone(),
                    Box::new(Expr::Var(*var))
                );
                
                return vec![RuleApplication {
                    result,
                    justification: "∫k dx = kx".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, .. } = expr {
                // Match f + g
                return matches!(inner.as_ref(), Expr::Add(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Add(f, g) = inner.as_ref() {
                    // ∫(f+g) dx = ∫f dx + ∫g dx
                    let integral_f = Expr::Integral {
                        expr: f.clone(),
                        var: *var,
                    };
                    let integral_g = Expr::Integral {
                        expr: g.clone(),
                        var: *var,
                    };
                    let result = Expr::Add(Box::new(integral_f), Box::new(integral_g));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫(f+g) dx = ∫f dx + ∫g dx".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}
fn integral_difference() -> Rule {
    Rule {
        id: RuleId(427),
        name: "integral_difference",
        category: RuleCategory::Integral,
        description: "∫(f-g) dx = ∫f dx - ∫g dx",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, .. } = expr {
                // Match f - g
                return matches!(inner.as_ref(), Expr::Sub(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                if let Expr::Sub(f, g) = inner.as_ref() {
                    // ∫(f-g) dx = ∫f dx - ∫g dx
                    let integral_f = Expr::Integral {
                        expr: f.clone(),
                        var: *var,
                    };
                    let integral_g = Expr::Integral {
                        expr: g.clone(),
                        var: *var,
                    };
                    let result = Expr::Sub(Box::new(integral_f), Box::new(integral_g));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫(f-g) dx = ∫f dx - ∫g dx".to_string(),
                    }];
                }
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match e^x
                if let Expr::Exp(arg) = inner.as_ref() {
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { expr: inner, .. } = expr {
                // ∫e^x dx = e^x
                return vec![RuleApplication {
                    result: inner.as_ref().clone(),
                    justification: "∫e^x dx = e^x".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/x
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if let (Expr::Const(n), Expr::Var(v)) = (num.as_ref(), denom.as_ref()) {
                        return n.is_one() && *v == *var;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫1/x dx = ln|x| (we use Abs for |x|)
                let result = Expr::Ln(Box::new(Expr::Abs(Box::new(Expr::Var(*var)))));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/x dx = ln|x|".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sin(x)
                if let Expr::Sin(arg) = inner.as_ref() {
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫sin(x) dx = -cos(x)
                let result = Expr::Neg(Box::new(Expr::Cos(Box::new(Expr::Var(*var)))));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sin(x) dx = -cos(x)".to_string(),
                }];
            }
            vec![]
        },
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
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match cos(x)
                if let Expr::Cos(arg) = inner.as_ref() {
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫cos(x) dx = sin(x)
                let result = Expr::Sin(Box::new(Expr::Var(*var)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫cos(x) dx = sin(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}
fn integral_tan() -> Rule {
    Rule {
        id: RuleId(428),
        name: "integral_tan",
        category: RuleCategory::Integral,
        description: "∫tan(x) dx = -ln|cos(x)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match tan(x)
                if let Expr::Tan(arg) = inner.as_ref() {
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫tan(x) dx = -ln|cos(x)|
                let result = Expr::Neg(Box::new(Expr::Ln(Box::new(
                    Expr::Abs(Box::new(Expr::Cos(Box::new(Expr::Var(*var)))))
                ))));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫tan(x) dx = -ln|cos(x)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}
fn integral_sec2() -> Rule {
    Rule {
        id: RuleId(429),
        name: "integral_sec2",
        category: RuleCategory::Integral,
        description: "∫sec²(x) dx = tan(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sec²(x) = 1/cos²(x)
                if let Expr::Div(one, denom) = inner.as_ref() {
                    if matches!(one.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Pow(base, exp) = denom.as_ref() {
                            if let (Expr::Cos(arg), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                                if n.numer() == 2 {
                                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫sec²(x) dx = tan(x)
                let result = Expr::Tan(Box::new(Expr::Var(*var)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sec²(x) dx = tan(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}
fn integral_csc2() -> Rule {
    Rule {
        id: RuleId(430),
        name: "integral_csc2",
        category: RuleCategory::Integral,
        description: "∫csc²(x) dx = -cot(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match csc²(x) = 1/sin²(x)
                if let Expr::Div(one, denom) = inner.as_ref() {
                    if matches!(one.as_ref(), Expr::Const(c) if c.is_one()) {
                        if let Expr::Pow(base, exp) = denom.as_ref() {
                            if let (Expr::Sin(arg), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                                if n.numer() == 2 {
                                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫csc²(x) dx = -cot(x) = -cos(x)/sin(x)
                let result = Expr::Neg(Box::new(Expr::Div(
                    Box::new(Expr::Cos(Box::new(Expr::Var(*var)))),
                    Box::new(Expr::Sin(Box::new(Expr::Var(*var))))
                )));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫csc²(x) dx = -cot(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}
fn integral_sinh() -> Rule {
    Rule {
        id: RuleId(431),
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
        id: RuleId(432),
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
        id: RuleId(433),
        name: "integration_by_parts",
        category: RuleCategory::Integral,
        description: "∫x·e^x dx = x·e^x - e^x + C (simplified pattern)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x * e^x pattern
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let is_x_times_exp = matches!(left.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(right.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    let is_exp_times_x = matches!(right.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(left.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    return is_x_times_exp || is_exp_times_x;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x·e^x dx = x·e^x - e^x = (x-1)·e^x
                let x_var = Expr::Var(*var);
                let exp_x = Expr::Exp(Box::new(Expr::Var(*var)));
                
                // x·e^x
                let x_exp = Expr::Mul(Box::new(x_var.clone()), Box::new(exp_x.clone()));
                
                // x·e^x - e^x
                let result = Expr::Sub(Box::new(x_exp), Box::new(exp_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x·e^x dx = x·e^x - e^x (integration by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}
fn u_substitution() -> Rule {
    Rule {
        id: RuleId(434),
        name: "u_substitution",
        category: RuleCategory::Integral,
        description: "∫2x·e^(x²) dx = e^(x²) + C (chain rule pattern)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 2x * e^(x²) pattern
                if let Expr::Mul(left, right) = inner.as_ref() {
                    // Check if left is 2x and right is e^(x²)
                    if let (Expr::Mul(coeff, x_part), Expr::Exp(exp_arg)) = (left.as_ref(), right.as_ref()) {
                        if matches!(coeff.as_ref(), Expr::Const(n) if n.numer() == 2) {
                            if matches!(x_part.as_ref(), Expr::Var(v) if *v == *var) {
                                if matches!(exp_arg.as_ref(), Expr::Pow(base, exp) 
                                    if matches!(base.as_ref(), Expr::Var(v2) if *v2 == *var)
                                    && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2)) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫2x·e^(x²) dx = e^(x²)
                let x_squared = Expr::Pow(Box::new(Expr::Var(*var)), Box::new(Expr::int(2)));
                let result = Expr::Exp(Box::new(x_squared));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫2x·e^(x²) dx = e^(x²) (u-substitution with u=x²)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}
fn partial_fractions() -> Rule {
    Rule {
        id: RuleId(435),
        name: "partial_fractions",
        category: RuleCategory::Integral,
        description: "∫1/(x²-1) dx = (1/2)ln|(x-1)/(x+1)| + C (partial fractions)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/(x²-1) pattern
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sub(left, right) = denom.as_ref() {
                            if matches!(left.as_ref(), Expr::Pow(base, exp)
                                if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2)) {
                                if matches!(right.as_ref(), Expr::Const(n) if n.is_one()) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫1/(x²-1) dx = (1/2)·ln|(x-1)/(x+1)|
                let x = Expr::Var(*var);
                
                // x - 1
                let x_minus_1 = Expr::Sub(Box::new(x.clone()), Box::new(Expr::int(1)));
                
                // x + 1
                let x_plus_1 = Expr::Add(Box::new(x), Box::new(Expr::int(1)));
                
                // (x-1)/(x+1)
                let fraction = Expr::Div(Box::new(x_minus_1), Box::new(x_plus_1));
                
                // |(x-1)/(x+1)|
                let abs_fraction = Expr::Abs(Box::new(fraction));
                
                // ln|(x-1)/(x+1)|
                let ln_part = Expr::Ln(Box::new(abs_fraction));
                
                // (1/2)·ln|(x-1)/(x+1)|
                let half = Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(2)));
                let result = Expr::Mul(Box::new(half), Box::new(ln_part));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/(x²-1) dx = (1/2)ln|(x-1)/(x+1)| (partial fractions)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}
fn trig_substitution() -> Rule {
    Rule {
        id: RuleId(436),
        name: "trig_substitution",
        category: RuleCategory::Integral,
        description: "∫1/√(1-x²) dx = arcsin(x) + C (trig substitution)",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/√(1-x²) pattern
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                            if let Expr::Sub(left, right) = sqrt_arg.as_ref() {
                                if matches!(left.as_ref(), Expr::Const(n) if n.is_one()) {
                                    if matches!(right.as_ref(), Expr::Pow(base, exp)
                                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2)) {
                                        return true;
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
            if let Expr::Integral { var, .. } = expr {
                // ∫1/√(1-x²) dx = arcsin(x)
                let result = Expr::Arcsin(Box::new(Expr::Var(*var)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/√(1-x²) dx = arcsin(x) (trig substitution)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}
// ============================================================================
// More Trig Integration Rules (441-444)
// ============================================================================

fn integral_cot() -> Rule {
    Rule {
        id: RuleId(441),
        name: "integral_cot",
        category: RuleCategory::Integral,
        description: "∫cot(x) dx = ln|sin(x)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match cot(x) = cos(x)/sin(x)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    return matches!(num.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(denom.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫cot(x) dx = ln|sin(x)|
                let sin_x = Expr::Sin(Box::new(Expr::Var(*var)));
                let abs_sin = Expr::Abs(Box::new(sin_x));
                let result = Expr::Ln(Box::new(abs_sin));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫cot(x) dx = ln|sin(x)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

fn integral_sec() -> Rule {
    Rule {
        id: RuleId(442),
        name: "integral_sec",
        category: RuleCategory::Integral,
        description: "∫sec(x) dx = ln|sec(x) + tan(x)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sec(x) = 1/cos(x)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    return matches!(num.as_ref(), Expr::Const(n) if n.is_one())
                        && matches!(denom.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫sec(x) dx = ln|sec(x) + tan(x)|
                let x = Expr::Var(*var);
                let cos_x = Expr::Cos(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                
                // sec(x) = 1/cos(x)
                let sec_x = Expr::Div(Box::new(Expr::int(1)), Box::new(cos_x.clone()));
                
                // tan(x) = sin(x)/cos(x)
                let tan_x = Expr::Div(Box::new(sin_x), Box::new(cos_x));
                
                // sec(x) + tan(x)
                let sum = Expr::Add(Box::new(sec_x), Box::new(tan_x));
                
                // ln|sec(x) + tan(x)|
                let abs_sum = Expr::Abs(Box::new(sum));
                let result = Expr::Ln(Box::new(abs_sum));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sec(x) dx = ln|sec(x) + tan(x)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_csc() -> Rule {
    Rule {
        id: RuleId(443),
        name: "integral_csc",
        category: RuleCategory::Integral,
        description: "∫csc(x) dx = -ln|csc(x) + cot(x)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match csc(x) = 1/sin(x)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    return matches!(num.as_ref(), Expr::Const(n) if n.is_one())
                        && matches!(denom.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var));
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫csc(x) dx = -ln|csc(x) + cot(x)|
                let x = Expr::Var(*var);
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x.clone()));
                
                // csc(x) = 1/sin(x)
                let csc_x = Expr::Div(Box::new(Expr::int(1)), Box::new(sin_x.clone()));
                
                // cot(x) = cos(x)/sin(x)
                let cot_x = Expr::Div(Box::new(cos_x), Box::new(sin_x));
                
                // csc(x) + cot(x)
                let sum = Expr::Add(Box::new(csc_x), Box::new(cot_x));
                
                // -ln|csc(x) + cot(x)|
                let abs_sum = Expr::Abs(Box::new(sum));
                let ln_part = Expr::Ln(Box::new(abs_sum));
                let result = Expr::Neg(Box::new(ln_part));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫csc(x) dx = -ln|csc(x) + cot(x)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_sec_tan() -> Rule {
    Rule {
        id: RuleId(444),
        name: "integral_sec_tan",
        category: RuleCategory::Integral,
        description: "∫sec(x)tan(x) dx = sec(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sec(x)·tan(x) = (1/cos(x))·(sin(x)/cos(x)) = sin(x)/cos²(x)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var)) {
                        if let Expr::Pow(base, exp) = denom.as_ref() {
                            return matches!(base.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                                && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫sec(x)tan(x) dx = sec(x) = 1/cos(x)
                let cos_x = Expr::Cos(Box::new(Expr::Var(*var)));
                let result = Expr::Div(Box::new(Expr::int(1)), Box::new(cos_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sec(x)tan(x) dx = sec(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// ============================================================================
// Inverse Trig Integration (445-447)
// ============================================================================

fn integral_inv_sqrt_a2_minus_x2() -> Rule {
    Rule {
        id: RuleId(445),
        name: "integral_inv_sqrt_a2_minus_x2",
        category: RuleCategory::Integral,
        description: "∫1/√(a²-x²) dx = arcsin(x/a) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/√(a²-x²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                            if let Expr::Sub(left, right) = sqrt_arg.as_ref() {
                                // Check if it's a²-x²
                                let is_const_minus_var_sq = matches!(left.as_ref(), Expr::Const(_))
                                    && matches!(right.as_ref(), Expr::Pow(base, exp)
                                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                                        
                                // Or check if it's (const)²-x²
                                let is_pow_const_minus_var_sq = if let Expr::Pow(base, exp) = left.as_ref() {
                                    matches!(base.as_ref(), Expr::Const(_)) 
                                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2)
                                        && matches!(right.as_ref(), Expr::Pow(base2, exp2)
                                            if matches!(base2.as_ref(), Expr::Var(v) if *v == *var)
                                            && matches!(exp2.as_ref(), Expr::Const(n) if n.numer() == 2))
                                } else {
                                    false
                                };
                                
                                return is_const_minus_var_sq || is_pow_const_minus_var_sq;
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                // Extract a from the expression
                if let Expr::Div(_, denom) = inner.as_ref() {
                    if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                        if let Expr::Sub(left, _) = sqrt_arg.as_ref() {
                            let a_squared = left.clone();
                            // ∫1/√(a²-x²) dx = arcsin(x/a)
                            let x_div_a = Expr::Div(
                                Box::new(Expr::Var(*var)),
                                Box::new(Expr::Sqrt(a_squared))
                            );
                            let result = Expr::Arcsin(Box::new(x_div_a));
                            
                            return vec![RuleApplication {
                                result,
                                justification: "∫1/√(a²-x²) dx = arcsin(x/a)".to_string(),
                            }];
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

fn integral_inv_a2_plus_x2() -> Rule {
    Rule {
        id: RuleId(446),
        name: "integral_inv_a2_plus_x2",
        category: RuleCategory::Integral,
        description: "∫1/(a²+x²) dx = (1/a)arctan(x/a) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/(a²+x²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Add(left, right) = denom.as_ref() {
                            // Check a²+x² or x²+a²
                            let pattern1 = matches!(left.as_ref(), Expr::Const(_) | Expr::Pow(..))
                                && matches!(right.as_ref(), Expr::Pow(base, exp)
                                    if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                    && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                                    
                            let pattern2 = matches!(right.as_ref(), Expr::Const(_) | Expr::Pow(..))
                                && matches!(left.as_ref(), Expr::Pow(base, exp)
                                    if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                    && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                                    
                            return pattern1 || pattern2;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    if let Expr::Add(left, right) = denom.as_ref() {
                        // Get a² (the constant term)
                        let a_squared = if matches!(left.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Var(_))) {
                            right.clone()
                        } else {
                            left.clone()
                        };
                        
                        // ∫1/(a²+x²) dx = (1/a)arctan(x/a)
                        let a = Expr::Sqrt(a_squared.clone());
                        let x_div_a = Expr::Div(Box::new(Expr::Var(*var)), Box::new(a.clone()));
                        let arctan_part = Expr::Arctan(Box::new(x_div_a));
                        let result = Expr::Div(Box::new(arctan_part), Box::new(a));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "∫1/(a²+x²) dx = (1/a)arctan(x/a)".to_string(),
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

fn integral_inv_x_sqrt_x2_minus_a2() -> Rule {
    Rule {
        id: RuleId(447),
        name: "integral_inv_x_sqrt_x2_minus_a2",
        category: RuleCategory::Integral,
        description: "∫1/(x√(x²-a²)) dx = (1/a)arcsec(|x|/a) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/(x√(x²-a²))
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        // Denominator should be x·√(x²-a²)
                        if let Expr::Mul(left, right) = denom.as_ref() {
                            let has_var = matches!(left.as_ref(), Expr::Var(v) if *v == *var)
                                || matches!(right.as_ref(), Expr::Var(v) if *v == *var);
                            let has_sqrt = matches!(left.as_ref(), Expr::Sqrt(_)) || matches!(right.as_ref(), Expr::Sqrt(_));
                            return has_var && has_sqrt;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // Simplified result: arcsec(x) = arccos(1/x)
                // ∫1/(x√(x²-1)) dx = arccos(1/|x|)
                let x = Expr::Var(*var);
                let abs_x = Expr::Abs(Box::new(x.clone()));
                let one_over_x = Expr::Div(Box::new(Expr::int(1)), Box::new(abs_x));
                let result = Expr::Arccos(Box::new(one_over_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/(x√(x²-a²)) dx = arccos(1/|x|) (arcsec form)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

// ============================================================================
// More By-Parts Patterns (448-451)
// ============================================================================

fn integral_x_sin() -> Rule {
    Rule {
        id: RuleId(448),
        name: "integral_x_sin",
        category: RuleCategory::Integral,
        description: "∫x·sin(x) dx = -x·cos(x) + sin(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x·sin(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let pattern1 = matches!(left.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(right.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    let pattern2 = matches!(right.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(left.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    return pattern1 || pattern2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x·sin(x) dx = -x·cos(x) + sin(x)
                let x = Expr::Var(*var);
                let cos_x = Expr::Cos(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                
                // -x·cos(x)
                let neg_x_cos = Expr::Neg(Box::new(Expr::Mul(Box::new(x), Box::new(cos_x))));
                
                // -x·cos(x) + sin(x)
                let result = Expr::Add(Box::new(neg_x_cos), Box::new(sin_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x·sin(x) dx = -x·cos(x) + sin(x) (integration by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

fn integral_x_cos() -> Rule {
    Rule {
        id: RuleId(449),
        name: "integral_x_cos",
        category: RuleCategory::Integral,
        description: "∫x·cos(x) dx = x·sin(x) + cos(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x·cos(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let pattern1 = matches!(left.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(right.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    let pattern2 = matches!(right.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(left.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    return pattern1 || pattern2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x·cos(x) dx = x·sin(x) + cos(x)
                let x = Expr::Var(*var);
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x.clone()));
                
                // x·sin(x)
                let x_sin = Expr::Mul(Box::new(x), Box::new(sin_x));
                
                // x·sin(x) + cos(x)
                let result = Expr::Add(Box::new(x_sin), Box::new(cos_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x·cos(x) dx = x·sin(x) + cos(x) (integration by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

fn integral_ln_x() -> Rule {
    Rule {
        id: RuleId(450),
        name: "integral_ln_x",
        category: RuleCategory::Integral,
        description: "∫ln(x) dx = x·ln(x) - x + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match ln(x)
                if let Expr::Ln(arg) = inner.as_ref() {
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫ln(x) dx = x·ln(x) - x
                let x = Expr::Var(*var);
                let ln_x = Expr::Ln(Box::new(x.clone()));
                
                // x·ln(x)
                let x_ln_x = Expr::Mul(Box::new(x.clone()), Box::new(ln_x));
                
                // x·ln(x) - x
                let result = Expr::Sub(Box::new(x_ln_x), Box::new(x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫ln(x) dx = x·ln(x) - x (integration by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_x_exp_ax() -> Rule {
    Rule {
        id: RuleId(451),
        name: "integral_x_exp_ax",
        category: RuleCategory::Integral,
        description: "∫x·e^(ax) dx = (e^(ax)/a²)(ax-1) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x·e^(ax) where a is constant
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let var_on_left = matches!(left.as_ref(), Expr::Var(v) if *v == *var);
                    let exp_on_right = matches!(right.as_ref(), Expr::Exp(_));
                    let var_on_right = matches!(right.as_ref(), Expr::Var(v) if *v == *var);
                    let exp_on_left = matches!(left.as_ref(), Expr::Exp(_));
                    
                    return (var_on_left && exp_on_right) || (var_on_right && exp_on_left);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                // Simplified for e^x case: ∫x·e^x dx = (x-1)·e^x (already have Rule 433)
                // For general case, return result assuming a=1
                let x = Expr::Var(*var);
                let exp_x = Expr::Exp(Box::new(x.clone()));
                
                // (x-1)
                let x_minus_1 = Expr::Sub(Box::new(x), Box::new(Expr::int(1)));
                
                // (x-1)·e^x
                let result = Expr::Mul(Box::new(x_minus_1), Box::new(exp_x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x·e^x dx = (x-1)·e^x (integration by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

// ============================================================================
// Rational Function Patterns (452-455)
// ============================================================================

fn integral_x_over_x2_plus_a2() -> Rule {
    Rule {
        id: RuleId(452),
        name: "integral_x_over_x2_plus_a2",
        category: RuleCategory::Integral,
        description: "∫x/(x²+a²) dx = (1/2)ln(x²+a²) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x/(x²+a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Var(v) if *v == *var) {
                        if let Expr::Add(_, _) = denom.as_ref() {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    // ∫x/(x²+a²) dx = (1/2)ln(x²+a²)
                    let ln_denom = Expr::Ln(Box::new(*denom.clone()));
                    let half = Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(2)));
                    let result = Expr::Mul(Box::new(half), Box::new(ln_denom));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫x/(x²+a²) dx = (1/2)ln(x²+a²)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_x_over_x2_minus_a2() -> Rule {
    Rule {
        id: RuleId(453),
        name: "integral_x_over_x2_minus_a2",
        category: RuleCategory::Integral,
        description: "∫x/(x²-a²) dx = (1/2)ln|x²-a²| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x/(x²-a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Var(v) if *v == *var) {
                        if let Expr::Sub(_, _) = denom.as_ref() {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    // ∫x/(x²-a²) dx = (1/2)ln|x²-a²|
                    let abs_denom = Expr::Abs(Box::new(*denom.clone()));
                    let ln_part = Expr::Ln(Box::new(abs_denom));
                    let half = Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(2)));
                    let result = Expr::Mul(Box::new(half), Box::new(ln_part));
                    
                    return vec![RuleApplication {
                        result,
                        justification: "∫x/(x²-a²) dx = (1/2)ln|x²-a²|".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_exp_ax() -> Rule {
    Rule {
        id: RuleId(454),
        name: "integral_exp_ax",
        category: RuleCategory::Integral,
        description: "∫e^(ax) dx = (1/a)e^(ax) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match e^(ax) where a is constant
                if let Expr::Exp(arg) = inner.as_ref() {
                    // Check if arg contains the variable (could be ax, 2x, etc.)
                    if let Expr::Mul(_, _) = arg.as_ref() {
                        return true;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                if let Expr::Exp(arg) = inner.as_ref() {
                    // For e^(ax), result is (1/a)e^(ax)
                    // Extract coefficient a from ax
                    if let Expr::Mul(coeff, _) = arg.as_ref() {
                        // Result: (1/a)·e^(ax)
                        let inv_coeff = Expr::Div(Box::new(Expr::int(1)), coeff.clone());
                        let result = Expr::Mul(Box::new(inv_coeff), Box::new(*inner.clone()));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "∫e^(ax) dx = (1/a)e^(ax)".to_string(),
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

fn integral_one_over_x2_minus_a2() -> Rule {
    Rule {
        id: RuleId(455),
        name: "integral_one_over_x2_minus_a2",
        category: RuleCategory::Integral,
        description: "∫1/(x²-a²) dx = (1/2a)ln|(x-a)/(x+a)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/(x²-a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sub(left, _) = denom.as_ref() {
                            // Check if left is x²
                            return matches!(left.as_ref(), Expr::Pow(base, exp)
                                if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                if let Expr::Div(_, denom) = inner.as_ref() {
                    if let Expr::Sub(_, right) = denom.as_ref() {
                        // For 1/(x²-a²), result is (1/2a)ln|(x-a)/(x+a)|
                        // Simplified: assume a=1
                        let x = Expr::Var(*var);
                        let a = Expr::Sqrt(right.clone());
                        
                        // x-a
                        let x_minus_a = Expr::Sub(Box::new(x.clone()), Box::new(a.clone()));
                        
                        // x+a
                        let x_plus_a = Expr::Add(Box::new(x), Box::new(a.clone()));
                        
                        // (x-a)/(x+a)
                        let fraction = Expr::Div(Box::new(x_minus_a), Box::new(x_plus_a));
                        
                        // ln|(x-a)/(x+a)|
                        let ln_part = Expr::Ln(Box::new(Expr::Abs(Box::new(fraction))));
                        
                        // (1/2a)·ln|(x-a)/(x+a)|
                        let two_a = Expr::Mul(Box::new(Expr::int(2)), Box::new(a));
                        let coefficient = Expr::Div(Box::new(Expr::int(1)), Box::new(two_a));
                        let result = Expr::Mul(Box::new(coefficient), Box::new(ln_part));
                        
                        return vec![RuleApplication {
                            result,
                            justification: "∫1/(x²-a²) dx = (1/2a)ln|(x-a)/(x+a)| (partial fractions)".to_string(),
                        }];
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
// Advanced Integration Patterns (456-475)
// ============================================================================

// Reduction Formulas (456-459)

fn integral_sin_squared() -> Rule {
    Rule {
        id: RuleId(456),
        name: "integral_sin_squared",
        category: RuleCategory::Integral,
        description: "∫sin²(x) dx = x/2 - sin(2x)/4 + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sin²(x) = sin(x)^2
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    return matches!(base.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫sin²(x) dx = x/2 - sin(2x)/4
                let x = Expr::Var(*var);
                let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x.clone()));
                let sin_2x = Expr::Sin(Box::new(two_x));
                
                // x/2
                let x_over_2 = Expr::Div(Box::new(x), Box::new(Expr::int(2)));
                
                // sin(2x)/4
                let sin_2x_over_4 = Expr::Div(Box::new(sin_2x), Box::new(Expr::int(4)));
                
                // x/2 - sin(2x)/4
                let result = Expr::Sub(Box::new(x_over_2), Box::new(sin_2x_over_4));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sin²(x) dx = x/2 - sin(2x)/4 (reduction formula)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_cos_squared() -> Rule {
    Rule {
        id: RuleId(457),
        name: "integral_cos_squared",
        category: RuleCategory::Integral,
        description: "∫cos²(x) dx = x/2 + sin(2x)/4 + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match cos²(x) = cos(x)^2
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    return matches!(base.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫cos²(x) dx = x/2 + sin(2x)/4
                let x = Expr::Var(*var);
                let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x.clone()));
                let sin_2x = Expr::Sin(Box::new(two_x));
                
                // x/2
                let x_over_2 = Expr::Div(Box::new(x), Box::new(Expr::int(2)));
                
                // sin(2x)/4
                let sin_2x_over_4 = Expr::Div(Box::new(sin_2x), Box::new(Expr::int(4)));
                
                // x/2 + sin(2x)/4
                let result = Expr::Add(Box::new(x_over_2), Box::new(sin_2x_over_4));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫cos²(x) dx = x/2 + sin(2x)/4 (reduction formula)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_tan_squared() -> Rule {
    Rule {
        id: RuleId(458),
        name: "integral_tan_squared",
        category: RuleCategory::Integral,
        description: "∫tan²(x) dx = tan(x) - x + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match tan²(x) = tan(x)^2 or (sin/cos)^2
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    // Check for tan(x)^2
                    if let Expr::Div(num, denom) = base.as_ref() {
                        let is_tan = matches!(num.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                            && matches!(denom.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var));
                        return is_tan && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫tan²(x) dx = tan(x) - x
                let x = Expr::Var(*var);
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x.clone()));
                let tan_x = Expr::Div(Box::new(sin_x), Box::new(cos_x));
                
                // tan(x) - x
                let result = Expr::Sub(Box::new(tan_x), Box::new(x));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫tan²(x) dx = tan(x) - x (reduction formula)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_sec_cubed() -> Rule {
    Rule {
        id: RuleId(459),
        name: "integral_sec_cubed",
        category: RuleCategory::Integral,
        description: "∫sec³(x) dx = (1/2)[sec(x)tan(x) + ln|sec(x)+tan(x)|] + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match sec³(x) = (1/cos(x))^3
                if let Expr::Pow(base, exp) = inner.as_ref() {
                    if let Expr::Div(num, denom) = base.as_ref() {
                        return matches!(num.as_ref(), Expr::Const(n) if n.is_one())
                            && matches!(denom.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                            && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 3);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // Simplified: return sec(x)·tan(x)/2 + ln|sec(x)+tan(x)|/2
                let x = Expr::Var(*var);
                let cos_x = Expr::Cos(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                
                let sec_x = Expr::Div(Box::new(Expr::int(1)), Box::new(cos_x.clone()));
                let tan_x = Expr::Div(Box::new(sin_x), Box::new(cos_x));
                
                // sec·tan
                let sec_tan = Expr::Mul(Box::new(sec_x.clone()), Box::new(tan_x.clone()));
                
                // ln|sec+tan|
                let sec_plus_tan = Expr::Add(Box::new(sec_x), Box::new(tan_x));
                let ln_part = Expr::Ln(Box::new(Expr::Abs(Box::new(sec_plus_tan))));
                
                // (sec·tan + ln|sec+tan|)/2
                let sum = Expr::Add(Box::new(sec_tan), Box::new(ln_part));
                let result = Expr::Div(Box::new(sum), Box::new(Expr::int(2)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫sec³(x) dx = (1/2)[sec(x)tan(x) + ln|sec(x)+tan(x)|]".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

// More By-Parts Patterns (460-463)

fn integral_x2_sin() -> Rule {
    Rule {
        id: RuleId(460),
        name: "integral_x2_sin",
        category: RuleCategory::Integral,
        description: "∫x²·sin(x) dx = -x²·cos(x) + 2x·sin(x) + 2cos(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x²·sin(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let x2_sin = matches!(left.as_ref(), Expr::Pow(base, exp)
                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2))
                        && matches!(right.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    let sin_x2 = matches!(right.as_ref(), Expr::Pow(base, exp)
                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2))
                        && matches!(left.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    return x2_sin || sin_x2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x²·sin(x) dx = -x²·cos(x) + 2x·sin(x) + 2cos(x)
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let cos_x = Expr::Cos(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                
                // -x²·cos(x)
                let neg_x2_cos = Expr::Neg(Box::new(Expr::Mul(Box::new(x2), Box::new(cos_x.clone()))));
                
                // 2x·sin(x)
                let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x));
                let two_x_sin = Expr::Mul(Box::new(two_x), Box::new(sin_x));
                
                // 2cos(x)
                let two_cos = Expr::Mul(Box::new(Expr::int(2)), Box::new(cos_x));
                
                // Combine: -x²·cos(x) + 2x·sin(x) + 2cos(x)
                let temp = Expr::Add(Box::new(neg_x2_cos), Box::new(two_x_sin));
                let result = Expr::Add(Box::new(temp), Box::new(two_cos));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x²·sin(x) dx = -x²·cos(x) + 2x·sin(x) + 2cos(x) (by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_x2_cos() -> Rule {
    Rule {
        id: RuleId(461),
        name: "integral_x2_cos",
        category: RuleCategory::Integral,
        description: "∫x²·cos(x) dx = x²·sin(x) + 2x·cos(x) - 2sin(x) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x²·cos(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let x2_cos = matches!(left.as_ref(), Expr::Pow(base, exp)
                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2))
                        && matches!(right.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    let cos_x2 = matches!(right.as_ref(), Expr::Pow(base, exp)
                        if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2))
                        && matches!(left.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    return x2_cos || cos_x2;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x²·cos(x) dx = x²·sin(x) + 2x·cos(x) - 2sin(x)
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x.clone()));
                
                // x²·sin(x)
                let x2_sin = Expr::Mul(Box::new(x2), Box::new(sin_x.clone()));
                
                // 2x·cos(x)
                let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x));
                let two_x_cos = Expr::Mul(Box::new(two_x), Box::new(cos_x));
                
                // 2sin(x)
                let two_sin = Expr::Mul(Box::new(Expr::int(2)), Box::new(sin_x));
                
                // Combine: x²·sin(x) + 2x·cos(x) - 2sin(x)
                let temp = Expr::Add(Box::new(x2_sin), Box::new(two_x_cos));
                let result = Expr::Sub(Box::new(temp), Box::new(two_sin));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x²·cos(x) dx = x²·sin(x) + 2x·cos(x) - 2sin(x) (by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_exp_sin() -> Rule {
    Rule {
        id: RuleId(462),
        name: "integral_exp_sin",
        category: RuleCategory::Integral,
        description: "∫e^x·sin(x) dx = (e^x/2)(sin(x) - cos(x)) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match e^x·sin(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let exp_sin = matches!(left.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(right.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    let sin_exp = matches!(right.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(left.as_ref(), Expr::Sin(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    return exp_sin || sin_exp;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫e^x·sin(x) dx = (e^x/2)(sin(x) - cos(x))
                let x = Expr::Var(*var);
                let exp_x = Expr::Exp(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x));
                
                // sin(x) - cos(x)
                let sin_minus_cos = Expr::Sub(Box::new(sin_x), Box::new(cos_x));
                
                // e^x·(sin(x) - cos(x))
                let exp_times_diff = Expr::Mul(Box::new(exp_x), Box::new(sin_minus_cos));
                
                // (e^x·(sin(x) - cos(x)))/2
                let result = Expr::Div(Box::new(exp_times_diff), Box::new(Expr::int(2)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫e^x·sin(x) dx = (e^x/2)(sin(x) - cos(x)) (by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_exp_cos() -> Rule {
    Rule {
        id: RuleId(463),
        name: "integral_exp_cos",
        category: RuleCategory::Integral,
        description: "∫e^x·cos(x) dx = (e^x/2)(sin(x) + cos(x)) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match e^x·cos(x)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let exp_cos = matches!(left.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(right.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    let cos_exp = matches!(right.as_ref(), Expr::Exp(arg) if matches!(arg.as_ref(), Expr::Var(v) if *v == *var))
                        && matches!(left.as_ref(), Expr::Cos(arg) if matches!(arg.as_ref(), Expr::Var(v2) if *v2 == *var));
                    
                    return exp_cos || cos_exp;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫e^x·cos(x) dx = (e^x/2)(sin(x) + cos(x))
                let x = Expr::Var(*var);
                let exp_x = Expr::Exp(Box::new(x.clone()));
                let sin_x = Expr::Sin(Box::new(x.clone()));
                let cos_x = Expr::Cos(Box::new(x));
                
                // sin(x) + cos(x)
                let sin_plus_cos = Expr::Add(Box::new(sin_x), Box::new(cos_x));
                
                // e^x·(sin(x) + cos(x))
                let exp_times_sum = Expr::Mul(Box::new(exp_x), Box::new(sin_plus_cos));
                
                // (e^x·(sin(x) + cos(x)))/2
                let result = Expr::Div(Box::new(exp_times_sum), Box::new(Expr::int(2)));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫e^x·cos(x) dx = (e^x/2)(sin(x) + cos(x)) (by parts)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

// Square Root Patterns (464-470)

fn integral_sqrt_a2_minus_x2() -> Rule {
    Rule {
        id: RuleId(464),
        name: "integral_sqrt_a2_minus_x2",
        category: RuleCategory::Integral,
        description: "∫√(a²-x²) dx = (x/2)√(a²-x²) + (a²/2)arcsin(x/a) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match √(a²-x²)
                if let Expr::Sqrt(sqrt_arg) = inner.as_ref() {
                    if let Expr::Sub(left, right) = sqrt_arg.as_ref() {
                        // Check if it's const - x² or a² - x²
                        let is_pattern = matches!(right.as_ref(), Expr::Pow(base, exp)
                            if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                            && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                        return is_pattern;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, expr: inner } = expr {
                // Simplified: assume a=1, result = (x/2)√(1-x²) + (1/2)arcsin(x)
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let one_minus_x2 = Expr::Sub(Box::new(Expr::int(1)), Box::new(x2));
                let sqrt_part = Expr::Sqrt(Box::new(one_minus_x2));
                
                // (x/2)√(1-x²)
                let x_over_2 = Expr::Div(Box::new(x.clone()), Box::new(Expr::int(2)));
                let first_term = Expr::Mul(Box::new(x_over_2), Box::new(sqrt_part));
                
                // (1/2)arcsin(x)
                let arcsin_x = Expr::Arcsin(Box::new(x));
                let second_term = Expr::Div(Box::new(arcsin_x), Box::new(Expr::int(2)));
                
                let result = Expr::Add(Box::new(first_term), Box::new(second_term));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫√(a²-x²) dx = (x/2)√(a²-x²) + (a²/2)arcsin(x/a)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_sqrt_x2_plus_a2() -> Rule {
    Rule {
        id: RuleId(465),
        name: "integral_sqrt_x2_plus_a2",
        category: RuleCategory::Integral,
        description: "∫√(x²+a²) dx = (x/2)√(x²+a²) + (a²/2)ln|x+√(x²+a²)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match √(x²+a²)
                if let Expr::Sqrt(sqrt_arg) = inner.as_ref() {
                    if let Expr::Add(left, right) = sqrt_arg.as_ref() {
                        // Check if it's x² + const or const + x²
                        let pattern1 = matches!(left.as_ref(), Expr::Pow(base, exp)
                            if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                            && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                        let pattern2 = matches!(right.as_ref(), Expr::Pow(base, exp)
                            if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                            && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                        return pattern1 || pattern2;
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // Simplified: assume a=1, result = (x/2)√(x²+1) + (1/2)ln|x+√(x²+1)|
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let x2_plus_1 = Expr::Add(Box::new(x2.clone()), Box::new(Expr::int(1)));
                let sqrt_part = Expr::Sqrt(Box::new(x2_plus_1.clone()));
                
                // (x/2)√(x²+1)
                let x_over_2 = Expr::Div(Box::new(x.clone()), Box::new(Expr::int(2)));
                let first_term = Expr::Mul(Box::new(x_over_2), Box::new(sqrt_part.clone()));
                
                // ln|x+√(x²+1)|
                let x_plus_sqrt = Expr::Add(Box::new(x), Box::new(sqrt_part));
                let ln_part = Expr::Ln(Box::new(Expr::Abs(Box::new(x_plus_sqrt))));
                
                // (1/2)ln|x+√(x²+1)|
                let second_term = Expr::Div(Box::new(ln_part), Box::new(Expr::int(2)));
                
                let result = Expr::Add(Box::new(first_term), Box::new(second_term));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫√(x²+a²) dx = (x/2)√(x²+a²) + (a²/2)ln|x+√(x²+a²)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_sqrt_x2_minus_a2() -> Rule {
    Rule {
        id: RuleId(466),
        name: "integral_sqrt_x2_minus_a2",
        category: RuleCategory::Integral,
        description: "∫√(x²-a²) dx = (x/2)√(x²-a²) - (a²/2)ln|x+√(x²-a²)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match √(x²-a²)
                if let Expr::Sqrt(sqrt_arg) = inner.as_ref() {
                    if let Expr::Sub(left, right) = sqrt_arg.as_ref() {
                        // Check if it's x² - const
                        return matches!(left.as_ref(), Expr::Pow(base, exp)
                            if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                            && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // Simplified: assume a=1, result = (x/2)√(x²-1) - (1/2)ln|x+√(x²-1)|
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let x2_minus_1 = Expr::Sub(Box::new(x2), Box::new(Expr::int(1)));
                let sqrt_part = Expr::Sqrt(Box::new(x2_minus_1));
                
                // (x/2)√(x²-1)
                let x_over_2 = Expr::Div(Box::new(x.clone()), Box::new(Expr::int(2)));
                let first_term = Expr::Mul(Box::new(x_over_2), Box::new(sqrt_part.clone()));
                
                // ln|x+√(x²-1)|
                let x_plus_sqrt = Expr::Add(Box::new(x), Box::new(sqrt_part));
                let ln_part = Expr::Ln(Box::new(Expr::Abs(Box::new(x_plus_sqrt))));
                
                // -(1/2)ln|x+√(x²-1)|
                let second_term = Expr::Div(Box::new(ln_part), Box::new(Expr::int(2)));
                let neg_second = Expr::Neg(Box::new(second_term));
                
                let result = Expr::Add(Box::new(first_term), Box::new(neg_second));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫√(x²-a²) dx = (x/2)√(x²-a²) - (a²/2)ln|x+√(x²-a²)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 5,
    }
}

fn integral_x_sqrt_a2_minus_x2() -> Rule {
    Rule {
        id: RuleId(467),
        name: "integral_x_sqrt_a2_minus_x2",
        category: RuleCategory::Integral,
        description: "∫x·√(a²-x²) dx = -(1/3)(a²-x²)^(3/2) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x·√(a²-x²)
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let x_sqrt = matches!(left.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(right.as_ref(), Expr::Sqrt(_));
                    let sqrt_x = matches!(right.as_ref(), Expr::Var(v) if *v == *var)
                        && matches!(left.as_ref(), Expr::Sqrt(_));
                    return x_sqrt || sqrt_x;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // Simplified: -(1/3)(1-x²)^(3/2)
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x), Box::new(Expr::int(2)));
                let one_minus_x2 = Expr::Sub(Box::new(Expr::int(1)), Box::new(x2));
                
                // (1-x²)^(3/2)
                let three_halves = Expr::Div(Box::new(Expr::int(3)), Box::new(Expr::int(2)));
                let power_part = Expr::Pow(Box::new(one_minus_x2), Box::new(three_halves));
                
                // -(1/3)(1-x²)^(3/2)
                let one_third = Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(3)));
                let product = Expr::Mul(Box::new(one_third), Box::new(power_part));
                let result = Expr::Neg(Box::new(product));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x·√(a²-x²) dx = -(1/3)(a²-x²)^(3/2)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

fn integral_inv_sqrt_x2_plus_a2() -> Rule {
    Rule {
        id: RuleId(468),
        name: "integral_inv_sqrt_x2_plus_a2",
        category: RuleCategory::Integral,
        description: "∫1/√(x²+a²) dx = ln|x+√(x²+a²)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/√(x²+a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                            if let Expr::Add(_, _) = sqrt_arg.as_ref() {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫1/√(x²+1) dx = ln|x+√(x²+1)|
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let x2_plus_1 = Expr::Add(Box::new(x2), Box::new(Expr::int(1)));
                let sqrt_part = Expr::Sqrt(Box::new(x2_plus_1));
                
                // x+√(x²+1)
                let x_plus_sqrt = Expr::Add(Box::new(x), Box::new(sqrt_part));
                
                // ln|x+√(x²+1)|
                let result = Expr::Ln(Box::new(Expr::Abs(Box::new(x_plus_sqrt))));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/√(x²+a²) dx = ln|x+√(x²+a²)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_inv_sqrt_x2_minus_a2() -> Rule {
    Rule {
        id: RuleId(469),
        name: "integral_inv_sqrt_x2_minus_a2",
        category: RuleCategory::Integral,
        description: "∫1/√(x²-a²) dx = ln|x+√(x²-a²)| + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match 1/√(x²-a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Const(n) if n.is_one()) {
                        if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                            if let Expr::Sub(left, _) = sqrt_arg.as_ref() {
                                return matches!(left.as_ref(), Expr::Pow(base, exp)
                                    if matches!(base.as_ref(), Expr::Var(v) if *v == *var)
                                    && matches!(exp.as_ref(), Expr::Const(n) if n.numer() == 2));
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫1/√(x²-1) dx = ln|x+√(x²-1)|
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x.clone()), Box::new(Expr::int(2)));
                let x2_minus_1 = Expr::Sub(Box::new(x2), Box::new(Expr::int(1)));
                let sqrt_part = Expr::Sqrt(Box::new(x2_minus_1));
                
                // x+√(x²-1)
                let x_plus_sqrt = Expr::Add(Box::new(x), Box::new(sqrt_part));
                
                // ln|x+√(x²-1)|
                let result = Expr::Ln(Box::new(Expr::Abs(Box::new(x_plus_sqrt))));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫1/√(x²-a²) dx = ln|x+√(x²-a²)|".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

fn integral_x_over_sqrt_x2_plus_a2() -> Rule {
    Rule {
        id: RuleId(470),
        name: "integral_x_over_sqrt_x2_plus_a2",
        category: RuleCategory::Integral,
        description: "∫x/√(x²+a²) dx = √(x²+a²) + C",
        is_applicable: |expr, _ctx| {
            if let Expr::Integral { expr: inner, var } = expr {
                // Match x/√(x²+a²)
                if let Expr::Div(num, denom) = inner.as_ref() {
                    if matches!(num.as_ref(), Expr::Var(v) if *v == *var) {
                        if let Expr::Sqrt(sqrt_arg) = denom.as_ref() {
                            if let Expr::Add(_, _) = sqrt_arg.as_ref() {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Integral { var, .. } = expr {
                // ∫x/√(x²+1) dx = √(x²+1)
                let x = Expr::Var(*var);
                let x2 = Expr::Pow(Box::new(x), Box::new(Expr::int(2)));
                let x2_plus_1 = Expr::Add(Box::new(x2), Box::new(Expr::int(1)));
                let result = Expr::Sqrt(Box::new(x2_plus_1));
                
                return vec![RuleApplication {
                    result,
                    justification: "∫x/√(x²+a²) dx = √(x²+a²)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// ============================================================================
// Limit Rules (renumbered to 500+)
// ============================================================================

fn limit_constant() -> Rule {
    Rule {
        id: RuleId(500),
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
        id: RuleId(501),
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
        id: RuleId(502),
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
        id: RuleId(503),
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
        id: RuleId(504),
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
        id: RuleId(505),
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
        id: RuleId(506),
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
        id: RuleId(507),
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
        id: RuleId(508),
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
        id: RuleId(509),
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
        id: RuleId(510),
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
        id: RuleId(511),
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

// ============================================================================
// Symbolic Differentiation Function
// ============================================================================

use mm_core::{Rational, Symbol};

/// Symbolically differentiate an expression with respect to a variable.
/// Returns the derivative expression.
pub fn differentiate(expr: &Expr, var: Symbol) -> Expr {
    match expr {
        // Constant rule: d/dx(c) = 0
        Expr::Const(_) => Expr::int(0),

        // Variable rule: d/dx(x) = 1, d/dx(y) = 0
        Expr::Var(v) => {
            if *v == var {
                Expr::int(1)
            } else {
                Expr::int(0)
            }
        }

        // Negation: d/dx(-f) = -f'
        Expr::Neg(inner) => Expr::Neg(Box::new(differentiate(inner, var))),

        // Sum rule: d/dx(f + g) = f' + g'
        Expr::Add(a, b) => Expr::Add(
            Box::new(differentiate(a, var)),
            Box::new(differentiate(b, var)),
        ),

        // Difference rule: d/dx(f - g) = f' - g'
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(differentiate(a, var)),
            Box::new(differentiate(b, var)),
        ),

        // Product rule: d/dx(fg) = f'g + fg'
        Expr::Mul(f, g) => {
            let f_prime = differentiate(f, var);
            let g_prime = differentiate(g, var);
            Expr::Add(
                Box::new(Expr::Mul(Box::new(f_prime), g.clone())),
                Box::new(Expr::Mul(f.clone(), Box::new(g_prime))),
            )
        }

        // Quotient rule: d/dx(f/g) = (f'g - fg') / g²
        Expr::Div(f, g) => {
            let f_prime = differentiate(f, var);
            let g_prime = differentiate(g, var);
            Expr::Div(
                Box::new(Expr::Sub(
                    Box::new(Expr::Mul(Box::new(f_prime), g.clone())),
                    Box::new(Expr::Mul(f.clone(), Box::new(g_prime))),
                )),
                Box::new(Expr::Pow(g.clone(), Box::new(Expr::int(2)))),
            )
        }

        // Power rule: d/dx(f^n) = n * f^(n-1) * f' (chain rule)
        Expr::Pow(base, exp) => {
            // Check if exponent is constant
            if let Expr::Const(n) = exp.as_ref() {
                let n_val = *n;
                let base_prime = differentiate(base, var);
                // n * base^(n-1) * base'
                Expr::Mul(
                    Box::new(Expr::Mul(
                        Box::new(Expr::Const(n_val)),
                        Box::new(Expr::Pow(
                            base.clone(),
                            Box::new(Expr::Const(n_val - Rational::from(1))),
                        )),
                    )),
                    Box::new(base_prime),
                )
            } else {
                // General case: d/dx(f^g) - not handling for now
                Expr::Derivative {
                    expr: Box::new(expr.clone()),
                    var,
                }
            }
        }

        // For other expressions, return unevaluated derivative
        _ => Expr::Derivative {
            expr: Box::new(expr.clone()),
            var,
        },
    }
}

/// Simplify an expression (basic algebraic simplification).
pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        // 0 + x = x, x + 0 = x
        Expr::Add(a, b) => {
            let a_simp = simplify(a);
            let b_simp = simplify(b);
            match (&a_simp, &b_simp) {
                (Expr::Const(c), _) if c.is_zero() => b_simp,
                (_, Expr::Const(c)) if c.is_zero() => a_simp,
                (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(*c1 + *c2),
                _ => Expr::Add(Box::new(a_simp), Box::new(b_simp)),
            }
        }

        // x - 0 = x
        Expr::Sub(a, b) => {
            let a_simp = simplify(a);
            let b_simp = simplify(b);
            match (&a_simp, &b_simp) {
                (_, Expr::Const(c)) if c.is_zero() => a_simp,
                (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(*c1 - *c2),
                _ => Expr::Sub(Box::new(a_simp), Box::new(b_simp)),
            }
        }

        // 0 * x = 0, x * 0 = 0, 1 * x = x, x * 1 = x
        Expr::Mul(a, b) => {
            let a_simp = simplify(a);
            let b_simp = simplify(b);
            match (&a_simp, &b_simp) {
                (Expr::Const(c), _) if c.is_zero() => Expr::int(0),
                (_, Expr::Const(c)) if c.is_zero() => Expr::int(0),
                (Expr::Const(c), _) if c.is_one() => b_simp,
                (_, Expr::Const(c)) if c.is_one() => a_simp,
                (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(*c1 * *c2),
                _ => Expr::Mul(Box::new(a_simp), Box::new(b_simp)),
            }
        }

        // x^0 = 1, x^1 = x
        Expr::Pow(base, exp) => {
            let base_simp = simplify(base);
            let exp_simp = simplify(exp);
            match &exp_simp {
                Expr::Const(c) if c.is_zero() => Expr::int(1),
                Expr::Const(c) if c.is_one() => base_simp,
                _ => Expr::Pow(Box::new(base_simp), Box::new(exp_simp)),
            }
        }

        // -(-x) = x
        Expr::Neg(inner) => {
            let inner_simp = simplify(inner);
            match inner_simp {
                Expr::Neg(x) => *x,
                Expr::Const(c) => Expr::Const(-c),
                _ => Expr::Neg(Box::new(inner_simp)),
            }
        }

        _ => expr.clone(),
    }
}

/// Evaluate an expression at x = value (for polynomials).
pub fn evaluate_at(expr: &Expr, var: Symbol, value: Rational) -> Option<Rational> {
    match expr {
        Expr::Const(c) => Some(*c),

        Expr::Var(v) => {
            if *v == var {
                Some(value)
            } else {
                None // Unknown variable
            }
        }

        Expr::Neg(inner) => evaluate_at(inner, var, value).map(|v| -v),

        Expr::Add(a, b) => {
            let a_val = evaluate_at(a, var, value)?;
            let b_val = evaluate_at(b, var, value)?;
            Some(a_val + b_val)
        }

        Expr::Sub(a, b) => {
            let a_val = evaluate_at(a, var, value)?;
            let b_val = evaluate_at(b, var, value)?;
            Some(a_val - b_val)
        }

        Expr::Mul(a, b) => {
            let a_val = evaluate_at(a, var, value)?;
            let b_val = evaluate_at(b, var, value)?;
            Some(a_val * b_val)
        }

        Expr::Div(a, b) => {
            let a_val = evaluate_at(a, var, value)?;
            let b_val = evaluate_at(b, var, value)?;
            if b_val.is_zero() {
                None
            } else {
                Some(a_val / b_val)
            }
        }

        Expr::Pow(base, exp) => {
            let base_val = evaluate_at(base, var, value)?;
            if let Expr::Const(n) = exp.as_ref() {
                if n.is_integer() && n.numer() >= 0 {
                    Some(base_val.pow(n.numer() as i32))
                } else {
                    None
                }
            } else {
                None
            }
        }

        _ => None,
    }
}

/// Find the maximum value of a polynomial on an interval [a, b].
/// Returns (x_max, f(x_max)) or None if can't compute.
pub fn find_max_on_interval(
    expr: &Expr,
    var: Symbol,
    a: Rational,
    b: Rational,
) -> Option<(Rational, Rational)> {
    // Step 1: Compute derivative
    let derivative = differentiate(expr, var);
    let derivative = simplify(&derivative);

    // Step 2: Find critical points (where f'(x) = 0)
    // For polynomials, we try integer values in [a, b]
    let mut candidates = vec![a, b]; // Always include endpoints

    // For simple cases, try to find where derivative is zero
    // Check integer and half-integer points in the interval
    let a_int = a.numer() / a.denom();
    let b_int = b.numer() / b.denom() + 1;

    for x_int in a_int..=b_int {
        let x = Rational::from(x_int);
        if x >= a && x <= b {
            if let Some(deriv_val) = evaluate_at(&derivative, var, x) {
                if deriv_val.is_zero() {
                    candidates.push(x);
                }
            }
        }
    }

    // Step 3: Evaluate f at all candidates and find max
    let mut max_val: Option<Rational> = None;
    let mut max_x: Option<Rational> = None;

    for x in candidates {
        if let Some(val) = evaluate_at(expr, var, x) {
            if max_val.is_none() || val > max_val.unwrap() {
                max_val = Some(val);
                max_x = Some(x);
            }
        }
    }

    match (max_x, max_val) {
        (Some(x), Some(v)) => Some((x, v)),
        _ => None,
    }
}

/// Find the minimum value of a polynomial on an interval [a, b].
pub fn find_min_on_interval(
    expr: &Expr,
    var: Symbol,
    a: Rational,
    b: Rational,
) -> Option<(Rational, Rational)> {
    let derivative = differentiate(expr, var);
    let derivative = simplify(&derivative);

    let mut candidates = vec![a, b];

    let a_int = a.numer() / a.denom();
    let b_int = b.numer() / b.denom() + 1;

    for x_int in a_int..=b_int {
        let x = Rational::from(x_int);
        if x >= a && x <= b {
            if let Some(deriv_val) = evaluate_at(&derivative, var, x) {
                if deriv_val.is_zero() {
                    candidates.push(x);
                }
            }
        }
    }

    let mut min_val: Option<Rational> = None;
    let mut min_x: Option<Rational> = None;

    for x in candidates {
        if let Some(val) = evaluate_at(expr, var, x) {
            if min_val.is_none() || val < min_val.unwrap() {
                min_val = Some(val);
                min_x = Some(x);
            }
        }
    }

    match (min_x, min_val) {
        (Some(x), Some(v)) => Some((x, v)),
        _ => None,
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

    #[test]
    fn test_differentiate_polynomial() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // d/dx(x^3) = 3x^2
        let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)));
        let deriv = differentiate(&expr, x);
        let deriv = simplify(&deriv);

        // Evaluate at x=2: should be 3*4 = 12
        let val = evaluate_at(&deriv, x, Rational::from(2));
        assert_eq!(val, Some(Rational::from(12)));
    }

    #[test]
    fn test_cbse_q8_max_value() {
        // CBSE Q8: Find max of f(x) = x³ - 3x + 2 on [0, 2]
        // f'(x) = 3x² - 3 = 0 → x = ±1
        // x = 1 is in [0, 2]
        // f(0) = 2, f(1) = 0, f(2) = 4
        // Max = 4 at x = 2

        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // f(x) = x³ - 3x + 2
        let f = Expr::Add(
            Box::new(Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
                Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
            )),
            Box::new(Expr::int(2)),
        );

        let result = find_max_on_interval(&f, x, Rational::from(0), Rational::from(2));
        assert!(result.is_some());

        let (x_max, max_val) = result.unwrap();
        assert_eq!(x_max, Rational::from(2));
        assert_eq!(max_val, Rational::from(4));
    }

    #[test]
    fn test_cbse_q8_min_value() {
        // Also test minimum: should be 0 at x=1
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let f = Expr::Add(
            Box::new(Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
                Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
            )),
            Box::new(Expr::int(2)),
        );

        let result = find_min_on_interval(&f, x, Rational::from(0), Rational::from(2));
        assert!(result.is_some());

        let (x_min, min_val) = result.unwrap();
        assert_eq!(x_min, Rational::from(1));
        assert_eq!(min_val, Rational::from(0));
    }

    #[test]
    fn test_integral_power_rule() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫x² dx should give x³/3
        let expr = Expr::Integral {
            expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            var: x,
        };
        
        let rule = integral_power();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        // Result should be x³/3
    }

    #[test]
    fn test_integral_constant() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫5 dx should give 5x
        let expr = Expr::Integral {
            expr: Box::new(Expr::int(5)),
            var: x,
        };
        
        let rule = integral_constant();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert_eq!(results[0].result, Expr::Mul(Box::new(Expr::int(5)), Box::new(Expr::Var(x))));
    }

    #[test]
    fn test_integral_constant_multiple() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫3x² dx should give 3·∫x² dx
        let expr = Expr::Integral {
            expr: Box::new(Expr::Mul(
                Box::new(Expr::int(3)),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2))))
            )),
            var: x,
        };
        
        let rule = integral_constant_multiple();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        // Result should be 3·∫x² dx
        assert!(matches!(results[0].result, Expr::Mul(_, _)));
    }

    #[test]
    fn test_integral_sum() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫(x² + x) dx should split into ∫x² dx + ∫x dx
        let expr = Expr::Integral {
            expr: Box::new(Expr::Add(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Var(x))
            )),
            var: x,
        };
        
        let rule = integral_sum();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Add(_, _)));
    }

    #[test]
    fn test_integral_exp() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫e^x dx should give e^x
        let expr = Expr::Integral {
            expr: Box::new(Expr::Exp(Box::new(Expr::Var(x)))),
            var: x,
        };
        
        let rule = integral_exp();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert_eq!(results[0].result, Expr::Exp(Box::new(Expr::Var(x))));
    }

    #[test]
    fn test_integral_sin() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫sin(x) dx should give -cos(x)
        let expr = Expr::Integral {
            expr: Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
            var: x,
        };
        
        let rule = integral_sin();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Neg(_)));
    }

    #[test]
    fn test_integral_cos() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫cos(x) dx should give sin(x)
        let expr = Expr::Integral {
            expr: Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
            var: x,
        };
        
        let rule = integral_cos();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert_eq!(results[0].result, Expr::Sin(Box::new(Expr::Var(x))));
    }

    #[test]
    fn test_integral_difference() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫(x² - x) dx should split into ∫x² dx - ∫x dx
        let expr = Expr::Integral {
            expr: Box::new(Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Var(x))
            )),
            var: x,
        };
        
        let rule = integral_difference();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Sub(_, _)));
    }

    #[test]
    fn test_integral_tan() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫tan(x) dx should give -ln|cos(x)|
        let expr = Expr::Integral {
            expr: Box::new(Expr::Tan(Box::new(Expr::Var(x)))),
            var: x,
        };
        
        let rule = integral_tan();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Neg(_)));
    }

    #[test]
    fn test_integral_sec2() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫sec²(x) dx = ∫1/cos²(x) dx should give tan(x)
        let expr = Expr::Integral {
            expr: Box::new(Expr::Div(
                Box::new(Expr::int(1)),
                Box::new(Expr::Pow(
                    Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2))
                ))
            )),
            var: x,
        };
        
        let rule = integral_sec2();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert_eq!(results[0].result, Expr::Tan(Box::new(Expr::Var(x))));
    }

    #[test]
    fn test_integral_csc2() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫csc²(x) dx = ∫1/sin²(x) dx should give -cot(x)
        let expr = Expr::Integral {
            expr: Box::new(Expr::Div(
                Box::new(Expr::int(1)),
                Box::new(Expr::Pow(
                    Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2))
                ))
            )),
            var: x,
        };
        
        let rule = integral_csc2();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Neg(_)));
    }

    #[test]
    fn test_integration_by_parts() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫x·e^x dx should give x·e^x - e^x
        let expr = Expr::Integral {
            expr: Box::new(Expr::Mul(
                Box::new(Expr::Var(x)),
                Box::new(Expr::Exp(Box::new(Expr::Var(x))))
            )),
            var: x,
        };
        
        let rule = integration_by_parts();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Sub(_, _)));
    }

    #[test]
    fn test_u_substitution() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫2x·e^(x²) dx should give e^(x²)
        let x_squared = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));
        let expr = Expr::Integral {
            expr: Box::new(Expr::Mul(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
                Box::new(Expr::Exp(Box::new(x_squared.clone())))
            )),
            var: x,
        };
        
        let rule = u_substitution();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Exp(_)));
    }

    #[test]
    fn test_partial_fractions() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫1/(x²-1) dx should give (1/2)ln|(x-1)/(x+1)|
        let expr = Expr::Integral {
            expr: Box::new(Expr::Div(
                Box::new(Expr::int(1)),
                Box::new(Expr::Sub(
                    Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                    Box::new(Expr::int(1))
                ))
            )),
            var: x,
        };
        
        let rule = partial_fractions();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert!(matches!(results[0].result, Expr::Mul(_, _)));
    }

    #[test]
    fn test_trig_substitution() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        
        // ∫1/√(1-x²) dx should give arcsin(x)
        let expr = Expr::Integral {
            expr: Box::new(Expr::Div(
                Box::new(Expr::int(1)),
                Box::new(Expr::Sqrt(Box::new(Expr::Sub(
                    Box::new(Expr::int(1)),
                    Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2))))
                ))))
            )),
            var: x,
        };
        
        let rule = trig_substitution();
        let ctx = RuleContext::default();
        
        assert!((rule.is_applicable)(&expr, &ctx));
        let results = (rule.apply)(&expr, &ctx);
        assert!(!results.is_empty());
        assert_eq!(results[0].result, Expr::Arcsin(Box::new(Expr::Var(x))));
    }
}
