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

// ============================================================================
// Phase 3: Advanced Combinatorics Rules (ID 600+)
// ============================================================================

/// Get all advanced combinatorics rules
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
    ]
}

// D(n) = n! * Σ(-1)^k/k! for k=0 to n
fn derangement_formula() -> Rule {
    Rule {
        id: RuleId(600),
        name: "derangement_formula",
        category: RuleCategory::Simplification,
        description: "D(n) = n! * Σ(-1)^k/k!",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
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
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 3,
    }
}

// L(n) = F(n-1) + F(n+1)
fn lucas_numbers() -> Rule {
    Rule {
        id: RuleId(625),
        name: "lucas_numbers",
        category: RuleCategory::Simplification,
        description: "L(n) = F(n-1) + F(n+1)",
        is_applicable: |_expr, _ctx| false,
        apply: |_expr, _ctx| vec![],
        reversible: false,
        cost: 2,
    }
}
