// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Trigonometric identity rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleContext, RuleId};
use mm_core::Expr;

/// Get all trigonometric rules.
pub fn trig_rules() -> Vec<Rule> {
    vec![
        pythagorean_identity(),
        sin_double_angle(),
        cos_double_angle(),
        sin_zero(),
        cos_zero(),
        tan_zero(),
    ]
}

// ============================================================================
// Rule 19: Pythagorean Identity sin²x + cos²x = 1
// ============================================================================

fn pythagorean_identity() -> Rule {
    Rule {
        id: RuleId(19),
        name: "pythagorean_identity",
        category: RuleCategory::TrigIdentity,
        description: "Pythagorean identity: sin²(x) + cos²(x) = 1",
        is_applicable: |expr, _ctx| {
            // Check for sin²(x) + cos²(x) pattern
            if let Expr::Add(left, right) = expr {
                let left_is_sin_sq = is_squared_trig(left, true);
                let right_is_cos_sq = is_squared_trig(right, false);
                let left_is_cos_sq = is_squared_trig(left, false);
                let right_is_sin_sq = is_squared_trig(right, true);

                return (left_is_sin_sq && right_is_cos_sq) || (left_is_cos_sq && right_is_sin_sq);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(left, right) = expr {
                let left_is_sin_sq = is_squared_trig(left, true);
                let right_is_cos_sq = is_squared_trig(right, false);
                let left_is_cos_sq = is_squared_trig(left, false);
                let right_is_sin_sq = is_squared_trig(right, true);

                if (left_is_sin_sq && right_is_cos_sq) || (left_is_cos_sq && right_is_sin_sq) {
                    // Check that they have the same argument
                    if let (Some(arg1), Some(arg2)) = (get_trig_arg(left), get_trig_arg(right)) {
                        if arg1 == arg2 {
                            return vec![RuleApplication {
                                result: Expr::int(1),
                                justification: "sin²(x) + cos²(x) = 1".to_string(),
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
// Rule 20: Sin Double Angle sin(2x) = 2sin(x)cos(x)
// ============================================================================

fn sin_double_angle() -> Rule {
    Rule {
        id: RuleId(20),
        name: "sin_double_angle",
        category: RuleCategory::TrigIdentity,
        description: "Double angle: 2sin(x)cos(x) = sin(2x)",
        is_applicable: |expr, _ctx| {
            // Check for 2 * sin(x) * cos(x) pattern
            if let Expr::Mul(outer_left, outer_right) = expr {
                // Check for 2 * (sin(x) * cos(x))
                if outer_left.as_ref() == &Expr::int(2) {
                    if let Expr::Mul(sin_part, cos_part) = outer_right.as_ref() {
                        return matches!(sin_part.as_ref(), Expr::Sin(_))
                            && matches!(cos_part.as_ref(), Expr::Cos(_));
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(outer_left, outer_right) = expr {
                if outer_left.as_ref() == &Expr::int(2) {
                    if let Expr::Mul(sin_part, cos_part) = outer_right.as_ref() {
                        if let (Expr::Sin(arg1), Expr::Cos(arg2)) =
                            (sin_part.as_ref(), cos_part.as_ref())
                        {
                            if arg1 == arg2 {
                                return vec![RuleApplication {
                                    result: Expr::Sin(Box::new(Expr::Mul(
                                        Box::new(Expr::int(2)),
                                        arg1.clone(),
                                    ))),
                                    justification: "2sin(x)cos(x) = sin(2x)".to_string(),
                                }];
                            }
                        }
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
// Rule 21: Cos Double Angle cos²x - sin²x = cos(2x)
// ============================================================================

fn cos_double_angle() -> Rule {
    Rule {
        id: RuleId(21),
        name: "cos_double_angle",
        category: RuleCategory::TrigIdentity,
        description: "Double angle: cos²(x) - sin²(x) = cos(2x)",
        is_applicable: |expr, _ctx| {
            // Check for cos²(x) - sin²(x) pattern
            if let Expr::Sub(left, right) = expr {
                return is_squared_trig(left, false) && is_squared_trig(right, true);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(left, right) = expr {
                if is_squared_trig(left, false) && is_squared_trig(right, true) {
                    if let (Some(arg1), Some(arg2)) = (get_trig_arg(left), get_trig_arg(right)) {
                        if arg1 == arg2 {
                            return vec![RuleApplication {
                                result: Expr::Cos(Box::new(Expr::Mul(
                                    Box::new(Expr::int(2)),
                                    Box::new(arg1),
                                ))),
                                justification: "cos²(x) - sin²(x) = cos(2x)".to_string(),
                            }];
                        }
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
// Rule 22: sin(0) = 0
// ============================================================================

fn sin_zero() -> Rule {
    Rule {
        id: RuleId(22),
        name: "sin_zero",
        category: RuleCategory::TrigIdentity,
        description: "sin(0) = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                return matches!(inner.as_ref(), Expr::Const(r) if r.is_zero());
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if matches!(inner.as_ref(), Expr::Const(r) if r.is_zero()) {
                    return vec![RuleApplication {
                        result: Expr::int(0),
                        justification: "sin(0) = 0".to_string(),
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
// Rule 23: cos(0) = 1
// ============================================================================

fn cos_zero() -> Rule {
    Rule {
        id: RuleId(23),
        name: "cos_zero",
        category: RuleCategory::TrigIdentity,
        description: "cos(0) = 1",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                return matches!(inner.as_ref(), Expr::Const(r) if r.is_zero());
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if matches!(inner.as_ref(), Expr::Const(r) if r.is_zero()) {
                    return vec![RuleApplication {
                        result: Expr::int(1),
                        justification: "cos(0) = 1".to_string(),
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
// Rule 24: tan(0) = 0
// ============================================================================

fn tan_zero() -> Rule {
    Rule {
        id: RuleId(24),
        name: "tan_zero",
        category: RuleCategory::TrigIdentity,
        description: "tan(0) = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Tan(inner) = expr {
                return matches!(inner.as_ref(), Expr::Const(r) if r.is_zero());
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Tan(inner) = expr {
                if matches!(inner.as_ref(), Expr::Const(r) if r.is_zero()) {
                    return vec![RuleApplication {
                        result: Expr::int(0),
                        justification: "tan(0) = 0".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

/// Check if expression is sin²(x) or cos²(x).
fn is_squared_trig(expr: &Expr, is_sin: bool) -> bool {
    if let Expr::Pow(base, exp) = expr {
        if exp.as_ref() == &Expr::int(2) {
            if is_sin {
                return matches!(base.as_ref(), Expr::Sin(_));
            } else {
                return matches!(base.as_ref(), Expr::Cos(_));
            }
        }
    }
    false
}

/// Extract the argument from a squared trig function.
fn get_trig_arg(expr: &Expr) -> Option<Expr> {
    if let Expr::Pow(base, _) = expr {
        match base.as_ref() {
            Expr::Sin(arg) | Expr::Cos(arg) => return Some(arg.as_ref().clone()),
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_pythagorean_identity() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let rule = pythagorean_identity();
        let ctx = RuleContext::default();

        // sin²(x) + cos²(x)
        let expr = Expr::Add(
            Box::new(Expr::Pow(
                Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
        );

        assert!(rule.can_apply(&expr, &ctx));
        let results = rule.apply(&expr, &ctx);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].result, Expr::int(1));
    }
}
