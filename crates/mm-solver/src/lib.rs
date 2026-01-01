// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-solver
//!
//! The unified API for the LEMMA system.
//!
//! This crate combines all components into a single, easy-to-use interface
//! for solving mathematical problems with step-by-step reasoning.
//!
//! ## Example
//!
//! ```rust
//! use mm_solver::LemmaSolver;
//!
//! let mut solver = LemmaSolver::new();
//!
//! // Simplify an expression
//! let result = solver.simplify("2 + 3").unwrap();
//! println!("Result: {:?}", result.result);  // Const(5)
//!
//! // Compute a derivative
//! let result = solver.differentiate("x^2", "x").unwrap();
//! println!("Derivative: {:?}", result.result);
//! ```

use mm_core::{Expr, MathError, SymbolTable};
use mm_rules::{rule::standard_rules, RuleSet};
use mm_search::{BeamSearch, SearchConfig, Step};
use mm_verifier::{Verifier, VerifyResult};

/// The LEMMA solver.
///
/// This is the main entry point for mathematical reasoning.
pub struct LemmaSolver {
    rules: RuleSet,
    verifier: Verifier,
    search: BeamSearch,
    symbols: SymbolTable,
}

impl Default for LemmaSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl LemmaSolver {
    /// Create a new LEMMA solver with default settings.
    pub fn new() -> Self {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let search = BeamSearch::new(standard_rules(), Verifier::new());
        let symbols = SymbolTable::new();

        Self {
            rules,
            verifier,
            search,
            symbols,
        }
    }

    /// Create with custom configuration.
    pub fn with_config(config: SearchConfig) -> Self {
        let rules = standard_rules();
        let verifier = Verifier::new();
        let search = BeamSearch::with_config(standard_rules(), Verifier::new(), config);
        let symbols = SymbolTable::new();

        Self {
            rules,
            verifier,
            search,
            symbols,
        }
    }

    /// Parse an expression from a string.
    pub fn parse(&mut self, input: &str) -> Result<Expr, MathError> {
        use mm_core::parse::Parser;
        let mut parser = Parser::new(&mut self.symbols);
        parser.parse(input)
    }

    /// Simplify an expression.
    pub fn simplify(&mut self, input: &str) -> Result<SolveResult, MathError> {
        let expr = self.parse(input)?;
        let solution = self.search.simplify(expr);

        Ok(SolveResult {
            result: solution.result,
            steps: solution.steps,
            verified: solution.verified,
        })
    }

    /// Simplify an already-parsed expression.
    pub fn simplify_expr(&self, expr: Expr) -> SolveResult {
        let solution = self.search.simplify(expr);

        SolveResult {
            result: solution.result,
            steps: solution.steps,
            verified: solution.verified,
        }
    }

    /// Compute the derivative of an expression.
    pub fn differentiate(&mut self, input: &str, var: &str) -> Result<SolveResult, MathError> {
        let expr = self.parse(input)?;
        let var_symbol = self.symbols.intern(var);

        // Create derivative expression
        let deriv = Expr::Derivative {
            expr: Box::new(expr),
            var: var_symbol,
        };

        // Simplify to evaluate the derivative
        let solution = self.search.simplify(deriv);

        Ok(SolveResult {
            result: solution.result,
            steps: solution.steps,
            verified: solution.verified,
        })
    }

    /// Solve an equation for a variable.
    ///
    /// Returns all solutions found.
    pub fn solve_for(&mut self, equation: &str, var: &str) -> Result<Vec<SolveResult>, MathError> {
        // Parse the equation
        // For now, we expect "lhs = rhs" format
        let parts: Vec<&str> = equation.split('=').collect();
        if parts.len() != 2 {
            return Err(MathError::ParseError(
                "Expected equation in 'lhs = rhs' format".to_string(),
            ));
        }

        let lhs = self.parse(parts[0].trim())?;
        let rhs = self.parse(parts[1].trim())?;
        let var_symbol = self.symbols.intern(var);

        let eq = Expr::Equation {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        // TODO: Implement equation solving
        // For now, return empty
        Ok(vec![])
    }

    /// Verify that a value is a solution to an equation.
    pub fn verify_solution(
        &mut self,
        equation: &str,
        var: &str,
        value: &str,
    ) -> Result<VerifyResult, MathError> {
        let parts: Vec<&str> = equation.split('=').collect();
        if parts.len() != 2 {
            return Err(MathError::ParseError(
                "Expected equation in 'lhs = rhs' format".to_string(),
            ));
        }

        let lhs = self.parse(parts[0].trim())?;
        let rhs = self.parse(parts[1].trim())?;
        let var_symbol = self.symbols.intern(var);
        let value_expr = self.parse(value)?;

        let eq = Expr::Equation {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        Ok(self.verifier.verify_solution(&eq, var_symbol, &value_expr))
    }

    /// Get a reference to the symbol table.
    pub fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    /// Get a mutable reference to the symbol table.
    pub fn symbols_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbols
    }

    /// Get the number of rules loaded.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }
}

/// Result of solving a problem.
#[derive(Debug, Clone)]
pub struct SolveResult {
    /// The final result expression.
    pub result: Expr,
    /// The steps taken to reach the result.
    pub steps: Vec<Step>,
    /// Whether the result was verified.
    pub verified: bool,
}

impl SolveResult {
    /// Get the number of steps.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }

    /// Check if no steps were needed.
    pub fn is_trivial(&self) -> bool {
        self.steps.is_empty()
    }

    /// Format the solution as a human-readable string.
    pub fn format(&self, symbols: &SymbolTable) -> String {
        let mut output = String::new();

        if self.steps.is_empty() {
            output.push_str(&format!("Result: {:?}\n", self.result));
            output.push_str("(No simplification needed)\n");
        } else {
            for (i, step) in self.steps.iter().enumerate() {
                output.push_str(&format!(
                    "Step {}: {} ({})\n",
                    i + 1,
                    step.rule_name,
                    step.justification
                ));
                output.push_str(&format!("  → {:?}\n", step.after));
            }
            output.push_str(&format!("\nFinal Result: {:?}\n", self.result));
        }

        if self.verified {
            output.push_str("✓ Verified\n");
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = LemmaSolver::new();
        assert!(solver.num_rules() > 0);
    }

    #[test]
    fn test_simplify() {
        let mut solver = LemmaSolver::new();

        let result = solver.simplify("2 + 3").unwrap();
        assert_eq!(result.result.canonicalize(), Expr::int(5));
    }

    #[test]
    fn test_parse() {
        let mut solver = LemmaSolver::new();

        let expr = solver.parse("x + 1").unwrap();
        assert!(matches!(expr, Expr::Add(_, _)));
    }
}
