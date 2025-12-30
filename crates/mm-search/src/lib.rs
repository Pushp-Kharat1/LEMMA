// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-search
//!
//! Search algorithms for finding solution paths through the mathematical
//! transformation space.
//!
//! This crate provides:
//! - [`BeamSearch`] - A simple beam search algorithm (good for v1)
//! - [`NeuralMCTS`] - Neural-guided Monte Carlo Tree Search
//! - [`MCTS`] - Legacy MCTS (delegates to NeuralMCTS)

pub mod beam;
pub mod mcts;

use mm_core::Expr;
use mm_rules::RuleId;

/// A step in a solution path.
#[derive(Debug, Clone)]
pub struct Step {
    /// The expression before this step.
    pub before: Expr,
    /// The expression after this step.
    pub after: Expr,
    /// The rule that was applied.
    pub rule_id: RuleId,
    /// The rule name.
    pub rule_name: &'static str,
    /// Justification for this step.
    pub justification: String,
}

/// A complete solution.
#[derive(Debug, Clone)]
pub struct Solution {
    /// The original problem.
    pub problem: Expr,
    /// The final result.
    pub result: Expr,
    /// The steps taken.
    pub steps: Vec<Step>,
    /// Whether the solution was verified.
    pub verified: bool,
}

impl Solution {
    /// Get the number of steps in this solution.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }

    /// Check if this solution is empty (no steps needed).
    pub fn is_trivial(&self) -> bool {
        self.steps.is_empty()
    }
}

/// A predicate for checking if a goal has been reached.
pub trait GoalPredicate: Fn(&Expr) -> bool {}
impl<F: Fn(&Expr) -> bool> GoalPredicate for F {}

/// Search configuration.
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Maximum search depth.
    pub max_depth: usize,
    /// Beam width for beam search.
    pub beam_width: usize,
    /// Number of MCTS iterations.
    pub mcts_iterations: usize,
    /// Exploration weight for UCB.
    pub exploration_weight: f64,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_depth: 20,
            beam_width: 10,
            mcts_iterations: 1000,
            exploration_weight: 1.41,
        }
    }
}

pub use beam::BeamSearch;
pub use mcts::{MCTSConfig, MCTSNode, NeuralMCTS, MCTS};
