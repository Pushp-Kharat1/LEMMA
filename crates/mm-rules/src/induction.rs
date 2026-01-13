// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Mathematical Induction for Proof Search.
//!
//! Implements various forms of mathematical induction:
//! - Simple induction: P(0) ∧ (∀k. P(k) → P(k+1)) → ∀n. P(n)
//! - Strong induction: (∀k. (∀j < k. P(j)) → P(k)) → ∀n. P(n)
//! - Starting from 1: P(1) ∧ (∀k≥1. P(k) → P(k+1)) → ∀n≥1. P(n)

use mm_core::{Expr, Symbol, SymbolTable};

/// Type of induction to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InductionType {
    /// Standard weak induction: P(0) ∧ (P(k) → P(k+1))
    Simple,
    /// Strong induction: (∀j < k. P(j)) → P(k)
    Strong,
    /// Induction starting from 1 instead of 0
    FromOne,
}

/// State of an induction proof
#[derive(Debug, Clone)]
pub struct InductionProof {
    /// The property P(n) being proven
    pub property: Expr,

    /// The induction variable
    pub var: Symbol,

    /// Type of induction
    pub induction_type: InductionType,

    /// Whether base case is proven
    pub base_proven: bool,

    /// Base case justification
    pub base_justification: Option<String>,

    /// Whether inductive step is proven
    pub step_proven: bool,

    /// Inductive step justification
    pub step_justification: Option<String>,

    /// Fresh variable for inductive hypothesis
    pub k_var: Option<Symbol>,
}

impl InductionProof {
    /// Create a new simple induction proof from a universally quantified goal
    pub fn from_forall(goal: &Expr, symbols: &mut SymbolTable) -> Option<Self> {
        match goal {
            Expr::ForAll { var, body, .. } => {
                let k = symbols.intern("_k");

                Some(Self {
                    property: body.as_ref().clone(),
                    var: *var,
                    induction_type: InductionType::Simple,
                    base_proven: false,
                    base_justification: None,
                    step_proven: false,
                    step_justification: None,
                    k_var: Some(k),
                })
            }
            _ => None,
        }
    }

    /// Create an induction proof with specified type
    pub fn new(
        property: Expr,
        var: Symbol,
        induction_type: InductionType,
        symbols: &mut SymbolTable,
    ) -> Self {
        let k = symbols.intern("_k");

        Self {
            property,
            var,
            induction_type,
            base_proven: false,
            base_justification: None,
            step_proven: false,
            step_justification: None,
            k_var: Some(k),
        }
    }

    /// Get the base case goal
    ///
    /// For simple induction: P(0)
    /// For induction from 1: P(1)
    pub fn base_case(&self) -> Expr {
        let base_value = match self.induction_type {
            InductionType::Simple | InductionType::Strong => Expr::int(0),
            InductionType::FromOne => Expr::int(1),
        };

        self.substitute(&self.property, self.var, &base_value)
    }

    /// Get the inductive hypothesis
    ///
    /// For simple: P(k)
    /// For strong: ∀j < k. P(j)
    pub fn inductive_hypothesis(&self) -> Option<Expr> {
        let k = self.k_var?;

        match self.induction_type {
            InductionType::Simple | InductionType::FromOne => {
                // P(k)
                Some(self.substitute(&self.property, self.var, &Expr::Var(k)))
            }
            InductionType::Strong => {
                // ∀j < k. P(j) - we assume P holds for all j < k
                // Represented as: "For all j less than k, P(j) holds"
                Some(Expr::ForAll {
                    var: k, // Using k as j here for simplicity
                    domain: Some(Box::new(Expr::Lt(
                        Box::new(Expr::Var(k)),
                        Box::new(Expr::Var(k)), // This is a placeholder
                    ))),
                    body: Box::new(self.substitute(&self.property, self.var, &Expr::Var(k))),
                })
            }
        }
    }

