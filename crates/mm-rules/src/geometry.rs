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
        Rule {
            id: RuleId(5001),
            name: "parabola_tangent_parametric",
            category: RuleCategory::Simplification,
            description: "Tangent to parabola y²=4ax at parameter t: ty = x + at²",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
        Rule {
            id: RuleId(5003),
            name: "parabola_focal_chord_reciprocal",
            category: RuleCategory::Simplification,
            description: "Focal chord property: 1/SP + 1/SQ = 2/l",
            domains: &[Domain::Geometry],
            requires: &[Feature::ConicSection],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
            is_applicable: |_expr, _ctx| true,
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
            is_applicable: |_expr, _ctx| true,
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
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
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
        Rule {
            id: RuleId(5401),
            name: "distance_formula",
            category: RuleCategory::Simplification,
            description: "Distance: d = √((x₂-x₁)² + (y₂-y₁)²)",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 1,
        },
        Rule {
            id: RuleId(5402),
            name: "section_formula_internal",
            category: RuleCategory::Simplification,
            description: "Internal division m:n: ((mx₂+nx₁)/(m+n), (my₂+ny₁)/(m+n))",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5403),
            name: "triangle_area_coordinates",
            category: RuleCategory::Simplification,
            description: "Triangle area: ½|x₁(y₂-y₃) + x₂(y₃-y₁) + x₃(y₁-y₂)|",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
        Rule {
            id: RuleId(5404),
            name: "collinearity_condition",
            category: RuleCategory::Simplification,
            description: "Collinear if area = 0: x₁(y₂-y₃) + x₂(y₃-y₁) + x₃(y₁-y₂) = 0",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        Rule {
            id: RuleId(5405),
            name: "triangle_centroid",
            category: RuleCategory::Simplification,
            description: "Centroid: ((x₁+x₂+x₃)/3, (y₁+y₂+y₃)/3)",
            domains: &[Domain::Geometry],
            requires: &[],
            is_applicable: |_expr, _ctx| true,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 1,
        },
    ]
}
