// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Monte Carlo Tree Search with neural network guidance.
//!
//! Implements AlphaZero-style MCTS where:
//! - Policy network provides action priors
//! - Value network evaluates leaf nodes
//! - UCB formula balances exploration/exploitation

use crate::{SearchConfig, Solution, Step};
use mm_brain::PolicyNetwork;
use mm_core::{Expr, Rational};
use mm_rules::{RuleCategory, RuleContext, RuleId, RuleSet};
use mm_verifier::Verifier;
use std::collections::HashMap;

/// Compute GCD using Euclidean algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute factorial.
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// A node in the MCTS tree.
pub struct MCTSNode {
    /// The expression at this node.
    pub state: Expr,
    /// Number of times this node has been visited.
    pub visits: u32,
    /// Sum of values from rollouts through this node.
    pub value_sum: f64,
    /// Prior probability from neural network.
    pub prior: f64,
    /// Rule that led to this state (None for root).
    pub rule_id: Option<RuleId>,
    /// Rule name for step recording.
    pub rule_name: Option<&'static str>,
    /// Child nodes indexed by rule ID.
    pub children: HashMap<u32, Box<MCTSNode>>,
    /// Whether this node has been expanded.
    pub expanded: bool,
}

impl MCTSNode {
    /// Create a new MCTS node.
    pub fn new(state: Expr, prior: f64) -> Self {
        Self {
            state,
            visits: 0,
            value_sum: 0.0,
            prior,
            rule_id: None,
            rule_name: None,
            children: HashMap::new(),
            expanded: false,
        }
    }

    /// Create a node with rule information.
    pub fn with_rule(state: Expr, prior: f64, rule_id: RuleId, rule_name: &'static str) -> Self {
        Self {
            state,
            visits: 0,
            value_sum: 0.0,
            prior,
            rule_id: Some(rule_id),
            rule_name: Some(rule_name),
            children: HashMap::new(),
            expanded: false,
        }
    }

    /// Get the average value of this node.
    pub fn value(&self) -> f64 {
        if self.visits == 0 {
            0.0
        } else {
            self.value_sum / self.visits as f64
        }
    }

    /// Calculate UCB score for selection (PUCT formula from AlphaZero).
    pub fn ucb_score(&self, parent_visits: u32, exploration_weight: f64) -> f64 {
        if self.visits == 0 {
            // Prefer unexplored nodes with high prior
            exploration_weight * self.prior * (parent_visits as f64).sqrt()
        } else {
            // Q + U formula
            self.value()
                + exploration_weight
                    * self.prior
                    * ((parent_visits as f64).sqrt() / (1.0 + self.visits as f64))
        }
    }
}

/// Neural-guided Monte Carlo Tree Search solver.
pub struct NeuralMCTS {
    rules: RuleSet,
    verifier: Verifier,
    policy: PolicyNetwork,
    config: MCTSConfig,
}

/// MCTS configuration.
#[derive(Debug, Clone)]
pub struct MCTSConfig {
    /// Number of MCTS simulations per search.
    pub simulations: usize,
    /// Exploration weight (c_puct in AlphaZero).
    pub exploration_weight: f64,
    /// Maximum search depth.
    pub max_depth: usize,
    /// Temperature for action selection (higher = more exploration).
    pub temperature: f64,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            simulations: 100,
            exploration_weight: 1.41,
            max_depth: 20,
            temperature: 1.0,
        }
    }
}

impl NeuralMCTS {
    /// Create a new neural MCTS solver.
    pub fn new(rules: RuleSet, verifier: Verifier) -> Self {
        let policy = PolicyNetwork::new().expect("Failed to create policy network");
        Self {
            rules,
            verifier,
            policy,
            config: MCTSConfig::default(),
        }
    }

    /// Create with custom configuration.
    pub fn with_config(rules: RuleSet, verifier: Verifier, config: MCTSConfig) -> Self {
        let policy = PolicyNetwork::new().expect("Failed to create policy network");
        Self {
            rules,
            verifier,
            policy,
            config,
        }
    }

