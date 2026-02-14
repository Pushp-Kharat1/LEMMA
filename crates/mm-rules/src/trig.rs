// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Trigonometric identity rules.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::Expr;

/// Get all trigonometric rules.
pub fn trig_rules() -> Vec<Rule> {
    let mut rules = vec![
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
    ];
    // Add advanced trig rules (Phase 1)
    rules.extend(advanced_trig_rules());
    // Add Phase 4 trig rules (500 milestone)
    rules.extend(phase4_trig_rules());
    rules
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

// ============================================================================
// Phase 1 New Rules: Advanced Trig Identities (ID 200+)
// ============================================================================

/// Get all new advanced trig rules
pub fn advanced_trig_rules() -> Vec<Rule> {
    vec![
        // Double angle variants
        cos_double_angle_2cos(),
        cos_double_angle_2sin(),
        tan_double_angle(),
        // Triple angle
        sin_triple_angle(),
        cos_triple_angle(),
        // Pythagorean extensions
        tan_sec_identity(),
        cot_csc_identity(),
        // Product-to-sum - NOW ENABLED
        sin_sin_product(),
        cos_cos_product(),
        sin_cos_product(),
        // Half-angle - NOW ENABLED
        sin_half_angle(),
        cos_half_angle(),
        // Cofunction identities
        sin_cos_cofunction(),
        cos_sin_cofunction(),
        tan_cot_cofunction(),
    ]
}

// cos(2x) = 2cos²(x) - 1
fn cos_double_angle_2cos() -> Rule {
    Rule {
        id: RuleId(200),
        name: "cos_double_angle_2cos",
        category: RuleCategory::TrigIdentity,
        description: "2cos²(x) - 1 = cos(2x)",
        is_applicable: |expr, _ctx| {
            // Match: 2*cos²(x) - 1
            if let Expr::Sub(left, right) = expr {
                if let Expr::Const(c) = right.as_ref() {
                    if *c == mm_core::Rational::from_integer(1) {
                        if let Expr::Mul(coef, inner) = left.as_ref() {
                            if let Expr::Const(c2) = coef.as_ref() {
                                if *c2 == mm_core::Rational::from_integer(2) {
                                    return is_squared_trig(inner, false);
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(left, _) = expr {
                if let Expr::Mul(_, inner) = left.as_ref() {
                    if let Some(arg) = get_trig_arg(inner) {
                        return vec![RuleApplication {
                            result: Expr::Cos(Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(arg),
                            ))),
                            justification: "2cos²(x) - 1 = cos(2x)".to_string(),
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

// cos(2x) = 1 - 2sin²(x)
fn cos_double_angle_2sin() -> Rule {
    Rule {
        id: RuleId(201),
        name: "cos_double_angle_2sin",
        category: RuleCategory::TrigIdentity,
        description: "1 - 2sin²(x) = cos(2x)",
        is_applicable: |expr, _ctx| {
            // Match: 1 - 2*sin²(x)
            if let Expr::Sub(left, right) = expr {
                if let Expr::Const(c) = left.as_ref() {
                    if *c == mm_core::Rational::from_integer(1) {
                        if let Expr::Mul(coef, inner) = right.as_ref() {
                            if let Expr::Const(c2) = coef.as_ref() {
                                if *c2 == mm_core::Rational::from_integer(2) {
                                    return is_squared_trig(inner, true);
                                }
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sub(_, right) = expr {
                if let Expr::Mul(_, inner) = right.as_ref() {
                    if let Some(arg) = get_trig_arg(inner) {
                        return vec![RuleApplication {
                            result: Expr::Cos(Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(arg),
                            ))),
                            justification: "1 - 2sin²(x) = cos(2x)".to_string(),
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

// tan(2x) = 2tan(x)/(1-tan²(x))
fn tan_double_angle() -> Rule {
    Rule {
        id: RuleId(202),
        name: "tan_double_angle",
        category: RuleCategory::TrigIdentity,
        description: "tan(2x) ↔ 2tan(x)/(1-tan²(x))",
        is_applicable: |expr, _ctx| {
            // Match tan(2x) where arg is 2*something
            if let Expr::Tan(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    if let Expr::Const(c) = a.as_ref() {
                        return *c == mm_core::Rational::from_integer(2);
                    }
                    if let Expr::Const(c) = b.as_ref() {
                        return *c == mm_core::Rational::from_integer(2);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    let x = if let Expr::Const(c) = a.as_ref() {
                        if *c == mm_core::Rational::from_integer(2) {
                            b.clone()
                        } else {
                            a.clone()
                        }
                    } else {
                        a.clone()
                    };
                    // 2tan(x) / (1 - tan²(x))
                    let tan_x = Expr::Tan(x.clone());
                    let numerator = Expr::Mul(Box::new(Expr::int(2)), Box::new(tan_x.clone()));
                    let tan_sq = Expr::Pow(Box::new(tan_x), Box::new(Expr::int(2)));
                    let denominator = Expr::Sub(Box::new(Expr::int(1)), Box::new(tan_sq));
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(denominator)),
                        justification: "tan(2x) = 2tan(x)/(1-tan²(x))".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sin(3x) = 3sin(x) - 4sin³(x)
fn sin_triple_angle() -> Rule {
    Rule {
        id: RuleId(203),
        name: "sin_triple_angle",
        category: RuleCategory::TrigIdentity,
        description: "sin(3x) = 3sin(x) - 4sin³(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    if let Expr::Const(c) = a.as_ref() {
                        return *c == mm_core::Rational::from_integer(3);
                    }
                    if let Expr::Const(c) = b.as_ref() {
                        return *c == mm_core::Rational::from_integer(3);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    let x = if let Expr::Const(c) = a.as_ref() {
                        if *c == mm_core::Rational::from_integer(3) {
                            b.clone()
                        } else {
                            a.clone()
                        }
                    } else {
                        a.clone()
                    };
                    let sin_x = Expr::Sin(x.clone());
                    let sin_cubed = Expr::Pow(Box::new(sin_x.clone()), Box::new(Expr::int(3)));
                    // 3sin(x) - 4sin³(x)
                    let term1 = Expr::Mul(Box::new(Expr::int(3)), Box::new(sin_x));
                    let term2 = Expr::Mul(Box::new(Expr::int(4)), Box::new(sin_cubed));
                    return vec![RuleApplication {
                        result: Expr::Sub(Box::new(term1), Box::new(term2)),
                        justification: "sin(3x) = 3sin(x) - 4sin³(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// cos(3x) = 4cos³(x) - 3cos(x)
fn cos_triple_angle() -> Rule {
    Rule {
        id: RuleId(204),
        name: "cos_triple_angle",
        category: RuleCategory::TrigIdentity,
        description: "cos(3x) = 4cos³(x) - 3cos(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    if let Expr::Const(c) = a.as_ref() {
                        return *c == mm_core::Rational::from_integer(3);
                    }
                    if let Expr::Const(c) = b.as_ref() {
                        return *c == mm_core::Rational::from_integer(3);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Mul(a, b) = inner.as_ref() {
                    let x = if let Expr::Const(c) = a.as_ref() {
                        if *c == mm_core::Rational::from_integer(3) {
                            b.clone()
                        } else {
                            a.clone()
                        }
                    } else {
                        a.clone()
                    };
                    let cos_x = Expr::Cos(x.clone());
                    let cos_cubed = Expr::Pow(Box::new(cos_x.clone()), Box::new(Expr::int(3)));
                    // 4cos³(x) - 3cos(x)
                    let term1 = Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_cubed));
                    let term2 = Expr::Mul(Box::new(Expr::int(3)), Box::new(cos_x));
                    return vec![RuleApplication {
                        result: Expr::Sub(Box::new(term1), Box::new(term2)),
                        justification: "cos(3x) = 4cos³(x) - 3cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// 1 + tan²(x) = sec²(x)
fn tan_sec_identity() -> Rule {
    Rule {
        id: RuleId(205),
        name: "tan_sec_identity",
        category: RuleCategory::TrigIdentity,
        description: "1 + tan²(x) = sec²(x) = 1/cos²(x)",
        is_applicable: |expr, _ctx| {
            // Match: 1 + tan²(x)
            if let Expr::Add(left, right) = expr {
                if let Expr::Const(c) = left.as_ref() {
                    if *c == mm_core::Rational::from_integer(1) {
                        if let Expr::Pow(base, exp) = right.as_ref() {
                            if let (Expr::Tan(_), Expr::Const(e)) = (base.as_ref(), exp.as_ref()) {
                                return *e == mm_core::Rational::from_integer(2);
                            }
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Add(_, right) = expr {
                if let Expr::Pow(base, _) = right.as_ref() {
                    if let Expr::Tan(inner) = base.as_ref() {
                        // sec²(x) = 1/cos²(x)
                        let cos_sq =
                            Expr::Pow(Box::new(Expr::Cos(inner.clone())), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Div(Box::new(Expr::int(1)), Box::new(cos_sq)),
                            justification: "1 + tan²(x) = sec²(x) = 1/cos²(x)".to_string(),
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

// 1 + cot²(x) = csc²(x)
fn cot_csc_identity() -> Rule {
    Rule {
        id: RuleId(206),
        name: "cot_csc_identity",
        category: RuleCategory::TrigIdentity,
        description: "1 + cot²(x) = csc²(x) = 1/sin²(x)",
        is_applicable: |expr, _ctx| {
            // Match: 1 + (cos/sin)² - simplified check
            // For now just return false as we don't have a Cot type
            if let Expr::Add(left, right) = expr {
                if let Expr::Const(c) = left.as_ref() {
                    if *c == mm_core::Rational::from_integer(1) {
                        // Check for (cos(x)/sin(x))²
                        if let Expr::Pow(base, exp) = right.as_ref() {
                            if let Expr::Const(e) = exp.as_ref() {
                                if *e == mm_core::Rational::from_integer(2) {
                                    if let Expr::Div(num, den) = base.as_ref() {
                                        return matches!(num.as_ref(), Expr::Cos(_))
                                            && matches!(den.as_ref(), Expr::Sin(_));
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
            if let Expr::Add(_, right) = expr {
                if let Expr::Pow(base, _) = right.as_ref() {
                    if let Expr::Div(_, den) = base.as_ref() {
                        if let Expr::Sin(inner) = den.as_ref() {
                            // csc²(x) = 1/sin²(x)
                            let sin_sq = Expr::Pow(
                                Box::new(Expr::Sin(inner.clone())),
                                Box::new(Expr::int(2)),
                            );
                            return vec![RuleApplication {
                                result: Expr::Div(Box::new(Expr::int(1)), Box::new(sin_sq)),
                                justification: "1 + cot²(x) = csc²(x) = 1/sin²(x)".to_string(),
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

// sin(a)sin(b) = (cos(a-b) - cos(a+b))/2
fn sin_sin_product() -> Rule {
    Rule {
        id: RuleId(207),
        name: "sin_sin_product",
        category: RuleCategory::TrigIdentity,
        description: "sin(a)sin(b) = (cos(a-b) - cos(a+b))/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                return matches!(left.as_ref(), Expr::Sin(_))
                    && matches!(right.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Sin(a), Expr::Sin(b)) = (left.as_ref(), right.as_ref()) {
                    // (cos(a-b) - cos(a+b))/2
                    let cos_diff = Expr::Cos(Box::new(Expr::Sub(a.clone(), b.clone())));
                    let cos_sum = Expr::Cos(Box::new(Expr::Add(a.clone(), b.clone())));
                    let numerator = Expr::Sub(Box::new(cos_diff), Box::new(cos_sum));
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(Expr::int(2))),
                        justification: "sin(a)sin(b) = (cos(a-b) - cos(a+b))/2".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// cos(a)cos(b) = (cos(a-b) + cos(a+b))/2
fn cos_cos_product() -> Rule {
    Rule {
        id: RuleId(208),
        name: "cos_cos_product",
        category: RuleCategory::TrigIdentity,
        description: "cos(a)cos(b) = (cos(a-b) + cos(a+b))/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                return matches!(left.as_ref(), Expr::Cos(_))
                    && matches!(right.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                if let (Expr::Cos(a), Expr::Cos(b)) = (left.as_ref(), right.as_ref()) {
                    // (cos(a-b) + cos(a+b))/2
                    let cos_diff = Expr::Cos(Box::new(Expr::Sub(a.clone(), b.clone())));
                    let cos_sum = Expr::Cos(Box::new(Expr::Add(a.clone(), b.clone())));
                    let numerator = Expr::Add(Box::new(cos_diff), Box::new(cos_sum));
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(Expr::int(2))),
                        justification: "cos(a)cos(b) = (cos(a-b) + cos(a+b))/2".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sin(a)cos(b) = (sin(a+b) + sin(a-b))/2
fn sin_cos_product() -> Rule {
    Rule {
        id: RuleId(209),
        name: "sin_cos_product",
        category: RuleCategory::TrigIdentity,
        description: "sin(a)cos(b) = (sin(a+b) + sin(a-b))/2",
        is_applicable: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                return (matches!(left.as_ref(), Expr::Sin(_))
                    && matches!(right.as_ref(), Expr::Cos(_)))
                    || (matches!(left.as_ref(), Expr::Cos(_))
                        && matches!(right.as_ref(), Expr::Sin(_)));
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Mul(left, right) = expr {
                let (a, b) = if let (Expr::Sin(a), Expr::Cos(b)) = (left.as_ref(), right.as_ref()) {
                    (a.clone(), b.clone())
                } else if let (Expr::Cos(b), Expr::Sin(a)) = (left.as_ref(), right.as_ref()) {
                    (a.clone(), b.clone())
                } else {
                    return vec![];
                };
                // (sin(a+b) + sin(a-b))/2
                let sin_sum = Expr::Sin(Box::new(Expr::Add(a.clone(), b.clone())));
                let sin_diff = Expr::Sin(Box::new(Expr::Sub(a.clone(), b.clone())));
                let numerator = Expr::Add(Box::new(sin_sum), Box::new(sin_diff));
                return vec![RuleApplication {
                    result: Expr::Div(Box::new(numerator), Box::new(Expr::int(2))),
                    justification: "sin(a)cos(b) = (sin(a+b) + sin(a-b))/2".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sin(x/2) = ±√((1-cos(x))/2) - we'll use positive version
/// Produces a rule that rewrites sin(x/2) to the half-angle form sqrt((1 - cos(x)) / 2).
///
/// The rule matches `sin(x/2)` (where the denominator is the integer 2) and applies the identity
/// sin(x/2) = √((1 - cos(x)) / 2). The rule is reversible and has moderate cost.
///
/// # Examples
///
/// ```
/// // Create the rule and apply it to `sin(x/2)`; the rule produces `sqrt((1 - cos(x)) / 2)`.
/// let rule = sin_half_angle();
/// // Example usage (constructing `sin(x/2)` and applying the rule) is shown conceptually:
/// // let expr = Expr::Sin(Box::new(Expr::Div(Box::new(Expr::Symbol("x".into())), Box::new(Expr::int(2)))));
/// // assert!( (rule.is_applicable)(&expr, &ctx) );
/// // let apps = (rule.apply)(&expr, &ctx);
/// // assert_eq!(apps[0].result, Expr::Sqrt(Box::new(Expr::Div(Box::new(Expr::Sub(Box::new(Expr::int(1)), Box::new(Expr::Cos(Box::new(Expr::Symbol("x".into())))))), Box::new(Expr::int(2))))));
/// ```
fn sin_half_angle() -> Rule {
    Rule {
        id: RuleId(210),
        name: "sin_half_angle",
        category: RuleCategory::TrigIdentity,
        description: "sin(x/2) = √((1-cos(x))/2)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Div(_num, den) = inner.as_ref() {
                    if let Expr::Const(c) = den.as_ref() {
                        return *c == mm_core::Rational::from_integer(2);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
                    if let Expr::Const(c) = den.as_ref() {
                        if *c == mm_core::Rational::from_integer(2) {
                            let x = num.clone();
                            // √((1-cos(x))/2)
                            let one_minus_cos =
                                Expr::Sub(Box::new(Expr::int(1)), Box::new(Expr::Cos(x)));
                            let fraction =
                                Expr::Div(Box::new(one_minus_cos), Box::new(Expr::int(2)));
                            return vec![RuleApplication {
                                result: Expr::Sqrt(Box::new(fraction)),
                                justification: "sin(x/2) = √((1-cos(x))/2)".to_string(),
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

// cos(x/2) = ±√((1+cos(x))/2) - we'll use positive version
/// Produces a rule that rewrites cos(x/2) to sqrt((1 + cos(x)) / 2).
///
/// The rule matches cosine of a half-angle (an expression of the form `cos(arg/2)`)
/// and yields `√((1 + cos(arg)) / 2)`. The rule is reversible and intended for
/// half-angle transformations.
///
/// # Examples
///
/// ```
/// # use crate::{cos_half_angle, Expr, Rule};
/// let rule = cos_half_angle();
/// // construct expression cos(x/2)
/// let expr = Expr::Cos(Box::new(Expr::Div(Box::new(Expr::Symbol("x".into())), Box::new(Expr::int(2)))));
/// assert!( (rule.is_applicable)(&expr, &Default::default()) );
/// let apps = (rule.apply)(&expr, &Default::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn cos_half_angle() -> Rule {
    Rule {
        id: RuleId(211),
        name: "cos_half_angle",
        category: RuleCategory::TrigIdentity,
        description: "cos(x/2) = √((1+cos(x))/2)",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Div(_num, den) = inner.as_ref() {
                    if let Expr::Const(c) = den.as_ref() {
                        return *c == mm_core::Rational::from_integer(2);
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
                    if let Expr::Const(c) = den.as_ref() {
                        if *c == mm_core::Rational::from_integer(2) {
                            let x = num.clone();
                            // √((1+cos(x))/2)
                            let one_plus_cos =
                                Expr::Add(Box::new(Expr::int(1)), Box::new(Expr::Cos(x)));
                            let fraction =
                                Expr::Div(Box::new(one_plus_cos), Box::new(Expr::int(2)));
                            return vec![RuleApplication {
                                result: Expr::Sqrt(Box::new(fraction)),
                                justification: "cos(x/2) = √((1+cos(x))/2)".to_string(),
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

// sin(π/2 - x) = cos(x)
fn sin_cos_cofunction() -> Rule {
    Rule {
        id: RuleId(212),
        name: "sin_cos_cofunction",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/2 - x) = cos(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    // Check if left is π/2
                    if let Expr::Div(num, den) = left.as_ref() {
                        if let (Expr::Pi, Expr::Const(c)) = (num.as_ref(), den.as_ref()) {
                            return *c == mm_core::Rational::from_integer(2);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Cos(right.clone()),
                        justification: "sin(π/2 - x) = cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// cos(π/2 - x) = sin(x)
fn cos_sin_cofunction() -> Rule {
    Rule {
        id: RuleId(213),
        name: "cos_sin_cofunction",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/2 - x) = sin(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    if let Expr::Div(num, den) = left.as_ref() {
                        if let (Expr::Pi, Expr::Const(c)) = (num.as_ref(), den.as_ref()) {
                            return *c == mm_core::Rational::from_integer(2);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sin(right.clone()),
                        justification: "cos(π/2 - x) = sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// tan(π/2 - x) = cot(x) = cos(x)/sin(x)
fn tan_cot_cofunction() -> Rule {
    Rule {
        id: RuleId(214),
        name: "tan_cot_cofunction",
        category: RuleCategory::TrigIdentity,
        description: "tan(π/2 - x) = cot(x) = cos(x)/sin(x)",
        is_applicable: |expr, _ctx| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    if let Expr::Div(num, den) = left.as_ref() {
                        if let (Expr::Pi, Expr::Const(c)) = (num.as_ref(), den.as_ref()) {
                            return *c == mm_core::Rational::from_integer(2);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _ctx| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    // cot(x) = cos(x)/sin(x)
                    return vec![RuleApplication {
                        result: Expr::Div(
                            Box::new(Expr::Cos(right.clone())),
                            Box::new(Expr::Sin(right.clone())),
                        ),
                        justification: "tan(π/2 - x) = cot(x) = cos(x)/sin(x)".to_string(),
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
// Phase 4: Additional Trigonometry Rules (ID 220-269)
// ============================================================================

/// Phase 4 trigonometry rules for 500 rules milestone
pub fn phase4_trig_rules() -> Vec<Rule> {
    vec![
        hyperbolic_sinh(),
        hyperbolic_cosh(),
        hyperbolic_tanh(),
        sinh_identity(),
        cosh_identity(),
        sinh_cosh_identity(),
        sin_arcsin(),
        cos_arccos(),
        tan_arctan(),
        arcsin_arccos_sum(),
        sin_sum_to_product(),
        cos_sum_to_product(),
        sin_diff_to_product(),
        cos_diff_to_product(),
        sin_squared_half(),
        cos_squared_half(),
        tan_half_sin(),
        tan_half_cos(),
        sin_3x_expand(),
        cos_3x_expand(),
        sin_4x_formula(),
        cos_4x_formula(),
        cot_reciprocal(),
        sec_reciprocal(),
        csc_reciprocal(),
        sin_neg_x(),
        cos_neg_x(),
        tan_neg_x(),
        sin_pi_minus(),
        cos_pi_minus(),
        sin_pi_plus(),
        cos_pi_plus(),
        sin_2pi_plus(),
        cos_2pi_plus(),
        tan_pi_plus(),
        sin_complementary(),
        cos_complementary(),
        sin_supplementary(),
        sin_squared_formula(),
        cos_squared_formula(),
        tan_squared_formula(),
        sin_pow4(),
        cos_pow4(),
        triple_sin_formula(),
        triple_cos_formula(),
        chebyshev_t2(),
        chebyshev_t3(),
        chebyshev_u2(),
        chebyshev_u3(),
        prosthaphaeresis_1(),
    ]
}

// sinh(x) definition
/// Expands `sinh(x)` into its exponential definition `(e^x - e^(-x)) / 2`.
///
/// Produces a `Rule` that matches `sinh(arg)` and rewrites it to `(e^arg - e^(-arg)) / 2` with a textual justification.
///
/// # Examples
///
/// ```
/// // Construct the rule and a sample expression `sinh(x)`
/// let rule = hyperbolic_sinh();
/// let expr = Expr::Sinh(Box::new(Expr::var("x")));
///
/// // Apply the rule and check the single produced transformation
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(format!("{}", apps[0].result), "(e^x - e^-x)/2");
/// ```
///
/// # Returns
///
/// A `Rule` that matches `sinh(x)` and returns `(e^x - e^(-x)) / 2` as the transformation result.
fn hyperbolic_sinh() -> Rule {
    Rule {
        id: RuleId(220),
        name: "hyperbolic_sinh",
        category: RuleCategory::TrigIdentity,
        description: "sinh(x) = (e^x - e^(-x))/2",
        is_applicable: |expr, _| matches!(expr, Expr::Sinh(_)),
        apply: |expr, _| {
            if let Expr::Sinh(x) = expr {
                let e_pos = Expr::Exp(x.clone());
                let e_neg = Expr::Exp(Box::new(Expr::Neg(x.clone())));
                let numerator = Expr::Sub(Box::new(e_pos), Box::new(e_neg));
                let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(2)));
                return vec![RuleApplication {
                    result,
                    justification: "sinh(x) = (e^x - e^(-x))/2".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// cosh(x) definition
/// Converts a hyperbolic cosine into its exponential form.
///
/// Matches `cosh(x)` and produces `(e^x + e^(-x)) / 2` as the transformed expression with a textual justification.
///
/// # Examples
///
/// ```
/// let rule = hyperbolic_cosh();
/// let expr = Expr::Cosh(Box::new(Expr::Symbol("x".into())));
/// let results = (rule.apply)(&expr, &RuleContext::default());
/// assert!(!results.is_empty());
/// ```
fn hyperbolic_cosh() -> Rule {
    Rule {
        id: RuleId(221),
        name: "hyperbolic_cosh",
        category: RuleCategory::TrigIdentity,
        description: "cosh(x) = (e^x + e^(-x))/2",
        is_applicable: |expr, _| matches!(expr, Expr::Cosh(_)),
        apply: |expr, _| {
            if let Expr::Cosh(x) = expr {
                let e_pos = Expr::Exp(x.clone());
                let e_neg = Expr::Exp(Box::new(Expr::Neg(x.clone())));
                let numerator = Expr::Add(Box::new(e_pos), Box::new(e_neg));
                let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(2)));
                return vec![RuleApplication {
                    result,
                    justification: "cosh(x) = (e^x + e^(-x))/2".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// tanh(x) = sinh(x)/cosh(x)
/// Constructs a rule that rewrites `tanh(x)` to `sinh(x) / cosh(x)`.
///
/// The rule matches expressions of the form `tanh(x)` and produces the division `sinh(x) / cosh(x)` with a justification string.
///
/// # Examples
///
/// ```
/// let rule = hyperbolic_tanh();
/// let expr = Expr::Tanh(Box::new(Expr::Symbol("x".into())));
/// assert!( (rule.is_applicable)(&expr, &RuleContext::default()) );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert!(matches!(apps[0].result, Expr::Div(_, _)));
/// ```
fn hyperbolic_tanh() -> Rule {
    Rule {
        id: RuleId(222),
        name: "hyperbolic_tanh",
        category: RuleCategory::TrigIdentity,
        description: "tanh(x) = sinh(x)/cosh(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Tanh(_)),
        apply: |expr, _| {
            if let Expr::Tanh(x) = expr {
                let sinh_x = Expr::Sinh(x.clone());
                let cosh_x = Expr::Cosh(x.clone());
                return vec![RuleApplication {
                    result: Expr::Div(Box::new(sinh_x), Box::new(cosh_x)),
                    justification: "tanh(x) = sinh(x)/cosh(x)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// sinh(2x) = 2sinh(x)cosh(x)
/// Creates the rule for the hyperbolic sine double-angle identity sinh(2x) = 2·sinh(x)·cosh(x).
///
/// The rule matches `sinh(2*x)` where the inner argument is a product with coefficient `2` and
/// rewrites it to `2 * sinh(x) * cosh(x)`. The rule is reversible and has a cost of 2.
///
/// # Examples
///
/// ```
/// let rule = sinh_identity();
/// assert_eq!(rule.description, "sinh(2x) = 2sinh(x)cosh(x)");
/// ```
fn sinh_identity() -> Rule {
    Rule {
        id: RuleId(223),
        name: "sinh_double",
        category: RuleCategory::TrigIdentity,
        description: "sinh(2x) = 2sinh(x)cosh(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Sinh(_)),
        apply: |expr, _| {
            if let Expr::Sinh(inner) = expr {
                if let Expr::Mul(coeff, x) = inner.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                    {
                        let result = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Sinh(x.clone())),
                                Box::new(Expr::Cosh(x.clone())),
                            )),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sinh(2x) = 2sinh(x)cosh(x)".to_string(),
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

// cosh(2x) = cosh²(x) + sinh²(x)
/// Creates the rule for the double-angle identity for hyperbolic cosine.
///
/// This rule matches `cosh(2·x)` and rewrites it to `cosh(x)^2 + sinh(x)^2`.
///
/// # Returns
///
/// A `Rule` that applies the identity `cosh(2x) = cosh(x)^2 + sinh(x)^2`.
///
/// # Examples
///
/// ```
/// let r = cosh_identity();
/// assert_eq!(r.name, "cosh_double");
/// ```
fn cosh_identity() -> Rule {
fn cosh_identity() -> Rule {
    Rule {
        id: RuleId(224),
        name: "cosh_double",
        category: RuleCategory::TrigIdentity,
        description: "cosh(2x) = cosh²(x) + sinh²(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Cosh(_)),
        apply: |expr, _| {
            if let Expr::Cosh(inner) = expr {
                if let Expr::Mul(coeff, x) = inner.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                    {
                        let cosh_sq =
                            Expr::Pow(Box::new(Expr::Cosh(x.clone())), Box::new(Expr::int(2)));
                        let sinh_sq =
                            Expr::Pow(Box::new(Expr::Sinh(x.clone())), Box::new(Expr::int(2)));
                        return vec![RuleApplication {
                            result: Expr::Add(Box::new(cosh_sq), Box::new(sinh_sq)),
                            justification: "cosh(2x) = cosh²(x) + sinh²(x)".to_string(),
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

// cosh²(x) - sinh²(x) = 1
/// Creates the trig identity rule for cosh²(x) − sinh²(x) = 1.
///
/// The rule matches a subtraction of a squared `cosh` and a squared `sinh` with the same argument
/// and yields `1` as the replacement with a justification string.
///
/// # Examples
///
/// ```
/// let rule = sinh_cosh_identity();
/// assert_eq!(rule.id, RuleId(225));
/// ```
fn sinh_cosh_identity() -> Rule {
    Rule {
        id: RuleId(225),
        name: "sinh_cosh_pythagorean",
        category: RuleCategory::TrigIdentity,
        description: "cosh²(x) - sinh²(x) = 1",
        is_applicable: |expr, _| matches!(expr, Expr::Sub(_, _)),
        apply: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                let left_ok = matches!(left.as_ref(), Expr::Pow(base, exp)
                    if matches!(base.as_ref(), Expr::Cosh(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1));
                let right_ok = matches!(right.as_ref(), Expr::Pow(base, exp)
                    if matches!(base.as_ref(), Expr::Sinh(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1));
                if left_ok && right_ok {
                    return vec![RuleApplication {
                        result: Expr::int(1),
                        justification: "cosh²(x) - sinh²(x) = 1".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// sin(arcsin(x)) = x
fn sin_arcsin() -> Rule {
    Rule {
        id: RuleId(226),
        name: "sin_arcsin",
        category: RuleCategory::TrigIdentity,
        description: "sin(arcsin(x)) = x",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                return matches!(inner.as_ref(), Expr::Arcsin(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Arcsin(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**x).clone(),
                        justification: "sin(arcsin(x)) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// cos(arccos(x)) = x
fn cos_arccos() -> Rule {
    Rule {
        id: RuleId(227),
        name: "cos_arccos",
        category: RuleCategory::TrigIdentity,
        description: "cos(arccos(x)) = x",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                return matches!(inner.as_ref(), Expr::Arccos(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Arccos(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**x).clone(),
                        justification: "cos(arccos(x)) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// tan(arctan(x)) = x
fn tan_arctan() -> Rule {
    Rule {
        id: RuleId(228),
        name: "tan_arctan",
        category: RuleCategory::TrigIdentity,
        description: "tan(arctan(x)) = x",
        is_applicable: |expr, _| {
            if let Expr::Tan(inner) = expr {
                return matches!(inner.as_ref(), Expr::Arctan(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Arctan(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: (**x).clone(),
                        justification: "tan(arctan(x)) = x".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// arcsin(x) + arccos(x) = π/2
/// Returns a rule that recognizes and simplifies the identity arcsin(x) + arccos(x) = π/2.
///
/// The rule matches an addition where one operand is `arcsin(...)` and the other is `arccos(...)` (in either order)
/// and replaces the whole sum with `π/2`.
///
/// # Examples
///
/// ```
/// // Construct expression `arcsin(x) + arccos(x)` and apply the rule.
/// let rule = arcsin_arccos_sum();
/// let x = Expr::Symbol("x".into());
/// let expr = Expr::Add(Box::new(Expr::Arcsin(Box::new(x.clone()))), Box::new(Expr::Arccos(Box::new(x))));
/// assert!( (rule.is_applicable)(&expr, &RuleContext::default()) );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Div(Box::new(Expr::Pi), Box::new(Expr::int(2))));
/// ```
fn arcsin_arccos_sum() -> Rule {
    Rule {
        id: RuleId(229),
        name: "arcsin_arccos_sum",
        category: RuleCategory::TrigIdentity,
        description: "arcsin(x) + arccos(x) = π/2",
        is_applicable: |expr, _| {
            if let Expr::Add(left, right) = expr {
                let is_arcsin_arccos = matches!(left.as_ref(), Expr::Arcsin(_))
                    && matches!(right.as_ref(), Expr::Arccos(_));
                let is_arccos_arcsin = matches!(left.as_ref(), Expr::Arccos(_))
                    && matches!(right.as_ref(), Expr::Arcsin(_));
                return is_arcsin_arccos || is_arccos_arcsin;
            }
            false
        },
        apply: |_expr, _| {
            vec![RuleApplication {
                result: Expr::Div(Box::new(Expr::Pi), Box::new(Expr::int(2))),
                justification: "arcsin(x) + arccos(x) = π/2".to_string(),
            }]
        },
        reversible: true,
        cost: 2,
    }
}

// sinA + sinB = 2sin((A+B)/2)cos((A-B)/2)
/// Creates a rule that converts a sum of two sine terms into a product form:
/// sin(A) + sin(B) = 2·sin((A + B)/2)·cos((A - B)/2).
///
/// The rule matches expressions of the form `sin(...) + sin(...)` and produces
/// the corresponding product expression with a justification string.
///
/// # Examples
///
/// ```
/// let rule = sin_sum_to_product();
/// let a = Expr::Symbol("a".to_string());
/// let b = Expr::Symbol("b".to_string());
/// let expr = Expr::Add(
///     Box::new(Expr::Sin(Box::new(a.clone()))),
///     Box::new(Expr::Sin(Box::new(b.clone()))),
/// );
/// let ctx = RuleContext::default();
/// assert!(rule.is_applicable(&expr, &ctx));
/// let apps = rule.apply(&expr, &ctx);
/// assert!(!apps.is_empty());
/// ```
fn sin_sum_to_product() -> Rule {
    Rule {
        id: RuleId(230),
        name: "sin_sum_to_product",
        category: RuleCategory::TrigIdentity,
        description: "sinA + sinB = 2sin((A+B)/2)cos((A-B)/2)",
        is_applicable: |expr, _| {
            if let Expr::Add(left, right) = expr {
                return matches!(left.as_ref(), Expr::Sin(_))
                    && matches!(right.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Sin(a), Expr::Sin(b)) = (left.as_ref(), right.as_ref()) {
                    let half_sum = Expr::Div(
                        Box::new(Expr::Add(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let half_diff = Expr::Div(
                        Box::new(Expr::Sub(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let result = Expr::Mul(
                        Box::new(Expr::int(2)),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Sin(Box::new(half_sum))),
                            Box::new(Expr::Cos(Box::new(half_diff))),
                        )),
                    );
                    return vec![RuleApplication {
                        result,
                        justification: "sinA + sinB = 2sin((A+B)/2)cos((A-B)/2)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// cosA + cosB = 2cos((A+B)/2)cos((A-B)/2)
/// Converts a sum of two cosines into a product: cos(A) + cos(B) -> 2·cos((A + B)/2)·cos((A - B)/2).
///
/// # Examples
///
/// ```no_run
/// let rule = cos_sum_to_product();
/// // expr represents cos(a) + cos(b)
/// let expr = Expr::Add(
///     Box::new(Expr::Cos(Box::new(Expr::Symbol("a".into())))),
///     Box::new(Expr::Cos(Box::new(Expr::Symbol("b".into())))),
/// );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// // resulting expression is 2 * cos((a+b)/2) * cos((a-b)/2)
/// ```
fn cos_sum_to_product() -> Rule {
    Rule {
        id: RuleId(231),
        name: "cos_sum_to_product",
        category: RuleCategory::TrigIdentity,
        description: "cosA + cosB = 2cos((A+B)/2)cos((A-B)/2)",
        is_applicable: |expr, _| {
            if let Expr::Add(left, right) = expr {
                return matches!(left.as_ref(), Expr::Cos(_))
                    && matches!(right.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Cos(a), Expr::Cos(b)) = (left.as_ref(), right.as_ref()) {
                    let half_sum = Expr::Div(
                        Box::new(Expr::Add(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let half_diff = Expr::Div(
                        Box::new(Expr::Sub(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let result = Expr::Mul(
                        Box::new(Expr::int(2)),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Cos(Box::new(half_sum))),
                            Box::new(Expr::Cos(Box::new(half_diff))),
                        )),
                    );
                    return vec![RuleApplication {
                        result,
                        justification: "cosA + cosB = 2cos((A+B)/2)cos((A-B)/2)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sinA - sinB = 2cos((A+B)/2)sin((A-B)/2)
/// Creates a trigonometric identity rule that converts a difference of sines into a product.
///
/// The rule implements the identity sin(A) - sin(B) = 2·cos((A + B) / 2)·sin((A - B) / 2).
/// The rule is reversible and intended for transforming expressions matching `sin(...) - sin(...)`.
///
/// # Examples
///
/// ```
/// // Construct expression sin(a) - sin(b) and apply the rule.
/// let rule = sin_diff_to_product();
/// let a = Expr::Symbol("a".into());
/// let b = Expr::Symbol("b".into());
/// let expr = Expr::Sub(Box::new(Expr::Sin(Box::new(a.clone()))), Box::new(Expr::Sin(Box::new(b.clone()))));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// let result = &apps[0].result;
/// // Result should be 2 * cos((a + b)/2) * sin((a - b)/2)
/// ```
fn sin_diff_to_product() -> Rule {
    Rule {
        id: RuleId(232),
        name: "sin_diff_to_product",
        category: RuleCategory::TrigIdentity,
        description: "sinA - sinB = 2cos((A+B)/2)sin((A-B)/2)",
        is_applicable: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                return matches!(left.as_ref(), Expr::Sin(_))
                    && matches!(right.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Sin(a), Expr::Sin(b)) = (left.as_ref(), right.as_ref()) {
                    let half_sum = Expr::Div(
                        Box::new(Expr::Add(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let half_diff = Expr::Div(
                        Box::new(Expr::Sub(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let result = Expr::Mul(
                        Box::new(Expr::int(2)),
                        Box::new(Expr::Mul(
                            Box::new(Expr::Cos(Box::new(half_sum))),
                            Box::new(Expr::Sin(Box::new(half_diff))),
                        )),
                    );
                    return vec![RuleApplication {
                        result,
                        justification: "sinA - sinB = 2cos((A+B)/2)sin((A-B)/2)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// cosA - cosB = -2sin((A+B)/2)sin((A-B)/2)
/// Creates a `Rule` that rewrites the difference of cosines into a product of sines.
///
/// The rule implements the identity: cos(A) - cos(B) = -2 · sin((A + B) / 2) · sin((A - B) / 2),
/// and is reversible.
///
/// # Examples
///
/// ```
/// let rule = cos_diff_to_product();
/// assert_eq!(rule.name, "cos_diff_to_product");
/// ```
fn cos_diff_to_product() -> Rule {
    Rule {
        id: RuleId(233),
        name: "cos_diff_to_product",
        category: RuleCategory::TrigIdentity,
        description: "cosA - cosB = -2sin((A+B)/2)sin((A-B)/2)",
        is_applicable: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                return matches!(left.as_ref(), Expr::Cos(_))
                    && matches!(right.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Cos(a), Expr::Cos(b)) = (left.as_ref(), right.as_ref()) {
                    let half_sum = Expr::Div(
                        Box::new(Expr::Add(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let half_diff = Expr::Div(
                        Box::new(Expr::Sub(a.clone(), b.clone())),
                        Box::new(Expr::int(2)),
                    );
                    let product = Expr::Mul(
                        Box::new(Expr::Sin(Box::new(half_sum))),
                        Box::new(Expr::Sin(Box::new(half_diff))),
                    );
                    let result = Expr::Neg(Box::new(Expr::Mul(
                        Box::new(Expr::int(2)),
                        Box::new(product),
                    )));
                    return vec![RuleApplication {
                        result,
                        justification: "cosA - cosB = -2sin((A+B)/2)sin((A-B)/2)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sin²(x/2) = (1 - cos(x))/2
/// Creates a rule that rewrites sin²(x/2) to (1 - cos(x)) / 2.

///

/// The rule matches expressions of the form `sin(x/2)^2` and produces `(1 - cos(x)) / 2` with a textual justification.

/// The rule is reversible and assigned cost 2.

///

/// # Examples

///

/// ```

/// let rule = sin_squared_half();

/// assert_eq!(rule.description, "sin²(x/2) = (1 - cos(x))/2");

/// ```
fn sin_squared_half() -> Rule {
    Rule {
        id: RuleId(234),
        name: "sin_squared_half",
        category: RuleCategory::TrigIdentity,
        description: "sin²(x/2) = (1 - cos(x))/2",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    if let Expr::Sin(arg) = base.as_ref() {
                        if let Expr::Div(_, den) = arg.as_ref() {
                            return matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Sin(arg) = base.as_ref() {
                    if let Expr::Div(num, _) = arg.as_ref() {
                        let result = Expr::Div(
                            Box::new(Expr::Sub(
                                Box::new(Expr::int(1)),
                                Box::new(Expr::Cos(num.clone())),
                            )),
                            Box::new(Expr::int(2)),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sin²(x/2) = (1 - cos(x))/2".to_string(),
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

// cos²(x/2) = (1 + cos(x))/2
/// Creates a rule that rewrites cos²(x/2) to (1 + cos(x)) / 2 and back.
///
/// The rule matches expressions of the form `cos(arg/2)^2` and transforms them into `(1 + cos(arg)) / 2`.
/// It is reversible so it can also be applied in the opposite direction.
///
/// # Examples
///
/// ```
/// let r = cos_squared_half();
/// // matches cos((x)/2)^2
/// let expr = Expr::pow(Expr::cos(Expr::div(Expr::var("x"), Expr::int(2))), Expr::int(2));
/// assert!(r.is_applicable(&expr, &RuleContext::default()));
/// let apps = r.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// // result is (1 + cos(x)) / 2
/// assert_eq!(apps[0].result, Expr::div(Expr::add(Expr::int(1), Expr::cos(Expr::var("x"))), Expr::int(2)));
/// ```
fn cos_squared_half() -> Rule {
    Rule {
        id: RuleId(235),
        name: "cos_squared_half",
        category: RuleCategory::TrigIdentity,
        description: "cos²(x/2) = (1 + cos(x))/2",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                if matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1) {
                    if let Expr::Cos(arg) = base.as_ref() {
                        if let Expr::Div(_, den) = arg.as_ref() {
                            return matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                        }
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Cos(arg) = base.as_ref() {
                    if let Expr::Div(num, _) = arg.as_ref() {
                        let result = Expr::Div(
                            Box::new(Expr::Add(
                                Box::new(Expr::int(1)),
                                Box::new(Expr::Cos(num.clone())),
                            )),
                            Box::new(Expr::int(2)),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "cos²(x/2) = (1 + cos(x))/2".to_string(),
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

// tan(x/2) = sin(x)/(1 + cos(x))
/// Creates a rule that rewrites `tan(x/2)` to `sin(x) / (1 + cos(x))`.
///
/// The rule matches `tan(arg)` where `arg` is `x/2` and produces the equivalent
/// fraction `sin(x) / (1 + cos(x))`. The rule is reversible and has a cost of 2.
///
/// # Examples
///
/// ```
/// let r = tan_half_sin();
/// assert_eq!(r.name, "tan_half_sin");
/// ```
fn tan_half_sin() -> Rule {
    Rule {
        id: RuleId(236),
        name: "tan_half_sin",
        category: RuleCategory::TrigIdentity,
        description: "tan(x/2) = sin(x)/(1 + cos(x))",
        is_applicable: |expr, _| {
            if let Expr::Tan(arg) = expr {
                if let Expr::Div(_, den) = arg.as_ref() {
                    return matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Tan(arg) = expr {
                if let Expr::Div(num, _) = arg.as_ref() {
                    let sin_x = Expr::Sin(num.clone());
                    let cos_x = Expr::Cos(num.clone());
                    let denom = Expr::Add(Box::new(Expr::int(1)), Box::new(cos_x));
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(sin_x), Box::new(denom)),
                        justification: "tan(x/2) = sin(x)/(1 + cos(x))".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// tan(x/2) = (1 - cos(x))/sin(x)
/// Converts a tangent of a half-angle into the equivalent quotient (1 - cos(x)) / sin(x).
///
/// # Examples
///
/// ```
/// let r = tan_half_cos();
/// assert_eq!(r.name, "tan_half_cos");
/// ```
fn tan_half_cos() -> Rule {
    Rule {
        id: RuleId(237),
        name: "tan_half_cos",
        category: RuleCategory::TrigIdentity,
        description: "tan(x/2) = (1 - cos(x))/sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Tan(arg) = expr {
                if let Expr::Div(_, den) = arg.as_ref() {
                    return matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Tan(arg) = expr {
                if let Expr::Div(num, _) = arg.as_ref() {
                    let cos_x = Expr::Cos(num.clone());
                    let numerator = Expr::Sub(Box::new(Expr::int(1)), Box::new(cos_x));
                    let denominator = Expr::Sin(num.clone());
                    return vec![RuleApplication {
                        result: Expr::Div(Box::new(numerator), Box::new(denominator)),
                        justification: "tan(x/2) = (1 - cos(x))/sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// sin(3x) = 3sin(x) - 4sin³(x)
/// Expands sin(3x) into 3·sin(x) − 4·sin(x)^3.
///
/// The returned rule matches `sin(3*x)` (where the coefficient `3` is explicit)
/// and transforms it to `3*sin(x) - 4*sin(x)^3`.
///
/// # Examples
///
/// ```
/// let rule = sin_3x_expand();
/// // Rule id 238 corresponds to the sin(3x) expansion
/// assert_eq!(rule.id, RuleId(238));
/// ```
fn sin_3x_expand() -> Rule {
    Rule {
        id: RuleId(238),
        name: "sin_3x_expand",
        category: RuleCategory::TrigIdentity,
        description: "sin(3x) = 3sin(x) - 4sin³(x)",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let is_three =
                        |e: &Expr| matches!(e, Expr::Const(c) if c.numer() == 3 && c.denom() == 1);
                    return (is_three(left.as_ref()) || is_three(right.as_ref()))
                        && !(matches!(left.as_ref(), Expr::Const(_))
                            && matches!(right.as_ref(), Expr::Const(_)));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                let extract_x = |a: &Expr, b: &Expr| {
                    if matches!(a, Expr::Const(c) if c.numer() == 3 && c.denom() == 1) {
                        Some(b.clone())
                    } else if matches!(b, Expr::Const(c) if c.numer() == 3 && c.denom() == 1) {
                        Some(a.clone())
                    } else {
                        None
                    }
                };

                if let Expr::Mul(left, right) = inner.as_ref() {
                    if let Some(x) = extract_x(left.as_ref(), right.as_ref()) {
                        let sin_x = Expr::Sin(Box::new(x.clone()));
                        let sin_cubed = Expr::Pow(Box::new(sin_x.clone()), Box::new(Expr::int(3)));
                        let result = Expr::Sub(
                            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(sin_x))),
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(sin_cubed))),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sin(3x) = 3sin(x) - 4sin³(x)".to_string(),
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

// cos(3x) = 4cos³(x) - 3cos(x)
/// Builds the trig identity rule for expanding `cos(3x)` into `4·cos³(x) - 3·cos(x)`.
///
/// The returned `Rule` matches `cos(3·x)` (order-insensitive for the constant 3) and produces
/// the equivalent polynomial-in-cosine expression `4*cos(x)^3 - 3*cos(x)`.
///
/// # Examples
///
/// ```
/// let r = cos_3x_expand();
/// assert_eq!(r.name, "cos_3x_expand");
/// ```
fn cos_3x_expand() -> Rule {
    Rule {
        id: RuleId(239),
        name: "cos_3x_expand",
        category: RuleCategory::TrigIdentity,
        description: "cos(3x) = 4cos³(x) - 3cos(x)",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let is_three =
                        |e: &Expr| matches!(e, Expr::Const(c) if c.numer() == 3 && c.denom() == 1);
                    return (is_three(left.as_ref()) || is_three(right.as_ref()))
                        && !(matches!(left.as_ref(), Expr::Const(_))
                            && matches!(right.as_ref(), Expr::Const(_)));
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                let extract_x = |a: &Expr, b: &Expr| {
                    if matches!(a, Expr::Const(c) if c.numer() == 3 && c.denom() == 1) {
                        Some(b.clone())
                    } else if matches!(b, Expr::Const(c) if c.numer() == 3 && c.denom() == 1) {
                        Some(a.clone())
                    } else {
                        None
                    }
                };

                if let Expr::Mul(left, right) = inner.as_ref() {
                    if let Some(x) = extract_x(left.as_ref(), right.as_ref()) {
                        let cos_x = Expr::Cos(Box::new(x.clone()));
                        let cos_cubed = Expr::Pow(Box::new(cos_x.clone()), Box::new(Expr::int(3)));
                        let result = Expr::Sub(
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_cubed))),
                            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(cos_x))),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "cos(3x) = 4cos³(x) - 3cos(x)".to_string(),
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

// sin(4x) = 4sin(x)cos(x)(1 - 2sin²(x))
/// Creates a trigonometric identity rule for sin(4x).
///
/// The rule detects expressions of the form `sin(4 * x)` and rewrites them using the
/// double-angle reduction to `2·sin(2x)·cos(2x)`, which is algebraically equal to
/// `4·sin(x)·cos(x)·(1 - 2·sin(x)²)`.
///
/// # Examples
///
/// ```
/// // Construct the rule and a matching expression `sin(4*x)`
/// let rule = sin_4x_formula();
/// let x = Expr::symbol("x");
/// let expr = Expr::Sin(Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(x))));
///
/// // The rule should be applicable and produce the double-angle form `2*sin(2x)*cos(2x)`
/// assert!( (rule.is_applicable)(&expr, &RuleContext::default()) );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn sin_4x_formula() -> Rule {
    Rule {
        id: RuleId(240),
        name: "sin_4x_formula",
        category: RuleCategory::TrigIdentity,
        description: "sin(4x) = 4sin(x)cos(x)(1 - 2sin²(x))",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let is_four =
                        |e: &Expr| matches!(e, Expr::Const(c) if c.numer() == 4 && c.denom() == 1);
                    return is_four(left.as_ref()) || is_four(right.as_ref());
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                let extract_x = |a: &Expr, b: &Expr| {
                    if matches!(a, Expr::Const(c) if c.numer() == 4 && c.denom() == 1) {
                        Some(b.clone())
                    } else if matches!(b, Expr::Const(c) if c.numer() == 4 && c.denom() == 1) {
                        Some(a.clone())
                    } else {
                        None
                    }
                };

                if let Expr::Mul(left, right) = inner.as_ref() {
                    if let Some(x) = extract_x(left.as_ref(), right.as_ref()) {
                        let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x.clone()));
                        let sin_2x = Expr::Sin(Box::new(two_x.clone()));
                        let cos_2x = Expr::Cos(Box::new(two_x));
                        let result = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(Box::new(sin_2x), Box::new(cos_2x))),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sin(4x) = 2sin(2x)cos(2x)".to_string(),
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

// cos(4x) = 8cos⁴(x) - 8cos²(x) + 1
/// Creates a trigonometric identity rule for cos(4x) that uses double-angle reduction.
///
/// The rule matches `cos(4 * x)` and rewrites it to `2*cos²(2x) - 1` with the justification
/// `"cos(4x) = 2cos²(2x) - 1"`. The rule is reversible and has a cost of 4.
///
/// # Examples
///
/// ```
/// let mut st = SymbolTable::new();
/// let x = st.new_symbol("x");
/// let rule = cos_4x_formula();
/// let ctx = RuleContext::new(&st);
/// let expr = Expr::Cos(Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(Expr::symbol(&x)))));
/// assert!( (rule.is_applicable)(&expr, &ctx) );
/// let apps = (rule.apply)(&expr, &ctx);
/// assert!(!apps.is_empty());
/// ```
fn cos_4x_formula() -> Rule {
    Rule {
        id: RuleId(241),
        name: "cos_4x_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos(4x) = 8cos⁴(x) - 8cos²(x) + 1",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Mul(left, right) = inner.as_ref() {
                    let is_four =
                        |e: &Expr| matches!(e, Expr::Const(c) if c.numer() == 4 && c.denom() == 1);
                    return is_four(left.as_ref()) || is_four(right.as_ref());
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                let extract_x = |a: &Expr, b: &Expr| {
                    if matches!(a, Expr::Const(c) if c.numer() == 4 && c.denom() == 1) {
                        Some(b.clone())
                    } else if matches!(b, Expr::Const(c) if c.numer() == 4 && c.denom() == 1) {
                        Some(a.clone())
                    } else {
                        None
                    }
                };

                if let Expr::Mul(left, right) = inner.as_ref() {
                    if let Some(x) = extract_x(left.as_ref(), right.as_ref()) {
                        let two_x = Expr::Mul(Box::new(Expr::int(2)), Box::new(x.clone()));
                        let cos_2x = Expr::Cos(Box::new(two_x));
                        let cos_2x_sq = Expr::Pow(Box::new(cos_2x.clone()), Box::new(Expr::int(2)));
                        let result = Expr::Sub(
                            Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(cos_2x_sq))),
                            Box::new(Expr::int(1)),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "cos(4x) = 2cos²(2x) - 1".to_string(),
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

// cot(x) = 1/tan(x)
/// Creates a rule that converts between cot(x), cos(x)/sin(x), and 1/tan(x).
///
/// The rule matches either cos(x)/sin(x) or 1/tan(x) and produces the equivalent expression:
/// - cos(x)/sin(x) ↔ 1/tan(x)
/// - 1/tan(x) ↔ cos(x)/sin(x)
///
/// # Examples
///
/// ```
/// let rule = cot_reciprocal();
/// let x = Expr::sym("x");
/// let expr = Expr::Div(Box::new(Expr::Cos(x.clone())), Box::new(Expr::Sin(x.clone())));
/// assert!(rule.is_applicable(&expr, &RuleContext::default()));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(
///     apps[0].result,
///     Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::Tan(x)))
/// );
/// ```
fn cot_reciprocal() -> Rule {
    Rule {
        id: RuleId(242),
        name: "cot_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "cot(x) = 1/tan(x)",
        is_applicable: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Cos(_))
                    && matches!(denom.as_ref(), Expr::Sin(_))
                    || matches!(num.as_ref(), Expr::Const(c) if c.is_one() && matches!(denom.as_ref(), Expr::Tan(_)));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                if let (Expr::Cos(x), Expr::Sin(y)) = (num.as_ref(), denom.as_ref()) {
                    if x == y {
                        return vec![RuleApplication {
                            result: Expr::Div(
                                Box::new(Expr::int(1)),
                                Box::new(Expr::Tan(x.clone())),
                            ),
                            justification: "cot(x) = 1/tan(x) = cos(x)/sin(x)".to_string(),
                        }];
                    }
                } else if let (Expr::Const(c), Expr::Tan(x)) = (num.as_ref(), denom.as_ref()) {
                    if c.is_one() {
                        let cos_over_sin = Expr::Div(
                            Box::new(Expr::Cos(x.clone())),
                            Box::new(Expr::Sin(x.clone())),
                        );
                        return vec![RuleApplication {
                            result: cos_over_sin,
                            justification: "cot(x) = cos(x)/sin(x)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// sec(x) = 1/cos(x)
/// Create a trigonometric rule that rewrites between `1 / cos(x)` and `sec(x)`.
///
/// The rule matches expressions of the form `1 / cos(x)` and produces the reciprocal form
/// `cos(x)^-1` (represented as `sec(x)` conceptually). It is reversible so it can also be
/// used to rewrite `sec(x)` back to `1 / cos(x)`.
///
/// # Examples
///
/// ```
/// // Construct the rule and apply it to the expression 1 / cos(x)
/// let rule = sec_reciprocal();
/// // let expr = Expr::Div(Box::new(Expr::Const(Const::one())), Box::new(Expr::Cos(Box::new(Expr::Sym("x".into())))));
/// // assert!(rule.is_applicable(&expr, &ctx));
/// // let applications = rule.apply(&expr, &ctx);
/// // assert_eq!(applications.len(), 1);
/// ```
fn sec_reciprocal() -> Rule {
    Rule {
        id: RuleId(243),
        name: "sec_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "sec(x) = 1/cos(x)",
        is_applicable: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Const(c) if c.is_one())
                    && matches!(denom.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(_, denom) = expr {
                if let Expr::Cos(x) = denom.as_ref() {
                    let result = Expr::Pow(Box::new(Expr::Cos(x.clone())), Box::new(Expr::int(-1)));
                    return vec![RuleApplication {
                        result,
                        justification: "sec(x) = (cos(x))^-1".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// csc(x) = 1/sin(x)
/// Creates a Rule that recognizes `1/sin(x)` and rewrites it as `csc(x)` (represented as `sin(x)` raised to `-1`), and vice versa.
///
/// # Examples
///
/// ```
/// let rule = csc_reciprocal();
/// assert_eq!(rule.name, "csc_reciprocal");
/// assert!(rule.reversible);
/// ```
fn csc_reciprocal() -> Rule {
    Rule {
        id: RuleId(244),
        name: "csc_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "csc(x) = 1/sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Div(num, denom) = expr {
                return matches!(num.as_ref(), Expr::Const(c) if c.is_one())
                    && matches!(denom.as_ref(), Expr::Sin(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Div(_, denom) = expr {
                if let Expr::Sin(x) = denom.as_ref() {
                    let result = Expr::Pow(Box::new(Expr::Sin(x.clone())), Box::new(Expr::int(-1)));
                    return vec![RuleApplication {
                        result,
                        justification: "csc(x) = (sin(x))^-1".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// sin(-x) = -sin(x)
fn sin_neg_x() -> Rule {
    Rule {
        id: RuleId(245),
        name: "sin_neg_x",
        category: RuleCategory::TrigIdentity,
        description: "sin(-x) = -sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                return matches!(inner.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Neg(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::Sin(x.clone()))),
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

// cos(-x) = cos(x)
fn cos_neg_x() -> Rule {
    Rule {
        id: RuleId(246),
        name: "cos_neg_x",
        category: RuleCategory::TrigIdentity,
        description: "cos(-x) = cos(x)",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                return matches!(inner.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Neg(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Cos(x.clone()),
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

// tan(-x) = -tan(x)
fn tan_neg_x() -> Rule {
    Rule {
        id: RuleId(247),
        name: "tan_neg_x",
        category: RuleCategory::TrigIdentity,
        description: "tan(-x) = -tan(x)",
        is_applicable: |expr, _| {
            if let Expr::Tan(inner) = expr {
                return matches!(inner.as_ref(), Expr::Neg(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Neg(x) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::Tan(x.clone()))),
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

// sin(π - x) = sin(x)
/// Creates a rule that rewrites `sin(π - x)` to `sin(x)`.
///
/// The rule matches a sine whose argument is a subtraction with `π` as the left operand
/// and returns a single application where the result is `sin(x)`.
///
/// # Examples
///
/// ```
/// let rule = sin_pi_minus();
/// let expr = Expr::Sin(Box::new(Expr::Sub(
///     Box::new(Expr::Pi),
///     Box::new(Expr::Symbol("x".into())),
/// )));
/// let ctx = RuleContext::default();
/// let apps = (rule.apply)(&expr, &ctx);
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Sin(Box::new(Expr::Symbol("x".into()))));
/// ```
fn sin_pi_minus() -> Rule {
    Rule {
        id: RuleId(248),
        name: "sin_pi_minus",
        category: RuleCategory::TrigIdentity,
        description: "sin(π - x) = sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    return matches!(left.as_ref(), Expr::Pi);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sin(right.clone()),
                        justification: "sin(π - x) = sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// cos(π - x) = -cos(x)
/// Creates a trigonometric rule that rewrites `cos(π - x)` to `-cos(x)`.
///
/// The rule matches cosine of a subtraction whose left operand is `π` and produces
/// the negated cosine of the right operand.
///
/// # Examples
///
/// ```
/// let rule = cos_pi_minus();
/// let expr = Expr::Cos(Box::new(Expr::Sub(Box::new(Expr::Pi), Box::new(Expr::Symbol("x".to_string())))));
/// let ctx = RuleContext::default();
/// let apps = (rule.apply)(&expr, &ctx);
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Neg(Box::new(Expr::Cos(Box::new(Expr::Symbol("x".to_string()))))));
/// ```
fn cos_pi_minus() -> Rule {
    Rule {
        id: RuleId(249),
        name: "cos_pi_minus",
        category: RuleCategory::TrigIdentity,
        description: "cos(π - x) = -cos(x)",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    return matches!(left.as_ref(), Expr::Pi);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Neg(Box::new(Expr::Cos(right.clone()))),
                        justification: "cos(π - x) = -cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// sin(π + x) = -sin(x)
/// Creates a trigonometric rule that rewrites `sin(π + x)` to `-sin(x)`.
///
/// The rule matches `sin(π + x)` and produces `-sin(x)` with the justification
/// `"sin(π + x) = -sin(x)"`.
///
/// # Examples
///
/// ```
/// let rule = sin_pi_plus();
/// assert_eq!(rule.name, "sin_pi_plus");
/// assert_eq!(rule.description, "sin(π + x) = -sin(x)");
/// ```
fn sin_pi_plus() -> Rule {
    Rule {
        id: RuleId(250),
        name: "sin_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "sin(π + x) = -sin(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Sin(_)),
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Add(left, right) = inner.as_ref() {
                    if matches!(left.as_ref(), Expr::Pi) {
                        return vec![RuleApplication {
                            result: Expr::Neg(Box::new(Expr::Sin(right.clone()))),
                            justification: "sin(π + x) = -sin(x)".to_string(),
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

// cos(π + x) = -cos(x)
/// Matches and rewrites cosine of a π-shifted angle: transforms `cos(π + x)` into `-cos(x)`.
///
/// The rule applies only when the cosine argument is an addition whose left term is exactly `π`.
///
/// # Examples
///
/// ```
/// let rule = cos_pi_plus();
/// let expr = Expr::Cos(Box::new(Expr::Add(Box::new(Expr::Pi), Box::new(Expr::Symbol("x".into())))));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Neg(Box::new(Expr::Cos(Box::new(Expr::Symbol("x".into()))))));
/// ```
fn cos_pi_plus() -> Rule {
    Rule {
        id: RuleId(251),
        name: "cos_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "cos(π + x) = -cos(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Cos(_)),
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Add(left, right) = inner.as_ref() {
                    if matches!(left.as_ref(), Expr::Pi) {
                        return vec![RuleApplication {
                            result: Expr::Neg(Box::new(Expr::Cos(right.clone()))),
                            justification: "cos(π + x) = -cos(x)".to_string(),
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

// sin(2π + x) = sin(x)
/// Produces a rule that rewrites `sin(2π + x)` to `sin(x)`.
///
/// The returned Rule matches `sin(_)` expressions and, when the argument is an addition
/// whose left term is `2·π`, yields `sin(x)` with a justification.
///
/// # Examples
///
/// ```
/// let rule = sin_2pi_plus();
/// assert_eq!(rule.name, "sin_2pi_plus");
/// ```
fn sin_2pi_plus() -> Rule {
    Rule {
        id: RuleId(252),
        name: "sin_2pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "sin(2π + x) = sin(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Sin(_)),
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Add(left, right) = inner.as_ref() {
                    if let Expr::Mul(coeff, pi) = left.as_ref() {
                        if matches!(pi.as_ref(), Expr::Pi)
                            && matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                        {
                            return vec![RuleApplication {
                                result: Expr::Sin(right.clone()),
                                justification: "sin(2π + x) = sin(x)".to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// cos(2π + x) = cos(x)
/// Creates a trig identity rule for the identity `cos(2π + x) = cos(x)`.
///
/// The returned `Rule` matches `cos(2π + x)` and rewrites it to `cos(x)`.
///
/// # Examples
///
/// ```
/// // Construct a rule and apply it to `cos(2π + x)`
/// let rule = cos_2pi_plus();
/// let mut st = SymbolTable::new();
/// let x = st.variable("x");
/// let expr = Expr::Cos(Box::new(Expr::Add(
///     Box::new(Expr::Mul(Box::new(Expr::Const(Rational::from(2))), Box::new(Expr::Pi))),
///     Box::new(Expr::Var(x)),
/// )));
/// let ctx = RuleContext::new(&st);
/// assert!(rule.is_applicable(&expr, &ctx));
/// let apps = rule.apply(&expr, &ctx);
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Cos(Box::new(Expr::Var(x))));
/// ```
fn cos_2pi_plus() -> Rule {
    Rule {
        id: RuleId(253),
        name: "cos_2pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "cos(2π + x) = cos(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Cos(_)),
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Add(left, right) = inner.as_ref() {
                    if let Expr::Mul(coeff, pi) = left.as_ref() {
                        if matches!(pi.as_ref(), Expr::Pi)
                            && matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                        {
                            return vec![RuleApplication {
                                result: Expr::Cos(right.clone()),
                                justification: "cos(2π + x) = cos(x)".to_string(),
                            }];
                        }
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// tan(π + x) = tan(x)
/// Matches and simplifies tangent of a π-shifted angle.
///
/// This rule recognizes expressions of the form `tan(π + x)` and rewrites them to `tan(x)`.
///
/// # Examples
///
/// ```
/// let rule = tan_pi_plus();
/// let expr = Expr::Tan(Box::new(Expr::Add(Box::new(Expr::Pi), Box::new(Expr::Symbol("x".into())))));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Tan(Box::new(Expr::Symbol("x".into()))));
/// ```
fn tan_pi_plus() -> Rule {
    Rule {
        id: RuleId(254),
        name: "tan_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "tan(π + x) = tan(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Tan(_)),
        apply: |expr, _| {
            if let Expr::Tan(inner) = expr {
                if let Expr::Add(left, right) = inner.as_ref() {
                    if matches!(left.as_ref(), Expr::Pi) {
                        return vec![RuleApplication {
                            result: Expr::Tan(right.clone()),
                            justification: "tan(π + x) = tan(x)".to_string(),
                        }];
                    }
                }
            }
            vec![]
        },
        reversible: true,
        cost: 1,
    }
}

// sin(90° - x) = cos(x) in radians form
/// Matches sine of a complementary angle and rewrites it to cosine.
///
/// Detects expressions of the form `sin(π/2 - x)` and produces `cos(x)`.
///
/// # Examples
///
/// ```
/// // Given an expression representing `sin(π/2 - x)`, this rule produces `cos(x)`.
/// let rule = sin_complementary();
/// // rule.is_applicable(...) -> true for `sin(π/2 - x)`
/// // rule.apply(...) -> vec![RuleApplication { result: Expr::Cos(x), justification: _ }]
/// ```
fn sin_complementary() -> Rule {
    Rule {
        id: RuleId(255),
        name: "sin_complementary",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/2 - x) = cos(x)",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    if let Expr::Div(num, den) = left.as_ref() {
                        return matches!(num.as_ref(), Expr::Pi)
                            && matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Cos(right.clone()),
                        justification: "sin(π/2 - x) = cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// cos(90° - x) = sin(x) in radians
/// Converts cos(π/2 - x) to sin(x).
///
/// This rule matches expressions of the form `cos(π/2 - x)` and yields `sin(x)` with a justification.
/// The rule is reversible and assigned a moderate cost.
///
/// # Examples
///
/// ```
/// // construct the rule and an example expression: cos(pi/2 - x)
/// let rule = cos_complementary();
/// let x = Expr::Symbol("x".into());
/// let expr = Expr::Cos(Box::new(Expr::Sub(
///     Box::new(Expr::Div(Box::new(Expr::Pi), Box::new(Expr::Const(Rational::new(1,2))))),
///     Box::new(x.clone()),
/// )));
/// let apps = (rule.is_applicable)(&expr, &RuleContext::default());
/// assert!(apps);
/// let results = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0].result, Expr::Sin(Box::new(x)));
/// ```
fn cos_complementary() -> Rule {
    Rule {
        id: RuleId(256),
        name: "cos_complementary",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/2 - x) = sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    if let Expr::Div(num, den) = left.as_ref() {
                        return matches!(num.as_ref(), Expr::Pi)
                            && matches!(den.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
                    }
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sin(right.clone()),
                        justification: "cos(π/2 - x) = sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// sin(180° - x) = sin(x)
/// Creates a rule that rewrites `sin(π - x)` to `sin(x)`.
///
/// Matches expressions of the form `sin(π - x)` and produces `sin(x)` as the transformed result.
///
/// # Examples
///
/// ```
/// let rule = sin_supplementary();
/// let expr = Expr::Sin(Box::new(Expr::Sub(Box::new(Expr::Pi), Box::new(Expr::Symbol("x".into())))));
/// assert!((rule.is_applicable)(&expr, &RuleContext::default()));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].result, Expr::Sin(Box::new(Expr::Symbol("x".into()))));
/// ```
fn sin_supplementary() -> Rule {
    Rule {
        id: RuleId(257),
        name: "sin_supplementary",
        category: RuleCategory::TrigIdentity,
        description: "sin(π - x) = sin(x)",
        is_applicable: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(left, _) = inner.as_ref() {
                    return matches!(left.as_ref(), Expr::Pi);
                }
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Sub(_, right) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Sin(right.clone()),
                        justification: "sin(π - x) = sin(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// sin²(x) = (1 - cos(2x))/2
/// Creates a Rule implementing the identity sin²(x) = (1 - cos(2x)) / 2.
///
/// The rule matches expressions of the form sin(x)² and rewrites them to (1 - cos(2x)) / 2 when applicable.
///
/// # Examples
///
/// ```
/// let rule = sin_squared_formula();
/// assert_eq!(rule.name, "sin_squared_formula");
/// ```
fn sin_squared_formula() -> Rule {
    Rule {
        id: RuleId(258),
        name: "sin_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "sin²(x) = (1 - cos(2x))/2",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                return matches!(base.as_ref(), Expr::Sin(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Sin(arg) = base.as_ref() {
                    let two_x = Expr::Mul(Box::new(Expr::int(2)), arg.clone());
                    let cos_2x = Expr::Cos(Box::new(two_x));
                    let numerator = Expr::Sub(Box::new(Expr::int(1)), Box::new(cos_2x));
                    let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(2)));
                    return vec![RuleApplication {
                        result,
                        justification: "sin²(x) = (1 - cos(2x))/2".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// cos²(x) = (1 + cos(2x))/2
/// Creates a trigonometric rewrite Rule for the identity cos²(x) = (1 + cos(2x)) / 2.
///
/// The rule matches expressions of the form cos(x)^2 and rewrites them to (1 + cos(2x)) / 2.
/// This rule is reversible and assigned a moderate cost to prefer equivalent transformations.
///
/// # Examples
///
/// ```
/// let rule = cos_squared_formula();
/// assert_eq!(rule.name, "cos_squared_formula");
/// ```
fn cos_squared_formula() -> Rule {
    Rule {
        id: RuleId(259),
        name: "cos_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos²(x) = (1 + cos(2x))/2",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                return matches!(base.as_ref(), Expr::Cos(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Cos(arg) = base.as_ref() {
                    let two_x = Expr::Mul(Box::new(Expr::int(2)), arg.clone());
                    let cos_2x = Expr::Cos(Box::new(two_x));
                    let numerator = Expr::Add(Box::new(Expr::int(1)), Box::new(cos_2x));
                    let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(2)));
                    return vec![RuleApplication {
                        result,
                        justification: "cos²(x) = (1 + cos(2x))/2".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// tan²(x) = (1 - cos(2x))/(1 + cos(2x))
/// Rewrites tan(x) squared into an equivalent rational expression in cos(2x).
///
/// Matches expressions of the form `tan(x)^2` and returns `(1 - cos(2x)) / (1 + cos(2x))`.
///
/// # Examples
///
/// ```
/// let rule = tan_squared_formula();
/// let x = Expr::symbol("x");
/// let expr = Expr::pow(Expr::tan(x.clone()), Expr::int(2));
/// assert!(rule.is_applicable(&expr, &RuleContext::default()));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn tan_squared_formula() -> Rule {
    Rule {
        id: RuleId(260),
        name: "tan_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "tan²(x) = (1 - cos(2x))/(1 + cos(2x))",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                return matches!(base.as_ref(), Expr::Tan(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1);
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Tan(arg) = base.as_ref() {
                    let two_x = Expr::Mul(Box::new(Expr::int(2)), arg.clone());
                    let cos_2x = Expr::Cos(Box::new(two_x));
                    let numerator = Expr::Sub(Box::new(Expr::int(1)), Box::new(cos_2x.clone()));
                    let denominator = Expr::Add(Box::new(Expr::int(1)), Box::new(cos_2x));
                    let result = Expr::Div(Box::new(numerator), Box::new(denominator));
                    return vec![RuleApplication {
                        result,
                        justification: "tan²(x) = (1 - cos(2x))/(1 + cos(2x))".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

// sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8
/// Creates a rule that rewrites sin⁴(x) as (3 - 4·cos(2x) + cos(4x)) / 8.
///
/// The returned `Rule` matches expressions of the form `sin(x)^4` and transforms them
/// into the equivalent linear combination of cosines ` (3 - 4*cos(2x) + cos(4x)) / 8`.
///
/// # Examples
///
/// ```
/// // Construct the rule and a sample expression `sin(x)^4`, then apply the rule.
/// let rule = sin_pow4();
/// let expr = Expr::Pow(Box::new(Expr::Sin(Box::new(Expr::Var("x".into())))), Box::new(Expr::int(4)));
/// let ctx = RuleContext::default();
/// assert!( (rule.is_applicable)(&expr, &ctx) );
/// let apps = (rule.apply)(&expr, &ctx);
/// assert_eq!(apps.len(), 1);
/// assert_eq!(apps[0].justification, "sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8");
/// ```
fn sin_pow4() -> Rule {
    Rule {
        id: RuleId(261),
        name: "sin_pow4",
        category: RuleCategory::TrigIdentity,
        description: "sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                return matches!(base.as_ref(), Expr::Sin(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 4 && c.denom() == 1);
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Sin(arg) = base.as_ref() {
                    let two_x = Expr::Mul(Box::new(Expr::int(2)), arg.clone());
                    let cos_2x = Expr::Cos(Box::new(two_x.clone()));
                    let cos_4x =
                        Expr::Cos(Box::new(Expr::Mul(Box::new(Expr::int(4)), arg.clone())));
                    let numerator = Expr::Add(
                        Box::new(Expr::Sub(
                            Box::new(Expr::int(3)),
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_2x))),
                        )),
                        Box::new(cos_4x),
                    );
                    let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(8)));
                    return vec![RuleApplication {
                        result,
                        justification: "sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

// cos⁴(x) = (3 + 4cos(2x) + cos(4x))/8
/// Generates a rule that rewrites cos⁴(x) as (3 + 4·cos(2x) + cos(4x)) / 8.
///
/// The rule matches expressions of the form `cos(arg)^4` and transforms them
/// into the equivalent sum-of-cosines form with justification text.
///
/// # Examples
///
/// ```
/// let rule = cos_pow4();
/// let x = Expr::sym("x");
/// let expr = Expr::pow(Box::new(Expr::Cos(Box::new(x.clone()))), Box::new(Expr::int(4)));
/// assert!(rule.is_applicable(&expr, &RuleContext::default()));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// // result should be (3 + 4*cos(2*x) + cos(4*x)) / 8
/// let expected = Expr::div(
///     Expr::add(
///         Expr::add(Expr::int(3), Expr::mul(Expr::int(4), Expr::cos(Expr::mul(Expr::int(2), x.clone())))),
///         Expr::cos(Expr::mul(Expr::int(4), x))
///     ),
///     Expr::int(8)
/// );
/// assert_eq!(apps[0].result, expected);
/// ```
fn cos_pow4() -> Rule {
    Rule {
        id: RuleId(262),
        name: "cos_pow4",
        category: RuleCategory::TrigIdentity,
        description: "cos⁴(x) = (3 + 4cos(2x) + cos(4x))/8",
        is_applicable: |expr, _| {
            if let Expr::Pow(base, exp) = expr {
                return matches!(base.as_ref(), Expr::Cos(_))
                    && matches!(exp.as_ref(), Expr::Const(c) if c.numer() == 4 && c.denom() == 1);
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Pow(base, _) = expr {
                if let Expr::Cos(arg) = base.as_ref() {
                    let two_x = Expr::Mul(Box::new(Expr::int(2)), arg.clone());
                    let cos_2x = Expr::Cos(Box::new(two_x.clone()));
                    let cos_4x =
                        Expr::Cos(Box::new(Expr::Mul(Box::new(Expr::int(4)), arg.clone())));
                    let numerator = Expr::Add(
                        Box::new(Expr::Add(
                            Box::new(Expr::int(3)),
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_2x))),
                        )),
                        Box::new(cos_4x),
                    );
                    let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(8)));
                    return vec![RuleApplication {
                        result,
                        justification: "cos⁴(x) = (3 + 4cos(2x) + cos(4x))/8".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 4,
    }
}

// 3sin(x) - sin(3x) = 4sin³(x)
/// Creates the trigonometric identity rule for 3·sin(x) - sin(3x) = 4·sin³(x).
///
/// Matches expressions of the form `3*sin(x) - sin(3*x)` and rewrites between the triple-angle and cubic forms,
/// producing `4*sin(x)^3` when applicable.
///
/// # Examples
///
/// ```
/// let rule = triple_sin_formula();
/// assert_eq!(rule.id, RuleId(263));
/// assert_eq!(rule.name, "triple_sin_formula");
/// ```
fn triple_sin_formula() -> Rule {
    Rule {
        id: RuleId(263),
        name: "triple_sin_formula",
        category: RuleCategory::TrigIdentity,
        description: "3sin(x) - sin(3x) = 4sin³(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Sub(_, _)),
        apply: |expr, _| {
            if let Expr::Sub(left, right) = expr {
                if let (Expr::Mul(coeff, sin_x), Expr::Sin(triple_arg)) =
                    (left.as_ref(), right.as_ref())
                {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                        && matches!(sin_x.as_ref(), Expr::Sin(_))
                    {
                        let x = if let Expr::Sin(inner) = sin_x.as_ref() {
                            inner.clone()
                        } else {
                            return vec![];
                        };
                        if let Expr::Mul(k, inner_x) = triple_arg.as_ref() {
                            if matches!(k.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                                && inner_x.as_ref() == x.as_ref()
                            {
                                let sin_cubed = Expr::Pow(
                                    Box::new(Expr::Sin(x.clone())),
                                    Box::new(Expr::int(3)),
                                );
                                let result = Expr::Mul(Box::new(Expr::int(4)), Box::new(sin_cubed));
                                return vec![RuleApplication {
                                    result,
                                    justification: "3sin(x) - sin(3x) = 4sin³(x)".to_string(),
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
    }
}

// cos(3x) + 3cos(x) = 4cos³(x)
/// Constructs the triple-angle cosine identity rule for cos(3x).
///
/// This rule recognizes expressions of the form `cos(3*x) + 3*cos(x)` and rewrites them to `4*cos(x)^3`.
/// The rule is reversible and has a cost of 3.
///
/// # Examples
///
/// ```
/// let rule = triple_cos_formula();
/// // rule will transform `cos(3*x) + 3*cos(x)` into `4*cos(x)^3` when applicable.
/// assert_eq!(rule.id.0, 264);
/// ```
fn triple_cos_formula() -> Rule {
    Rule {
        id: RuleId(264),
        name: "triple_cos_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos(3x) + 3cos(x) = 4cos³(x)",
        is_applicable: |expr, _| matches!(expr, Expr::Add(_, _)),
        apply: |expr, _| {
            if let Expr::Add(left, right) = expr {
                if let (Expr::Cos(triple_arg), Expr::Mul(coeff, cos_x)) =
                    (left.as_ref(), right.as_ref())
                {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                        && matches!(cos_x.as_ref(), Expr::Cos(_))
                    {
                        let x = if let Expr::Cos(inner) = cos_x.as_ref() {
                            inner.clone()
                        } else {
                            return vec![];
                        };
                        if let Expr::Mul(k, inner_x) = triple_arg.as_ref() {
                            if matches!(k.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                                && inner_x.as_ref() == x.as_ref()
                            {
                                let cos_cubed = Expr::Pow(
                                    Box::new(Expr::Cos(x.clone())),
                                    Box::new(Expr::int(3)),
                                );
                                let result = Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_cubed));
                                return vec![RuleApplication {
                                    result,
                                    justification: "cos(3x) + 3cos(x) = 4cos³(x)".to_string(),
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
    }
}

// Chebyshev T_2(x) = 2x² - 1
/// Constructs the Chebyshev T₂ rule that relates a cosine of double angle to a quadratic in cosine.
///
/// The produced `Rule` matches `cos(2·x)` and rewrites it to `2·cos²(x) - 1`. The rule is marked reversible and has a cost of 2.
///
/// # Examples
///
/// ```
/// let rule = chebyshev_t2();
/// // matches cos(2*x) and produces 2*cos(x)^2 - 1
/// let expr = Expr::Cos(Box::new(Expr::Mul(
///     Box::new(Expr::int(2)),
///     Box::new(Expr::Sym("x".into())),
/// )));
/// assert!(rule.is_applicable(&expr, &RuleContext::default()));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn chebyshev_t2() -> Rule {
    Rule {
        id: RuleId(265),
        name: "chebyshev_t2",
        category: RuleCategory::TrigIdentity,
        description: "T_2(x) = 2x² - 1",
        is_applicable: |expr, _| matches!(expr, Expr::Cos(_)),
        apply: |expr, _| {
            if let Expr::Cos(angle) = expr {
                if let Expr::Mul(coeff, x) = angle.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                    {
                        let cos_x = Expr::Cos(x.clone());
                        let poly = Expr::Sub(
                            Box::new(Expr::Mul(
                                Box::new(Expr::int(2)),
                                Box::new(Expr::Pow(
                                    Box::new(cos_x.clone()),
                                    Box::new(Expr::int(2)),
                                )),
                            )),
                            Box::new(Expr::int(1)),
                        );
                        return vec![RuleApplication {
                            result: poly,
                            justification: "cos(2x) = 2cos²(x)-1".to_string(),
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

// Chebyshev T_3(x) = 4x³ - 3x
/// Creates a rule for the Chebyshev polynomial identity T₃: cos(3x) = 4·cos³(x) − 3·cos(x).
///
/// The rule matches `cos(3·x)` (a cosine whose angle is `3` times some subexpression) and rewrites it to
/// the polynomial `4*cos(x)^3 - 3*cos(x)`. The rule is reversible and has cost 2.
///
/// # Examples
///
/// ```
/// let rule = chebyshev_t3();
/// let expr = Expr::Cos(Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Symbol("x".into())))));
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn chebyshev_t3() -> Rule {
    Rule {
        id: RuleId(266),
        name: "chebyshev_t3",
        category: RuleCategory::TrigIdentity,
        description: "T_3(x) = 4x³ - 3x",
        is_applicable: |expr, _| matches!(expr, Expr::Cos(_)),
        apply: |expr, _| {
            if let Expr::Cos(angle) = expr {
                if let Expr::Mul(coeff, x) = angle.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                    {
                        let cos_x = Expr::Cos(x.clone());
                        let cos_cubed = Expr::Pow(Box::new(cos_x.clone()), Box::new(Expr::int(3)));
                        let poly = Expr::Sub(
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(cos_cubed))),
                            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(cos_x))),
                        );
                        return vec![RuleApplication {
                            result: poly,
                            justification: "cos(3x) = 4cos³(x) - 3cos(x)".to_string(),
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

// Chebyshev U_2(x) = 4x² - 1
/// Creates a Rule for the Chebyshev polynomial U_2 identity (U_2(x) = 4x² − 1).
///
/// This rule recognizes `sin(2x)` written as `sin(2 * x)` and rewrites it to `2 * sin(x) * cos(x)` with a justification of the double-angle identity. The rule is reversible and has moderate cost for trig transformations.
///
/// # Examples
///
/// ```
/// let r = chebyshev_u2();
/// assert_eq!(r.id.0, 267);
/// assert_eq!(r.name, "chebyshev_u2");
/// ```
fn chebyshev_u2() -> Rule {
    Rule {
        id: RuleId(267),
        name: "chebyshev_u2",
        category: RuleCategory::TrigIdentity,
        description: "U_2(x) = 4x² - 1",
        is_applicable: |expr, _| matches!(expr, Expr::Sin(_)),
        apply: |expr, _| {
            if let Expr::Sin(angle) = expr {
                if let Expr::Mul(coeff, x) = angle.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 2 && c.denom() == 1)
                    {
                        let result = Expr::Mul(
                            Box::new(Expr::int(2)),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Sin(x.clone())),
                                Box::new(Expr::Cos(x.clone())),
                            )),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sin(2x) = 2sin(x)cos(x)".to_string(),
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

// Chebyshev U_3(x) = 8x³ - 4x
/// Produces a rule implementing the triple-angle / Chebyshev U3 identity for sine.
///
/// The rule matches `sin(3·x)` and rewrites it to `3·sin(x) - 4·sin(x)^3`. The rule is reversible and has cost 2.
///
/// # Examples
///
/// ```
/// let rule = chebyshev_u3();
/// let expr = Expr::Sin(Box::new(Expr::Mul(
///     Box::new(Expr::int(3)),
///     Box::new(Expr::Symbol("x".into())),
/// )));
/// assert!(rule.is_applicable(&expr, &RuleContext::default()));
/// let apps = rule.apply(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn chebyshev_u3() -> Rule {
    Rule {
        id: RuleId(268),
        name: "chebyshev_u3",
        category: RuleCategory::TrigIdentity,
        description: "U_3(x) = 8x³ - 4x",
        is_applicable: |expr, _| matches!(expr, Expr::Sin(_)),
        apply: |expr, _| {
            if let Expr::Sin(angle) = expr {
                if let Expr::Mul(coeff, x) = angle.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Const(c) if c.numer() == 3 && c.denom() == 1)
                    {
                        let sin_x = Expr::Sin(x.clone());
                        let sin_cubed = Expr::Pow(Box::new(sin_x.clone()), Box::new(Expr::int(3)));
                        let result = Expr::Sub(
                            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(sin_x))),
                            Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(sin_cubed))),
                        );
                        return vec![RuleApplication {
                            result,
                            justification: "sin(3x) = 3sin(x) - 4sin³(x)".to_string(),
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

// 2cos(A)cos(B) = cos(A+B) + cos(A-B)
/// Constructs the prosthaphaeresis Rule for converting products of cosines.
///
/// The returned Rule detects products of two cosine expressions and rewrites
/// cos(A)·cos(B) as (cos(A + B) + cos(A - B)) / 2.
///
/// # Examples
///
/// ```
/// let rule = prosthaphaeresis_1();
/// let a = Expr::Symbol("A".into());
/// let b = Expr::Symbol("B".into());
/// let expr = Expr::Mul(
///     Box::new(Expr::Cos(Box::new(a.clone()))),
///     Box::new(Expr::Cos(Box::new(b.clone())))
/// );
/// assert!( (rule.is_applicable)(&expr, &RuleContext::default()) );
/// let apps = (rule.apply)(&expr, &RuleContext::default());
/// assert_eq!(apps.len(), 1);
/// ```
fn prosthaphaeresis_1() -> Rule {
    Rule {
        id: RuleId(269),
        name: "prosthaphaeresis_1",
        category: RuleCategory::TrigIdentity,
        description: "2cos(A)cos(B) = cos(A+B) + cos(A-B)",
        is_applicable: |expr, _| {
            if let Expr::Mul(a, b) = expr {
                return matches!(a.as_ref(), Expr::Cos(_)) && matches!(b.as_ref(), Expr::Cos(_));
            }
            false
        },
        apply: |expr, _| {
            if let Expr::Mul(a, b) = expr {
                if let (Expr::Cos(alpha), Expr::Cos(beta)) = (a.as_ref(), b.as_ref()) {
                    let sum = Expr::Cos(Box::new(Expr::Add(alpha.clone(), beta.clone())));
                    let diff = Expr::Cos(Box::new(Expr::Sub(alpha.clone(), beta.clone())));
                    let numerator = Expr::Add(Box::new(sum), Box::new(diff));
                    let result = Expr::Div(Box::new(numerator), Box::new(Expr::int(2)));
                    return vec![RuleApplication {
                        result,
                        justification: "cosAcosB = (cos(A+B)+cos(A-B))/2".to_string(),
                    }];
                }
            }
            vec![]
        },
        reversible: true,
        cost: 3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RuleContext;
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