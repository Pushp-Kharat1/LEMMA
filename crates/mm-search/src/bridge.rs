// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Bridge detection for bidirectional proof search
//!
//! Detects when forward and backward search trees meet,
//! indicating a complete proof path has been found.

use mm_core::Expr;
use std::collections::HashSet;

/// A connection point between forward and backward search
#[derive(Debug, Clone)]
pub struct Bridge {
    /// The expression where forward and backward meet
    pub meeting_point: Expr,

    /// Expressions reached from forward search
    pub forward_expressions: Vec<Expr>,

    /// Expressions reached from backward search
    pub backward_expressions: Vec<Expr>,
}

/// Detects connections between forward and backward search trees
pub struct BridgeFinder {
    /// Expressions reached from hypotheses (forward)
    forward_reached: HashSet<String>,

    /// Expressions reached from goals (backward)
    backward_reached: HashSet<String>,
}

impl BridgeFinder {
    /// Create a new bridge finder
    pub fn new() -> Self {
        BridgeFinder {
            forward_reached: HashSet::new(),
            backward_reached: HashSet::new(),
        }
    }

    /// Add an expression reached from forward search
    pub fn add_forward(&mut self, expr: &Expr) {
        let key = Self::expr_key(expr);
        self.forward_reached.insert(key);
    }

    /// Add an expression reached from backward search
    pub fn add_backward(&mut self, expr: &Expr) {
        let key = Self::expr_key(expr);
        self.backward_reached.insert(key);
    }

    /// Check if a bridge exists (forward and backward have met)
    pub fn has_bridge(&self) -> bool {
        !self.forward_reached.is_disjoint(&self.backward_reached)
    }

    /// Find all bridge points
    pub fn find_bridges(&self) -> Vec<String> {
        self.forward_reached
            .intersection(&self.backward_reached)
            .cloned()
            .collect()
    }

    /// Create a canonical string key for an expression
    ///
    /// This is used for equality checking.
    /// Two expressions are considered equal if they have the same canonical form.
    fn expr_key(expr: &Expr) -> String {
        // For now, use Debug representation
        // TODO: Proper canonical form (modulo commutativity/associativity)
        format!("{:?}", expr)
    }

    /// Check if two expressions are equivalent
    ///
    /// This is more sophisticated than syntactic equality - it should
    /// handle commutativity, associativity, etc.
    pub fn are_equivalent(e1: &Expr, e2: &Expr) -> bool {
        // Simple syntactic check for now
        Self::expr_key(e1) == Self::expr_key(e2)
    }
}

impl Default for BridgeFinder {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of bridge detection
#[derive(Debug)]
pub enum BridgeResult {
    /// No bridge found yet
    NotFound,

    /// Bridge found at this expression
    Found {
        /// The expression where forward and backward met
        meeting_point: String,

        /// Number of forward expressions considered
        forward_count: usize,

        /// Number of backward expressions considered
        backward_count: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::{Rational, SymbolTable};

    #[test]
    fn test_bridge_detection() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        let mut finder = BridgeFinder::new();

        // Forward reaches: x + y
        let expr1 = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
        finder.add_forward(&expr1);

        // Backward hasn't reached it yet
        assert!(!finder.has_bridge());

        // Backward reaches: x + y (same expression!)
        finder.add_backward(&expr1);

        // Now there's a bridge!
        assert!(finder.has_bridge());

        let bridges = finder.find_bridges();
        assert_eq!(bridges.len(), 1);
    }

    #[test]
    fn test_no_bridge() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        let mut finder = BridgeFinder::new();

        // Forward reaches: x + y
        let expr1 = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
        finder.add_forward(&expr1);

        // Backward reaches: x * y (different!)
        let expr2 = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
        finder.add_backward(&expr2);

        // No bridge
        assert!(!finder.has_bridge());
    }

    #[test]
    fn test_multiple_bridges() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let mut finder = BridgeFinder::new();

        // Forward reaches: x, x²
        let expr1 = Expr::Var(x);
        let expr2 = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));

        finder.add_forward(&expr1);
        finder.add_forward(&expr2);

        // Backward reaches: x, x²
        finder.add_backward(&expr1);
        finder.add_backward(&expr2);

        // Two bridges!
        assert!(finder.has_bridge());
        let bridges = finder.find_bridges();
        assert_eq!(bridges.len(), 2);
    }
}