    /// Set a custom policy network (e.g., trained).
    pub fn with_policy(mut self, policy: PolicyNetwork) -> Self {
        self.policy = policy;
        self
    }

    /// Search for a solution using neural MCTS.
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

        // Initialize root node
        let mut root = MCTSNode::new(start.clone(), 1.0);

        // Run MCTS simulations
        for _ in 0..self.config.simulations {
            self.simulate(&mut root, &goal, 0);
        }

        // Extract best path
        self.extract_solution(&root, &start, &goal)
    }

    /// Run one MCTS simulation (SELECT, EXPAND, EVALUATE, BACKUP).
    fn simulate<F>(&self, node: &mut MCTSNode, goal: &F, depth: usize) -> f64
    where
        F: Fn(&Expr) -> bool,
    {
        // Check terminal conditions
        if goal(&node.state) {
            return 1.0; // Goal reached
        }

        if depth >= self.config.max_depth {
            return self.evaluate(&node.state);
        }

        // EXPAND if not yet expanded
        if !node.expanded {
            self.expand(node);
            node.expanded = true;

            // Evaluate this node
            let value = self.evaluate(&node.state);
            node.visits += 1;
            node.value_sum += value;
            return value;
        }

        // SELECT best child using UCB
        if node.children.is_empty() {
            // No valid moves - terminal state
            return self.evaluate(&node.state);
        }

        let best_child_id = self.select_child(node);

        if let Some(child) = node.children.get_mut(&best_child_id) {
            // RECURSE
            let value = self.simulate(child, goal, depth + 1);

            // BACKUP
            node.visits += 1;
            node.value_sum += value;

            value
        } else {
            0.0
        }
    }

    /// Expand a node by adding children for all valid actions.
    /// Uses the guardrail to filter rules by domain/features before expansion.
    fn expand(&self, node: &mut MCTSNode) {
        let ctx = RuleContext::default();

        // GUARDRAIL: Analyze the problem to get domains/features
        let profile = mm_rules::analyze(&node.state);

        // GUARDRAIL: Filter rules by domain and features BEFORE NN scoring
        let valid_rules = mm_rules::filter_rules(self.rules.all(), &profile, &node.state, &ctx);

        // Get policy priors from neural network (for all rules)
        let priors = self
            .policy
            .forward(&node.state)
            .unwrap_or_else(|_| vec![1.0 / self.rules.len() as f32; self.rules.len()]);

        // Expand only using guardrail-filtered rules
        for rule in valid_rules {
            let applications = rule.apply(&node.state, &ctx);

            for app in applications {
                // Verify the transformation
                let verify_result = self
                    .verifier
                    .verify_step(&node.state, &app.result, rule, &ctx);

                if verify_result.is_valid() {
                    let prior = priors.get(rule.id.0 as usize).copied().unwrap_or(0.01);
                    let child = MCTSNode::with_rule(app.result, prior as f64, rule.id, rule.name);
                    node.children.insert(rule.id.0, Box::new(child));
                }
            }
        }
    }

    /// Select the best child using UCB.
    fn select_child(&self, node: &MCTSNode) -> u32 {
        let mut best_score = f64::NEG_INFINITY;
        let mut best_id = 0;

        for (&id, child) in &node.children {
            let score = child.ucb_score(node.visits, self.config.exploration_weight);
            if score > best_score {
                best_score = score;
                best_id = id;
            }
        }

        best_id
    }

    /// Evaluate a state using the value network.
    fn evaluate(&self, state: &Expr) -> f64 {
        self.policy.get_value(state).unwrap_or(0.0) as f64
    }

    /// Extract the best solution path from the tree.
    fn extract_solution<F>(&self, root: &MCTSNode, start: &Expr, goal: &F) -> Option<Solution>
    where
        F: Fn(&Expr) -> bool,
    {
        let mut steps = Vec::new();
        let mut current = root;
        let mut prev_state = start.clone();

        // Follow the most-visited path
        while !current.children.is_empty() {
            // Find most-visited child
            let (best_id, best_child) = current
                .children
                .iter()
                .max_by_key(|(_, child)| child.visits)?;

            // Record step
            if let (Some(rule_id), Some(rule_name)) = (best_child.rule_id, best_child.rule_name) {
                steps.push(Step {
                    before: prev_state.clone(),
                    after: best_child.state.clone(),
                    rule_id,
                    rule_name,
                    justification: format!("Applied {} (visits: {})", rule_name, best_child.visits),
                });
            }

            prev_state = best_child.state.clone();

            // Check if goal reached
            if goal(&best_child.state) {
                return Some(Solution {
                    problem: start.clone(),
                    result: best_child.state.clone(),
                    steps,
                    verified: true,
                });
            }

            current = best_child;
        }

        // If we have steps but didn't reach goal, still return partial result
        if !steps.is_empty() {
            Some(Solution {
                problem: start.clone(),
                result: prev_state,
                steps,
                verified: false,
            })
        } else {
            None
        }
    }

    /// Simplify an expression using neural MCTS with multi-step chaining.
    /// Continues applying rules until no more simplifications are possible.
    pub fn simplify(&self, expr: Expr) -> Solution {
        const MAX_ITERATIONS: usize = 50;

        let mut current = expr.clone();
        let mut all_steps: Vec<Step> = Vec::new();
        let ctx = RuleContext::default();

        // Track seen expressions to prevent infinite loops (e.g., distribute <-> factor_common)
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        seen.insert(format!("{:?}", current));

        // GUARDRAIL: Analyze the problem once to get domains/features
        let profile = mm_rules::analyze(&expr);

        for _iteration in 0..MAX_ITERATIONS {
            // GUARDRAIL: Filter rules by domain and features
            let applicable = mm_rules::filter_rules(self.rules.all(), &profile, &current, &ctx);
            if applicable.is_empty() {
                break; // No more rules - we're done
            }

            // Define goal for this iteration: any simplification
            let current_complexity = current.complexity();
            let goal = |e: &Expr| {
                let ctx = RuleContext::default();
                let applicable = self.rules.applicable(e, &ctx);

                // For equations: if LHS is just a variable, we're done!
                if let Expr::Equation { lhs, .. } = e {
                    if matches!(lhs.as_ref(), Expr::Var(_)) {
                        return true;
                    }
                }

                // Stop when simpler OR no rules apply OR different structure (progress)
                applicable.is_empty() || e.complexity() < current_complexity || e != &current
            };

            // Run MCTS to find best next step
            if let Some(solution) = self.search(current.clone(), goal) {
                if solution.steps.is_empty() {
                    // MCTS didn't find path - try direct rule application as fallback
                    let mut found_rule = false;
                    for rule in &applicable {
                        let applications = rule.apply(&current, &ctx);
                        if let Some(app) = applications.first() {
                            // Skip if this leads to a seen state (prevents infinite loops)
                            let result_key = format!("{:?}", app.result);
                            if seen.contains(&result_key) {
                                continue;
                            }
                            seen.insert(result_key);

                            all_steps.push(Step {
                                before: current.clone(),
                                after: app.result.clone(),
                                rule_id: rule.id,
                                rule_name: rule.name,
                                justification: app.justification.clone(),
                            });
                            current = app.result.clone();
                            found_rule = true;
                            break;
                        }
                    }
                    if !found_rule {
                        break;
                    }
                    continue; // Try another iteration with new expression
                }

                // Collect steps from this iteration
                all_steps.extend(solution.steps);

                // Check if result is in seen set (loop detection)
                let result_key = format!("{:?}", solution.result);
                if seen.contains(&result_key) {
                    break; // Would loop - stop here
                }
                seen.insert(result_key);

                // Update current expression
                if solution.result == current {
                    break; // No progress made
                }
                current = solution.result;
            } else {
                // search() returned None - try direct fallback
                let mut found_rule = false;
                for rule in &applicable {
                    let applications = rule.apply(&current, &ctx);
                    if let Some(app) = applications.first() {
                        // Skip if this leads to a seen state
                        let result_key = format!("{:?}", app.result);
                        if seen.contains(&result_key) {
                            continue;
                        }
                        seen.insert(result_key);

                        all_steps.push(Step {
                            before: current.clone(),
                            after: app.result.clone(),
                            rule_id: rule.id,
                            rule_name: rule.name,
                            justification: app.justification.clone(),
                        });
                        current = app.result.clone();
                        found_rule = true;
                        break;
                    }
                }
                if !found_rule {
                    break;
                }
            }
        }

        // Recursively simplify sub-expressions (for nested derivatives, etc.)
        let mut simplified = self.simplify_subexpressions(&current);

        // Keep applying SIMPLIFICATION rules until stable (handles chained patterns like x^2 * x^3 * x^4)
        // Skip expansion rules (distribute) to avoid undoing collect_like_terms
        let ctx = RuleContext::default();
        for _ in 0..10 {
            let applicable = self.rules.applicable(&simplified, &ctx);
            if applicable.is_empty() {
                break;
            }
            // Only apply simplification rules, skip expansion (distribute)
            let simplification_rules: Vec<_> = applicable
                .iter()
                .filter(|r| r.category != RuleCategory::Expansion)
                .collect();

            if let Some(rule) = simplification_rules.first() {
                let results = (rule.apply)(&simplified, &ctx);
                if let Some(app) = results.first() {
                    // Only apply if result is simpler or same complexity
                    if app.result.complexity() <= simplified.complexity() {
                        simplified = app.result.clone();
                        continue;
                    }
                }
            }
            break;
        }

        // Apply constant folding to final result if possible
        let final_result = self.try_const_fold(&simplified);

        Solution {
            problem: expr,
            result: final_result,
            steps: all_steps,
            verified: true,
        }
    }

    /// Progressive solve: pattern match → decompose → solve easiest first → recombine.
    ///
    /// This is the main entry point for solving problems like integrals of sums.
    pub fn progressive_solve(&self, expr: Expr) -> Solution {
        // 1. Try pattern match first (fast path for known forms)
        if let Some(result) = mm_rules::match_integral_pattern(&expr) {
            return Solution {
                problem: expr.clone(),
                result,
                steps: vec![Step {
                    before: expr.clone(),
                    after: expr.clone(),
                    rule_id: RuleId(0),
                    rule_name: "pattern_match",
                    justification: "Matched known integral pattern".to_string(),
                }],
                verified: true,
            };
        }

        // 2. Check if this is an integral of a sum - decompose and solve each term
        if let Expr::Integral { expr: inner, var } = &expr {
            if matches!(inner.as_ref(), Expr::Add(_, _) | Expr::Sub(_, _)) {
                // Decompose into individual terms
                let terms = mm_rules::decompose_additive(inner);

                if terms.len() > 1 {
                    // Score and sort by solvability (easiest first)
                    let mut scored: Vec<_> = terms
                        .iter()
                        .map(|t| (t.clone(), mm_rules::solvability_score(t)))
                        .collect();
                    scored.sort_by(|a, b| b.1.cmp(&a.1));

                    // Solve each term as a separate integral
                    let mut all_steps = Vec::new();
                    let mut partial_results = Vec::new();

                    for (term, _score) in &scored {
                        let term_integral = Expr::Integral {
                            expr: Box::new(term.clone()),
                            var: *var,
                        };

                        // Try pattern match on this term
                        if let Some(result) = mm_rules::match_integral_pattern(&term_integral) {
                            all_steps.push(Step {
                                before: term_integral.clone(),
                                after: result.clone(),
                                rule_id: RuleId(0),
                                rule_name: "pattern_match",
                                justification: format!("Pattern matched: ∫{:?}", term),
                            });
                            partial_results.push(result);
                        } else {
                            // Fall back to regular simplify
                            let term_solution = self.simplify(term_integral);
                            all_steps.extend(term_solution.steps);
                            partial_results.push(term_solution.result);
                        }
                    }

                    // Recombine results by addition
                    let combined = partial_results
                        .into_iter()
                        .reduce(|acc, r| Expr::Add(Box::new(acc), Box::new(r)))
                        .unwrap_or(Expr::int(0));

                    return Solution {
                        problem: expr,
                        result: combined,
                        steps: all_steps,
                        verified: true,
                    };
                }
            }
        }

        // 3. Fall back to regular simplify for non-decomposable expressions
        self.simplify(expr)
    }

    /// Recursively simplify sub-expressions by applying rules to inner parts.
    fn simplify_subexpressions(&self, expr: &Expr) -> Expr {
        let ctx = RuleContext::default();

        match expr {
            // For derivatives inside Add/Sub/Mul, simplify each side
            Expr::Add(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Add(Box::new(a_simp), Box::new(b_simp))
            }
            Expr::Sub(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Sub(Box::new(a_simp), Box::new(b_simp))
            }
            Expr::Mul(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Mul(Box::new(a_simp), Box::new(b_simp))
            }
            _ => expr.clone(),
        }
    }

    /// Apply first applicable rule to an expression (single step).
    fn simplify_single_step(&self, expr: &Expr) -> Expr {
        let ctx = RuleContext::default();

        // First recursively handle sub-expressions
        let processed = match expr {
            Expr::Add(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Add(Box::new(a_simp), Box::new(b_simp))
            }
            Expr::Sub(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Sub(Box::new(a_simp), Box::new(b_simp))
            }
            Expr::Mul(a, b) => {
                let a_simp = self.simplify_single_step(a);
                let b_simp = self.simplify_single_step(b);
                Expr::Mul(Box::new(a_simp), Box::new(b_simp))
            }
            Expr::Derivative {
                expr: inner,
                var: _,
            } => {
                // Apply derivative rules if possible
                let applicable = self.rules.applicable(expr, &ctx);
                if let Some(rule) = applicable.first() {
                    let results = (rule.apply)(expr, &ctx);
                    if let Some(app) = results.first() {
                        return self.simplify_single_step(&app.result);
                    }
                }
                expr.clone()
            }
            _ => expr.clone(),
        };

        // Now try to simplify the processed expression
        let applicable = self.rules.applicable(&processed, &ctx);
        if let Some(rule) = applicable.first() {
            let results = (rule.apply)(&processed, &ctx);
            if let Some(app) = results.first() {
                return app.result.clone();
            }
        }

        processed
    }

    /// Try to constant fold an expression if all parts are constants.
    fn try_const_fold(&self, expr: &Expr) -> Expr {
        // Recursively try to fold sub-expressions
        match expr {
            Expr::Add(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                // Constant fold
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    return Expr::Const(*ra + *rb);
                }
                // x + 0 = x
                if let Expr::Const(r) = &b_folded {
                    if r.is_zero() {
                        return a_folded;
                    }
                }
                if let Expr::Const(r) = &a_folded {
                    if r.is_zero() {
                        return b_folded;
                    }
                }
                Expr::Add(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Sub(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    return Expr::Const(*ra - *rb);
                }
                // x - 0 = x
                if let Expr::Const(r) = &b_folded {
                    if r.is_zero() {
                        return a_folded;
                    }
                }
                Expr::Sub(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Mul(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    return Expr::Const(*ra * *rb);
                }
                // x * 1 = x
                if let Expr::Const(r) = &b_folded {
                    if r.numer() == 1 && r.denom() == 1 {
                        return a_folded;
                    }
                    if r.is_zero() {
                        return Expr::int(0);
                    }
                }
                if let Expr::Const(r) = &a_folded {
                    if r.numer() == 1 && r.denom() == 1 {
                        return b_folded;
                    }
                    if r.is_zero() {
                        return Expr::int(0);
                    }
                }
                Expr::Mul(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Div(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    if !rb.is_zero() {
                        return Expr::Const(*ra / *rb);
                    }
                }
                Expr::Div(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Pow(base, exp) => {
                let base_folded = self.try_const_fold(base);
                let exp_folded = self.try_const_fold(exp);
                // x^1 = x
                if let Expr::Const(r) = &exp_folded {
                    if r.numer() == 1 && r.denom() == 1 {
                        return base_folded;
                    }
                    // x^0 = 1
                    if r.is_zero() {
                        return Expr::int(1);
                    }
                }
                // Constant folding: a^n when both are constants and n is small integer
                if let (Expr::Const(base_r), Expr::Const(exp_r)) = (&base_folded, &exp_folded) {
                    if exp_r.denom() == 1 && exp_r.numer() >= 0 && exp_r.numer() <= 30 {
                        let n = exp_r.numer() as u32;
                        // Compute base^n using repeated multiplication
                        let mut result = Rational::from_integer(1);
                        for _ in 0..n {
                            result = result * *base_r;
                        }
                        return Expr::Const(result);
                    }
                }
                Expr::Pow(Box::new(base_folded), Box::new(exp_folded))
            }
            Expr::Neg(inner) => {
                let folded = self.try_const_fold(inner);
                if let Expr::Const(r) = &folded {
                    return Expr::Const(-*r);
                }
                Expr::Neg(Box::new(folded))
            }
            Expr::Equation { lhs, rhs } => {
                // Fold both sides of equation
                let lhs_folded = self.try_const_fold(lhs);
                let rhs_folded = self.try_const_fold(rhs);
                Expr::Equation {
                    lhs: Box::new(lhs_folded),
                    rhs: Box::new(rhs_folded),
                }
            }

            // Phase 1: Add constant folding for number theory operations
            Expr::GCD(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    let a_val = ra.numer().abs();
                    let b_val = rb.numer().abs();
                    if ra.denom() == 1 && rb.denom() == 1 {
                        return Expr::Const(Rational::from_integer(gcd(a_val, b_val)));
                    }
                }
                Expr::GCD(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::LCM(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    let a_val = ra.numer().abs();
                    let b_val = rb.numer().abs();
                    if ra.denom() == 1 && rb.denom() == 1 && a_val > 0 && b_val > 0 {
                        let g = gcd(a_val, b_val);
                        return Expr::Const(Rational::from_integer(a_val / g * b_val));
                    }
                }
                Expr::LCM(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Mod(a, b) => {
                let a_folded = self.try_const_fold(a);
                let b_folded = self.try_const_fold(b);
                if let (Expr::Const(ra), Expr::Const(rb)) = (&a_folded, &b_folded) {
                    if ra.denom() == 1 && rb.denom() == 1 && rb.numer() != 0 {
                        let a_val = ra.numer();
                        let b_val = rb.numer();
                        let result = ((a_val % b_val) + b_val) % b_val; // Ensure positive
                        return Expr::Const(Rational::from_integer(result));
                    }
                }
                Expr::Mod(Box::new(a_folded), Box::new(b_folded))
            }
            Expr::Factorial(n) => {
                let n_folded = self.try_const_fold(n);
                if let Expr::Const(r) = &n_folded {
                    if r.denom() == 1 && r.numer() >= 0 && r.numer() <= 20 {
                        let n_val = r.numer() as u64;
                        let result = factorial(n_val);
                        return Expr::Const(Rational::from_integer(result as i64));
                    }
                }
                Expr::Factorial(Box::new(n_folded))
            }
            Expr::Binomial(n, k) => {
                let n_folded = self.try_const_fold(n);
                let k_folded = self.try_const_fold(k);
                if let (Expr::Const(rn), Expr::Const(rk)) = (&n_folded, &k_folded) {
                    if rn.denom() == 1 && rk.denom() == 1 {
                        let n_val = rn.numer();
                        let k_val = rk.numer();
                        if n_val >= 0 && k_val >= 0 && k_val <= n_val && n_val <= 20 {
                            let n_u = n_val as u64;
                            let k_u = k_val as u64;
                            let result = factorial(n_u) / (factorial(k_u) * factorial(n_u - k_u));
                            return Expr::Const(Rational::from_integer(result as i64));
                        }
                    }
                }
                Expr::Binomial(Box::new(n_folded), Box::new(k_folded))
            }
            Expr::Floor(e) => {
                let folded = self.try_const_fold(e);
                if let Expr::Const(r) = &folded {
                    // Floor of a rational: numer / denom (integer division towards -∞)
                    let n = r.numer();
                    let d = r.denom();
                    let result = if n >= 0 { n / d } else { (n - d + 1) / d };
                    return Expr::Const(Rational::from_integer(result));
                }
                Expr::Floor(Box::new(folded))
            }
            Expr::Ceiling(e) => {
                let folded = self.try_const_fold(e);
                if let Expr::Const(r) = &folded {
                    let n = r.numer();
                    let d = r.denom();
                    let result = if n >= 0 { (n + d - 1) / d } else { n / d };
                    return Expr::Const(Rational::from_integer(result));
                }
                Expr::Ceiling(Box::new(folded))
            }
            Expr::Sqrt(e) => {
                let folded = self.try_const_fold(e);
                // Check for perfect squares
                if let Expr::Const(r) = &folded {
                    if r.denom() == 1 && r.numer() >= 0 {
                        let n = r.numer() as u64;
                        let sqrt_n = (n as f64).sqrt() as u64;
                        if sqrt_n * sqrt_n == n {
                            return Expr::Const(Rational::from_integer(sqrt_n as i64));
                        }
                    }
                }
                Expr::Sqrt(Box::new(folded))
            }
            Expr::Abs(e) => {
                let folded = self.try_const_fold(e);
                if let Expr::Const(r) = &folded {
                    return Expr::Const(r.abs());
                }
                Expr::Abs(Box::new(folded))
            }

            _ => expr.clone(),
        }
    }
}

// Keep the old MCTS struct for backwards compatibility
pub struct MCTS {
    _rules: RuleSet,
    _verifier: Verifier,
    _config: SearchConfig,
}

impl MCTS {
    pub fn new(rules: RuleSet, verifier: Verifier) -> Self {
        Self {
            _rules: rules,
            _verifier: verifier,
            _config: SearchConfig::default(),
        }
    }

    pub fn search<F>(&self, start: Expr, goal: F) -> Option<Solution>
    where
        F: Fn(&Expr) -> bool,
    {
        // Delegate to NeuralMCTS
        let neural = NeuralMCTS::new(
            mm_rules::rule::standard_rules(),
            mm_verifier::Verifier::new(),
        );
        neural.search(start, goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_rules::rule::standard_rules;

    #[test]
    fn test_mcts_node_creation() {
        let node = MCTSNode::new(Expr::int(0), 0.5);
        assert_eq!(node.visits, 0);
        assert_eq!(node.value(), 0.0);
    }

    #[test]
    fn test_ucb_score() {
        let mut node = MCTSNode::new(Expr::int(0), 0.5);
        node.visits = 10;
        node.value_sum = 5.0;

        let score = node.ucb_score(100, 1.41);
        assert!(score > 0.0);
    }

    #[test]
    fn test_neural_mcts_creation() {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let _mcts = NeuralMCTS::new(rules, verifier);
    }

    #[test]
    fn test_neural_mcts_simplify() {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let mcts = NeuralMCTS::with_config(
            rules,
            verifier,
            MCTSConfig {
                simulations: 10, // Reduced for tests
                ..Default::default()
            },
        );

        // Test simple constant folding
        let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        let result = mcts.simplify(expr);

        // Should simplify to 5
        assert_eq!(result.result.canonicalize(), Expr::int(5));
    }
}
