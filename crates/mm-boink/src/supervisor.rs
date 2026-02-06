// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! BOINK Supervisor - The self-regulating control layer for LEMMA.
//!
//! Manages the feedback loop between problem solving and credit allocation.

use crate::bank::Bank;
use crate::budget::Budget;
use crate::guardrail::{analyze, ProblemProfile};
use mm_core::Expr;

/// Result of a problem-solving run.
#[derive(Debug, Clone)]
pub struct RunResult {
    /// Whether the problem was solved
    pub solved: bool,
    /// Credits used in this run
    pub credits_used: u64,
    /// Budget allocated for this run
    pub budget_allocated: u64,
    /// Number of rules applied
    pub rules_applied: u32,
    /// Rule names applied (for debugging)
    pub rule_names: Vec<String>,
}

impl RunResult {
    /// Check if this run was under budget.
    pub fn under_budget(&self) -> bool {
        self.credits_used <= self.budget_allocated
    }

    /// Get savings (0 if over budget).
    pub fn savings(&self) -> u64 {
        self.budget_allocated.saturating_sub(self.credits_used)
    }

    /// Get overspend (0 if under budget).
    pub fn overspend(&self) -> u64 {
        self.credits_used.saturating_sub(self.budget_allocated)
    }
}

/// BOINK Supervisor - controls LEMMA's resource usage.
#[derive(Debug)]
pub struct BoinkSupervisor {
    /// Global credit bank
    pub bank: Bank,
    /// Budget adjustment from penalties (negative value)
    pub budget_penalty: i64,
    /// History of overspends for penalty calculation
    overspend_history: Vec<u64>,
    /// Current problem profile
    current_profile: Option<ProblemProfile>,
    /// Current budget
    current_budget: Option<Budget>,
}

impl Default for BoinkSupervisor {
    fn default() -> Self {
        Self::new()
    }
}

impl BoinkSupervisor {
    /// Create a new supervisor with empty bank.
    pub fn new() -> Self {
        Self {
            bank: Bank::new(),
            budget_penalty: 0,
            overspend_history: Vec::new(),
            current_profile: None,
            current_budget: None,
        }
    }

    /// Create a supervisor with existing bank state.
    pub fn with_bank(bank: Bank) -> Self {
        Self {
            bank,
            budget_penalty: 0,
            overspend_history: Vec::new(),
            current_profile: None,
            current_budget: None,
        }
    }

    /// Analyze a problem and allocate budget.
    pub fn begin_problem(&mut self, expr: &Expr) -> Budget {
        // Analyze the problem
        let profile = analyze(expr);
        self.current_profile = Some(profile.clone());

        // Create budget from profile
        let mut budget = Budget::from_profile(&profile);

        // Apply any penalty from previous failures
        if self.budget_penalty > 0 {
            let penalty = self.budget_penalty as u64;
            budget.total = budget.total.saturating_sub(penalty);
        }

        // Add bank bonuses (extra retries from trades)
        budget.max_retries += self.bank.extra_retries;

        self.current_budget = Some(budget.clone());
        budget
    }

    /// Record the result of a problem-solving attempt.
    pub fn record_run(&mut self, result: RunResult) {
        if result.solved && result.under_budget() {
            // SUCCESS: Bank the savings!
            let savings = result.savings();
            self.bank.deposit(savings);

            // Clear overspend history on success
            self.overspend_history.clear();
            self.budget_penalty = 0;

            println!(
                "💰 BOINK: Saved {} credits! Bank balance: {}",
                savings, self.bank.credits
            );
        } else if !result.solved || !result.under_budget() {
            // FAILURE or OVER BUDGET: Record overspend
            let overspend = result.overspend();
            self.overspend_history.push(overspend);

            // Check if we've hit the retry limit
            if let Some(budget) = &self.current_budget {
                if self.overspend_history.len() >= budget.max_retries as usize {
                    // Apply penalty
                    let penalty = budget.calculate_penalty(&self.overspend_history);
                    self.budget_penalty = penalty as i64;

                    println!(
                        "⚠️ BOINK: Penalty applied! Next budget reduced by {}",
                        penalty
                    );

                    // Clear history after applying penalty
                    self.overspend_history.clear();
                }
            }
        }

        // Check if trading is available
        if self.bank.can_trade() {
            println!(
                "🎁 BOINK: You have {} credits! Trade available.",
                self.bank.credits
            );
        }
    }

    /// Get the current bank balance.
    pub fn balance(&self) -> u64 {
        self.bank.credits
    }

    /// Get current budget if active.
    pub fn current_budget(&self) -> Option<&Budget> {
        self.current_budget.as_ref()
    }

    /// Get effective MCTS depth (base + purchased).
    pub fn effective_depth(&self, base_depth: u32) -> u32 {
        base_depth + self.bank.extra_depth
    }

    /// Save state to JSON.
    pub fn save_state(&self) -> String {
        self.bank.save()
    }

    /// Load state from JSON.
    pub fn load_state(&mut self, json: &str) -> bool {
        if let Some(bank) = Bank::load(json) {
            self.bank = bank;
            true
        } else {
            false
        }
    }
}
