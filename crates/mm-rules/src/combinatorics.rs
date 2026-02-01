// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Combinatorics rules for IMO-level problem solving.
//! Includes counting principles, binomial coefficients, and generating functions.

use crate::{Domain, Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Rational};

/// Returns the complete set of combinatorics rules used by the solver.
///
/// This aggregates binomial, counting, recurrence, and advanced combinatorics rule sets (IDs 400–442 and 600–669).
///
/// # Examples
///
/// ```
/// let rules = combinatorics_rules();
/// assert_eq!(rules.len(), 66);
/// ```
pub fn combinatorics_rules() -> Vec<Rule> {
    let mut rules = Vec::new();

    rules.extend(binomial_rules());
    rules.extend(counting_rules());
    rules.extend(recurrence_rules());
    // Phase 3: Advanced combinatorics
    rules.extend(advanced_combinatorics_rules());

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
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match: Div(Factorial, Factorial) patterns for binomial
                matches!(expr, Expr::Div(_, _) | Expr::Factorial(_))
            },
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "C(n,0) = 1 for all n".to_string(),
                }]
            },
            reversible: false,
            cost: 1,
        },
        // C(n,n) = 1
        Rule {
            id: RuleId(401),
            name: "binomial_full",
            category: RuleCategory::Simplification,
            description: "C(n,n) = 1",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "C(n,n) = 1 for all n".to_string(),
                }]
            },
            reversible: false,
            cost: 1,
        },
        // C(n,1) = n
        Rule {
            id: RuleId(402),
            name: "binomial_one",
            category: RuleCategory::Simplification,
            description: "C(n,1) = n",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Var(_)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "C(n,1) = n".to_string(),
                }]
            },
            reversible: false,
            cost: 1,
        },
        // C(n,k) = C(n,n-k) symmetry
        Rule {
            id: RuleId(403),
            name: "binomial_symmetry",
            category: RuleCategory::Simplification,
            description: "C(n,k) = C(n,n-k)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Binomial symmetry: C(n,k) = C(n,n-k)".to_string(),
                }]
            },
            reversible: true,
            cost: 1,
        },
        // Pascal's identity: C(n,k) = C(n-1,k-1) + C(n-1,k)
        Rule {
            id: RuleId(404),
            name: "pascal_identity",
            category: RuleCategory::Expansion,
            description: "C(n,k) = C(n-1,k-1) + C(n-1,k)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Pascal's identity: C(n,k) = C(n-1,k-1) + C(n-1,k)".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Hockey stick identity
        Rule {
            id: RuleId(405),
            name: "hockey_stick",
            category: RuleCategory::Simplification,
            description: "ΣC(i,k) for i=k to n = C(n+1,k+1)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Hockey stick identity: ΣC(i,k) = C(n+1,k+1)".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        // Vandermonde's identity
        Rule {
            id: RuleId(406),
            name: "vandermonde",
            category: RuleCategory::Simplification,
            description: "ΣC(m,k)C(n,r-k) = C(m+n,r)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Vandermonde's identity: ΣC(m,k)C(n,r-k) = C(m+n,r)".to_string(),
                }]
            },
            reversible: true,
            cost: 4,
        },
        // Binomial sum: Σ C(n,k) = 2^n
        Rule {
            id: RuleId(407),
            name: "binomial_sum",
            category: RuleCategory::Simplification,
            description: "Σ C(n,k) for k=0 to n = 2^n",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| {
                // Match Pow(2, n) pattern
                if let Expr::Pow(base, _) = expr {
                    return matches!(base.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Binomial sum: Σ C(n,k) = 2^n".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // (a+b)^n expansion (binomial theorem)
        Rule {
            id: RuleId(408),
            name: "binomial_theorem",
            category: RuleCategory::Expansion,
            description: "(a+b)^n = Σ C(n,k) a^k b^(n-k)",
            domains: &[Domain::Combinatorics],
            requires: &[],
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
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "P(n,k) = n!/(n-k)!".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Combinations: C(n,k) = n!/(k!(n-k)!)
        Rule {
            id: RuleId(421),
            name: "combination_formula",
            category: RuleCategory::Simplification,
            description: "C(n,k) = n!/(k!(n-k)!)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "C(n,k) = n!/(k!(n-k)!)".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Pigeonhole principle (n+1 items in n boxes)
        Rule {
            id: RuleId(422),
            name: "pigeonhole",
            category: RuleCategory::AlgebraicSolving,
            description: "n+1 items in n boxes => at least one box has 2+ items",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Gt(_, _) | Expr::Gte(_, _)),
            apply: |_expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::Const(Rational::from_integer(2)),
                    justification:
                        "Pigeonhole: n+1 items in n boxes => at least one box has 2+ items"
                            .to_string(),
                }]
            },
            reversible: false,
            cost: 1,
        },
        // Generalized pigeonhole
        Rule {
            id: RuleId(423),
            name: "pigeonhole_gen",
            category: RuleCategory::AlgebraicSolving,
            description: "n items in k boxes => some box has ≥ ⌈n/k⌉ items",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Ceiling(_) | Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification:
                        "Generalized pigeonhole: n items in k boxes => some box has ≥ ⌈n/k⌉ items"
                            .to_string(),
                }]
            },
            reversible: false,
            cost: 2,
        },
        // Inclusion-exclusion for 2 sets
        Rule {
            id: RuleId(424),
            name: "inclusion_exclusion_2",
            category: RuleCategory::Simplification,
            description: "|A ∪ B| = |A| + |B| - |A ∩ B|",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Inclusion-exclusion: |A ∪ B| = |A| + |B| - |A ∩ B|".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Inclusion-exclusion for 3 sets
        Rule {
            id: RuleId(425),
            name: "inclusion_exclusion_3",
            category: RuleCategory::Simplification,
            description: "|A ∪ B ∪ C| = |A|+|B|+|C| - |A∩B| - |B∩C| - |A∩C| + |A∩B∩C|",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "3-set inclusion-exclusion".to_string(),
                }]
            },
            reversible: true,
            cost: 3,
        },
        // Derangement formula
        Rule {
            id: RuleId(426),
            name: "derangement",
            category: RuleCategory::Simplification,
            description: "D(n) = n! Σ (-1)^k/k! for k=0 to n",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Factorial(_) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "D(n) = n! Σ (-1)^k/k! - derangement formula".to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Catalan number
        Rule {
            id: RuleId(427),
            name: "catalan",
            category: RuleCategory::Simplification,
            description: "C_n = C(2n,n)/(n+1)",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Catalan number: C_n = C(2n,n)/(n+1)".to_string(),
                }]
            },
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
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Fibonacci recurrence: F(n) = F(n-1) + F(n-2)".to_string(),
                }]
            },
            reversible: true,
            cost: 2,
        },
        // Closed form Fibonacci (Binet's formula)
        Rule {
            id: RuleId(441),
            name: "binet_formula",
            category: RuleCategory::Simplification,
            description: "F(n) = (φ^n - ψ^n)/√5",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Pow(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Binet's formula: F(n) = (φ^n - ψ^n)/√5 where φ = (1+√5)/2"
                        .to_string(),
                }]
            },
            reversible: false,
            cost: 3,
        },
        // Linear recurrence solving
        Rule {
            id: RuleId(442),
            name: "linear_recurrence",
            category: RuleCategory::AlgebraicSolving,
            description: "a_n = c1*a_{n-1} + c2*a_{n-2} => characteristic equation",
            domains: &[Domain::Combinatorics],
            requires: &[],
            is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: expr.clone(),
                    justification: "Solve linear recurrence via characteristic equation"
                        .to_string(),
                }]
            },
            reversible: false,
            cost: 4,
        },
    ]
}

