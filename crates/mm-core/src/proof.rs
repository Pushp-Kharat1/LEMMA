// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Proof state management for theorem proving.
//!
//! This module provides the foundational structures for managing proof states,
//! including hypotheses (known facts), goals (what to prove), variables with
//! domains, and constraints.
//!
//! # Example
//! ```ignore
//! use mm_core::proof::{ProofState, Hypothesis, Goal, Variable, Domain};
//!
//! // Prove: For positive a,b,c with abc=1, prove a+b+c ≥ 3
//! let mut state = ProofState::new();
//! state.add_variable("a", Domain::PositiveReal);
//! state.add_variable("b", Domain::PositiveReal);
//! state.add_variable("c", Domain::PositiveReal);
//! state.add_constraint(constraint); // abc = 1
//! state.add_goal(goal);             // a+b+c ≥ 3
//! ```

use crate::{Expr, Symbol, SymbolTable};

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for hypotheses
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HypId(pub u32);

/// Unique identifier for goals
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GoalId(pub u32);

// ============================================================================
// ProofState - The main proof context
// ============================================================================

/// A complete proof state containing all context for theorem proving.
///
/// This is the central data structure for IMO-level problem solving.
/// It tracks:
/// - Hypotheses: Known facts (given conditions)
/// - Goals: What we need to prove
/// - Variables: The mathematical variables and their domains
/// - Constraints: Relationships between variables (e.g., abc=1)
#[derive(Debug)]
pub struct ProofState {
    /// Known facts (hypotheses)
    pub hypotheses: Vec<Hypothesis>,
    
    /// Goals to prove
    pub goals: Vec<Goal>,
    
    /// Variables in scope with their domains
    pub variables: Vec<Variable>,
    
    /// Constraints on variables (e.g., abc = 1)
    pub constraints: Vec<Constraint>,
    
    /// Counter for generating unique hypothesis IDs
    next_hyp_id: u32,
    
    /// Counter for generating unique goal IDs
    next_goal_id: u32,
    
    /// Symbol table for variable names
    pub symbols: SymbolTable,
}

impl ProofState {
    /// Create a new empty proof state.
    pub fn new() -> Self {
        ProofState {
            hypotheses: Vec::new(),
            goals: Vec::new(),
            variables: Vec::new(),
            constraints: Vec::new(),
            next_hyp_id: 0,
            next_goal_id: 0,
            symbols: SymbolTable::new(),
        }
    }
    
    /// Add a variable with a domain.
    pub fn add_variable(&mut self, name: &str, domain: Domain) -> Symbol {
        let symbol = self.symbols.intern(name);
        self.variables.push(Variable { symbol, domain });
        symbol
    }
    
    /// Add a hypothesis (known fact).
    pub fn add_hypothesis(&mut self, expr: Expr, origin: HypothesisOrigin) -> HypId {
        let id = HypId(self.next_hyp_id);
        self.next_hyp_id += 1;
        self.hypotheses.push(Hypothesis { id, expr, origin });
        id
    }
    
    /// Add a hypothesis from the problem statement.
    pub fn add_given(&mut self, expr: Expr) -> HypId {
        self.add_hypothesis(expr, HypothesisOrigin::Given)
    }
    
    /// Add a goal to prove.
    pub fn add_goal(&mut self, expr: Expr) -> GoalId {
        let id = GoalId(self.next_goal_id);
        self.next_goal_id += 1;
        self.goals.push(Goal {
            id,
            expr,
            status: GoalStatus::Open,
        });
        id
    }
    
    /// Add a constraint on variables.
    pub fn add_constraint(&mut self, expr: Expr) {
        self.constraints.push(Constraint { expr });
    }
    
    /// Check if all goals are proved.
    pub fn is_complete(&self) -> bool {
        self.goals.iter().all(|g| matches!(g.status, GoalStatus::Proved(_)))
    }
    
    /// Get open (unproved) goals.
    pub fn open_goals(&self) -> Vec<&Goal> {
        self.goals.iter().filter(|g| matches!(g.status, GoalStatus::Open)).collect()
    }
    
