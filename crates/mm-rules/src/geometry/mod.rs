// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Geometry rules for LEMMA.
//!
//! This module provides rules for:
//! - Conic Sections (Parabola, Ellipse, Hyperbola, Circle)
//! - Coordinate Geometry (Tangents, Normals, Loci)
//! - Synthetic Geometry (Triangle centers, Theorems) [Future]
//!
//! Note: These rules focus on *detection* of geometric patterns.
//! Full transformation requires a FactBank with geometric context.

use crate::{Domain, Feature, Rule, RuleApplication, RuleCategory, RuleContext, RuleId};
use mm_core::Expr;

/// Create all geometry rules.
pub fn geometry_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    // Conic Section Rules
    rules.extend(parabola_rules());
    rules.extend(ellipse_rules());
    rules.extend(hyperbola_rules());
    rules.extend(circle_rules());

    // Coordinate Geometry
    rules.extend(coordinate_rules());

    rules
}

// =============================================================================
// PARABOLA RULES (y² = 4ax form)
// =============================================================================

fn parabola_rules() -> Vec<Rule> {
    vec![
        // Detect parabola equation y² = 4ax
        Rule {
            id: RuleId(5001),
            name: "parabola_tangent_parametric",
            category: RuleCategory::Simplification,
            description: "Tangent to parabola y²=4ax at parameter t: ty = x + at²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Detect y² on left side of equation (parabola signature)
                if let Expr::Equation { lhs, rhs: _ } = expr {
                    return is_y_squared(lhs);
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Parabola detected. Tangent at parameter t: ty = x + at²"
                        .to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        Rule {
            id: RuleId(5002),
            name: "parabola_normal_parametric",
            category: RuleCategory::Simplification,
            description: "Normal to parabola y²=4ax at parameter t: y + tx = 2at + at³",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                if let Expr::Equation { lhs, rhs: _ } = expr {
                    return is_y_squared(lhs);
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Parabola detected. Normal at parameter t: y + tx = 2at + at³"
                        .to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        Rule {
            id: RuleId(5003),
            name: "parabola_focal_chord_reciprocal",
            category: RuleCategory::Simplification,
            description: "Focal chord property: 1/SP + 1/SQ = 2/l (l = semi-latus rectum)",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match 1/a + 1/b pattern
                if let Expr::Add(left, right) = expr {
                    return is_reciprocal(left) && is_reciprocal(right);
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Focal chord reciprocal property: 1/SP + 1/SQ = 2/l".to_string(),
                }]
            },
            reversible: true,
            cost: 4,
        },
        Rule {
            id: RuleId(5004),
            name: "parabola_reflection_property",
            category: RuleCategory::Simplification,
            description: "Parabola reflection: tangent bisects angle between focal radius and axis",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |_expr, _ctx| false, // Informational only
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 5,
        },
        Rule {
            id: RuleId(5005),
            name: "parabola_chord_of_contact",
            category: RuleCategory::Simplification,
            description: "Chord of contact from (x₁,y₁): yy₁ = 2a(x + x₁)",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                if let Expr::Equation { lhs, rhs: _ } = expr {
                    return is_y_squared(lhs);
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Chord of contact from external point: yy₁ = 2a(x + x₁)"
                        .to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
    ]
}

// =============================================================================
// ELLIPSE RULES (x²/a² + y²/b² = 1 form)
// =============================================================================

fn ellipse_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: RuleId(5101),
            name: "ellipse_tangent_parametric",
            category: RuleCategory::Simplification,
            description: "Tangent to ellipse at θ: (x cos θ)/a + (y sin θ)/b = 1",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match x²/a² + y²/b² = 1 (sum of two quotients = 1)
                if let Expr::Equation { lhs, rhs } = expr {
                    if is_const_one(rhs) {
                        if let Expr::Add(_, _) = lhs.as_ref() {
                            return true;
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Ellipse tangent at parameter θ: (x cos θ)/a + (y sin θ)/b = 1"
                        .to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        Rule {
            id: RuleId(5102),
            name: "ellipse_eccentricity",
            category: RuleCategory::Simplification,
            description: "Ellipse eccentricity: e = √(1 - b²/a²)",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match sqrt(1 - something)
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Sub(one, _) = inner.as_ref() {
                        return is_const_one(one);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Ellipse eccentricity formula: e = √(1 - b²/a²)".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5103),
            name: "ellipse_director_circle",
            category: RuleCategory::Simplification,
            description: "Director circle of ellipse: x² + y² = a² + b²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| is_circle_equation(expr),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Director circle: x² + y² = a² + b²".to_string(),
                }]
            },
            reversible: true,
            cost: 4,
        },
        Rule {
            id: RuleId(5104),
            name: "ellipse_auxiliary_circle",
            category: RuleCategory::Simplification,
            description: "Auxiliary circle of ellipse: x² + y² = a²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| is_circle_equation(expr),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Auxiliary circle: x² + y² = a²".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        Rule {
            id: RuleId(5105),
            name: "ellipse_focal_sum",
            category: RuleCategory::Simplification,
            description: "Sum of focal distances: SP + S'P = 2a",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match sum of two terms
                matches!(expr, Expr::Add(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Ellipse property: SP + S'P = 2a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
    ]
}

// =============================================================================
// HYPERBOLA RULES (x²/a² - y²/b² = 1 form)
// =============================================================================

fn hyperbola_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: RuleId(5201),
            name: "hyperbola_asymptotes",
            category: RuleCategory::Simplification,
            description: "Asymptotes of hyperbola: y = ±(b/a)x",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match x²/a² - y²/b² = 1 (subtraction pattern)
                if let Expr::Equation { lhs, rhs } = expr {
                    if is_const_one(rhs) {
                        if let Expr::Sub(_, _) = lhs.as_ref() {
                            return true;
                        }
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Hyperbola asymptotes: y = ±(b/a)x".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5202),
            name: "hyperbola_eccentricity",
            category: RuleCategory::Simplification,
            description: "Hyperbola eccentricity: e = √(1 + b²/a²)",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match sqrt(1 + something)
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Add(one, _) = inner.as_ref() {
                        return is_const_one(one);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Hyperbola eccentricity: e = √(1 + b²/a²)".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5203),
            name: "hyperbola_rectangular",
            category: RuleCategory::Simplification,
            description: "Rectangular hyperbola (a=b): xy = c²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match xy pattern on left side
                if let Expr::Equation { lhs, rhs: _ } = expr {
                    if let Expr::Mul(_, _) = lhs.as_ref() {
                        return true; // Simplified check for xy product
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Rectangular hyperbola: xy = c²".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        Rule {
            id: RuleId(5204),
            name: "hyperbola_focal_difference",
            category: RuleCategory::Simplification,
            description: "Difference of focal distances: |SP - S'P| = 2a",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match |a - b| pattern
                if let Expr::Abs(inner) = expr {
                    return matches!(inner.as_ref(), Expr::Sub(_, _));
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Hyperbola property: |SP - S'P| = 2a".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5205),
            name: "hyperbola_conjugate",
            category: RuleCategory::Simplification,
            description: "Conjugate hyperbola: x²/a² - y²/b² = -1",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match = -1 on right side
                if let Expr::Equation { lhs: _, rhs } = expr {
                    if let Expr::Neg(inner) = rhs.as_ref() {
                        return is_const_one(inner);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Conjugate hyperbola detected".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
    ]
}

// =============================================================================
// CIRCLE RULES (x² + y² = r² form)
// =============================================================================

fn circle_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: RuleId(5301),
            name: "circle_tangent_at_point",
            category: RuleCategory::Simplification,
            description: "Tangent to circle at (x₁,y₁): xx₁ + yy₁ = r²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| is_circle_equation(expr),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Circle tangent at (x₁,y₁): xx₁ + yy₁ = r²".to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        Rule {
            id: RuleId(5302),
            name: "circle_power_of_point",
            category: RuleCategory::Simplification,
            description: "Power of point: PA · PB = d² - r²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match product pattern
                matches!(expr, Expr::Mul(_, _))
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Power of point: PA · PB = d² - r²".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        Rule {
            id: RuleId(5303),
            name: "circle_radical_axis",
            category: RuleCategory::Simplification,
            description: "Radical axis: S₁ - S₂ = 0",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |_expr, _ctx| false, // Needs two-circle context
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
        Rule {
            id: RuleId(5304),
            name: "circle_orthogonal_condition",
            category: RuleCategory::Simplification,
            description: "Orthogonal circles: 2g₁g₂ + 2f₁f₂ = c₁ + c₂",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |_expr, _ctx| false, // Needs two-circle context
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
        Rule {
            id: RuleId(5305),
            name: "circle_tangent_length",
            category: RuleCategory::Simplification,
            description: "Tangent length from (x₁,y₁): √(x₁² + y₁² - r²)",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |expr, _ctx| {
                // Match √(a² + b² - c²) pattern
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Sub(sum, _) = inner.as_ref() {
                        return matches!(sum.as_ref(), Expr::Add(_, _));
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Tangent length from external point".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
    ]
}

// =============================================================================
// COORDINATE GEOMETRY RULES
// =============================================================================

fn coordinate_rules() -> Vec<Rule> {
    vec![
        // Distance formula
        Rule {
            id: RuleId(5401),
            name: "distance_formula",
            category: RuleCategory::Simplification,
            description: "Distance: d = √((x₂-x₁)² + (y₂-y₁)²)",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match √(a² + b²) pattern
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Add(left, right) = inner.as_ref() {
                        return is_squared(left) && is_squared(right);
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                // If both are constants, compute
                if let Expr::Sqrt(inner) = expr {
                    if let Expr::Add(left, right) = inner.as_ref() {
                        if let (Expr::Pow(a, _), Expr::Pow(b, _)) = (left.as_ref(), right.as_ref())
                        {
                            if let (Expr::Const(val_a), Expr::Const(val_b)) =
                                (a.as_ref(), b.as_ref())
                            {
                                let a_sq = val_a.clone() * val_a.clone();
                                let b_sq = val_b.clone() * val_b.clone();
                                let sum = a_sq + b_sq;
                                return vec![RuleApplication {
                                    result: Expr::Sqrt(Box::new(Expr::Const(sum))),
                                    justification: "Distance formula: √(a² + b²)".to_string(),
                                }];
                            }
                        }
                    }
                }
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Distance formula pattern: √(Δx² + Δy²)".to_string(),
                }]
            },
            reversible: true,
            cost: 1,
        },
        // Section formula
        Rule {
            id: RuleId(5402),
            name: "section_formula_internal",
            category: RuleCategory::Simplification,
            description: "Internal division m:n: ((mx₂+nx₁)/(m+n), (my₂+ny₁)/(m+n))",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match (a + b)/(c + d) pattern
                if let Expr::Div(num, den) = expr {
                    if matches!(num.as_ref(), Expr::Add(_, _))
                        && matches!(den.as_ref(), Expr::Add(_, _))
                    {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Section formula: point dividing line in ratio m:n".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Triangle area
        Rule {
            id: RuleId(5403),
            name: "triangle_area_coordinates",
            category: RuleCategory::Simplification,
            description: "Triangle area: ½|x₁(y₂-y₃) + x₂(y₃-y₁) + x₃(y₁-y₂)|",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match ½ * |...| pattern
                if let Expr::Mul(half, abs_part) = expr {
                    if is_one_half(half) && matches!(abs_part.as_ref(), Expr::Abs(_)) {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Triangle area using coordinate formula".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Collinearity
        Rule {
            id: RuleId(5404),
            name: "collinearity_condition",
            category: RuleCategory::Simplification,
            description: "Collinear if area = 0: x₁(y₂-y₃) + x₂(y₃-y₁) + x₃(y₁-y₂) = 0",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match expr = 0
                if let Expr::Equation { lhs: _, rhs } = expr {
                    return is_const_zero(rhs);
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Collinearity condition: determinant = 0".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Centroid
        Rule {
            id: RuleId(5405),
            name: "triangle_centroid",
            category: RuleCategory::Simplification,
            description: "Centroid: ((x₁+x₂+x₃)/3, (y₁+y₂+y₃)/3)",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match (a + b + c)/3 pattern
                if let Expr::Div(num, den) = expr {
                    if is_const_three(den) && matches!(num.as_ref(), Expr::Add(_, _)) {
                        return true;
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Centroid formula: (sum of coordinates)/3".to_string(),
                }]
            },
            reversible: true,
            cost: 1,
        },
    ]
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

/// Check if expression is y² pattern
fn is_y_squared(expr: &Expr) -> bool {
    if let Expr::Pow(base, exp) = expr {
        if let Expr::Const(n) = exp.as_ref() {
            if *n == 2.into() {
                return matches!(base.as_ref(), Expr::Var(_));
            }
        }
    }
    false
}

/// Check if expression is 1/x pattern (reciprocal)
fn is_reciprocal(expr: &Expr) -> bool {
    if let Expr::Div(num, _) = expr {
        return is_const_one(num);
    }
    false
}

/// Check if expression equals constant 1
fn is_const_one(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n) if *n == 1.into())
}

/// Check if expression equals constant 0
fn is_const_zero(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n) if *n == 0.into())
}

/// Check if expression equals constant 3
fn is_const_three(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(n) if *n == 3.into())
}

/// Check if expression is 1/2
fn is_one_half(expr: &Expr) -> bool {
    if let Expr::Div(num, den) = expr {
        return is_const_one(num) && matches!(den.as_ref(), Expr::Const(n) if *n == 2.into());
    }
    false
}

/// Check if expression is x² + y² = r² pattern (circle equation)
fn is_circle_equation(expr: &Expr) -> bool {
    if let Expr::Equation { lhs, rhs: _ } = expr {
        if let Expr::Add(left, right) = lhs.as_ref() {
            return is_squared(left) && is_squared(right);
        }
    }
    false
}

/// Check if expression is a² (squared)
fn is_squared(expr: &Expr) -> bool {
    if let Expr::Pow(_, exp) = expr {
        return matches!(exp.as_ref(), Expr::Const(n) if *n == 2.into());
    }
    false
}
