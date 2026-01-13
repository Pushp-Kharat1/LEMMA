// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Case Analysis for Mathematical Proofs.
//!
//! Case analysis allows proving goals by splitting into exhaustive cases.
//! For example:
//! - To prove P(x) for all real x, prove P(x) for x > 0, x = 0, and x < 0
//! - To prove P(n) for integer n, prove P(even n) and P(odd n)

use mm_core::{Expr, Symbol};

/// A case in case analysis
#[derive(Debug, Clone)]
pub struct Case {
    /// Human-readable name of the case
    pub name: String,

    /// The condition that defines this case
    pub condition: Expr,

    /// The goal to prove in this case (original goal under this condition)
    pub goal: Expr,

    /// Whether this case has been proven
    pub proven: bool,

    /// Proof justification if proven
    pub justification: Option<String>,
}

/// Case analysis state
#[derive(Debug, Clone)]
pub struct CaseAnalysis {
    /// The original goal being proven
    pub original_goal: Expr,

    /// The variable being case-split on
    pub split_var: Option<Symbol>,

    /// The cases to prove
    pub cases: Vec<Case>,

    /// Whether the case split is exhaustive
    pub is_exhaustive: bool,
}

impl CaseAnalysis {
    /// Create a new case analysis for a goal
    pub fn new(goal: Expr) -> Self {
        Self {
            original_goal: goal,
            split_var: None,
            cases: Vec::new(),
            is_exhaustive: false,
        }
    }

    /// Split into positive, zero, and negative cases for a variable
    ///
    /// Splits: x > 0, x = 0, x < 0 (exhaustive for reals)
    pub fn split_by_sign(mut self, var: Symbol) -> Self {
        self.split_var = Some(var);

        let var_expr = Expr::Var(var);
        let zero = Expr::int(0);

        // Case 1: x > 0
        self.cases.push(Case {
            name: format!("{:?} > 0 (positive)", var),
            condition: Expr::Gt(Box::new(var_expr.clone()), Box::new(zero.clone())),
            goal: self.original_goal.clone(),
            proven: false,
            justification: None,
        });

        // Case 2: x = 0
        self.cases.push(Case {
            name: format!("{:?} = 0 (zero)", var),
            condition: Expr::Equation {
                lhs: Box::new(var_expr.clone()),
                rhs: Box::new(zero.clone()),
            },
            goal: self.original_goal.clone(),
            proven: false,
            justification: None,
        });

        // Case 3: x < 0
        self.cases.push(Case {
            name: format!("{:?} < 0 (negative)", var),
            condition: Expr::Lt(Box::new(var_expr), Box::new(zero)),
            goal: self.original_goal.clone(),
            proven: false,
            justification: None,
        });

        self.is_exhaustive = true; // Sign trichotomy is exhaustive for reals
        self
    }

