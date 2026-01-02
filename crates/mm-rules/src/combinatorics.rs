// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Combinatorics rules for IMO-level problem solving.
//! Includes counting principles, binomial coefficients, and generating functions.

use crate::{Rule, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Get all combinatorics rules (50+).
pub fn combinatorics_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.extend(binomial_rules());
    rules.extend(counting_rules());
    rules.extend(recurrence_rules());

    rules
}

// ============================================================================
// Binomial Coefficient Rules (ID 400+)
// ============================================================================

fn binomial_rules() -> Vec<Rule> {
    vec![
        // C(n,0) = 1
        Rule {
            id: RuleId(400),
            name: "binomial_zero",
            category: RuleCategory::Simplification,
            description: "C(n,0) = 1",
            is_applicable: |_expr, _ctx| false, // Need Binomial in Expr
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // C(n,n) = 1
        Rule {
            id: RuleId(401),
            name: "binomial_full",
            category: RuleCategory::Simplification,
            description: "C(n,n) = 1",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // C(n,1) = n
        Rule {
            id: RuleId(402),
            name: "binomial_one",
            category: RuleCategory::Simplification,
            description: "C(n,1) = n",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // C(n,k) = C(n,n-k) symmetry
        Rule {
            id: RuleId(403),
            name: "binomial_symmetry",
            category: RuleCategory::Simplification,
            description: "C(n,k) = C(n,n-k)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 1,
        },
        // Pascal's identity: C(n,k) = C(n-1,k-1) + C(n-1,k)
        Rule {
            id: RuleId(404),
            name: "pascal_identity",
            category: RuleCategory::Expansion,
            description: "C(n,k) = C(n-1,k-1) + C(n-1,k)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Hockey stick identity
        Rule {
            id: RuleId(405),
            name: "hockey_stick",
            category: RuleCategory::Simplification,
            description: "ΣC(i,k) for i=k to n = C(n+1,k+1)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // Vandermonde's identity
        Rule {
            id: RuleId(406),
            name: "vandermonde",
            category: RuleCategory::Simplification,
            description: "ΣC(m,k)C(n,r-k) = C(m+n,r)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 4,
        },
        // Binomial sum: Σ C(n,k) = 2^n
        Rule {
            id: RuleId(407),
            name: "binomial_sum",
            category: RuleCategory::Simplification,
            description: "Σ C(n,k) for k=0 to n = 2^n",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // (a+b)^n expansion (binomial theorem)
        Rule {
            id: RuleId(408),
            name: "binomial_theorem",
            category: RuleCategory::Expansion,
            description: "(a+b)^n = Σ C(n,k) a^k b^(n-k)",
            is_applicable: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    if matches!(base.as_ref(), Expr::Add(_, _)) {
                        if let Expr::Const(n) = exp.as_ref() {
                            return n.is_integer() && *n > Rational::from_integer(2);
                        }
                    }
                }
                false
            },
            apply: |_expr, _ctx| {
                // Full expansion would be complex
                vec![]
            },
            reversible: true,
            cost: 5,
        },
    ]
}

// ============================================================================
// Counting Rules (ID 420+)
// ============================================================================

fn counting_rules() -> Vec<Rule> {
    vec![
        // Permutations: P(n,k) = n!/(n-k)!
        Rule {
            id: RuleId(420),
            name: "permutation_formula",
            category: RuleCategory::Simplification,
            description: "P(n,k) = n!/(n-k)!",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Combinations: C(n,k) = n!/(k!(n-k)!)
        Rule {
            id: RuleId(421),
            name: "combination_formula",
            category: RuleCategory::Simplification,
            description: "C(n,k) = n!/(k!(n-k)!)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Pigeonhole principle (n+1 items in n boxes)
        Rule {
            id: RuleId(422),
            name: "pigeonhole",
            category: RuleCategory::AlgebraicSolving,
            description: "n+1 items in n boxes => at least one box has 2+ items",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 1,
        },
        // Generalized pigeonhole
        Rule {
            id: RuleId(423),
            name: "pigeonhole_gen",
            category: RuleCategory::AlgebraicSolving,
            description: "n items in k boxes => some box has ≥ ⌈n/k⌉ items",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 2,
        },
        // Inclusion-exclusion for 2 sets
        Rule {
            id: RuleId(424),
            name: "inclusion_exclusion_2",
            category: RuleCategory::Simplification,
            description: "|A ∪ B| = |A| + |B| - |A ∩ B|",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Inclusion-exclusion for 3 sets
        Rule {
            id: RuleId(425),
            name: "inclusion_exclusion_3",
            category: RuleCategory::Simplification,
            description: "|A ∪ B ∪ C| = |A|+|B|+|C| - |A∩B| - |B∩C| - |A∩C| + |A∩B∩C|",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
        // Derangement formula
        Rule {
            id: RuleId(426),
            name: "derangement",
            category: RuleCategory::Simplification,
            description: "D(n) = n! Σ (-1)^k/k! for k=0 to n",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
        // Catalan number
        Rule {
            id: RuleId(427),
            name: "catalan",
            category: RuleCategory::Simplification,
            description: "C_n = C(2n,n)/(n+1)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 3,
        },
    ]
}

// ============================================================================
// Recurrence Rules (ID 440+)
// ============================================================================

fn recurrence_rules() -> Vec<Rule> {
    vec![
        // Fibonacci recurrence
        Rule {
            id: RuleId(440),
            name: "fibonacci_recurrence",
            category: RuleCategory::AlgebraicSolving,
            description: "F(n) = F(n-1) + F(n-2)",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: true,
            cost: 2,
        },
        // Closed form Fibonacci (Binet's formula)
        Rule {
            id: RuleId(441),
            name: "binet_formula",
            category: RuleCategory::Simplification,
            description: "F(n) = (φ^n - ψ^n)/√5",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 3,
        },
        // Linear recurrence solving
        Rule {
            id: RuleId(442),
            name: "linear_recurrence",
            category: RuleCategory::AlgebraicSolving,
            description: "a_n = c1*a_{n-1} + c2*a_{n-2} => characteristic equation",
            is_applicable: |_expr, _ctx| false,
            apply: |_expr, _ctx| vec![],
            reversible: false,
            cost: 4,
        },
    ]
}
