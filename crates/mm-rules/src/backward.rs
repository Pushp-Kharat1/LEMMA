// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Backward rule application for proof search
//!
//! This module enables reasoning BACKWARDS from goals:
//! - Given goal G, find expression E such that E implies G
//! - Example: Goal is `x² + y² ≥ 2xy`
//!   - What would prove this? `(x-y)² ≥ 0` would!
//!   - Because (x-y)² = x² - 2xy + y² ≥ 0 implies x² + y² ≥ 2xy

use mm_core::{Expr, Rational};

/// Strategies for backward reasoning from a goal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackwardStrategy {
    /// Find nonnegative expression that proves inequality
    InequalityToNonneg,

    /// Expand/factor to find equivalent form
    EquivalentForm,

    /// Apply known theorem backwards
    TheoremApplication,

    /// Substitute to create simpler goal
    Substitution,

    /// Universal introduction: To prove ∀x.P(x), prove P for arbitrary x
    UniversalIntro,

    /// Existential witness: To prove ∃x.P(x), find witness w and prove P(w)
    ExistentialWitness,
}

/// Result of backward reasoning
#[derive(Debug, Clone)]
pub struct BackwardStep {
    /// The new subgoal(s) that would prove the original goal
    pub subgoals: Vec<Expr>,

    /// Strategy used
    pub strategy: BackwardStrategy,

    /// Human-readable justification
    pub justification: String,
}

/// Backward rule application
pub trait BackwardRule {
    /// Given a goal, find what would prove it
    fn would_prove(&self, goal: &Expr) -> Vec<BackwardStep>;
}

// ============================================================================
// Core backward reasoning functions
// ============================================================================

/// Find an expression that, if proven, would prove the goal
pub fn find_proof_of(goal: &Expr) -> Vec<BackwardStep> {
    let mut steps = Vec::new();

    // Strategy 1: Inequality A ≥ B ← (A-B) ≥ 0
    if let Some(step) = inequality_to_difference(goal) {
        steps.push(step);
    }

    // Strategy 2: Inequality A ≥ B ← squared form
    if let Some(step) = inequality_to_squared(goal) {
        steps.push(step);
    }

    // Strategy 3: Equation A = B ← factored/expanded form
    if let Some(step) = equation_to_factored(goal) {
        steps.push(step);
    }

    // Strategy 4: Universal quantifier ← prove for arbitrary variable
    if let Some(step) = forall_introduction(goal) {
        steps.push(step);
    }

    // Strategy 5: Existential quantifier ← find witness
    if let Some(step) = exists_witness(goal) {
        steps.push(step);
    }

    steps
}

/// Strategy: A ≥ B ← Prove (A - B) ≥ 0
/// Also handles: A ≤ B ← Prove (B - A) ≥ 0
fn inequality_to_difference(goal: &Expr) -> Option<BackwardStep> {
    match goal {
        Expr::Gte(lhs, rhs) => {
            let diff = Expr::Sub(lhs.clone(), rhs.clone());
            let zero = Box::new(Expr::Const(Rational::from(0)));
            let new_goal = Expr::Gte(Box::new(diff), zero);

            Some(BackwardStep {
                subgoals: vec![new_goal],
                strategy: BackwardStrategy::InequalityToNonneg,
                justification: "A ≥ B if and only if A - B ≥ 0".to_string(),
            })
        }
        Expr::Lte(lhs, rhs) => {
            // A ≤ B ↔ B ≥ A ↔ B - A ≥ 0
            let diff = Expr::Sub(rhs.clone(), lhs.clone());
            let zero = Box::new(Expr::Const(Rational::from(0)));
            let new_goal = Expr::Gte(Box::new(diff), zero);

            Some(BackwardStep {
                subgoals: vec![new_goal],
                strategy: BackwardStrategy::InequalityToNonneg,
                justification: "A ≤ B if and only if B - A ≥ 0".to_string(),
            })
        }
        _ => None,
    }
}