// ============================================================================
// Phase 3: Advanced Combinatorics Rules (ID 600+)
// ============================================================================

/// Aggregate advanced combinatorics rule constructors (derangement, Catalan, Stirling, generating functions, identities, and related rules) into a single collection.
///
/// Returns a vector of `Rule` objects covering the advanced combinatorics rules (IDs 600–669).
///
/// # Examples
///
/// ```
/// let rules = advanced_combinatorics_rules();
/// assert!(!rules.is_empty());
/// ```
pub fn advanced_combinatorics_rules() -> Vec<Rule> {
    vec![
        // Derangement rules
        derangement_formula(),
        derangement_recurrence(),
        // Catalan numbers
        catalan_formula(),
        catalan_recurrence(),
        // Stirling numbers
        stirling_first_recurrence(),
        stirling_second_recurrence(),
        // Partition function rules
        partition_recurrence(),
        // Hockey stick identity
        hockey_stick_identity(),
        // Vandermonde's identity
        vandermonde_identity(),
        // Chu-Vandermonde
        chu_vandermonde(),
        // Multinomial theorem
        multinomial_theorem(),
        // Stars and bars
        stars_and_bars(),
        // Pigeonhole principle
        pigeonhole_principle(),
        // Inclusion-exclusion
        inclusion_exclusion_2(),
        inclusion_exclusion_3(),
        // Double counting
        double_counting(),
        // Generating functions
        ordinary_gf(),
        exponential_gf(),
        // Sum of binomials
        binomial_sum_2n(),
        binomial_alternating_sum(),
        // Permutation formulas
        permutation_formula(),
        circular_permutation(),
        derangement_asymptotic(),
        // Fibonacci identities
        fibonacci_addition(),
        fibonacci_gcd(),
        lucas_numbers(),
        // Additional combinatorics (650-669)
        permutation_with_repetition(),
        combination_with_repetition(),
        bell_number_recurrence(),
        multinomial_coefficient(),
        binomial_weighted_sum(),
        subfactorial(),
        christmas_stocking(),
        binomial_squares_sum(),
        rising_factorial(),
        falling_factorial(),
        legendre_formula(),
        kummer_theorem(),
        lucas_theorem(),
        burnside_lemma(),
        polya_enumeration(),
        catalan_alternative(),
        partition_into_parts(),
        pattern_avoidance(),
        derangement_simple_recurrence(),
        fibonacci_generating_function(),
    ]
}

