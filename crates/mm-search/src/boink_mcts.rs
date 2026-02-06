// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! BOINK-enhanced MCTS with budget tracking.
//!
//! This module provides [`BoinkMCTS`], a wrapper around [`NeuralMCTS`] that adds:
//! - Budget allocation based on problem analysis
//! - Cost tracking for each rule applied
//! - Reward/penalty feedback loop via [`BoinkSupervisor`]
//! - Integration with the global [`Bank`] for credits

use crate::{NeuralMCTS, Solution, Step};
use mm_boink::{analyze, Bank, BoinkSupervisor, Budget, ProblemProfile, RunResult};
use mm_core::Expr;
use mm_rules::{Rule, RuleSet};
use mm_verifier::Verifier;
use std::cell::RefCell;

/// Statistics from a BOINK-tracked run.
#[derive(Debug, Clone)]
pub struct BoinkStats {
    /// Total budget allocated for this problem.
    pub budget_allocated: u64,
    /// Total cost spent during solving.
    pub cost_spent: u64,
    /// Credits remaining (budget - cost).
    pub credits_remaining: i64,
    /// Number of rules applied.
    pub rules_applied: usize,
    /// Whether the problem was solved.
    pub solved: bool,
    /// Detected domains.
    pub domains_detected: Vec<String>,
}

/// A BOINK-enhanced MCTS solver with budget tracking.
///
/// Wraps [`NeuralMCTS`] and adds:
/// - Budget allocation based on problem complexity
/// - Cost tracking per rule application
/// - Reward/penalty feedback loop
///
/// # Example
/// ```ignore
/// let mcts = NeuralMCTS::new(rules, verifier);
/// let boink = BoinkMCTS::new(mcts);
///
/// let (solution, stats) = boink.simplify_tracked(expr);
/// println!("Credits saved: {}", stats.credits_remaining);
/// ```
pub struct BoinkMCTS {
    mcts: NeuralMCTS,
    supervisor: RefCell<BoinkSupervisor>,
}

impl BoinkMCTS {
    /// Create a new BOINK-enhanced MCTS solver.
    pub fn new(mcts: NeuralMCTS) -> Self {
        Self {
            mcts,
            supervisor: RefCell::new(BoinkSupervisor::new()),
        }
    }

    /// Create from components (convenience constructor).
    pub fn from_parts(rules: RuleSet, verifier: Verifier) -> Self {
        Self::new(NeuralMCTS::new(rules, verifier))
    }

    /// Simplify with full budget tracking.
    ///
    /// Returns the solution AND statistics about budget usage.
    pub fn simplify_tracked(&self, expr: Expr) -> (Solution, BoinkStats) {
        // BOINK: Analyze problem and allocate budget
        let profile = analyze(&expr);
        let mut supervisor = self.supervisor.borrow_mut();
        let mut budget = supervisor.begin_problem(&expr);

        // Track domains detected
        let domains_detected: Vec<String> =
            profile.domains.iter().map(|d| format!("{:?}", d)).collect();

        let initial_budget = budget.remaining();

        // Run the actual MCTS simplification
        let solution = self.mcts.simplify(expr.clone());

        // Calculate cost spent (each step has a rule with an associated cost)
        let cost_spent = self.calculate_cost(&solution);

        // Update budget with spent cost
        budget.spend(cost_spent);

        // Collect rule names for debugging
        let rule_names: Vec<String> = solution
            .steps
            .iter()
            .map(|step| step.rule_name.to_string())
            .collect();

        // Record run result for reward/penalty
        let result = RunResult {
            solved: !solution.steps.is_empty(),
            credits_used: cost_spent as u64,
            budget_allocated: initial_budget as u64,
            rules_applied: solution.steps.len() as u32,
            rule_names,
        };
        supervisor.record_run(result);

        // Build stats
        let stats = BoinkStats {
            budget_allocated: initial_budget,
            cost_spent: cost_spent as u64,
            credits_remaining: initial_budget as i64 - cost_spent as i64,
            rules_applied: solution.steps.len(),
            solved: !solution.steps.is_empty(),
            domains_detected,
        };

        (solution, stats)
    }

    /// Calculate total cost of a solution.
    fn calculate_cost(&self, solution: &Solution) -> u32 {
        // Each step has an associated rule with a cost
        // For now, use a fixed cost per step (rules have .cost field)
        // In a full implementation, we'd look up each rule's cost
        solution.steps.len() as u32 * 3 // Average cost = 3 per rule
    }

    /// Get the current bank from the supervisor.
    pub fn bank(&self) -> Bank {
        self.supervisor.borrow().bank.clone()
    }

    /// Get the inner MCTS solver (for direct access).
    pub fn inner(&self) -> &NeuralMCTS {
        &self.mcts
    }

    /// Print BOINK stats to stdout.
    pub fn print_stats(stats: &BoinkStats) {
        println!("╔══════════════════════════════════════════╗");
        println!("║           BOINK Statistics               ║");
        println!("╠══════════════════════════════════════════╣");
        println!(
            "║  Budget Allocated: {:>6}                ║",
            stats.budget_allocated
        );
        println!(
            "║  Cost Spent:       {:>6}                ║",
            stats.cost_spent
        );
        println!(
            "║  Credits Saved:    {:>6}                ║",
            stats.credits_remaining
        );
        println!(
            "║  Rules Applied:    {:>6}                ║",
            stats.rules_applied
        );
        println!(
            "║  Solved:           {:>6}                ║",
            if stats.solved { "YES" } else { "NO" }
        );
        println!("╚══════════════════════════════════════════╝");
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here but require full infrastructure
}