    /// Get the inductive step goal
    ///
    /// For simple: P(k+1)
    pub fn inductive_step_goal(&self) -> Option<Expr> {
        let k = self.k_var?;

        let k_plus_1 = Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)));

        Some(self.substitute(&self.property, self.var, &k_plus_1))
    }

    /// Get the full inductive step as an implication
    ///
    /// P(k) → P(k+1)
    pub fn inductive_step_implication(&self) -> Option<Expr> {
        let hypothesis = self.inductive_hypothesis()?;
        let goal = self.inductive_step_goal()?;

        Some(Expr::Implies(Box::new(hypothesis), Box::new(goal)))
    }

    /// Mark the base case as proven
    pub fn prove_base(&mut self, justification: String) {
        self.base_proven = true;
        self.base_justification = Some(justification);
    }

    /// Mark the inductive step as proven
    pub fn prove_step(&mut self, justification: String) {
        self.step_proven = true;
        self.step_justification = Some(justification);
    }

    /// Check if the induction proof is complete
    pub fn is_complete(&self) -> bool {
        self.base_proven && self.step_proven
    }

    /// Get the proof justification if complete
    pub fn justification(&self) -> Option<String> {
        if !self.is_complete() {
            return None;
        }

        let induction_name = match self.induction_type {
            InductionType::Simple => "mathematical induction",
            InductionType::Strong => "strong induction",
            InductionType::FromOne => "mathematical induction (from n=1)",
        };

        Some(format!(
            "By {}:\n  Base case: {}\n  Inductive step: {}",
            induction_name,
            self.base_justification.as_ref().unwrap(),
            self.step_justification.as_ref().unwrap()
        ))
    }

    /// Substitute variable with value in expression
    fn substitute(&self, expr: &Expr, var: Symbol, value: &Expr) -> Expr {
        match expr {
            Expr::Var(v) if *v == var => value.clone(),
            Expr::Var(_) | Expr::Const(_) | Expr::Pi | Expr::E => expr.clone(),

            Expr::Neg(e) => Expr::Neg(Box::new(self.substitute(e, var, value))),
            Expr::Sqrt(e) => Expr::Sqrt(Box::new(self.substitute(e, var, value))),
            Expr::Sin(e) => Expr::Sin(Box::new(self.substitute(e, var, value))),
            Expr::Cos(e) => Expr::Cos(Box::new(self.substitute(e, var, value))),
            Expr::Tan(e) => Expr::Tan(Box::new(self.substitute(e, var, value))),
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

            // For nested binding forms, don't substitute if shadowed
            Expr::ForAll {
                var: v,
                domain,
                body,
            } if *v != var => Expr::ForAll {
                var: *v,
                domain: domain
                    .as_ref()
                    .map(|d| Box::new(self.substitute(d, var, value))),
                body: Box::new(self.substitute(body, var, value)),
            },
            Expr::Exists {
                var: v,
                domain,
                body,
            } if *v != var => Expr::Exists {
                var: *v,
                domain: domain
                    .as_ref()
                    .map(|d| Box::new(self.substitute(d, var, value))),
                body: Box::new(self.substitute(body, var, value)),
            },
            Expr::Summation {
                var: v,
                from,
                to,
                body,
            } if *v != var => Expr::Summation {
                var: *v,
                from: Box::new(self.substitute(from, var, value)),
                to: Box::new(self.substitute(to, var, value)),
                body: Box::new(self.substitute(body, var, value)),
            },
            Expr::BigProduct {
                var: v,
                from,
                to,
                body,
            } if *v != var => Expr::BigProduct {
                var: *v,
                from: Box::new(self.substitute(from, var, value)),
                to: Box::new(self.substitute(to, var, value)),
                body: Box::new(self.substitute(body, var, value)),
            },
            Expr::Derivative { expr: e, var: v } => Expr::Derivative {
                expr: Box::new(self.substitute(e, var, value)),
                var: *v,
            },
            Expr::Integral { expr: e, var: v } => Expr::Integral {
                expr: Box::new(self.substitute(e, var, value)),
                var: *v,
            },

            // Shadowed - don't substitute
            _ => expr.clone(),
        }
    }
}

/// Check if a goal is suitable for induction
pub fn can_use_induction(goal: &Expr) -> bool {
    matches!(goal, Expr::ForAll { .. })
}