    /// Split into even and odd cases for an integer variable
    pub fn split_by_parity(mut self, var: Symbol, symbols: &mut mm_core::SymbolTable) -> Self {
        self.split_var = Some(var);

        let k = symbols.intern("_k");
        let var_expr = Expr::Var(var);

        // Case 1: n = 2k (even)
        self.cases.push(Case {
            name: format!("{:?} is even", var),
            condition: Expr::Exists {
                var: k,
                domain: None,
                body: Box::new(Expr::Equation {
                    lhs: Box::new(var_expr.clone()),
                    rhs: Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(k)))),
                }),
            },
            goal: self.original_goal.clone(),
            proven: false,
            justification: None,
        });

        // Case 2: n = 2k + 1 (odd)
        self.cases.push(Case {
            name: format!("{:?} is odd", var),
            condition: Expr::Exists {
                var: k,
                domain: None,
                body: Box::new(Expr::Equation {
                    lhs: Box::new(var_expr),
                    rhs: Box::new(Expr::Add(
                        Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(k)))),
                        Box::new(Expr::int(1)),
                    )),
                }),
            },
            goal: self.original_goal.clone(),
            proven: false,
            justification: None,
        });

        self.is_exhaustive = true; // Parity is exhaustive for integers
        self
    }

    /// Split into custom cases with given conditions
    pub fn split_custom(mut self, cases: Vec<(String, Expr)>) -> Self {
        for (name, condition) in cases {
            self.cases.push(Case {
                name,
                condition,
                goal: self.original_goal.clone(),
                proven: false,
                justification: None,
            });
        }
        // User must verify exhaustiveness for custom cases
        self.is_exhaustive = false;
        self
    }

    /// Mark exhaustiveness as verified
    pub fn set_exhaustive(mut self, exhaustive: bool) -> Self {
        self.is_exhaustive = exhaustive;
        self
    }

    /// Mark a case as proven
    pub fn prove_case(&mut self, case_index: usize, justification: String) -> bool {
        if case_index < self.cases.len() {
            self.cases[case_index].proven = true;
            self.cases[case_index].justification = Some(justification);
            true
        } else {
            false
        }
    }

    /// Check if all cases are proven
    pub fn all_proven(&self) -> bool {
        self.cases.iter().all(|c| c.proven)
    }

    /// Get unproven cases
    pub fn unproven_cases(&self) -> Vec<(usize, &Case)> {
        self.cases
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.proven)
            .collect()
    }

    /// Check if the case analysis is complete (exhaustive and all proven)
    pub fn is_complete(&self) -> bool {
        self.is_exhaustive && self.all_proven()
    }

    /// Generate the proof justification
    pub fn justification(&self) -> Option<String> {
        if !self.is_complete() {
            return None;
        }

        let case_proofs: Vec<String> = self
            .cases
            .iter()
            .map(|c| {
                format!(
                    "  - {}: {}",
                    c.name,
                    c.justification.as_ref().unwrap_or(&"proven".to_string())
                )
            })
            .collect();

        Some(format!(
            "By exhaustive case analysis:\n{}",
            case_proofs.join("\n")
        ))
    }
}

/// Common case splits for backward reasoning
pub fn suggest_case_splits(goal: &Expr) -> Vec<CaseSplitSuggestion> {
    let mut suggestions = Vec::new();

    // Find variables in the goal
    let vars = collect_variables(goal);

    for var in vars {
        // Suggest sign split for any variable
        suggestions.push(CaseSplitSuggestion {
            variable: var,
            split_type: SplitType::Sign,
            description: format!("Split {:?} into positive, zero, negative", var),
        });

        // Suggest parity split (useful for integer problems)
        suggestions.push(CaseSplitSuggestion {
            variable: var,
            split_type: SplitType::Parity,
            description: format!("Split {:?} into even and odd", var),
        });
    }

    suggestions
}

/// Type of case split
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitType {
    /// x > 0, x = 0, x < 0
    Sign,
    /// n = 2k, n = 2k+1
    Parity,
    /// Custom user-defined split
    Custom,
}

/// Suggested case split
#[derive(Debug, Clone)]
pub struct CaseSplitSuggestion {
    pub variable: Symbol,
    pub split_type: SplitType,
    pub description: String,
}

/// Collect all variables from an expression
fn collect_variables(expr: &Expr) -> Vec<Symbol> {
    let mut vars = Vec::new();
    collect_vars_recursive(expr, &mut vars);
    vars
}

