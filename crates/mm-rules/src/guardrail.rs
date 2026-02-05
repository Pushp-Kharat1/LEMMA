// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Guardrails for rule application.
//!
//! This module safeguards the rule engine by analyzing problem complexity,
//! preventing infinite loops, and filtering rules based on problem context
//! (e.g., differentiating between Algebra, Calculus, and Combinatorics).

use crate::{Domain, Rule};
use mm_core::Expr;

/// Characteristics of a problem or sub-problem.
#[derive(Debug, Clone, Default)]
pub struct ProblemProfile {
    pub has_trig: bool,
    pub has_calculus: bool,
    pub has_combinatorics: bool,
    pub complexity: u32,
}

/// Analyze an expression to determine its problem profile.
pub fn analyze(expr: &Expr) -> ProblemProfile {
    let mut profile = ProblemProfile::default();
    scan_expr(expr, &mut profile);
    profile
}

fn scan_expr(expr: &Expr, profile: &mut ProblemProfile) {
    match expr {
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            scan_expr(a, profile);
            scan_expr(b, profile);
            profile.complexity += 1;
        }
        Expr::Sin(_) | Expr::Cos(_) | Expr::Tan(_) => {
            profile.has_trig = true;
            profile.complexity += 2;
        }
        Expr::Derivative { .. } | Expr::Integral { .. } => {
            profile.has_calculus = true;
            profile.complexity += 5;
        }
        _ => {
            // Base cases (Const, Var) have low complexity
            profile.complexity += 1;
        }
    }
}

/// Determines if a rule is applicable based on the problem profile.
///
/// This acts as a high-level filter. For example, if a problem has no
/// trigonometric functions, we shouldn't waste time checking trig rules.
pub fn is_rule_applicable(rule: &Rule, profile: &ProblemProfile) -> bool {
    // Check domain constraints
    for domain in rule.domains {
        match domain {
            Domain::Trigonometry if !profile.has_trig => return false,
            Domain::CalculusDiff | Domain::CalculusInt if !profile.has_calculus => return false,
            _ => {}
        }
    }
    true
}

/// Filters a list of rules to those relevant for the current problem profile.
pub fn filter_rules<'a>(rules: &'a [Rule], profile: &ProblemProfile) -> Vec<&'a Rule> {
    rules
        .iter()
        .filter(|r| is_rule_applicable(r, profile))
        .collect()
}

/// Helper to decompose an additive expression into its terms.
///
/// Useful for analyzing or processing terms independently.
/// e.g., a + b + c -> [a, b, c]
pub fn decompose_additive(expr: &Expr) -> Vec<Expr> {
    let mut terms = Vec::new();
    collect_additive_terms(expr, &mut terms);
    terms
}

fn collect_additive_terms(expr: &Expr, terms: &mut Vec<Expr>) {
    if let Expr::Add(a, b) = expr {
        collect_additive_terms(a, terms);
        collect_additive_terms(b, terms);
    } else {
        terms.push(expr.clone());
    }
}

/// Calculate a heuristic solvability score (0.0 to 1.0).
///
/// Higher scores indicate the expression is "simpler" or closer to a solution.
pub fn solvability_score(expr: &Expr) -> f64 {
    let profile = analyze(expr);
    // Very basic heuristic: lower complexity is better.
    if profile.complexity == 0 {
        return 1.0;
    }
    1.0 / (profile.complexity as f64).sqrt()
}