/// Strategy: A ≥ B ← Find X² form
/// Also handles: A ≤ B ← Find X² form for B - A
///
/// Example: x² + y² ≥ 2xy ← (x-y)² ≥ 0
fn inequality_to_squared(goal: &Expr) -> Option<BackwardStep> {
    match goal {
        Expr::Gte(lhs, rhs) => {
            // Try to find (A - B)² pattern
            if let Some(squared_form) = try_find_squared_form(lhs, rhs) {
                let zero = Box::new(Expr::Const(Rational::from(0)));
                let new_goal = Expr::Gte(Box::new(squared_form.clone()), zero);

                Some(BackwardStep {
                    subgoals: vec![new_goal],
                    strategy: BackwardStrategy::InequalityToNonneg,
                    justification: format!(
                        "Proved by showing {:?} ≥ 0 (squares are nonnegative)",
                        squared_form
                    ),
                })
            } else {
                None
            }
        }
        Expr::Lte(lhs, rhs) => {
            // A ≤ B ↔ B ≥ A, so check B - A
            if let Some(squared_form) = try_find_squared_form(rhs, lhs) {
                let zero = Box::new(Expr::Const(Rational::from(0)));
                let new_goal = Expr::Gte(Box::new(squared_form.clone()), zero);

                Some(BackwardStep {
                    subgoals: vec![new_goal],
                    strategy: BackwardStrategy::InequalityToNonneg,
                    justification: format!(
                        "Proved by showing {:?} ≥ 0 (squares are nonnegative)",
                        squared_form
                    ),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Try to find X² form that would prove lhs ≥ rhs
///
/// Common patterns:
/// - x² + y² - 2xy = (x-y)²
/// - a² + b² + c² - ab - bc - ca = ½((a-b)² + (b-c)² + (c-a)²)
fn try_find_squared_form(lhs: &Expr, rhs: &Expr) -> Option<Expr> {
    // Create difference: lhs - rhs
    let diff = Expr::Sub(Box::new(lhs.clone()), Box::new(rhs.clone()));

    // Pattern matching for common squared forms
    // For now, handle the simple case: x² + y² - 2xy = (x-y)²
    match &diff {
        Expr::Sub(a, b) => {
            // Check if this matches x² + y² - 2xy pattern
            if let Expr::Add(term1, term2) = a.as_ref() {
                if let (Expr::Pow(x1, exp1), Expr::Pow(y1, exp2)) = (term1.as_ref(), term2.as_ref())
                {
                    // Check if both are squared
                    if is_const_2(exp1) && is_const_2(exp2) {
                        // Check if rhs is 2xy
                        if let Expr::Mul(coef, product) = b.as_ref() {
                            if is_const_2(coef) {
                                if let Expr::Mul(x2, y2) = product.as_ref() {
                                    // Check if x1, x2 match and y1, y2 match
                                    if exprs_match(x1, x2) && exprs_match(y1, y2) {
                                        // Return (x - y)²
                                        let x_minus_y = Expr::Sub(x1.clone(), y1.clone());
                                        return Some(Expr::Pow(
                                            Box::new(x_minus_y),
                                            Box::new(Expr::Const(Rational::from(2))),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    None
}

/// Strategy: A = B ← factored or expanded form
fn equation_to_factored(goal: &Expr) -> Option<BackwardStep> {
    match goal {
        Expr::Equation { lhs, rhs } => {
            // Try to factor lhs - rhs
            let diff = Expr::Sub(lhs.clone(), rhs.clone());

            // Common factorizations
            // For now, just return a symbolic suggestion
            Some(BackwardStep {
                subgoals: vec![Expr::Equation {
                    lhs: Box::new(diff),
                    rhs: Box::new(Expr::Const(Rational::from(0))),
                }],
                strategy: BackwardStrategy::EquivalentForm,
                justification: "A = B if and only if A - B = 0".to_string(),
            })
        }
        _ => None,
    }
}

/// Strategy: ∀x.P(x) ← prove P(x) for arbitrary x
///
/// To prove a universal statement, we prove it for an arbitrary/fresh variable.
/// The subgoal is the body with the bound variable.
fn forall_introduction(goal: &Expr) -> Option<BackwardStep> {
    match goal {
        Expr::ForAll { var, body, .. } => {
            // Subgoal is the body - prover will need to treat 'var' as arbitrary
            Some(BackwardStep {
                subgoals: vec![body.as_ref().clone()],
                strategy: BackwardStrategy::UniversalIntro,
                justification: format!("To prove ∀{:?}.P, prove P for arbitrary {:?}", var, var),
            })
        }
        _ => None,
    }
}

/// Strategy: ∃x.P(x) ← find witness w and prove P(w)
///
/// To prove an existential, we need to find a witness value.
/// Returns common potential witnesses as subgoals.
fn exists_witness(goal: &Expr) -> Option<BackwardStep> {
    match goal {
        Expr::Exists { var, body, .. } => {
            // For existential, we suggest trying common witnesses
            // The prover should try substituting these values
            let witnesses = vec![Expr::int(0), Expr::int(1), Expr::int(-1)];

            // Generate subgoals by substituting each witness
            let subgoals: Vec<Expr> = witnesses
                .iter()
                .map(|w| substitute_var(body, *var, w))
                .collect();

            Some(BackwardStep {
                subgoals,
                strategy: BackwardStrategy::ExistentialWitness,
                justification: format!("To prove ∃{:?}.P, find witness w where P(w) holds", var),
            })
        }
        _ => None,
    }
}

/// Simple variable substitution for backward reasoning
fn substitute_var(body: &Expr, var: mm_core::Symbol, value: &Expr) -> Expr {
    match body {
        Expr::Var(v) if *v == var => value.clone(),
        Expr::Var(_) | Expr::Const(_) | Expr::Pi | Expr::E => body.clone(),

        Expr::Neg(e) => Expr::Neg(Box::new(substitute_var(e, var, value))),
        Expr::Sqrt(e) => Expr::Sqrt(Box::new(substitute_var(e, var, value))),
        Expr::Sin(e) => Expr::Sin(Box::new(substitute_var(e, var, value))),
        Expr::Cos(e) => Expr::Cos(Box::new(substitute_var(e, var, value))),
        Expr::Tan(e) => Expr::Tan(Box::new(substitute_var(e, var, value))),
        Expr::Ln(e) => Expr::Ln(Box::new(substitute_var(e, var, value))),
        Expr::Exp(e) => Expr::Exp(Box::new(substitute_var(e, var, value))),
        Expr::Abs(e) => Expr::Abs(Box::new(substitute_var(e, var, value))),
        Expr::Floor(e) => Expr::Floor(Box::new(substitute_var(e, var, value))),
        Expr::Ceiling(e) => Expr::Ceiling(Box::new(substitute_var(e, var, value))),
        Expr::Factorial(e) => Expr::Factorial(Box::new(substitute_var(e, var, value))),
        Expr::Not(e) => Expr::Not(Box::new(substitute_var(e, var, value))),

        Expr::Add(a, b) => Expr::Add(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Sub(a, b) => Expr::Sub(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Mul(a, b) => Expr::Mul(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Div(a, b) => Expr::Div(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Pow(a, b) => Expr::Pow(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Gte(a, b) => Expr::Gte(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Gt(a, b) => Expr::Gt(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Lte(a, b) => Expr::Lte(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Lt(a, b) => Expr::Lt(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::And(a, b) => Expr::And(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Or(a, b) => Expr::Or(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),
        Expr::Implies(a, b) => Expr::Implies(
            Box::new(substitute_var(a, var, value)),
            Box::new(substitute_var(b, var, value)),
        ),

        Expr::Equation { lhs, rhs } => Expr::Equation {
            lhs: Box::new(substitute_var(lhs, var, value)),
            rhs: Box::new(substitute_var(rhs, var, value)),
        },

        // For other complex expressions, just clone (simplified)
        _ => body.clone(),
    }
}

// ============================================================================
// Helper functions
// ============================================================================

fn is_const_2(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(r) if *r == Rational::from(2))
}

fn exprs_match(e1: &Expr, e2: &Expr) -> bool {
    // Simple structural equality check
    // In a real system, this would be more sophisticated
    format!("{:?}", e1) == format!("{:?}", e2)
}

// ============================================================================
// Public API for backward search
// ============================================================================

/// Main entry point for backward reasoning
///
/// Given a goal expression, returns possible backward steps
pub fn backward_search(goal: &Expr) -> Vec<BackwardStep> {
    find_proof_of(goal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_inequality_to_difference() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        // Goal: x ≥ y
        let goal = Expr::Gte(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));

        let steps = backward_search(&goal);
        assert!(!steps.is_empty());

        // Should suggest: x - y ≥ 0
        let step = &steps[0];
        assert_eq!(step.strategy, BackwardStrategy::InequalityToNonneg);
    }

    #[test]
    fn test_squared_form_detection() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        // Goal: x² + y² ≥ 2xy
        let lhs = Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
        );

        let rhs = Expr::Mul(
            Box::new(Expr::int(2)),
            Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
        );

        let goal = Expr::Gte(Box::new(lhs), Box::new(rhs));

        let steps = backward_search(&goal);

        // Should find at least the difference strategy
        assert!(!steps.is_empty());
    }

    #[test]
    fn test_forall_backward() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        // Goal: ∀n. n² ≥ 0
        let goal = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Gte(
                Box::new(Expr::Pow(Box::new(Expr::Var(n)), Box::new(Expr::int(2)))),
                Box::new(Expr::int(0)),
            )),
        };

        let steps = backward_search(&goal);

        // Should find universal introduction strategy
        let has_forall_strategy = steps
            .iter()
            .any(|s| s.strategy == BackwardStrategy::UniversalIntro);
        assert!(
            has_forall_strategy,
            "Should find UniversalIntro strategy for ∀ goal"
        );
    }
}