    /// Mark a goal as proved.
    pub fn mark_proved(&mut self, goal_id: GoalId, proof: Proof) {
        if let Some(goal) = self.goals.iter_mut().find(|g| g.id == goal_id) {
            goal.status = GoalStatus::Proved(proof);
        }
    }
    
    /// Get a variable's domain.
    pub fn get_domain(&self, symbol: Symbol) -> Option<&Domain> {
        self.variables.iter()
            .find(|v| v.symbol == symbol)
            .map(|v| &v.domain)
    }
    
    /// Check if a variable is positive.
    pub fn is_positive(&self, symbol: Symbol) -> bool {
        self.get_domain(symbol)
            .map(|d| matches!(d, Domain::PositiveReal | Domain::PositiveInteger))
            .unwrap_or(false)
    }
    
    /// Check if all variables are positive (common in inequality problems).
    pub fn all_positive(&self) -> bool {
        self.variables.iter().all(|v| {
            matches!(v.domain, Domain::PositiveReal | Domain::PositiveInteger)
        })
    }
    
    /// Check if we have a constraint of the form `expr = 1` (common in IMO).
    pub fn has_product_one_constraint(&self) -> bool {
        self.constraints.iter().any(|c| {
            matches!(&c.expr, Expr::Equation { rhs, .. } 
                if matches!(rhs.as_ref(), Expr::Const(r) if r.numer() == 1 && r.denom() == 1))
        })
    }
}

impl Default for ProofState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Hypothesis - Known facts
// ============================================================================

/// A hypothesis (known fact) in the proof.
#[derive(Clone, Debug)]
pub struct Hypothesis {
    /// Unique identifier
    pub id: HypId,
    
    /// The expression representing this fact
    pub expr: Expr,
    
    /// How this hypothesis was introduced
    pub origin: HypothesisOrigin,
}

/// How a hypothesis was introduced into the proof.
#[derive(Clone, Debug)]
pub enum HypothesisOrigin {
    /// Given in the problem statement
    Given,
    
    /// Derived using a rule or tactic
    Derived {
        from: Vec<HypId>,
        justification: String,
    },
    
    /// Assumed for proof by contradiction
    Assumption,
    
    /// Case split (one branch of case analysis)
    CaseSplit { case_number: usize, total_cases: usize },
}

// ============================================================================
// Goal - What to prove
// ============================================================================

/// A goal (statement to prove).
#[derive(Clone, Debug)]
pub struct Goal {
    /// Unique identifier
    pub id: GoalId,
    
    /// The expression to prove
    /// For inequalities: Expr::Gte(lhs, rhs) means prove lhs ≥ rhs
    pub expr: Expr,
    
    /// Current status
    pub status: GoalStatus,
}

/// Status of a goal.
#[derive(Clone, Debug)]
pub enum GoalStatus {
    /// Not yet proved
    Open,
    
    /// Successfully proved
    Proved(Proof),
    
    /// Split into sub-goals
    Split(Vec<GoalId>),
    
    /// Reduced to a simpler goal
    Reduced(GoalId),
}

// ============================================================================
// Variable - Variables with domains
// ============================================================================

/// A variable with its domain constraint.
#[derive(Clone, Debug)]
pub struct Variable {
    /// The symbol representing this variable
    pub symbol: Symbol,
    
    /// Domain of the variable
    pub domain: Domain,
}

/// Domain of a variable.
#[derive(Clone, Debug, PartialEq)]
pub enum Domain {
    /// All real numbers
    Real,
    
    /// Positive real numbers (x > 0)
    PositiveReal,
    
    /// Non-negative real numbers (x ≥ 0)
    NonNegativeReal,
    
    /// All integers
    Integer,
    
    /// Positive integers (n > 0)
    PositiveInteger,
    
    /// Natural numbers (n ≥ 0)
    Natural,
    
    /// Bounded interval [a, b]
    Interval { min: f64, max: f64 },
    
    /// Custom domain specified by an expression
    Custom(Expr),
}

// ============================================================================
// Constraint - Relationships between variables
// ============================================================================

/// A constraint on variables (e.g., abc = 1).
#[derive(Clone, Debug)]
pub struct Constraint {
    /// The constraint expression (typically an equation)
    pub expr: Expr,
}

