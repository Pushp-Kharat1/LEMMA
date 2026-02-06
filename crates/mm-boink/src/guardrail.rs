// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Guardrails for rule application.
//!
//! This module provides intelligent filtering of rules based on detected
//! problem domains. It prevents irrelevant rules from being applied
//! (e.g., Tonelli-Shanks on calculus problems).
//!
//! **This is the enhanced version** in mm-boink. The stub in mm-rules
//! should delegate here for full functionality.

use mm_core::Expr;
use mm_rules::{Domain, Rule};

/// Comprehensive problem profile with all detectable domains.
#[derive(Debug, Clone, Default)]
pub struct ProblemProfile {
    // Domain flags
    pub has_trig: bool,
    pub has_calculus_diff: bool,
    pub has_calculus_int: bool,
    pub has_number_theory: bool,
    pub has_combinatorics: bool,
    pub has_inequalities: bool,
    pub has_polynomials: bool,
    pub has_equations: bool,
    pub has_logic: bool,

    // Complexity metrics
    pub complexity: u32,
    pub depth: u32,

    /// Detected domains for filtering
    pub domains: Vec<Domain>,
}

/// Analyze an expression to determine its complete problem profile.
pub fn analyze(expr: &Expr) -> ProblemProfile {
    let mut profile = ProblemProfile::default();
    scan_expr(expr, &mut profile, 0);
    build_domain_list(&mut profile);
    profile
}

fn scan_expr(expr: &Expr, profile: &mut ProblemProfile, depth: u32) {
    profile.depth = profile.depth.max(depth);

    match expr {
        // ========== Binary operations - recurse ==========
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            scan_expr(a, profile, depth + 1);
            scan_expr(b, profile, depth + 1);
            profile.complexity += 1;
        }

        // ========== Trigonometric functions ==========
        Expr::Sin(inner)
        | Expr::Cos(inner)
        | Expr::Tan(inner)
        | Expr::Arcsin(inner)
        | Expr::Arccos(inner)
        | Expr::Arctan(inner) => {
            profile.has_trig = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 2;
        }

        // ========== Calculus - Derivatives ==========
        Expr::Derivative { expr: inner, .. } => {
            profile.has_calculus_diff = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 5;
        }

        // ========== Calculus - Integrals ==========
        Expr::Integral { expr: inner, .. } => {
            profile.has_calculus_int = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 8; // Integrals are harder
        }

        // ========== Number Theory ==========
        Expr::GCD(a, b) | Expr::LCM(a, b) | Expr::Mod(a, b) => {
            profile.has_number_theory = true;
            scan_expr(a, profile, depth + 1);
            scan_expr(b, profile, depth + 1);
            profile.complexity += 3;
        }
        Expr::Floor(inner) | Expr::Ceiling(inner) => {
            profile.has_number_theory = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 2;
        }

        // ========== Combinatorics ==========
        Expr::Factorial(inner) => {
            profile.has_combinatorics = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 4;
        }
        Expr::Binomial(n, k) => {
            profile.has_combinatorics = true;
            scan_expr(n, profile, depth + 1);
            scan_expr(k, profile, depth + 1);
            profile.complexity += 5;
        }
        Expr::Summation { from, to, body, .. } | Expr::BigProduct { from, to, body, .. } => {
            profile.has_combinatorics = true;
            scan_expr(from, profile, depth + 1);
            scan_expr(to, profile, depth + 1);
            scan_expr(body, profile, depth + 1);
            profile.complexity += 6;
        }

        // ========== Inequalities ==========
        Expr::Lt(a, b) | Expr::Lte(a, b) | Expr::Gt(a, b) | Expr::Gte(a, b) => {
            profile.has_inequalities = true;
            scan_expr(a, profile, depth + 1);
            scan_expr(b, profile, depth + 1);
            profile.complexity += 2;
        }

        // ========== Equations ==========
        Expr::Equation { lhs, rhs } => {
            profile.has_equations = true;
            scan_expr(lhs, profile, depth + 1);
            scan_expr(rhs, profile, depth + 1);
            profile.complexity += 2;
        }

        // ========== Power - could indicate polynomials ==========
        Expr::Pow(base, exp) => {
            scan_expr(base, profile, depth + 1);
            scan_expr(exp, profile, depth + 1);
            profile.has_polynomials = true;
            profile.complexity += 2;
        }

        // ========== Exponential and Logarithmic ==========
        Expr::Exp(inner) | Expr::Ln(inner) => {
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 2;
        }

        // ========== Other unary operations ==========
        Expr::Sqrt(inner) | Expr::Neg(inner) | Expr::Abs(inner) => {
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 1;
        }

        // ========== Logical operations ==========
        Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) => {
            profile.has_logic = true;
            scan_expr(a, profile, depth + 1);
            scan_expr(b, profile, depth + 1);
            profile.complexity += 2;
        }
        Expr::Not(inner) => {
            profile.has_logic = true;
            scan_expr(inner, profile, depth + 1);
            profile.complexity += 1;
        }
        Expr::ForAll { body, domain, .. } | Expr::Exists { body, domain, .. } => {
            profile.has_logic = true;
            scan_expr(body, profile, depth + 1);
            if let Some(d) = domain {
                scan_expr(d, profile, depth + 1);
            }
            profile.complexity += 3;
        }

        // ========== N-ary operations ==========
        Expr::Sum(terms) => {
            for term in terms {
                scan_expr(&term.expr, profile, depth + 1);
            }
            profile.complexity += terms.len() as u32;
        }
        Expr::Product(factors) => {
            for factor in factors {
                scan_expr(&factor.base, profile, depth + 1);
                scan_expr(&factor.power, profile, depth + 1);
            }
            profile.complexity += factors.len() as u32;
        }

        // ========== Base cases ==========
        Expr::Const(_) | Expr::Var(_) | Expr::Pi | Expr::E => {
            profile.complexity += 1;
        }
    }
}

