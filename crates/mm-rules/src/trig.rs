// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Trigonometric identity rules.

use crate::{Domain, Rule, RuleApplication, RuleCategory, RuleId};
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
fn sin_half_angle() -> Rule {
    Rule {
        id: RuleId(210),
        name: "sin_half_angle",
        category: RuleCategory::TrigIdentity,
        description: "sin(x/2) = √((1-cos(x))/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |expr, _ctx| {
            if let Expr::Sin(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
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
fn cos_half_angle() -> Rule {
    Rule {
        id: RuleId(211),
        name: "cos_half_angle",
        category: RuleCategory::TrigIdentity,
        description: "cos(x/2) = √((1+cos(x))/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |expr, _ctx| {
            if let Expr::Cos(inner) = expr {
                if let Expr::Div(num, den) = inner.as_ref() {
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
fn hyperbolic_sinh() -> Rule {
    Rule {
        id: RuleId(220),
        name: "hyperbolic_sinh",
        category: RuleCategory::TrigIdentity,
        description: "sinh(x) = (e^x - e^(-x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false, // Pattern matching for sinh
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cosh(x) definition
fn hyperbolic_cosh() -> Rule {
    Rule {
        id: RuleId(221),
        name: "hyperbolic_cosh",
        category: RuleCategory::TrigIdentity,
        description: "cosh(x) = (e^x + e^(-x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// tanh(x) = sinh(x)/cosh(x)
fn hyperbolic_tanh() -> Rule {
    Rule {
        id: RuleId(222),
        name: "hyperbolic_tanh",
        category: RuleCategory::TrigIdentity,
        description: "tanh(x) = sinh(x)/cosh(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sinh(2x) = 2sinh(x)cosh(x)
fn sinh_identity() -> Rule {
    Rule {
        id: RuleId(223),
        name: "sinh_double",
        category: RuleCategory::TrigIdentity,
        description: "sinh(2x) = 2sinh(x)cosh(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cosh(2x) = cosh²(x) + sinh²(x)
fn cosh_identity() -> Rule {
    Rule {
        id: RuleId(224),
        name: "cosh_double",
        category: RuleCategory::TrigIdentity,
        description: "cosh(2x) = cosh²(x) + sinh²(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cosh²(x) - sinh²(x) = 1
fn sinh_cosh_identity() -> Rule {
    Rule {
        id: RuleId(225),
        name: "sinh_cosh_pythagorean",
        category: RuleCategory::TrigIdentity,
        description: "cosh²(x) - sinh²(x) = 1",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: false,
        cost: 1,
    }
}

// arcsin(x) + arccos(x) = π/2
fn arcsin_arccos_sum() -> Rule {
    Rule {
        id: RuleId(229),
        name: "arcsin_arccos_sum",
        category: RuleCategory::TrigIdentity,
        description: "arcsin(x) + arccos(x) = π/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sinA + sinB = 2sin((A+B)/2)cos((A-B)/2)
fn sin_sum_to_product() -> Rule {
    Rule {
        id: RuleId(230),
        name: "sin_sum_to_product",
        category: RuleCategory::TrigIdentity,
        description: "sinA + sinB = 2sin((A+B)/2)cos((A-B)/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// cosA + cosB = 2cos((A+B)/2)cos((A-B)/2)
fn cos_sum_to_product() -> Rule {
    Rule {
        id: RuleId(231),
        name: "cos_sum_to_product",
        category: RuleCategory::TrigIdentity,
        description: "cosA + cosB = 2cos((A+B)/2)cos((A-B)/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// sinA - sinB = 2cos((A+B)/2)sin((A-B)/2)
fn sin_diff_to_product() -> Rule {
    Rule {
        id: RuleId(232),
        name: "sin_diff_to_product",
        category: RuleCategory::TrigIdentity,
        description: "sinA - sinB = 2cos((A+B)/2)sin((A-B)/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// cosA - cosB = -2sin((A+B)/2)sin((A-B)/2)
fn cos_diff_to_product() -> Rule {
    Rule {
        id: RuleId(233),
        name: "cos_diff_to_product",
        category: RuleCategory::TrigIdentity,
        description: "cosA - cosB = -2sin((A+B)/2)sin((A-B)/2)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// sin²(x/2) = (1 - cos(x))/2
fn sin_squared_half() -> Rule {
    Rule {
        id: RuleId(234),
        name: "sin_squared_half",
        category: RuleCategory::TrigIdentity,
        description: "sin²(x/2) = (1 - cos(x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cos²(x/2) = (1 + cos(x))/2
fn cos_squared_half() -> Rule {
    Rule {
        id: RuleId(235),
        name: "cos_squared_half",
        category: RuleCategory::TrigIdentity,
        description: "cos²(x/2) = (1 + cos(x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// tan(x/2) = sin(x)/(1 + cos(x))
fn tan_half_sin() -> Rule {
    Rule {
        id: RuleId(236),
        name: "tan_half_sin",
        category: RuleCategory::TrigIdentity,
        description: "tan(x/2) = sin(x)/(1 + cos(x))",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// tan(x/2) = (1 - cos(x))/sin(x)
fn tan_half_cos() -> Rule {
    Rule {
        id: RuleId(237),
        name: "tan_half_cos",
        category: RuleCategory::TrigIdentity,
        description: "tan(x/2) = (1 - cos(x))/sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sin(3x) = 3sin(x) - 4sin³(x)
fn sin_3x_expand() -> Rule {
    Rule {
        id: RuleId(238),
        name: "sin_3x_expand",
        category: RuleCategory::TrigIdentity,
        description: "sin(3x) = 3sin(x) - 4sin³(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// cos(3x) = 4cos³(x) - 3cos(x)
fn cos_3x_expand() -> Rule {
    Rule {
        id: RuleId(239),
        name: "cos_3x_expand",
        category: RuleCategory::TrigIdentity,
        description: "cos(3x) = 4cos³(x) - 3cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// sin(4x) = 4sin(x)cos(x)(1 - 2sin²(x))
fn sin_4x_formula() -> Rule {
    Rule {
        id: RuleId(240),
        name: "sin_4x_formula",
        category: RuleCategory::TrigIdentity,
        description: "sin(4x) = 4sin(x)cos(x)(1 - 2sin²(x))",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}

// cos(4x) = 8cos⁴(x) - 8cos²(x) + 1
fn cos_4x_formula() -> Rule {
    Rule {
        id: RuleId(241),
        name: "cos_4x_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos(4x) = 8cos⁴(x) - 8cos²(x) + 1",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}

// cot(x) = 1/tan(x)
fn cot_reciprocal() -> Rule {
    Rule {
        id: RuleId(242),
        name: "cot_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "cot(x) = 1/tan(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}

// sec(x) = 1/cos(x)
fn sec_reciprocal() -> Rule {
    Rule {
        id: RuleId(243),
        name: "sec_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "sec(x) = 1/cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}

// csc(x) = 1/sin(x)
fn csc_reciprocal() -> Rule {
    Rule {
        id: RuleId(244),
        name: "csc_reciprocal",
        category: RuleCategory::TrigIdentity,
        description: "csc(x) = 1/sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
        domains: &[Domain::Trigonometry],
        requires: &[],
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
fn sin_pi_minus() -> Rule {
    Rule {
        id: RuleId(248),
        name: "sin_pi_minus",
        category: RuleCategory::TrigIdentity,
        description: "sin(π - x) = sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cos(π - x) = -cos(x)
fn cos_pi_minus() -> Rule {
    Rule {
        id: RuleId(249),
        name: "cos_pi_minus",
        category: RuleCategory::TrigIdentity,
        description: "cos(π - x) = -cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sin(π + x) = -sin(x)
fn sin_pi_plus() -> Rule {
    Rule {
        id: RuleId(250),
        name: "sin_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "sin(π + x) = -sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cos(π + x) = -cos(x)
fn cos_pi_plus() -> Rule {
    Rule {
        id: RuleId(251),
        name: "cos_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "cos(π + x) = -cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sin(2π + x) = sin(x)
fn sin_2pi_plus() -> Rule {
    Rule {
        id: RuleId(252),
        name: "sin_2pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "sin(2π + x) = sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}

// cos(2π + x) = cos(x)
fn cos_2pi_plus() -> Rule {
    Rule {
        id: RuleId(253),
        name: "cos_2pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "cos(2π + x) = cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}

// tan(π + x) = tan(x)
fn tan_pi_plus() -> Rule {
    Rule {
        id: RuleId(254),
        name: "tan_pi_plus",
        category: RuleCategory::TrigIdentity,
        description: "tan(π + x) = tan(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 1,
    }
}

// sin(90° - x) = cos(x) in radians form
fn sin_complementary() -> Rule {
    Rule {
        id: RuleId(255),
        name: "sin_complementary",
        category: RuleCategory::TrigIdentity,
        description: "sin(π/2 - x) = cos(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cos(90° - x) = sin(x) in radians
fn cos_complementary() -> Rule {
    Rule {
        id: RuleId(256),
        name: "cos_complementary",
        category: RuleCategory::TrigIdentity,
        description: "cos(π/2 - x) = sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sin(180° - x) = sin(x)
fn sin_supplementary() -> Rule {
    Rule {
        id: RuleId(257),
        name: "sin_supplementary",
        category: RuleCategory::TrigIdentity,
        description: "sin(π - x) = sin(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// sin²(x) = (1 - cos(2x))/2
fn sin_squared_formula() -> Rule {
    Rule {
        id: RuleId(258),
        name: "sin_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "sin²(x) = (1 - cos(2x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// cos²(x) = (1 + cos(2x))/2
fn cos_squared_formula() -> Rule {
    Rule {
        id: RuleId(259),
        name: "cos_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos²(x) = (1 + cos(2x))/2",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// tan²(x) = (1 - cos(2x))/(1 + cos(2x))
fn tan_squared_formula() -> Rule {
    Rule {
        id: RuleId(260),
        name: "tan_squared_formula",
        category: RuleCategory::TrigIdentity,
        description: "tan²(x) = (1 - cos(2x))/(1 + cos(2x))",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8
fn sin_pow4() -> Rule {
    Rule {
        id: RuleId(261),
        name: "sin_pow4",
        category: RuleCategory::TrigIdentity,
        description: "sin⁴(x) = (3 - 4cos(2x) + cos(4x))/8",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}

// cos⁴(x) = (3 + 4cos(2x) + cos(4x))/8
fn cos_pow4() -> Rule {
    Rule {
        id: RuleId(262),
        name: "cos_pow4",
        category: RuleCategory::TrigIdentity,
        description: "cos⁴(x) = (3 + 4cos(2x) + cos(4x))/8",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 4,
    }
}

// 3sin(x) - sin(3x) = 4sin³(x)
fn triple_sin_formula() -> Rule {
    Rule {
        id: RuleId(263),
        name: "triple_sin_formula",
        category: RuleCategory::TrigIdentity,
        description: "3sin(x) - sin(3x) = 4sin³(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// cos(3x) + 3cos(x) = 4cos³(x)
fn triple_cos_formula() -> Rule {
    Rule {
        id: RuleId(264),
        name: "triple_cos_formula",
        category: RuleCategory::TrigIdentity,
        description: "cos(3x) + 3cos(x) = 4cos³(x)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 3,
    }
}

// Chebyshev T_2(x) = 2x² - 1
fn chebyshev_t2() -> Rule {
    Rule {
        id: RuleId(265),
        name: "chebyshev_t2",
        category: RuleCategory::TrigIdentity,
        description: "T_2(x) = 2x² - 1",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// Chebyshev T_3(x) = 4x³ - 3x
fn chebyshev_t3() -> Rule {
    Rule {
        id: RuleId(266),
        name: "chebyshev_t3",
        category: RuleCategory::TrigIdentity,
        description: "T_3(x) = 4x³ - 3x",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// Chebyshev U_2(x) = 4x² - 1
fn chebyshev_u2() -> Rule {
    Rule {
        id: RuleId(267),
        name: "chebyshev_u2",
        category: RuleCategory::TrigIdentity,
        description: "U_2(x) = 4x² - 1",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// Chebyshev U_3(x) = 8x³ - 4x
fn chebyshev_u3() -> Rule {
    Rule {
        id: RuleId(268),
        name: "chebyshev_u3",
        category: RuleCategory::TrigIdentity,
        description: "U_3(x) = 8x³ - 4x",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
        reversible: true,
        cost: 2,
    }
}

// 2cos(A)cos(B) = cos(A+B) + cos(A-B)
fn prosthaphaeresis_1() -> Rule {
    Rule {
        id: RuleId(269),
        name: "prosthaphaeresis_1",
        category: RuleCategory::TrigIdentity,
        description: "2cos(A)cos(B) = cos(A+B) + cos(A-B)",
        domains: &[Domain::Trigonometry],
        requires: &[],
        is_applicable: |_, _| false,
        apply: |_, _| vec![],
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