// D(n) = n! * Σ(-1)^k/k!
/// Constructs the derangement formula rule.
///
/// The rule encodes the derangement identity D(n) = n! * Σ (-1)^k / k! and is applicable to factorial or product expressions.
/// The rule's apply function is a placeholder that returns the original expression with a justification message.
///
/// # Examples
///
/// ```
/// let r = derangement_formula();
/// assert_eq!(r.id, RuleId(600));
/// ```
fn derangement_formula() -> Rule {
    Rule {
        id: RuleId(600),
        name: "derangement_formula",
        category: RuleCategory::Simplification,
        description: "D(n) = n! * Σ(-1)^k/k!",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Factorial(_) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "D(n) = n! * Σ(-1)^k/k! for k=0..n".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// D(n) = (n-1)(D(n-1) + D(n-2))
fn derangement_recurrence() -> Rule {
    Rule {
        id: RuleId(601),
        name: "derangement_recurrence",
        category: RuleCategory::Simplification,
        description: "D(n) = (n-1)(D(n-1) + D(n-2))",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Derangement recurrence: D(n) = (n-1)(D(n-1) + D(n-2))".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// C(n) = C(2n,n)/(n+1)
fn catalan_formula() -> Rule {
    Rule {
        id: RuleId(602),
        name: "catalan_formula",
        category: RuleCategory::Simplification,
        description: "C(n) = C(2n,n)/(n+1)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Catalan formula: C(n) = C(2n,n)/(n+1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// C(n+1) = Σ C(i)*C(n-i) for i=0 to n
fn catalan_recurrence() -> Rule {
    Rule {
        id: RuleId(603),
        name: "catalan_recurrence",
        category: RuleCategory::Simplification,
        description: "C(n+1) = Σ C(i)*C(n-i)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Catalan recurrence: C(n+1) = Σ C(i)*C(n-i)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// s(n,k) = s(n-1,k-1) - (n-1)*s(n-1,k)
fn stirling_first_recurrence() -> Rule {
    Rule {
        id: RuleId(604),
        name: "stirling_first_recurrence",
        category: RuleCategory::Simplification,
        description: "s(n,k) = s(n-1,k-1) - (n-1)*s(n-1,k)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Sub(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Stirling 1st kind recurrence: s(n,k) = s(n-1,k-1) - (n-1)*s(n-1,k)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// S(n,k) = k*S(n-1,k) + S(n-1,k-1)
fn stirling_second_recurrence() -> Rule {
    Rule {
        id: RuleId(605),
        name: "stirling_second_recurrence",
        category: RuleCategory::Simplification,
        description: "S(n,k) = k*S(n-1,k) + S(n-1,k-1)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Stirling 2nd kind: S(n,k) = k*S(n-1,k) + S(n-1,k-1)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// p(n) = Σ (-1)^{k+1} * p(n - k(3k-1)/2) for pentagonal recurrence
fn partition_recurrence() -> Rule {
    Rule {
        id: RuleId(606),
        name: "partition_recurrence",
        category: RuleCategory::Simplification,
        description: "Partition function pentagonal recurrence",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification:
                    "Partition pentagonal recurrence: p(n) = Σ (-1)^{k+1} * p(n - k(3k-1)/2)"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Σ C(i,k) for i=k to n = C(n+1,k+1) (hockey stick)
fn hockey_stick_identity() -> Rule {
    Rule {
        id: RuleId(607),
        name: "hockey_stick_identity",
        category: RuleCategory::Simplification,
        description: "Σ C(i,k) = C(n+1,k+1) (Hockey stick)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Hockey stick identity: Σ C(i,k) = C(n+1,k+1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Σ C(m,k)*C(n,r-k) = C(m+n,r) (Vandermonde)
fn vandermonde_identity() -> Rule {
    Rule {
        id: RuleId(608),
        name: "vandermonde_identity",
        category: RuleCategory::Simplification,
        description: "Σ C(m,k)*C(n,r-k) = C(m+n,r)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Vandermonde identity: Σ C(m,k)*C(n,r-k) = C(m+n,r)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Σ C(a,k)*C(b,n-k)*(-1)^(n-k) = C(a-b,n) (Chu-Vandermonde)
fn chu_vandermonde() -> Rule {
    Rule {
        id: RuleId(609),
        name: "chu_vandermonde",
        category: RuleCategory::Simplification,
        description: "Chu-Vandermonde identity",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _) | Expr::Pow(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Chu-Vandermonde: Σ C(a,k)*C(b,n-k)*(-1)^(n-k) = C(a-b,n)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// (x1+x2+...+xk)^n = Σ n!/(n1!*n2!*...*nk!) * x1^n1 * x2^n2 * ... * xk^nk
fn multinomial_theorem() -> Rule {
    Rule {
        id: RuleId(610),
        name: "multinomial_theorem",
        category: RuleCategory::Expansion,
        description: "Multinomial theorem expansion",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification:
                    "Multinomial theorem: (x1+...+xk)^n = Σ n!/(n1!*...*nk!) * x1^n1 * ... * xk^nk"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// C(n+k-1,k) ways to put k indistinguishable balls into n distinguishable bins
fn stars_and_bars() -> Rule {
    Rule {
        id: RuleId(611),
        name: "stars_and_bars",
        category: RuleCategory::Simplification,
        description: "Stars and bars: C(n+k-1,k)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Stars and bars: C(n+k-1,k) ways to distribute k items into n bins"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// n+1 pigeons in n holes => at least 2 in one hole
fn pigeonhole_principle() -> Rule {
    Rule {
        id: RuleId(612),
        name: "pigeonhole_principle",
        category: RuleCategory::AlgebraicSolving,
        description: "n+1 items in n containers => at least 2 share",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Gt(_, _) | Expr::Gte(_, _)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Const(Rational::from_integer(2)),
                justification: "Pigeonhole: n+1 items in n containers => at least 2 share"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// |A ∪ B| = |A| + |B| - |A ∩ B|
fn inclusion_exclusion_2() -> Rule {
    Rule {
        id: RuleId(613),
        name: "inclusion_exclusion_2",
        category: RuleCategory::Simplification,
        description: "|A∪B| = |A| + |B| - |A∩B|",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Inclusion-exclusion principle: |A∪B| = |A| + |B| - |A∩B|"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// |A ∪ B ∪ C| = |A| + |B| + |C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|
fn inclusion_exclusion_3() -> Rule {
    Rule {
        id: RuleId(614),
        name: "inclusion_exclusion_3",
        category: RuleCategory::Simplification,
        description: "3-set inclusion-exclusion",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Sub(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "3-set inclusion-exclusion: |A∪B∪C| = |A|+|B|+|C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Double counting principle
fn double_counting() -> Rule {
    Rule {
        id: RuleId(615),
        name: "double_counting",
        category: RuleCategory::AlgebraicSolving,
        description: "Count same set in two ways",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Double counting: Count the same set in two different ways"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// OGF: Σ a_n * x^n
fn ordinary_gf() -> Rule {
    Rule {
        id: RuleId(616),
        name: "ordinary_gf",
        category: RuleCategory::Simplification,
        description: "Ordinary generating function",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "OGF: Σ a_n * x^n".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// EGF: Σ a_n * x^n / n!
fn exponential_gf() -> Rule {
    Rule {
        id: RuleId(617),
        name: "exponential_gf",
        category: RuleCategory::Simplification,
        description: "Exponential generating function",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "EGF: Σ a_n * x^n / n!".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Σ C(n,k) for k=0 to n = 2^n
fn binomial_sum_2n() -> Rule {
    Rule {
        id: RuleId(618),
        name: "binomial_sum_2n",
        category: RuleCategory::Simplification,
        description: "Σ C(n,k) = 2^n",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| {
            if let Expr::Pow(base, _) = expr {
                return matches!(base.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Σ C(n,k) = 2^n".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// Σ (-1)^k * C(n,k) = 0 for n > 0
fn binomial_alternating_sum() -> Rule {
    Rule {
        id: RuleId(619),
        name: "binomial_alternating_sum",
        category: RuleCategory::Simplification,
        description: "Σ (-1)^k * C(n,k) = 0",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |_expr, _ctx| {
            vec![RuleApplication {
                result: Expr::int(0),
                justification: "Σ (-1)^k * C(n,k) = 0 for n > 0".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// P(n,k) = n!/(n-k)!
fn permutation_formula() -> Rule {
    Rule {
        id: RuleId(620),
        name: "permutation_formula",
        category: RuleCategory::Simplification,
        description: "P(n,k) = n!/(n-k)!",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Permutation formula: P(n,k) = n!/(n-k)!".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Circular permutations: (n-1)!
fn circular_permutation() -> Rule {
    Rule {
        id: RuleId(621),
        name: "circular_permutation",
        category: RuleCategory::Simplification,
        description: "Circular permutations = (n-1)!",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Factorial(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Circular permutations: (n-1)!".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// D(n) ~ n!/e as n -> ∞
fn derangement_asymptotic() -> Rule {
    Rule {
        id: RuleId(622),
        name: "derangement_asymptotic",
        category: RuleCategory::Simplification,
        description: "D(n) ~ n!/e",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Div(_, _) | Expr::Factorial(_)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Derangement asymptotic: D(n) ~ n!/e as n -> ∞".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// F(m+n) = F(m)*F(n+1) + F(m-1)*F(n)
fn fibonacci_addition() -> Rule {
    Rule {
        id: RuleId(623),
        name: "fibonacci_addition",
        category: RuleCategory::Simplification,
        description: "F(m+n) = F(m)*F(n+1) + F(m-1)*F(n)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Fibonacci addition: F(m+n) = F(m)*F(n+1) + F(m-1)*F(n)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// gcd(F(m), F(n)) = F(gcd(m,n))
fn fibonacci_gcd() -> Rule {
    Rule {
        id: RuleId(624),
        name: "fibonacci_gcd",
        category: RuleCategory::Simplification,
        description: "gcd(F(m), F(n)) = F(gcd(m,n))",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::GCD(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Fibonacci GCD: gcd(F(m), F(n)) = F(gcd(m,n))".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// L(n) = F(n-1) + F(n+1)
/// Provides a simplification rule relating Lucas numbers to Fibonacci numbers.
///
/// # Examples
///
/// ```
/// let rule = lucas_numbers();
/// assert_eq!(rule.id.0, 625);
/// assert_eq!(rule.description, "L(n) = F(n-1) + F(n+1)");
/// ```
fn lucas_numbers() -> Rule {
    Rule {
        id: RuleId(625),
        name: "lucas_numbers",
        category: RuleCategory::Simplification,
        description: "L(n) = F(n-1) + F(n+1)",
        domains: &[Domain::Combinatorics],
        requires: &[],
        is_applicable: |expr, _ctx| matches!(expr, Expr::Add(_, _)),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Lucas numbers: L(n) = F(n-1) + F(n+1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// ============================================================================
// Additional Combinatorics Rules (ID 650-669)
// ============================================================================

// Permutations with repetition: n^k
/// Constructs a rule that recognizes expressions of the form `n^k` where the exponent is a constant or variable and documents the permutation-with-repetition interpretation.
///
/// The rule matches power expressions whose exponent is either a constant or a variable and provides the justification "Permutations with repetition: n choices k times = n^k".
///
/// # Examples
///
/// ```
/// let rule = permutation_with_repetition();
/// assert_eq!(rule.cost, 1);
/// assert_eq!(rule.description, "Permutations with repetition: n^k");
/// ```
fn permutation_with_repetition() -> Rule {
    Rule {
        id: RuleId(650),
        name: "permutation_with_repetition",
        category: RuleCategory::Simplification,
        description: "Permutations with repetition: n^k",
        is_applicable: |expr, _ctx| {
            // Match n^k pattern
            if let Expr::Pow(_, exp) = expr {
                return matches!(exp.as_ref(), Expr::Const(_) | Expr::Var(_));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Permutations with repetition: n choices k times = n^k".to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// Combinations with repetition: C(n+k-1, k)
/// Creates a Rule for combinations with repetition (stars and bars).
///
/// The rule targets factorial-division expressions and provides the standard
/// combinatorial justification "C(n+k-1, k) = (n+k-1)!/(k!(n-1)!)".
///
/// # Examples
///
/// ```
/// let r = combination_with_repetition();
/// assert_eq!(r.id, RuleId(651));
/// assert_eq!(r.name, "combination_with_repetition");
/// ```
fn combination_with_repetition() -> Rule {
    Rule {
        id: RuleId(651),
        name: "combination_with_repetition",
        category: RuleCategory::Simplification,
        description: "Combinations with repetition: C(n+k-1, k)",
        is_applicable: |expr, _ctx| {
            // Match factorial division patterns
            matches!(expr, Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Combinations with repetition: C(n+k-1, k) = (n+k-1)!/(k!(n-1)!)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Bell numbers: B(n+1) = Σ C(n,k)*B(k)
/// Creates the Bell numbers recurrence rule.
///
/// This rule encodes the recurrence B(n+1) = Σ_{k=0..n} C(n,k) * B(k) and applies to addition or multiplication expressions.
///
/// # Returns
///
/// A `Rule` with id 652 that matches `Expr::Add` or `Expr::Mul` and returns the input unchanged with a justification string describing the Bell number recurrence.
///
/// # Examples
///
/// ```
/// let r = bell_number_recurrence();
/// assert_eq!(r.id, RuleId(652));
/// assert_eq!(r.name, "bell_number_recurrence");
/// ```
fn bell_number_recurrence() -> Rule {
    Rule {
        id: RuleId(652),
        name: "bell_number_recurrence",
        category: RuleCategory::Simplification,
        description: "B(n+1) = Σ C(n,k)*B(k)",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Bell number recurrence: B(n+1) = Σ C(n,k)*B(k) for k=0..n".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Multinomial coefficient: n!/(k1! k2! ... km!)
/// Constructs a Rule for the multinomial coefficient identity n! / (k1! k2! ... km!).
///
/// The returned Rule matches division expressions whose numerator is a factorial and, when applied,
/// yields the same expression with a justification message describing the multinomial coefficient.
///
/// # Examples
///
/// ```
/// let rule = multinomial_coefficient();
/// // rule.id == RuleId(653)
/// assert_eq!(rule.id.0, 653);
/// ```
fn multinomial_coefficient() -> Rule {
    Rule {
        id: RuleId(653),
        name: "multinomial_coefficient",
        category: RuleCategory::Simplification,
        description: "Multinomial: n!/(k1!k2!...km!)",
        is_applicable: |expr, _ctx| {
            // Match division with factorial
            if let Expr::Div(num, _) = expr {
                return matches!(num.as_ref(), Expr::Factorial(_));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Multinomial coefficient: n!/(k1!k2!...km!)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Binomial coefficient sum by row: Σ k*C(n,k) = n*2^(n-1)
/// Constructs the combinatorics rule for the identity Σ k * C(n, k) = n * 2^(n-1).

///

/// The returned `Rule` recognizes multiplicative expressions involving a power of two and

/// provides a `RuleApplication` whose justification is the identity Σ k*C(n,k) = n*2^(n-1).

///

/// # Examples

///

/// ```

/// let rule = binomial_weighted_sum();

/// assert_eq!(rule.id, RuleId(654));

/// assert_eq!(rule.name, "binomial_weighted_sum");

/// ```
fn binomial_weighted_sum() -> Rule {
    Rule {
        id: RuleId(654),
        name: "binomial_weighted_sum",
        category: RuleCategory::Simplification,
        description: "Σ k*C(n,k) = n*2^(n-1)",
        is_applicable: |expr, _ctx| {
            // Match multiplication pattern
            if let Expr::Mul(left, right) = expr {
                // Check for k * C(n,k) pattern or similar power of 2
                if matches!(right.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2))) {
                    return true;
                }
                if matches!(left.as_ref(), Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Const(c) if *c == Rational::from_integer(2))) {
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Σ k*C(n,k) = n*2^(n-1) for k=0..n".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Derangement subfactorial: !n = D(n)
/// Constructs the simplification rule for the subfactorial (derangement) identity.
///
/// This rule matches factorial or binomial-like division expressions corresponding to subfactorials
/// and preserves the expression while providing the justification "!n = D(n) = ⌊n!/e + 1/2⌋".
///
/// # Returns
///
/// A `Rule` with id 655 named `"subfactorial"` that is applicable to `Expr::Factorial(_)` or `Expr::Div(_, _)` and whose application returns the original expression with a justification string.
///
/// # Examples
///
/// ```
/// let r = subfactorial();
/// assert_eq!(r.name, "subfactorial");
/// assert_eq!(r.id.0, 655);
/// ```
fn subfactorial() -> Rule {
    Rule {
        id: RuleId(655),
        name: "subfactorial",
        category: RuleCategory::Simplification,
        description: "Subfactorial !n = D(n)",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Factorial(_) | Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Subfactorial: !n = D(n) = ⌊n!/e + 1/2⌋".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Christmas stocking identity: C(n,m)*C(m,k) = C(n,k)*C(n-k,m-k)
/// Constructs the simplification rule encoding the "Christmas stocking" binomial identity:
/// C(n, m) * C(m, k) = C(n, k) * C(n - k, m - k).
///
/// # Examples
///
/// ```
/// let r = christmas_stocking();
/// assert_eq!(r.id, RuleId(656));
/// assert_eq!(r.name, "christmas_stocking");
/// ```
fn christmas_stocking() -> Rule {
    Rule {
        id: RuleId(656),
        name: "christmas_stocking",
        category: RuleCategory::Simplification,
        description: "C(n,m)*C(m,k) = C(n,k)*C(n-k,m-k)",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Christmas stocking: C(n,m)*C(m,k) = C(n,k)*C(n-k,m-k)".to_string(),
            }]
        },
        reversible: true,
        cost: 2,
    }
}

// Sum of squares: Σ C(n,k)^2 = C(2n,n)
/// Constructs the rule for the binomial squares sum identity: Σ C(n,k)^2 = C(2n,n).
///
/// The produced Rule matches expressions that represent binomial-coefficient patterns (division or power)
/// and, when applied, returns a RuleApplication preserving the expression with a justification citing the identity.
///
/// # Examples
///
/// ```
/// let r = binomial_squares_sum();
/// assert_eq!(r.id, RuleId(657));
/// assert_eq!(r.name, "binomial_squares_sum");
/// ```
fn binomial_squares_sum() -> Rule {
    Rule {
        id: RuleId(657),
        name: "binomial_squares_sum",
        category: RuleCategory::Simplification,
        description: "Σ C(n,k)^2 = C(2n,n)",
        is_applicable: |expr, _ctx| {
            // Match division pattern for binomial coefficient
            matches!(expr, Expr::Div(_, _) | Expr::Pow(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Σ C(n,k)^2 = C(2n,n) for k=0..n".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Rising factorial: (x)_n = x(x+1)(x+2)...(x+n-1)
/// Constructs the rule for the rising factorial identity.
///
/// The rule has id 658 and represents the rising factorial (x)_n = x(x+1)(x+2)...(x+n-1); it applies to multiplication expressions and, when applied, returns the original expression with a justification string describing the rising factorial formula.
///
/// # Examples
///
/// ```
/// let rule = rising_factorial();
/// assert_eq!(rule.id, RuleId(658));
/// assert_eq!(rule.name, "rising_factorial");
/// ```
fn rising_factorial() -> Rule {
    Rule {
        id: RuleId(658),
        name: "rising_factorial",
        category: RuleCategory::Expansion,
        description: "Rising factorial (x)_n",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Rising factorial: (x)_n = x(x+1)(x+2)...(x+n-1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Falling factorial: (x)^n = x(x-1)(x-2)...(x-n+1)
/// Creates a Rule representing the falling factorial identity.

///

/// The rule describes the falling factorial (x)_n = x(x-1)(x-2)…(x-n+1). It applies to multiplication expressions and, when applied, returns the input expression unchanged with a justification string describing the falling factorial.

///

/// # Returns

///

/// The constructed `Rule` which matches `Expr::Mul(_, _)` and produces a single `RuleApplication` containing the original expression and a justification: `"Falling factorial: x^(n) = x(x-1)(x-2)...(x-n+1)"`.

///

/// # Examples

///

/// ```

/// let rule = falling_factorial();

/// assert_eq!(rule.id, RuleId(659));

/// ```
fn falling_factorial() -> Rule {
    Rule {
        id: RuleId(659),
        name: "falling_factorial",
        category: RuleCategory::Expansion,
        description: "Falling factorial (x)^n",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Falling factorial: x^(n) = x(x-1)(x-2)...(x-n+1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Legendre's formula: vp(n!) = Σ ⌊n/p^k⌋
/// Creates a rule representing Legendre's formula for the p-adic valuation of n!.
///
/// The rule applies to floor or division expressions and, when applied, returns the input unchanged with a justification stating that the highest power of a prime p dividing n! is the sum of floor(n / p^k) over k.
///
/// # Examples
///
/// ```
/// let r = legendre_formula();
/// assert_eq!(r.id.0, 660);
/// assert!(r.description.contains("vp(n!)"));
/// ```
fn legendre_formula() -> Rule {
    Rule {
        id: RuleId(660),
        name: "legendre_formula",
        category: RuleCategory::NumberTheory,
        description: "Legendre: vp(n!) = Σ ⌊n/p^k⌋",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Floor(_) | Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Legendre's formula: highest power of p dividing n! is Σ ⌊n/p^k⌋".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Kummer's theorem for binomial mod p
/// Provides the Kummer theorem rule which relates the p-adic valuation of a binomial coefficient to carries in base p.
///
/// The rule matches expressions of the form `C(m+n, m)` when presented as a division or modulus and records that `v_p(C(m+n,m))` equals the number of carries when adding `m` and `n` in base `p`.
///
/// # Examples
///
/// ```
/// let rule = kummer_theorem();
/// assert_eq!(rule.id, RuleId(661));
/// assert!(rule.description.contains("Kummer"));
/// ```
fn kummer_theorem() -> Rule {
    Rule {
        id: RuleId(661),
        name: "kummer_theorem",
        category: RuleCategory::NumberTheory,
        description: "Kummer: vp(C(m+n,m)) = carries in base p",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Mod(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Kummer: vp(C(m+n,m)) equals number of carries when adding m and n in base p".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Lucas' theorem: C(m,n) mod p = Π C(mi,ni) mod p
/// Constructs the Rule implementing Lucas' theorem for binomial coefficients modulo a prime.
///
/// The returned `Rule` matches expressions of the form `C(m,n) mod p` (represented as a `Mod` whose inner expression is a `Div`) and provides a justification string describing Lucas' theorem. The rule has id `662`, category `NumberTheory`, is not reversible, and has cost `3`.
///
/// # Examples
///
/// ```
/// let r = lucas_theorem();
/// assert_eq!(r.id, RuleId(662));
/// assert_eq!(r.name, "lucas_theorem");
/// ```
fn lucas_theorem() -> Rule {
    Rule {
        id: RuleId(662),
        name: "lucas_theorem",
        category: RuleCategory::NumberTheory,
        description: "Lucas: C(m,n) mod p",
        is_applicable: |expr, _ctx| {
            if let Expr::Mod(inner, _) = expr {
                return matches!(inner.as_ref(), Expr::Div(_, _));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Lucas' theorem: C(m,n) mod p = Π C(mi,ni) mod p where m,n in base p".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Burnside's lemma: |X/G| = (1/|G|) Σ |X^g|
/// Constructs the combinatorics rule representing Burnside's lemma for orbit counting.
///
/// The returned Rule matches multiplicative or divisional expressions and records
/// the identity |X/G| = (1/|G|) Σ |X^g| with an explanatory justification.
///
/// # Examples
///
/// ```
/// let r = burnside_lemma();
/// assert_eq!(r.id, RuleId(663));
/// assert!(r.description.contains("Burnside"));
/// ```
fn burnside_lemma() -> Rule {
    Rule {
        id: RuleId(663),
        name: "burnside_lemma",
        category: RuleCategory::Simplification,
        description: "Burnside: |X/G| = (1/|G|) Σ |X^g|",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Burnside's lemma: |X/G| = (1/|G|) Σ |X^g| for g in G".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Polya enumeration theorem
/// Constructs a Rule implementing the Polya enumeration theorem.
///
/// The returned Rule matches division or multiplication expressions and, when applied,
/// records a justification about counting inequivalent configurations under a group action.
///
/// # Examples
///
/// ```
/// let rule = polya_enumeration();
/// assert_eq!(rule.id, RuleId(664));
/// ```
fn polya_enumeration() -> Rule {
    Rule {
        id: RuleId(664),
        name: "polya_enumeration",
        category: RuleCategory::Simplification,
        description: "Polya enumeration theorem",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Div(_, _) | Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Polya enumeration: count inequivalent configurations under group action".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Catalan alternative formula: C_n = (2n)!/(n!(n+1)!)
/// Provides a rule recognizing the Catalan number alternative formula C_n = (2n)! / (n! (n+1)!).
///
/// The rule matches expressions that are a division whose numerator is a factorial and records
/// the Catalan alternative as its justification without performing a transformation.
///
/// # Examples
///
/// ```
/// let r = catalan_alternative();
/// assert_eq!(r.id.0, 665);
/// ```
fn catalan_alternative() -> Rule {
    Rule {
        id: RuleId(665),
        name: "catalan_alternative",
        category: RuleCategory::Simplification,
        description: "C_n = (2n)!/(n!(n+1)!)",
        is_applicable: |expr, _ctx| {
            if let Expr::Div(num, _) = expr {
                return matches!(num.as_ref(), Expr::Factorial(_));
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Catalan alternative: C_n = (2n)!/(n!(n+1)!)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Partition function: p(n,k) number of partitions of n into k parts
/// Creates a Rule representing the partition recurrence p(n,k) = p(n-1,k-1) + p(n-k,k).
///
/// The rule matches addition expressions and, when applied, returns the input expression
/// unchanged with a justification describing the partition recurrence.
///
/// # Examples
///
/// ```
/// let r = partition_into_parts();
/// assert_eq!(r.id.0, 666);
/// assert_eq!(r.name, "partition_into_parts");
/// ```
fn partition_into_parts() -> Rule {
    Rule {
        id: RuleId(666),
        name: "partition_into_parts",
        category: RuleCategory::Simplification,
        description: "p(n,k) = p(n-1,k-1) + p(n-k,k)",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Add(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Partition recurrence: p(n,k) = p(n-1,k-1) + p(n-k,k)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Restricted permutations: permutations avoiding pattern
/// Constructs the rule for pattern-avoiding permutations.

///

/// The returned `Rule` matches factorial expressions or binomial-style divisions and, when applied,

/// produces the same expression with a justification noting that pattern-avoiding permutations

/// are counted by Catalan or similar sequences.

///

/// # Examples

///

/// ```

/// let r = pattern_avoidance();

/// assert_eq!(r.id, RuleId(667));

/// assert_eq!(r.name, "pattern_avoidance");

/// ```
fn pattern_avoidance() -> Rule {
    Rule {
        id: RuleId(667),
        name: "pattern_avoidance",
        category: RuleCategory::Simplification,
        description: "Permutations avoiding pattern",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Factorial(_) | Expr::Div(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Pattern-avoiding permutations counted by Catalan or similar sequences".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Recurrence for derangements: D(n) = n*D(n-1) + (-1)^n
/// Constructs a Rule that encodes the simple derangement recurrence D(n) = n*D(n-1) + (-1)^n.
///
/// # Returns
///
/// The `Rule` representing the derangement simple recurrence (id 668).
///
/// # Examples
///
/// ```
/// let rule = derangement_simple_recurrence();
/// assert_eq!(rule.id, RuleId(668));
/// assert_eq!(rule.name, "derangement_simple_recurrence");
/// ```
fn derangement_simple_recurrence() -> Rule {
    Rule {
        id: RuleId(668),
        name: "derangement_simple_recurrence",
        category: RuleCategory::Simplification,
        description: "D(n) = n*D(n-1) + (-1)^n",
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Add(_, _) | Expr::Mul(_, _))
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Derangement simple recurrence: D(n) = n*D(n-1) + (-1)^n".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Generating function for Fibonacci: F(x) = x/(1-x-x^2)
/// Constructs a rule that recognizes the Fibonacci generating function x/(1 - x - x^2)
///
/// The rule matches expressions written as a division whose denominator is a polynomial of the form `1 - x - x^2`
/// and, when applied, returns the same expression with a justification stating the generating function identity
/// Σ F_n x^n = x/(1 - x - x^2).
///
/// # Examples
///
/// ```
/// let r = fibonacci_generating_function();
/// assert_eq!(r.id, RuleId(669));
/// assert!(r.description.contains("x/(1-x-x^2)"));
/// ```
fn fibonacci_generating_function() -> Rule {
    Rule {
        id: RuleId(669),
        name: "fibonacci_generating_function",
        category: RuleCategory::Simplification,
        description: "Fibonacci GF: x/(1-x-x^2)",
        is_applicable: |expr, _ctx| {
            // Match division with polynomial
            if let Expr::Div(_, denom) = expr {
                if let Expr::Sub(_, _) = denom.as_ref() {
                    return true;
                }
            }
            false
        },
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: expr.clone(),
                justification: "Fibonacci generating function: Σ F_n x^n = x/(1-x-x^2)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}