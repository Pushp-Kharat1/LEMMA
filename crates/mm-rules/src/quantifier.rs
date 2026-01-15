// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Quantifier Reasoning Rules.
//!
//! Implements logical rules for universal and existential quantifiers:
//! - Universal Instantiation: ∀x.P(x) → P(a)
//! - Existential Introduction: P(a) → ∃x.P(x)  
//! - Quantifier Negation: ¬∀x.P(x) ↔ ∃x.¬P(x)

use mm_core::{Expr, Symbol, SymbolTable};

/// Result of instantiating a universally quantified statement.
#[derive(Debug, Clone)]
pub struct InstantiationResult {
    /// The instantiated expression
    pub result: Expr,
    /// The variable that was instantiated
    pub var: Symbol,
    /// The value substituted for the variable
    pub value: Expr,
    /// Human-readable justification
    pub justification: String,
}

/// Quantifier reasoning engine.
pub struct QuantifierEngine {
    /// Fresh variable counter for generating new variables
    fresh_counter: u32,
}

impl Default for QuantifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl QuantifierEngine {
    /// Create a new quantifier engine.
    pub fn new() -> Self {
        Self { fresh_counter: 0 }
    }

    /// Universal Instantiation: ∀x.P(x) → P(a)
    ///
    /// Given a universally quantified statement and a term, substitutes
    /// the term for the bound variable.
    ///
    /// # Example
    /// ```text
    /// ∀n. n² ≥ 0, instantiated with n=5 → 5² ≥ 0
    /// ```
    pub fn universal_instantiate(
        &self,
        forall: &Expr,
        value: &Expr,
    ) -> Option<InstantiationResult> {
        match forall {
            Expr::ForAll { var, body, .. } => {
                let result = self.substitute(body, *var, value);
                Some(InstantiationResult {
                    result,
                    var: *var,
                    value: value.clone(),
                    justification: format!(
                        "Universal instantiation: ∀{:?}.P → P[{:?}/{:?}]",
                        var, value, var
                    ),
                })
            }
            _ => None,
        }
    }

    /// Universal Introduction (backward reasoning):
    /// To prove ∀x.P(x), we need to prove P(x) for an arbitrary x.
    ///
    /// Returns a fresh variable and the goal to prove.
    pub fn universal_introduction(
        &mut self,
        forall: &Expr,
        symbols: &mut SymbolTable,
    ) -> Option<(Symbol, Expr)> {
        match forall {
            Expr::ForAll { var, body, .. } => {
                // Create a fresh "arbitrary" variable
                let fresh_name = format!("_{}", self.fresh_counter);
                self.fresh_counter += 1;
                let fresh_var = symbols.intern(&fresh_name);

                // Substitute fresh variable for bound variable
                let goal = self.substitute(body, *var, &Expr::Var(fresh_var));

                Some((fresh_var, goal))
            }
            _ => None,
        }
    }

    /// Existential Introduction: P(a) → ∃x.P(x)
    ///
    /// Given a statement P(a) for some specific term a, creates an existential statement.
    pub fn existential_introduction(&self, statement: &Expr, witness: &Expr, var: Symbol) -> Expr {
        // Create ∃var. statement where 'witness' becomes 'var'
        let body = self.substitute(
            statement,
            self.find_matching_var(statement, witness).unwrap_or(var),
            &Expr::Var(var),
        );
        Expr::Exists {
            var,
            domain: None,
            body: Box::new(body),
        }
    }

    /// Existential Elimination (backward reasoning):
    /// To use ∃x.P(x), we assume P(c) for some fresh constant c.
    pub fn existential_elimination(
        &mut self,
        exists: &Expr,
        symbols: &mut SymbolTable,
    ) -> Option<(Symbol, Expr)> {
        match exists {
            Expr::Exists { var, body, .. } => {
                // Create a fresh witness variable
                let fresh_name = format!("_w{}", self.fresh_counter);
                self.fresh_counter += 1;
                let witness = symbols.intern(&fresh_name);

                // Substitute witness for bound variable
                let assumption = self.substitute(body, *var, &Expr::Var(witness));

                Some((witness, assumption))
            }
            _ => None,
        }
    }

    /// Quantifier Negation: ¬∀x.P(x) ↔ ∃x.¬P(x)
    pub fn negate_forall(&self, forall: &Expr) -> Option<Expr> {
        match forall {
            Expr::ForAll { var, domain, body } => Some(Expr::Exists {
                var: *var,
                domain: domain.clone(),
                body: Box::new(Expr::Not(body.clone())),
            }),
            _ => None,
        }
    }

