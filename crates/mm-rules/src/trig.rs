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
        // Core identities
        pythagorean_identity(),
        sin_double_angle(),
        cos_double_angle(),
        // Special values
        sin_zero(),
        cos_zero(),
        tan_zero(),
        sin_pi(),
        cos_pi(),
        sin_pi_over_2(),
        cos_pi_over_2(),
        sin_pi_over_4(),
        cos_pi_over_4(),
        sin_pi_over_6(),
        cos_pi_over_6(),
        sin_pi_over_3(),
        cos_pi_over_3(),
        // Additional identities
        tan_identity(),
        sec_identity(),
        csc_identity(),
        cot_identity(),
        sin_neg(),
        cos_neg(),
        tan_neg(),
        // Sum and difference formulas (simplified versions)
        sin_sum_formula(),
        cos_sum_formula(),
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

// ============================================================================
// Special Pi Values - sin(π) = 0, cos(π) = -1
// ============================================================================

fn sin_pi() -> Rule {
    Rule {
        id: RuleId(40),
        name: "sin_pi",
        category: RuleCategory::TrigIdentity,
        description: "sin(π) = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                return matches!(arg.as_ref(), Expr::Pi);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if matches!(arg.as_ref(), Expr::Pi) {
                    return vec![RuleApplication {
                        result: Expr::int(0),
                        justification: "sin(π) = 0".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

fn cos_pi() -> Rule {
    Rule {
        id: RuleId(41),
        name: "cos_pi",
        category: RuleCategory::TrigIdentity,
        description: "cos(π) = -1",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                return matches!(arg.as_ref(), Expr::Pi);
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if matches!(arg.as_ref(), Expr::Pi) {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::int(1))),
                        justification: "cos(π) = -1".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

fn sin_pi_over_2() -> Rule {
    Rule {
        id: RuleId(42),
        name: "sin_pi_over_2",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/2) = 1",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(2);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(2) {
                        return vec![RuleApplication {
                            result: Expr::int(1),
                            justification: "sin(π/2) = 1".to_string(),
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

fn cos_pi_over_2() -> Rule {
    Rule {
        id: RuleId(43),
        name: "cos_pi_over_2",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/2) = 0",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(2);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(2) {
                        return vec![RuleApplication {
                            result: Expr::int(0),
                            justification: "cos(π/2) = 0".to_string(),
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

fn sin_pi_over_4() -> Rule {
    Rule {
        id: RuleId(44),
        name: "sin_pi_over_4",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/4) = √2/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(4);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(4) {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Sqrt(Box::new(Expr::int(2)))),
                                Box::new(Expr::int(2)),
                            ),
                            justification: "sin(π/4) = √2/2".to_string(),
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

fn cos_pi_over_4() -> Rule {
    Rule {
        id: RuleId(45),
        name: "cos_pi_over_4",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/4) = √2/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(4);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(4) {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Sqrt(Box::new(Expr::int(2)))),
                                Box::new(Expr::int(2)),
                            ),
                            justification: "cos(π/4) = √2/2".to_string(),
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

fn sin_pi_over_6() -> Rule {
    Rule {
        id: RuleId(46),
        name: "sin_pi_over_6",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/6) = 1/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(6);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(6) {
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(2))),
                            justification: "sin(π/6) = 1/2".to_string(),
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

fn cos_pi_over_6() -> Rule {
    Rule {
        id: RuleId(47),
        name: "cos_pi_over_6",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/6) = √3/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(6);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(6) {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Sqrt(Box::new(Expr::int(3)))),
                                Box::new(Expr::int(2)),
                            ),
                            justification: "cos(π/6) = √3/2".to_string(),
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

fn sin_pi_over_3() -> Rule {
    Rule {
        id: RuleId(48),
        name: "sin_pi_over_3",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/3) = √3/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(3);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(3) {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::Sqrt(Box::new(Expr::int(3)))),
                                Box::new(Expr::int(2)),
                            ),
                            justification: "sin(π/3) = √3/2".to_string(),
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

fn cos_pi_over_3() -> Rule {
    Rule {
        id: RuleId(49),
        name: "cos_pi_over_3",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/3) = 1/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    return matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(3);
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Div(num, denom) = arg.as_ref() {
                    if matches!(num.as_ref(), Expr::Pi) && *denom.as_ref() == Expr::int(3) {
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::int(2))),
                            justification: "cos(π/3) = 1/2".to_string(),
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
// Trig Function Definitions - tan, sec, csc, cot
// ============================================================================

fn tan_identity() -> Rule {
    Rule {
        id: RuleId(50),
        name: "tan_identity",
        category: RuleCategory::TrigIdentity,
        description: "tan(x) = sin(x)/cos(x)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Tan(_)),
        apply: |expr, _ctx| {
            if let Expr::Tan(arg) = expr {
                return vec![RuleApplication {
                    result: Expr::Div(
                        Box::new(Expr::Sin(arg.clone())),
                        Box::new(Expr::Cos(arg.clone())),
                    ),
                    justification: "tan(x) = sin(x)/cos(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

fn sec_identity() -> Rule {
    Rule {
        id: RuleId(51),
        name: "sec_identity",
        category: RuleCategory::TrigIdentity,
        description: "1/cos(x) = sec(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Const(c) if c.is_one())
                    && matches!(denom.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(_, denom) = expr {
                if let Expr::Cos(arg) = denom.as_ref() {
                    // We don't have Sec in Expr, so just document the pattern exists
                    return vec![RuleApplication {
                        result: Expr::Pow(
                            Box::new(Expr::Cos(arg.clone())),
                            Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                        ),
                        justification: "1/cos(x) = cos(x)^(-1)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

fn csc_identity() -> Rule {
    Rule {
        id: RuleId(52),
        name: "csc_identity",
        category: RuleCategory::TrigIdentity,
        description: "1/sin(x) = csc(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Const(c) if c.is_one())
                    && matches!(denom.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(_, denom) = expr {
                if let Expr::Sin(arg) = denom.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Pow(
                            Box::new(Expr::Sin(arg.clone())),
                            Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                        ),
                        justification: "1/sin(x) = sin(x)^(-1)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

fn cot_identity() -> Rule {
    Rule {
        id: RuleId(53),
        name: "cot_identity",
        category: RuleCategory::TrigIdentity,
        description: "cos(x)/sin(x) = cot(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Cos(_))
                    && matches!(denom.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Div(num, denom) = expr {
                if let (Expr::Cos(arg1), Expr::Sin(arg2)) = (num.as_ref(), denom.as_ref()) {
                    if arg1 == arg2 {
                        // cot(x) = 1/tan(x)
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::int(1)),
                                Box::new(Expr::Tan(arg1.clone())),
                            ),
                            justification: "cos(x)/sin(x) = cot(x) = 1/tan(x)".to_string(),
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
// Negative Angle Identities
// ============================================================================

fn sin_neg() -> Rule {
    Rule {
        id: RuleId(54),
        name: "sin_neg",
        category: RuleCategory::TrigIdentity,
        description: "sin(-x) = -sin(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                return matches!(arg.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(arg) = expr {
                if let Expr::Neg(inner) = arg.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::Sin(inner.clone()))),
                        justification: "sin(-x) = -sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

fn cos_neg() -> Rule {
    Rule {
        id: RuleId(55),
        name: "cos_neg",
        category: RuleCategory::TrigIdentity,
        description: "cos(-x) = cos(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                return matches!(arg.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(arg) = expr {
                if let Expr::Neg(inner) = arg.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Cos(inner.clone()),
                        justification: "cos(-x) = cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

fn tan_neg() -> Rule {
    Rule {
        id: RuleId(56),
        name: "tan_neg",
        category: RuleCategory::TrigIdentity,
        description: "tan(-x) = -tan(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Tan(arg) = expr {
                return matches!(arg.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Tan(arg) = expr {
                if let Expr::Neg(inner) = arg.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::Tan(inner.clone()))),
                        justification: "tan(-x) = -tan(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// ============================================================================
// Sum/Difference Formulas (simplified - detecting 2sin(x)cos(x) pattern for sin(2x))
// ============================================================================

fn sin_sum_formula() -> Rule {
    Rule {
        id: RuleId(57),
        name: "sin_sum_formula",
        category: RuleCategory::TrigIdentity,
        description: "2·sin(x)·cos(x) = sin(2x)",
        is_applicable: |expr, _ctx| {
            // Pattern: 2 * sin(x) * cos(x)
            if let Expr::Mul(a, b) = expr {
                if let Expr::Mul(c, d) = a.as_ref() {
                    // Check for 2 * sin(x) * cos(x)
                    if matches!(c.as_ref(), Expr::Const(n) if *n == mm_core::Rational::from_integer(2))
                    {
                        if matches!(d.as_ref(), Expr::Sin(_)) && matches!(b.as_ref(), Expr::Cos(_))
                        {
                            return true;
                        }
                        if matches!(d.as_ref(), Expr::Cos(_)) && matches!(b.as_ref(), Expr::Sin(_))
                        {
                            return true;
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(a, b) = expr {
                if let Expr::Mul(c, d) = a.as_ref() {
                    if matches!(c.as_ref(), Expr::Const(n) if *n == mm_core::Rational::from_integer(2))
                    {
                        // Get the argument
                        let arg = if let Expr::Sin(arg) = d.as_ref() {
                            Some(arg.clone())
                        } else if let Expr::Sin(arg) = b.as_ref() {
                            Some(arg.clone())
                        } else {
                            None
                        };

                        if let Some(arg) = arg {
                            return vec![RuleApplication {
                                result: Expr::Sin(Box::new(Expr::Mul(Box::new(Expr::int(2)), arg))),
                                justification: "2·sin(x)·cos(x) = sin(2x)".to_string(),
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

fn cos_sum_formula() -> Rule {
    Rule {
        id: RuleId(58),
        name: "cos_sum_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos²(x) - sin²(x) = cos(2x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sub(left, right) = expr {
                let left_is_cos_sq = is_squared_trig(left, false);
                let right_is_sin_sq = is_squared_trig(right, true);
                return left_is_cos_sq && right_is_sin_sq;
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
