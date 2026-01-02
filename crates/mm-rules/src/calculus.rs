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
    vec![
        power_rule(),
        constant_rule(),
        sum_rule(),
        product_rule(),
        quotient_rule(),
        chain_rule_sin(),
        chain_rule_cos(),
        exp_rule(),
        ln_rule(),
    ]
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
        Expr::Equation { lhs, rhs } => contains_var(lhs, var) || contains_var(rhs, var),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