fn collect_vars_recursive(expr: &Expr, vars: &mut Vec<Symbol>) {
    match expr {
        Expr::Var(v) => {
            if !vars.contains(v) {
                vars.push(*v);
            }
        }
        Expr::Const(_) | Expr::Pi | Expr::E => {}
        Expr::Neg(e)
        | Expr::Sqrt(e)
        | Expr::Sin(e)
        | Expr::Cos(e)
        | Expr::Tan(e)
        | Expr::Ln(e)
        | Expr::Exp(e)
        | Expr::Abs(e)
        | Expr::Floor(e)
        | Expr::Ceiling(e)
        | Expr::Factorial(e)
        | Expr::Not(e) => {
            collect_vars_recursive(e, vars);
        }
        Expr::Add(a, b)
        | Expr::Sub(a, b)
        | Expr::Mul(a, b)
        | Expr::Div(a, b)
        | Expr::Pow(a, b)
        | Expr::GCD(a, b)
        | Expr::LCM(a, b)
        | Expr::Mod(a, b)
        | Expr::Binomial(a, b)
        | Expr::Gte(a, b)
        | Expr::Gt(a, b)
        | Expr::Lte(a, b)
        | Expr::Lt(a, b)
        | Expr::And(a, b)
        | Expr::Or(a, b)
        | Expr::Implies(a, b) => {
            collect_vars_recursive(a, vars);
            collect_vars_recursive(b, vars);
        }
        Expr::Equation { lhs, rhs } => {
            collect_vars_recursive(lhs, vars);
            collect_vars_recursive(rhs, vars);
        }
        Expr::Sum(terms) => {
            for t in terms {
                collect_vars_recursive(&t.expr, vars);
            }
        }
        Expr::Product(factors) => {
            for f in factors {
                collect_vars_recursive(&f.base, vars);
                collect_vars_recursive(&f.power, vars);
            }
        }
        Expr::Derivative { expr, var } | Expr::Integral { expr, var } => {
            collect_vars_recursive(expr, vars);
            if !vars.contains(var) {
                vars.push(*var);
            }
        }
        Expr::ForAll { var, domain, body } | Expr::Exists { var, domain, body } => {
            if let Some(d) = domain {
                collect_vars_recursive(d, vars);
            }
            collect_vars_recursive(body, vars);
            vars.retain(|v| v != var); // Remove bound variable
        }
        Expr::Summation {
            var,
            from,
            to,
            body,
        }
        | Expr::BigProduct {
            var,
            from,
            to,
            body,
        } => {
            collect_vars_recursive(from, vars);
            collect_vars_recursive(to, vars);
            collect_vars_recursive(body, vars);
            vars.retain(|v| v != var);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_sign_split() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // Goal: x² ≥ 0
        let goal = Expr::Gte(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::int(0)),
        );

        let analysis = CaseAnalysis::new(goal).split_by_sign(x);

        assert_eq!(analysis.cases.len(), 3);
        assert!(analysis.is_exhaustive);
        assert!(!analysis.all_proven());

        // Case names should be descriptive
        assert!(analysis.cases[0].name.contains("positive"));
        assert!(analysis.cases[1].name.contains("zero"));
        assert!(analysis.cases[2].name.contains("negative"));
    }

    #[test]
    fn test_parity_split() {
        let mut symbols = SymbolTable::new();
        let n = symbols.intern("n");

        // Goal: n(n+1) is even
        let goal = Expr::Equation {
            lhs: Box::new(Expr::Mod(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(n)),
                    Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                )),
                Box::new(Expr::int(2)),
            )),
            rhs: Box::new(Expr::int(0)),
        };

        let analysis = CaseAnalysis::new(goal).split_by_parity(n, &mut symbols);

        assert_eq!(analysis.cases.len(), 2);
        assert!(analysis.is_exhaustive);
        assert!(analysis.cases[0].name.contains("even"));
        assert!(analysis.cases[1].name.contains("odd"));
    }

    #[test]
    fn test_prove_cases() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        let goal = Expr::Gte(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::int(0)),
        );

        let mut analysis = CaseAnalysis::new(goal).split_by_sign(x);

        // Initially not complete
        assert!(!analysis.is_complete());

        // Prove each case
        analysis.prove_case(0, "Positive squared is positive".to_string());
        analysis.prove_case(1, "0² = 0 ≥ 0".to_string());
        analysis.prove_case(2, "Negative squared is positive".to_string());

        // Now complete
        assert!(analysis.is_complete());
        assert!(analysis.justification().is_some());
    }

    #[test]
    fn test_suggest_splits() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");
        let y = symbols.intern("y");

        let goal = Expr::Gte(
            Box::new(Expr::Add(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
            )),
            Box::new(Expr::int(0)),
        );

        let suggestions = suggest_case_splits(&goal);

        // Should suggest splits for both x and y
        assert!(suggestions.len() >= 4); // At least sign and parity for each
    }
}
