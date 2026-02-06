// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Core rule definitions and structures.

use mm_core::Expr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Unique identifier for a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuleId(pub u32);

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rule#{}", self.0)
    }
}

/// Category of a mathematical rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    /// Simplification rules that are always beneficial.
    Simplification,
    /// Factoring rules (opposite of expansion).
    Factoring,
    /// Expansion rules (opposite of factoring).
    Expansion,
    /// Rules for solving algebraic equations.
    AlgebraicSolving,
    /// Equation solving rules (linear, quadratic, etc).
    EquationSolving,
    /// Trigonometric identities.
    TrigIdentity,
    /// Derivative rules.
    Derivative,
    /// Integration rules.
    Integral,
    /// Limit evaluation rules.
    Limit,
    /// Inequality rules (AM-GM, Cauchy-Schwarz, bounds).
    Inequality,
    /// Complex number rules.
    Complex,
    /// Logarithm and exponential rules.
    LogExp,
    /// Sequence and series rules.
    Sequence,
    /// Number theory rules.
    NumberTheory,
}

/// Mathematical domain for rule applicability filtering.
///
/// Rules are tagged with domains to prevent wrong-domain matches
/// (e.g., number theory rules on calculus problems).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    /// Algebraic manipulation (factoring, expansion, simplification).
    Algebra,
    /// Differentiation and derivative rules.
    CalculusDiff,
    /// Integration rules.
    CalculusInt,
    /// Trigonometric identities and evaluations.
    Trigonometry,
    /// Vector operations.
    Vector,
    /// Number theory (primes, divisibility, modular arithmetic).
    NumberTheory,
    /// Combinatorics (permutations, combinations, counting).
    Combinatorics,
    /// Inequality manipulation and bounds.
    Inequalities,
    /// Equation solving.
    Equations,
    /// Coordinate and synthetic geometry (conics, circles, triangles).
    Geometry,
}

/// AST features required for a rule to apply.
///
/// Used by the guardrail to filter rules based on expression structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feature {
    /// Expression contains an integral.
    Integral,
    /// Expression contains a derivative.
    Derivative,
    /// Expression contains trigonometric functions.
    Trig,
    /// Expression contains exponentials (a^x or e^x).
    Exponential,
    /// Expression contains logarithms.
    Logarithm,
    /// Expression is a product.
    Product,
    /// Expression is a composite function.
    Composite,
    /// Expression contains fractional powers (x^(1/2), etc).
    FractionalPower,
    /// Expression is a polynomial.
    Polynomial,
    /// Expression contains an equation.
    Equation,
    /// Expression contains an inequality.
    Inequality,
    /// Expression involves limits.
    Limit,
    /// Expression involves vectors.
    Vector,
    /// Expression involves partial derivatives.
    PartialDerivative,
    /// Expression involves combinatorics.
    Combinatorics,
    /// Expression involves conic sections (parabola, ellipse, hyperbola).
    ConicSection,
}

/// Context for rule application.
///
/// Contains information that rules might need, such as the variable
/// being solved for in an equation.
#[derive(Debug, Clone, Default)]
pub struct RuleContext {
    /// The variable we're trying to solve for (if any).
    pub target_var: Option<mm_core::Symbol>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

/// A single rule application result.
#[derive(Debug, Clone)]
pub struct RuleApplication {
    /// The resulting expression after applying the rule.
    pub result: Expr,
    /// Human-readable justification for this step.
    pub justification: String,
}

/// A mathematical transformation rule.
pub struct Rule {
    /// Unique identifier.
    pub id: RuleId,
    /// Human-readable name.
    pub name: &'static str,
    /// Category for organization and strategy.
    pub category: RuleCategory,
    /// Description for explanation.
    pub description: &'static str,
    /// Mathematical domains this rule applies to.
    /// Empty slice means "applicable to all domains" (backward compatibility).
    pub domains: &'static [Domain],
    /// AST features required for this rule to be considered.
    /// Empty slice means "no specific features required" (backward compatibility).
    pub requires: &'static [Feature],
    /// Check if this rule can be applied to the expression.
    pub is_applicable: fn(&Expr, &RuleContext) -> bool,
    /// Apply the rule, returning possible results.
    pub apply: fn(&Expr, &RuleContext) -> Vec<RuleApplication>,
    /// Is this rule bidirectional?
    pub reversible: bool,
    /// Cost heuristic (lower = prefer).
    pub cost: u32,
}

impl Rule {
    /// Check if this rule can be applied.
    pub fn can_apply(&self, expr: &Expr, ctx: &RuleContext) -> bool {
        (self.is_applicable)(expr, ctx)
    }