/// Build the domain list from detected flags.
fn build_domain_list(profile: &mut ProblemProfile) {
    if profile.has_trig {
        profile.domains.push(Domain::Trigonometry);
    }
    if profile.has_calculus_diff {
        profile.domains.push(Domain::CalculusDiff);
    }
    if profile.has_calculus_int {
        profile.domains.push(Domain::CalculusInt);
    }
    if profile.has_number_theory {
        profile.domains.push(Domain::NumberTheory);
    }
    if profile.has_combinatorics {
        profile.domains.push(Domain::Combinatorics);
    }
    if profile.has_inequalities {
        profile.domains.push(Domain::Inequalities);
    }
    if profile.has_equations {
        profile.domains.push(Domain::Equations);
    }
    if profile.has_polynomials {
        profile.domains.push(Domain::Algebra);
    }

    // Default: always allow Algebra if nothing specific
    if profile.domains.is_empty() {
        profile.domains.push(Domain::Algebra);
    }
}

/// Determines if a rule is applicable based on the problem profile.
///
/// **CRITICAL**: Only allows rules whose domains overlap with detected problem domains.
/// This prevents NumberTheory rules (like Tonelli-Shanks) from being applied to
/// Calculus problems.
pub fn is_rule_applicable(rule: &Rule, profile: &ProblemProfile) -> bool {
    // Empty domain rules (universal) are always applicable
    if rule.domains.is_empty() {
        return true;
    }

    // Check if ANY of the rule's domains match the problem
    for rule_domain in rule.domains {
        // Algebra is universal - always applicable
        if *rule_domain == Domain::Algebra {
            return true;
        }

        // Direct domain match
        if profile.domains.contains(rule_domain) {
            return true;
        }
    }

    // No matching domain - REJECT this rule
    false
}

/// Filters a list of rules to those relevant for the current problem profile.
pub fn filter_rules<'a>(rules: &'a [Rule], profile: &ProblemProfile) -> Vec<&'a Rule> {
    rules
        .iter()
        .filter(|r| is_rule_applicable(r, profile))
        .collect()
}

/// Helper to decompose an additive expression into its terms.
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
    if profile.complexity == 0 {
        return 1.0;
    }
    1.0 / (profile.complexity as f64).sqrt()
}