    /// Quantifier Negation: ¬∃x.P(x) ↔ ∀x.¬P(x)
    pub fn negate_exists(&self, exists: &Expr) -> Option<Expr> {
        match exists {
            Expr::Exists { var, domain, body } => Some(Expr::ForAll {
                var: *var,
                domain: domain.clone(),
                body: Box::new(Expr::Not(body.clone())),
            }),
            _ => None,
        }
    }

    /// Find potential instantiation values from the current proof context.
    ///
    /// For a goal like "∀n. P(n)" where we have terms in scope,
    /// suggests relevant values for instantiation.
    pub fn suggest_instantiations(&self, _forall: &Expr, context_terms: &[Expr]) -> Vec<Expr> {
        // Start with context terms
        let mut suggestions: Vec<Expr> = context_terms.to_vec();

        // Add common mathematical values
        suggestions.push(Expr::int(0));
        suggestions.push(Expr::int(1));
        suggestions.push(Expr::int(-1));

        suggestions
    }

    /// Substitute a variable with an expression in a body.
    fn substitute(&self, body: &Expr, var: Symbol, value: &Expr) -> Expr {
        match body {
            Expr::Var(v) if *v == var => value.clone(),
            Expr::Var(_) | Expr::Const(_) | Expr::Pi | Expr::E => body.clone(),

            Expr::Neg(e) => Expr::Neg(Box::new(self.substitute(e, var, value))),
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(self.substitute(e, var, value))),
            Expr::Sin(e) => Expr::Sin(Box::new(self.substitute(e, var, value))),
            Expr::Cos(e) => Expr::Cos(Box::new(self.substitute(e, var, value))),
            Expr::Tan(e) => Expr::Tan(Box::new(self.substitute(e, var, value))),
            Expr::Arcsin(e) => Expr::Arcsin(Box::new(self.substitute(e, var, value))),
            Expr::Arccos(e) => Expr::Arccos(Box::new(self.substitute(e, var, value))),
            Expr::Arctan(e) => Expr::Arctan(Box::new(self.substitute(e, var, value))),
            Expr::Ln(e) => Expr::Ln(Box::new(self.substitute(e, var, value))),
            Expr::Exp(e) => Expr::Exp(Box::new(self.substitute(e, var, value))),
            Expr::Abs(e) => Expr::Abs(Box::new(self.substitute(e, var, value))),
            Expr::Floor(e) => Expr::Floor(Box::new(self.substitute(e, var, value))),
            Expr::Ceiling(e) => Expr::Ceiling(Box::new(self.substitute(e, var, value))),
            Expr::Factorial(e) => Expr::Factorial(Box::new(self.substitute(e, var, value))),
            Expr::Not(e) => Expr::Not(Box::new(self.substitute(e, var, value))),

            Expr::Add(a, b) => Expr::Add(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Sub(a, b) => Expr::Sub(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Mul(a, b) => Expr::Mul(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Div(a, b) => Expr::Div(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Pow(a, b) => Expr::Pow(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::GCD(a, b) => Expr::GCD(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::LCM(a, b) => Expr::LCM(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Mod(a, b) => Expr::Mod(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Binomial(a, b) => Expr::Binomial(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Gte(a, b) => Expr::Gte(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Gt(a, b) => Expr::Gt(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Lte(a, b) => Expr::Lte(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Lt(a, b) => Expr::Lt(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::And(a, b) => Expr::And(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Or(a, b) => Expr::Or(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),
            Expr::Implies(a, b) => Expr::Implies(
                Box::new(self.substitute(a, var, value)),
                Box::new(self.substitute(b, var, value)),
            ),

            Expr::Equation { lhs, rhs } => Expr::Equation {
                lhs: Box::new(self.substitute(lhs, var, value)),
                rhs: Box::new(self.substitute(rhs, var, value)),
            },

            Expr::Sum(terms) => Expr::Sum(
                terms
                    .iter()
                    .map(|t| mm_core::Term {
                        coeff: t.coeff,
                        expr: self.substitute(&t.expr, var, value),
                    })
                    .collect(),
            ),
            Expr::Product(factors) => Expr::Product(
                factors
                    .iter()
                    .map(|f| mm_core::Factor {
                        base: self.substitute(&f.base, var, value),
                        power: self.substitute(&f.power, var, value),
                    })
                    .collect(),
            ),

            Expr::Derivative { expr, var: v } => Expr::Derivative {
                expr: Box::new(self.substitute(expr, var, value)),
                var: *v,
            },
            Expr::Integral { expr, var: v } => Expr::Integral {
                expr: Box::new(self.substitute(expr, var, value)),
                var: *v,
            },

            // For nested quantifiers, don't substitute if variable is shadowed
            Expr::ForAll {
                var: v,
                domain,
                body: b,
            } if *v != var => Expr::ForAll {
                var: *v,
                domain: domain
                    .as_ref()
                    .map(|d| Box::new(self.substitute(d, var, value))),
                body: Box::new(self.substitute(b, var, value)),
            },
            Expr::Exists {
                var: v,
                domain,
                body: b,
            } if *v != var => Expr::Exists {
                var: *v,
                domain: domain
                    .as_ref()
                    .map(|d| Box::new(self.substitute(d, var, value))),
                body: Box::new(self.substitute(b, var, value)),
            },
            Expr::Summation {
                var: v,
                from,
                to,
                body: b,
            } if *v != var => Expr::Summation {
                var: *v,
                from: Box::new(self.substitute(from, var, value)),
                to: Box::new(self.substitute(to, var, value)),
                body: Box::new(self.substitute(b, var, value)),
            },
            Expr::BigProduct {
                var: v,
                from,
                to,
                body: b,
            } if *v != var => Expr::BigProduct {
                var: *v,
                from: Box::new(self.substitute(from, var, value)),
                to: Box::new(self.substitute(to, var, value)),
                body: Box::new(self.substitute(b, var, value)),
            },

            // Shadowed - don't substitute in body
            Expr::ForAll { .. }
            | Expr::Exists { .. }
            | Expr::Summation { .. }
            | Expr::BigProduct { .. } => body.clone(),
        }
    }

    /// Find a variable in an expression that matches a given value.
    fn find_matching_var(&self, _expr: &Expr, _value: &Expr) -> Option<Symbol> {
        // Simple implementation - would need pattern matching for general case
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_universal_instantiation() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        // ∀n. n² ≥ 0
        let forall = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Gte(
                Box::new(Expr::Pow(Box::new(Expr::Var(n)), Box::new(Expr::int(2)))),
                Box::new(Expr::int(0)),
            )),
        };

        let engine = QuantifierEngine::new();

        // Instantiate with n = 5
        let result = engine.universal_instantiate(&forall, &Expr::int(5));
        assert!(result.is_some());

        let inst = result.unwrap();
        // Result should be 5² ≥ 0
        match &inst.result {
            Expr::Gte(lhs, rhs) => match (lhs.as_ref(), rhs.as_ref()) {
                (Expr::Pow(base, exp), Expr::Const(zero)) => {
                    assert_eq!(*base.as_ref(), Expr::int(5));
                    assert_eq!(*exp.as_ref(), Expr::int(2));
                    assert!(zero.is_zero());
                }
                _ => panic!("Unexpected structure"),
            },
            _ => panic!("Expected Gte"),
        }
    }

    #[test]
    fn test_universal_introduction() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        // ∀n. n + 0 = n
        let forall = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Equation {
                lhs: Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(0)))),
                rhs: Box::new(Expr::Var(n)),
            }),
        };

        let mut engine = QuantifierEngine::new();

        let result = engine.universal_introduction(&forall, &mut symbols);
        assert!(result.is_some());

        let (fresh_var, goal) = result.unwrap();
        // Goal should be: _0 + 0 = _0 (with fresh variable)
        match goal {
            Expr::Equation { lhs, rhs } => {
                match lhs.as_ref() {
                    Expr::Add(a, _) => {
                        assert_eq!(*a.as_ref(), Expr::Var(fresh_var));
                    }
                    _ => panic!("Expected Add"),
                }
                assert_eq!(*rhs, Expr::Var(fresh_var));
            }
            _ => panic!("Expected Equation"),
        }
    }

    #[test]
    fn test_quantifier_negation() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // ∀x. P(x)
        let forall = Expr::ForAll {
            var: x,
            domain: None,
            body: Box::new(Expr::Gte(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
        };

        let engine = QuantifierEngine::new();

        // ¬∀x.P(x) should become ∃x.¬P(x)
        let negated = engine.negate_forall(&forall);
        assert!(negated.is_some());

        match negated.unwrap() {
            Expr::Exists { var, body, .. } => {
                assert_eq!(var, x);
                match body.as_ref() {
                    Expr::Not(inner) => {
                        match inner.as_ref() {
                            Expr::Gte(_, _) => {} // Correct!
                            _ => panic!("Inner should be Gte"),
                        }
                    }
                    _ => panic!("Body should be Not"),
                }
            }
            _ => panic!("Expected Exists"),
        }
    }
}
