// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Inequality Chaining for Transitive Reasoning
//!
//! Enables proofs like:
//! - If a > b and b > c, then a > c
//! - If a ≥ b and b ≥ c, then a ≥ c
//! - If a > b and b ≥ c, then a > c

use crate::polynomial::algebraically_equal;
use mm_core::Expr;

/// An inequality fact: lhs OP rhs where OP is >, ≥, <, or ≤
#[derive(Debug, Clone)]
pub struct InequalityFact {
    pub lhs: Expr,
    pub rhs: Expr,
    pub is_strict: bool, // true for >, false for ≥
}

impl InequalityFact {
    /// Create a > b
    pub fn gt(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            is_strict: true,
        }
    }

    /// Create a ≥ b
    pub fn gte(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            is_strict: false,
        }
    }

    /// Try to chain two inequalities: self and other
    /// Returns Some(new_fact) if they can be chained
    pub fn chain(&self, other: &InequalityFact) -> Option<InequalityFact> {
        // Check if self.rhs == other.lhs (algebraically)
        if algebraically_equal(&self.rhs, &other.lhs) == Some(true) {
            // a OP1 b and b OP2 c => a OP c
            // where OP is strict if either OP1 or OP2 is strict
            Some(InequalityFact {
                lhs: self.lhs.clone(),
                rhs: other.rhs.clone(),
                is_strict: self.is_strict || other.is_strict,
            })
        } else {
            None
        }
    }

    /// Check if this fact implies the goal
    pub fn implies(&self, goal: &InequalityFact) -> bool {
        // self implies goal if:
        // 1. LHS matches algebraically
        // 2. RHS matches algebraically
        // 3. self is at least as strong (strict implies non-strict)

        let lhs_match = algebraically_equal(&self.lhs, &goal.lhs) == Some(true);
        let rhs_match = algebraically_equal(&self.rhs, &goal.rhs) == Some(true);

        if !lhs_match || !rhs_match {
            return false;
        }

        // a > b implies a > b (same strictness)
        // a > b implies a ≥ b (strict implies non-strict)
        // a ≥ b does NOT imply a > b
        if goal.is_strict && !self.is_strict {
            return false;
        }

        true
    }
}

/// A collection of known inequality facts for chaining
#[derive(Debug, Default)]
pub struct InequalityChain {
    facts: Vec<InequalityFact>,
}

impl InequalityChain {
    pub fn new() -> Self {
        Self { facts: Vec::new() }
    }

    /// Add a known fact
    pub fn add_fact(&mut self, fact: InequalityFact) {
        self.facts.push(fact);
    }

    /// Add a > b
    pub fn add_gt(&mut self, lhs: Expr, rhs: Expr) {
        self.add_fact(InequalityFact::gt(lhs, rhs));
    }

    /// Add a ≥ b
    pub fn add_gte(&mut self, lhs: Expr, rhs: Expr) {
        self.add_fact(InequalityFact::gte(lhs, rhs));
    }