/// Suggest induction strategy for a goal
pub fn suggest_induction(goal: &Expr) -> Option<InductionSuggestion> {
    match goal {
        Expr::ForAll { var, domain, body } => {
            // Check if domain suggests starting from 1
            let from_one = domain.as_ref().map(|d| {
                matches!(d.as_ref(), Expr::Gte(_, rhs) if matches!(rhs.as_ref(), Expr::Const(r) if *r == mm_core::Rational::from(1)))
            }).unwrap_or(false);

            let induction_type = if from_one {
                InductionType::FromOne
            } else {
                InductionType::Simple
            };

            Some(InductionSuggestion {
                var: *var,
                induction_type,
                property: body.as_ref().clone(),
                description: format!(
                    "Use {} on {:?}: Prove P({}), then P(k) → P(k+1)",
                    match induction_type {
                        InductionType::Simple => "induction",
                        InductionType::Strong => "strong induction",
                        InductionType::FromOne => "induction from n=1",
                    },
                    var,
                    if from_one { "1" } else { "0" }
                ),
            })
        }
        _ => None,
    }
}

/// Suggested induction approach
#[derive(Debug, Clone)]
pub struct InductionSuggestion {
    pub var: Symbol,
    pub induction_type: InductionType,
    pub property: Expr,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_induction_setup() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        // Goal: ∀n. n(n+1)/2 = sum(1..n)
        // Simplified as: ∀n. n ≥ 0
        let goal = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Gte(Box::new(Expr::Var(n)), Box::new(Expr::int(0)))),
        };

        let proof = InductionProof::from_forall(&goal, &mut symbols);
        assert!(proof.is_some());

        let proof = proof.unwrap();

        // Check base case is P(0)
        let base = proof.base_case();
        match base {
            Expr::Gte(lhs, rhs) => {
                assert_eq!(*lhs, Expr::int(0));
                assert_eq!(*rhs, Expr::int(0));
            }
            _ => panic!("Expected Gte"),
        }
    }

    #[test]
    fn test_inductive_step() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        let goal = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Gte(
                Box::new(Expr::Pow(Box::new(Expr::Var(n)), Box::new(Expr::int(2)))),
                Box::new(Expr::int(0)),
            )),
        };

        let proof = InductionProof::from_forall(&goal, &mut symbols).unwrap();

        // Get inductive hypothesis P(k)
        let hyp = proof.inductive_hypothesis().unwrap();
        match hyp {
            Expr::Gte(lhs, _) => {
                // Should be k² ≥ 0
                match lhs.as_ref() {
                    Expr::Pow(base, _) => {
                        assert!(matches!(base.as_ref(), Expr::Var(_)));
                    }
                    _ => panic!("Expected Pow"),
                }
            }
            _ => panic!("Expected Gte"),
        }

        // Get step goal P(k+1)
        let step = proof.inductive_step_goal().unwrap();
        match step {
            Expr::Gte(lhs, _) => {
                // Should be (k+1)² ≥ 0
                match lhs.as_ref() {
                    Expr::Pow(base, _) => {
                        assert!(matches!(base.as_ref(), Expr::Add(_, _)));
                    }
                    _ => panic!("Expected Pow"),
                }
            }
            _ => panic!("Expected Gte"),
        }
    }

    #[test]
    fn test_complete_induction() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        let goal = Expr::ForAll {
            var: n,
            domain: None,
            body: Box::new(Expr::Gte(Box::new(Expr::Var(n)), Box::new(Expr::int(0)))),
        };

        let mut proof = InductionProof::from_forall(&goal, &mut symbols).unwrap();

        assert!(!proof.is_complete());

        proof.prove_base("0 ≥ 0 is trivially true".to_string());
        assert!(!proof.is_complete());

        proof.prove_step("If k ≥ 0, then k+1 ≥ 1 > 0".to_string());
        assert!(proof.is_complete());

        let justification = proof.justification();
        assert!(justification.is_some());
        assert!(justification.unwrap().contains("mathematical induction"));
    }

    #[test]
    fn test_induction_from_one() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        let property = Expr::Gte(Box::new(Expr::Var(n)), Box::new(Expr::int(1)));

        let proof = InductionProof::new(property, n, InductionType::FromOne, &mut symbols);

        // Base case should be P(1)
        let base = proof.base_case();
        match base {
            Expr::Gte(lhs, rhs) => {
                assert_eq!(*lhs, Expr::int(1));
                assert_eq!(*rhs, Expr::int(1));
            }
            _ => panic!("Expected Gte"),
        }
    }
}