    /// Apply this rule to an expression.
    pub fn apply(&self, expr: &Expr, ctx: &RuleContext) -> Vec<RuleApplication> {
        (self.apply)(expr, ctx)
    }
}

/// Macro for creating Rule structs with default `domains` and `requires` fields.
///
/// This provides backward compatibility - existing rules don't need to specify
/// the new domain/feature fields. They default to empty slices, which means
/// "applicable to all domains" (guardrail won't filter them).
///
/// # Example
///
/// ```ignore
/// rule! {
///     id: RuleId(1),
///     name: "const_fold",
///     category: RuleCategory::Simplification,
///     description: "Evaluate constant expressions",
///     is_applicable: |expr, _ctx| { ... },
///     apply: |expr, _ctx| { ... },
///     reversible: false,
///     cost: 1,
/// }
/// ```
///
/// Or with explicit domains/requires:
///
/// ```ignore
/// rule! {
///     id: RuleId(101),
///     name: "power_rule",
///     category: RuleCategory::Derivative,
///     description: "d/dx[x^n] = n*x^(n-1)",
///     domains: &[Domain::CalculusDiff],
///     requires: &[Feature::Derivative],
///     is_applicable: |expr, _ctx| { ... },
///     apply: |expr, _ctx| { ... },
///     reversible: false,
///     cost: 2,
/// }
/// ```
#[macro_export]
macro_rules! rule {
    // Version with explicit domains and requires
    {
        id: $id:expr,
        name: $name:expr,
        category: $category:expr,
        description: $description:expr,
        domains: $domains:expr,
        requires: $requires:expr,
        is_applicable: $is_applicable:expr,
        apply: $apply:expr,
        reversible: $reversible:expr,
        cost: $cost:expr $(,)?
    } => {
        Rule {
            id: $id,
            name: $name,
            category: $category,
            description: $description,
            domains: $domains,
            requires: $requires,
            is_applicable: $is_applicable,
            apply: $apply,
            reversible: $reversible,
            cost: $cost,
        }
    };
    // Version without domains/requires (backward compatibility)
    {
        id: $id:expr,
        name: $name:expr,
        category: $category:expr,
        description: $description:expr,
        domains: &[],
        requires: &[],
        is_applicable: $is_applicable:expr,
        apply: $apply:expr,
        reversible: $reversible:expr,
        cost: $cost:expr $(,)?
    } => {
        Rule {
            id: $id,
            name: $name,
            category: $category,
            description: $description,
            domains: &[],  // Default: applicable to all domains
            requires: &[], // Default: no specific features required
            is_applicable: $is_applicable,
            apply: $apply,
            reversible: $reversible,
            cost: $cost,
        }
    };
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rule")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("category", &self.category)
            .finish()
    }
}

/// A collection of rules.
#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
    by_id: HashMap<RuleId, usize>,
    by_category: HashMap<RuleCategory, Vec<RuleId>>,
}

impl RuleSet {
    /// Create a new empty rule set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a rule to the set.
    pub fn add(&mut self, rule: Rule) {
        let id = rule.id;
        let category = rule.category;
        let idx = self.rules.len();

        self.rules.push(rule);
        self.by_id.insert(id, idx);
        self.by_category.entry(category).or_default().push(id);
    }

    /// Get a rule by ID.
    pub fn get(&self, id: RuleId) -> Option<&Rule> {
        self.by_id.get(&id).map(|&idx| &self.rules[idx])
    }

    /// Get all rules.
    pub fn all(&self) -> &[Rule] {
        &self.rules
    }

    /// Get rules by category.
    pub fn by_category(&self, category: RuleCategory) -> Vec<&Rule> {
        self.by_category
            .get(&category)
            .map(|ids| ids.iter().filter_map(|id| self.get(*id)).collect())
            .unwrap_or_default()
    }

    /// Find all applicable rules for an expression.
    pub fn applicable(&self, expr: &Expr, ctx: &RuleContext) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|rule| rule.can_apply(expr, ctx))
            .collect()
    }

    /// Get the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Check if the rule set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

/// Create a standard rule set with all built-in rules.
///
/// Current state: 162 working rules, 151 stubs in mixed modules
/// Stubs are included but won't match any expressions (is_applicable returns false)
pub fn standard_rules() -> RuleSet {
    let mut rules = RuleSet::new();

    // FULLY WORKING MODULES (0 stubs):

    // Add algebra rules - 36 working, 0 stubs
    for rule in crate::algebra::algebra_rules() {
        rules.add(rule);
    }

    // Add trig rules - 43 working, 0 stubs
    for rule in crate::trig::trig_rules() {
        rules.add(rule);
    }

    // Add equation solving rules - 7 working, 0 stubs
    for rule in crate::equations::equation_rules() {
        rules.add(rule);
    }

    // Add integration rules - 9 working, 0 stubs
    for rule in crate::integration::integration_rules() {
        rules.add(rule);
    }

    // MIXED MODULES (have both working and stub rules):

    // Add calculus rules - 15 working, 2 stubs
    for rule in crate::calculus::calculus_rules() {
        rules.add(rule);
    }

    // Add inequality rules - 20 working, 12 stubs
    for rule in crate::inequalities::inequality_rules() {
        rules.add(rule);
    }

    // Add number theory rules - 28 working, 56 stubs
    for rule in crate::number_theory::number_theory_rules() {
        rules.add(rule);
    }

    // Add combinatorics rules - 1 working, 45 stubs
    for rule in crate::combinatorics::combinatorics_rules() {
        rules.add(rule);
    }

    // Add polynomial rules - 3 working, 36 stubs
    for rule in crate::polynomials::polynomial_rules() {
        rules.add(rule);
    }

    // DELETED pure-stub modules (had 0 working rules):
    // complex.rs, logarithm.rs, sequences.rs, geometry.rs, modular.rs, functional.rs
    // These were created as stubs and never implemented - now deleted.

    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_set_operations() {
        let rules = standard_rules();
        assert!(!rules.is_empty());
        println!("Loaded {} rules", rules.len());
    }
}
