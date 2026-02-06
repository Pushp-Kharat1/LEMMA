// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Budget allocation based on problem complexity.
//!
//! Analyzes detected domains and allocates appropriate credit budgets.

use crate::guardrail::ProblemProfile;
use mm_rules::Domain;

/// Problem difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Single domain, simple operations
    Easy,
    /// 1-2 domains, moderate complexity
    Medium,
    /// 2-3 domains, requires multi-step reasoning
    Hard,
    /// 3+ domains, competition-level
    Olympiad,
}

impl Difficulty {
    /// Base retry count for this difficulty.
    pub fn base_retries(&self) -> u32 {
        match self {
            Difficulty::Easy => 2,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
            Difficulty::Olympiad => 6,
        }
    }

    /// Penalty multiplier (lower = harsher penalties).
    pub fn penalty_factor(&self) -> f64 {
        match self {
            Difficulty::Easy => 1.0, // Full penalty
            Difficulty::Medium => 0.75,
            Difficulty::Hard => 0.5,
            Difficulty::Olympiad => 0.3, // Lenient
        }
    }
}

/// Budget allocation for a problem.
#[derive(Debug, Clone)]
pub struct Budget {
    /// Total allocated credits
    pub total: u64,
    /// Credits used so far
    pub used: u64,
    /// Problem difficulty
    pub difficulty: Difficulty,
    /// Maximum retries allowed
    pub max_retries: u32,
    /// Current retry count
    pub retry_count: u32,
    /// Domains detected in problem
    pub domains: Vec<Domain>,
}

impl Budget {
    /// Domain cost table.
    const DOMAIN_COSTS: &'static [(Domain, u64)] = &[
        (Domain::Algebra, 30),
        (Domain::CalculusDiff, 80),
        (Domain::CalculusInt, 100),
        (Domain::Trigonometry, 50),
        (Domain::NumberTheory, 60),
        (Domain::Combinatorics, 70),
        (Domain::Inequalities, 40),
        (Domain::Equations, 35),
        (Domain::Vector, 55),
    ];

    /// Create a budget from a problem profile.
    pub fn from_profile(profile: &ProblemProfile) -> Self {
        let domains = Self::detect_domains(profile);
        let total = Self::calculate_budget(&domains);
        let difficulty = Self::determine_difficulty(&domains, profile);

        Self {
            total,
            used: 0,
            difficulty,
            max_retries: difficulty.base_retries(),
            retry_count: 0,
            domains,
        }
    }

    /// Detect domains from profile.
    fn detect_domains(profile: &ProblemProfile) -> Vec<Domain> {
        let mut domains = Vec::new();

        if profile.has_trig {
            domains.push(Domain::Trigonometry);
        }
        if profile.has_calculus_diff || profile.has_calculus_int {
            domains.push(Domain::CalculusDiff);
        }
        if profile.has_combinatorics {
            domains.push(Domain::Combinatorics);
        }

        // Default to Algebra if no specific domain detected
        if domains.is_empty() {
            domains.push(Domain::Algebra);
        }

        domains
    }

    /// Calculate total budget from domains.
    fn calculate_budget(domains: &[Domain]) -> u64 {
        let mut total: u64 = 0;

        for domain in domains {
            for (d, cost) in Self::DOMAIN_COSTS {
                if d == domain {
                    total += cost;
                    break;
                }
            }
        }

        // Minimum budget of 50
        total.max(50)
    }

    /// Determine difficulty from domain count and complexity.
    fn determine_difficulty(domains: &[Domain], profile: &ProblemProfile) -> Difficulty {
        let domain_count = domains.len();
        let complexity = profile.complexity;

        if domain_count == 1 && complexity < 10 {
            Difficulty::Easy
        } else if domain_count <= 2 && complexity < 20 {
            Difficulty::Medium
        } else if domain_count <= 3 && complexity < 40 {
            Difficulty::Hard
        } else {
            Difficulty::Olympiad
        }
    }

    /// Spend credits on a rule application.
    pub fn spend(&mut self, cost: u32) -> bool {
        let cost = cost as u64;
        self.used += cost;
        self.used <= self.total
    }

    /// Check if budget is exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.used >= self.total
    }

    /// Get remaining credits.
    pub fn remaining(&self) -> u64 {
        self.total.saturating_sub(self.used)
    }

    /// Get overspend amount (0 if under budget).
    pub fn overspend(&self) -> u64 {
        self.used.saturating_sub(self.total)
    }

    /// Check if more retries are available.
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    /// Increment retry count.
    pub fn record_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Calculate penalty based on overspend and difficulty.
    pub fn calculate_penalty(&self, overspends: &[u64]) -> u64 {
        if overspends.is_empty() {
            return 0;
        }

        let avg_overspend: u64 = overspends.iter().sum::<u64>() / overspends.len() as u64;
        (avg_overspend as f64 * self.difficulty.penalty_factor()) as u64
    }
}
