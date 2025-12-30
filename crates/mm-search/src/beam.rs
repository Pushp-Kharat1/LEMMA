// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Beam search algorithm for finding solution paths.

use crate::{SearchConfig, Solution, Step};
use mm_core::Expr;
use mm_rules::{RuleContext, RuleSet};
use mm_verifier::Verifier;
use std::collections::HashSet;

/// Beam search solver.
pub struct BeamSearch {
    rules: RuleSet,
    verifier: Verifier,
    config: SearchConfig,
}

/// A candidate state in beam search.
#[derive(Clone)]
struct Candidate {
    expr: Expr,
    steps: Vec<Step>,
    score: f64,
}

impl BeamSearch {
    /// Create a new beam search solver.
    pub fn new(rules: RuleSet, verifier: Verifier) -> Self {
        Self {
            rules,
            verifier,
            config: SearchConfig::default(),
        }
    }

    /// Create with custom configuration.
    pub fn with_config(rules: RuleSet, verifier: Verifier, config: SearchConfig) -> Self {
        Self {
            rules,
            verifier,
            config,
        }
    }

    /// Search for a solution that satisfies the goal predicate.
    pub fn search<F>(&self, start: Expr, goal: F) -> Option<Solution>
    where
        F: Fn(&Expr) -> bool,
    {
        // Check if already at goal
        if goal(&start) {
            return Some(Solution {
                problem: start.clone(),
                result: start,
                steps: vec![],
                verified: true,
            });
        }

        // Initialize beam with starting state
        let mut beam = vec![Candidate {
            expr: start.clone(),
            steps: vec![],
            score: 0.0,
        }];

        // Track visited states to avoid cycles
        let mut visited: HashSet<Expr> = HashSet::new();
        visited.insert(start.canonicalize());

        let ctx = RuleContext::default();

        // Search
        for _depth in 0..self.config.max_depth {
            let mut candidates = Vec::new();

            for candidate in &beam {
                // Check if this candidate reaches the goal
                if goal(&candidate.expr) {
                    return Some(Solution {
                        problem: start.clone(),
                        result: candidate.expr.clone(),
                        steps: candidate.steps.clone(),
                        verified: true,
                    });
                }

                // Find applicable rules
                let applicable = self.rules.applicable(&candidate.expr, &ctx);

                for rule in applicable {
                    let applications = rule.apply(&candidate.expr, &ctx);

                    for app in applications {
                        let canonical = app.result.canonicalize();

                        // Skip if already visited
                        if visited.contains(&canonical) {
                            continue;
                        }

                        // Verify the step
                        let verify_result =
                            self.verifier
                                .verify_step(&candidate.expr, &app.result, rule, &ctx);

                        if !verify_result.is_valid() {
                            continue;
                        }

                        // Create new step
                        let step = Step {
                            before: candidate.expr.clone(),
                            after: app.result.clone(),
                            rule_id: rule.id,
                            rule_name: rule.name,
                            justification: app.justification,
                        };

                        // Create new candidate
                        let mut new_steps = candidate.steps.clone();
                        new_steps.push(step);

                        let new_candidate = Candidate {
                            expr: app.result.clone(),
                            steps: new_steps,
                            score: self.score_expr(&app.result),
                        };

                        candidates.push(new_candidate);
                        visited.insert(canonical);
                    }
                }
            }

            if candidates.is_empty() {
                // No more moves possible
                break;
            }

            // Sort by score (lower is better - we want simpler expressions)
            candidates.sort_by(|a, b| {
                a.score
                    .partial_cmp(&b.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            // Keep top beam_width candidates
            beam = candidates
                .into_iter()
                .take(self.config.beam_width)
                .collect();

            // Check if any candidate reaches goal
            for candidate in &beam {
                if goal(&candidate.expr) {
                    return Some(Solution {
                        problem: start.clone(),
                        result: candidate.expr.clone(),
                        steps: candidate.steps.clone(),
                        verified: true,
                    });
                }
            }
        }

        // Search exhausted
        None
    }

    /// Simplify an expression by repeatedly applying simplification rules.
    ///
    /// Returns the simplest form found.
    pub fn simplify(&self, expr: Expr) -> Solution {
        // First, canonicalize to apply basic simplifications
        let canonical = expr.canonicalize();

        // If canonicalization already simplified it, we're done
        if canonical != expr {
            // Create a solution showing the simplification
            return Solution {
                problem: expr,
                result: canonical,
                steps: vec![], // Canonicalization is atomic - could expand later
                verified: true,
            };
        }

        // Otherwise, try to find a simplification path using rules
        let goal = |e: &Expr| {
            // Goal: expression is in simplest form (no applicable simplification rules)
            // OR it's simpler than what we started with
            let ctx = RuleContext::default();
            let applicable = self.rules.applicable(e, &ctx);
            let has_simplification = applicable
                .iter()
                .any(|r| r.category == mm_rules::RuleCategory::Simplification);

            !has_simplification || e.complexity() < expr.complexity()
        };

        // Try beam search
        if let Some(solution) = self.search(expr.clone(), goal) {
            // Return the best result, canonicalized
            return Solution {
                problem: solution.problem,
                result: solution.result.canonicalize(),
                steps: solution.steps,
                verified: solution.verified,
            };
        }

        // No simplification found, return canonical form
        Solution {
            problem: expr.clone(),
            result: canonical,
            steps: vec![],
            verified: true,
        }
    }

    /// Score an expression (lower is better).
    ///
    /// We prefer simpler expressions.
    fn score_expr(&self, expr: &Expr) -> f64 {
        expr.complexity() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_rules::rule::standard_rules;

    #[test]
    fn test_beam_search_creation() {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let _searcher = BeamSearch::new(rules, verifier);
    }

    #[test]
    fn test_simplify_constant() {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let searcher = BeamSearch::new(rules, verifier);

        // 2 + 3 should simplify to 5
        let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        let solution = searcher.simplify(expr);

        assert_eq!(solution.result.canonicalize(), Expr::int(5));
    }
}