    /// Try to prove a goal using known facts and chaining
    pub fn prove(&self, goal: &InequalityFact) -> Option<Vec<usize>> {
        // First check if any single fact implies the goal
        for (i, fact) in self.facts.iter().enumerate() {
            if fact.implies(goal) {
                return Some(vec![i]);
            }
        }

        // Try chaining two facts
        for (i, fact1) in self.facts.iter().enumerate() {
            for (j, fact2) in self.facts.iter().enumerate() {
                if i == j {
                    continue;
                }

                if let Some(chained) = fact1.chain(fact2) {
                    if chained.implies(goal) {
                        return Some(vec![i, j]);
                    }
                }
            }
        }

        // Try chaining three facts (limited depth)
        for (i, fact1) in self.facts.iter().enumerate() {
            for (j, fact2) in self.facts.iter().enumerate() {
                if i == j {
                    continue;
                }

                if let Some(chained12) = fact1.chain(fact2) {
                    for (k, fact3) in self.facts.iter().enumerate() {
                        if i == k || j == k {
                            continue;
                        }

                        if let Some(chained123) = chained12.chain(fact3) {
                            if chained123.implies(goal) {
                                return Some(vec![i, j, k]);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Extract facts from an expression context (e.g., hypothesis)
    pub fn extract_from_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Gt(lhs, rhs) => {
                self.add_gt(lhs.as_ref().clone(), rhs.as_ref().clone());
            }
            Expr::Gte(lhs, rhs) => {
                self.add_gte(lhs.as_ref().clone(), rhs.as_ref().clone());
            }
            Expr::Lt(lhs, rhs) => {
                // a < b is equivalent to b > a
                self.add_gt(rhs.as_ref().clone(), lhs.as_ref().clone());
            }
            Expr::Lte(lhs, rhs) => {
                // a ≤ b is equivalent to b ≥ a
                self.add_gte(rhs.as_ref().clone(), lhs.as_ref().clone());
            }
            Expr::And(a, b) => {
                self.extract_from_expr(a);
                self.extract_from_expr(b);
            }
            _ => {}
        }
    }
}

/// Try to prove an inequality goal from hypotheses using chaining
pub fn prove_inequality_by_chaining(hypotheses: &[Expr], goal: &Expr) -> bool {
    let mut chain = InequalityChain::new();

    // Extract facts from hypotheses
    for hyp in hypotheses {
        chain.extract_from_expr(hyp);
    }

    // Convert goal to InequalityFact
    let goal_fact = match goal {
        Expr::Gt(lhs, rhs) => InequalityFact::gt(lhs.as_ref().clone(), rhs.as_ref().clone()),
        Expr::Gte(lhs, rhs) => InequalityFact::gte(lhs.as_ref().clone(), rhs.as_ref().clone()),
        Expr::Lt(lhs, rhs) => InequalityFact::gt(rhs.as_ref().clone(), lhs.as_ref().clone()),
        Expr::Lte(lhs, rhs) => InequalityFact::gte(rhs.as_ref().clone(), lhs.as_ref().clone()),
        _ => return false,
    };

    chain.prove(&goal_fact).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_simple_transitivity() {
        let mut symbols = SymbolTable::new();
        let a = symbols.intern("a");
        let b = symbols.intern("b");
        let c = symbols.intern("c");

        let mut chain = InequalityChain::new();
        chain.add_gt(Expr::Var(a), Expr::Var(b)); // a > b
        chain.add_gt(Expr::Var(b), Expr::Var(c)); // b > c

        // Should prove a > c
        let goal = InequalityFact::gt(Expr::Var(a), Expr::Var(c));
        assert!(chain.prove(&goal).is_some());
    }

    #[test]
    fn test_mixed_strictness() {
        let mut symbols = SymbolTable::new();
        let a = symbols.intern("a");
        let b = symbols.intern("b");
        let c = symbols.intern("c");

        let mut chain = InequalityChain::new();
        chain.add_gt(Expr::Var(a), Expr::Var(b)); // a > b
        chain.add_gte(Expr::Var(b), Expr::Var(c)); // b ≥ c

        // Should prove a > c (strict because a > b is strict)
        let goal = InequalityFact::gt(Expr::Var(a), Expr::Var(c));
        assert!(chain.prove(&goal).is_some());

        // Should also prove a ≥ c
        let goal_weak = InequalityFact::gte(Expr::Var(a), Expr::Var(c));
        assert!(chain.prove(&goal_weak).is_some());
    }

    #[test]
    fn test_from_hypotheses() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");
        let z = symbols.intern("z");

        let hypotheses = vec![
            Expr::Gt(Box::new(Expr::Var(x)), Box::new(Expr::Var(y))),
            Expr::Gt(Box::new(Expr::Var(y)), Box::new(Expr::Var(z))),
        ];

        let goal = Expr::Gt(Box::new(Expr::Var(x)), Box::new(Expr::Var(z)));

        assert!(prove_inequality_by_chaining(&hypotheses, &goal));
    }
}