// ============================================================================
// Proof - Proof of a goal
// ============================================================================

/// A proof of a goal.
#[derive(Clone, Debug)]
pub struct Proof {
    /// Steps in the proof
    pub steps: Vec<ProofStep>,
    
    /// Final justification
    pub justification: String,
}

impl Proof {
    /// Create a new empty proof.
    pub fn new(justification: String) -> Self {
        Proof {
            steps: Vec::new(),
            justification,
        }
    }
    
    /// Create a proof by AM-GM inequality.
    pub fn by_am_gm() -> Self {
        Proof::new("AM-GM inequality".to_string())
    }
    
    /// Create a proof by Cauchy-Schwarz inequality.
    pub fn by_cauchy_schwarz() -> Self {
        Proof::new("Cauchy-Schwarz inequality".to_string())
    }
    
    /// Create a proof by Sum of Squares (SOS) decomposition.
    pub fn by_sos() -> Self {
        Proof::new("Sum of squares ≥ 0".to_string())
    }
    
    /// Create a proof by algebraic manipulation.
    pub fn by_algebra() -> Self {
        Proof::new("Algebraic manipulation".to_string())
    }
}

/// A single step in a proof.
#[derive(Clone, Debug)]
pub struct ProofStep {
    /// The expression at this step
    pub expr: Expr,
    
    /// Justification for this step
    pub justification: String,
    
    /// Hypotheses used in this step
    pub used_hypotheses: Vec<HypId>,
}

// ============================================================================
// Comparison Expressions (for inequality goals)
// ============================================================================

/// Extension methods for creating comparison expressions.
impl Expr {
    /// Create a greater-than-or-equal expression: lhs ≥ rhs
    pub fn gte(lhs: Expr, rhs: Expr) -> Expr {
        // We represent this as a special case - could add Gte variant later
        // For now, use a marker: lhs - rhs ≥ 0
        Expr::Sub(Box::new(lhs), Box::new(rhs))
    }
    
    /// Create a greater-than expression: lhs > rhs
    pub fn gt(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Sub(Box::new(lhs), Box::new(rhs))
    }
    
    /// Check if this expression represents a non-negativity claim.
    pub fn is_non_negative_form(&self) -> bool {
        // Check if it's of the form X - Y (representing X ≥ Y)
        matches!(self, Expr::Sub(_, _))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_state_creation() {
        let mut state = ProofState::new();
        
        // Add variables a, b, c > 0
        let a = state.add_variable("a", Domain::PositiveReal);
        let b = state.add_variable("b", Domain::PositiveReal);
        let c = state.add_variable("c", Domain::PositiveReal);
        
        assert!(state.is_positive(a));
        assert!(state.is_positive(b));
        assert!(state.is_positive(c));
        assert!(state.all_positive());
    }
    
    #[test]
    fn test_add_hypothesis() {
        let mut state = ProofState::new();
        let a = state.add_variable("a", Domain::PositiveReal);
        
        // Add hypothesis: a > 0
        let hyp_id = state.add_given(Expr::Var(a));
        
        assert_eq!(state.hypotheses.len(), 1);
        assert_eq!(hyp_id, HypId(0));
    }
    
    #[test]
    fn test_add_goal() {
        let mut state = ProofState::new();
        let a = state.add_variable("a", Domain::PositiveReal);
        let b = state.add_variable("b", Domain::PositiveReal);
        
        // Goal: a + b ≥ 2√(ab)
        let goal_expr = Expr::Sub(
            Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            Box::new(Expr::Mul(
                Box::new(Expr::int(2)),
                Box::new(Expr::Sqrt(Box::new(Expr::Mul(
                    Box::new(Expr::Var(a)),
                    Box::new(Expr::Var(b)),
                )))),
            )),
        );
        
        let goal_id = state.add_goal(goal_expr);
        
        assert_eq!(state.goals.len(), 1);
        assert!(!state.is_complete());
        
        // Mark as proved
        state.mark_proved(goal_id, Proof::by_am_gm());
        assert!(state.is_complete());
    }
}
