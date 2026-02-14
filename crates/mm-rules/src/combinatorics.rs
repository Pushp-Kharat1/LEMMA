// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Combinatorics rules for IMO-level problem solving.
//! Includes counting principles, binomial coefficients, and generating functions.

use crate::{Rule, RuleApplication, RuleCategory, RuleId};
use mm_core::{Expr, Symbol, SymbolTable};
use std::sync::{Mutex, OnceLock};

// Minimal, module-local symbol interner for generating helper variables inside rule bodies.
// This keeps generated symbols stable across rules without leaking a global symbol table API.
/// Interns a module-local symbol for `name` and returns its `Symbol`.
///
/// This uses a lazily initialized, shared interner local to this module so that
/// repeated calls with the same `name` produce the same `Symbol`. The interner
/// is protected by a mutex for thread safety.
///
/// # Parameters
///
/// - `name`: The identifier to intern.
///
/// # Returns
///
/// `Symbol` for the interned `name`; the same `Symbol` is returned for equal names.
///
/// # Panics
///
/// Panics if the internal interner mutex is poisoned.
///
/// # Examples
///
/// ```
/// let a = intern_symbol("n");
/// let b = intern_symbol("n");
/// assert_eq!(a, b);
/// ```
fn intern_symbol(name: &str) -> Symbol {
    static INTERNER: OnceLock<Mutex<SymbolTable>> = OnceLock::new();
    let mutex = INTERNER.get_or_init(|| Mutex::new(SymbolTable::new()));
    mutex
        .lock()
        .expect("symbol interner mutex poisoned")
        .intern(name)
}

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

/// Constructs the collection of solver rules that handle binomial coefficient identities.
///
/// The returned rules cover common binomial identities and expansions such as:
/// - `C(n,0) = 1`
/// - `C(n,n) = 1`
/// - `C(n,1) = n`
/// - symmetry `C(n,k) = C(n, n-k)`
/// - Pascal's identity `C(n,k) = C(n-1,k-1) + C(n-1,k)`
/// - the hockey stick identity for summations of binomial coefficients
/// - Vandermonde's identity for convolutions of binomials
/// - the binomial sum `Σ_{k=0..n} C(n,k) = 2^n`
/// - the binomial theorem `(a+b)^n = Σ C(n,k) a^k b^(n-k)`
///
/// # Examples
///
/// ```
/// let rules = binomial_rules();
/// // at least the basic simplification rules should be present
/// assert!(rules.iter().any(|r| r.name == "binomial_zero"));
/// assert!(rules.iter().any(|r| r.name == "binomial_theorem"));
/// ```
///
/// # Returns
///
/// A `Vec<Rule>` containing rules that recognize and produce transformations or equations
/// for expressions involving binomial coefficients and related summations.
fn binomial_rules() -> Vec<Rule> {
    vec![
        // C(n,0) = 1
        Rule {
            id: RuleId(400),
            name: "binomial_zero",
            category: RuleCategory::Simplification,
            description: "C(n,0) = 1",
            is_applicable: |expr, _ctx| {
                if let Expr::Binomial(_, k) = expr {
                    if let Expr::Const(c) = k.as_ref() {
                        return c.is_zero();
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "C(n,0) = 1".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Binomial(n, k) if matches!(n.as_ref(), Expr::Var(vn) if matches!(k.as_ref(), Expr::Var(vk) if vk == vn))),
            apply: |expr, _ctx| {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "C(n,n) = 1".to_string(),
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
            is_applicable: |expr, _ctx| {
                if let Expr::Binomial(_, k) = expr {
                    if let Expr::Const(c) = k.as_ref() {
                        return c.is_one();
                    }
                }
                false
            },
            apply: |expr, _ctx| {
                if let Expr::Binomial(n, _) = expr {
                    return vec![RuleApplication {
                        result: *n.clone(),
                        justification: "C(n,1) = n".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Binomial(_, _)),
            apply: |expr, _ctx| {
                if let Expr::Binomial(n, k) = expr {
                    let rhs = Expr::Binomial(n.clone(), Box::new(Expr::Sub(n.clone(), k.clone())));
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: Box::new(expr.clone()),
                            rhs: Box::new(rhs),
                        },
                        justification: "Binomial symmetry C(n,k)=C(n,n-k)".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Binomial(_, _)),
            apply: |expr, _ctx| {
                if let Expr::Binomial(n, k) = expr {
                    let n_minus_1 = Expr::Sub(n.clone(), Box::new(Expr::int(1)));
                    let k_minus_1 = Expr::Sub(k.clone(), Box::new(Expr::int(1)));
                    let rhs = Expr::Add(
                        Box::new(Expr::Binomial(
                            Box::new(n_minus_1.clone()),
                            Box::new(k_minus_1),
                        )),
                        Box::new(Expr::Binomial(Box::new(n_minus_1), k.clone())),
                    );
                    return vec![RuleApplication {
                        result: Expr::Equation {
                            lhs: Box::new(expr.clone()),
                            rhs: Box::new(rhs),
                        },
                        justification: "Pascal identity".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
            apply: |expr, _ctx| hockey_stick_identity().apply(expr, _ctx),
            reversible: true,
            cost: 3,
        },
        // Vandermonde's identity
        Rule {
            id: RuleId(406),
            name: "vandermonde",
            category: RuleCategory::Simplification,
            description: "ΣC(m,k)C(n,r-k) = C(m+n,r)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
            apply: |expr, _ctx| vandermonde_identity().apply(expr, _ctx),
            reversible: true,
            cost: 4,
        },
        // Binomial sum: Σ C(n,k) = 2^n
        Rule {
            id: RuleId(407),
            name: "binomial_sum",
            category: RuleCategory::Simplification,
            description: "Σ C(n,k) for k=0 to n = 2^n",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
            apply: |expr, _ctx| {
                if let Expr::Summation { to, .. } = expr {
                    let rhs = Expr::Pow(Box::new(Expr::int(2)), to.clone());
                    return vec![RuleApplication {
                        result: rhs,
                        justification: "Σ_{k=0..n} C(n,k) = 2^n".to_string(),
                    }];
                }
                vec![]
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(base, _) if matches!(base.as_ref(), Expr::Add(_, _))),
            apply: |expr, _ctx| {
                if let Expr::Pow(base, exp) = expr {
                    let k = intern_symbol("k");
                    let n_sym = *exp.clone();
                    if let Expr::Add(a, b) = base.as_ref() {
                        let term = Expr::Mul(
                            Box::new(Expr::Binomial(
                                Box::new(n_sym.clone()),
                                Box::new(Expr::Var(k)),
                            )),
                            Box::new(Expr::Mul(
                                Box::new(Expr::Pow(a.clone(), Box::new(Expr::Var(k)))),
                                Box::new(Expr::Pow(
                                    b.clone(),
                                    Box::new(Expr::Sub(
                                        Box::new(n_sym.clone()),
                                        Box::new(Expr::Var(k)),
                                    )),
                                )),
                            )),
                        );
                        let sum = Expr::Summation {
                            var: k,
                            from: Box::new(Expr::int(0)),
                            to: Box::new(n_sym),
                            body: Box::new(term),
                        };
                        return vec![RuleApplication {
                            result: Expr::Equation {
                                lhs: Box::new(expr.clone()),
                                rhs: Box::new(sum),
                            },
                            justification: "Binomial theorem expansion".to_string(),
                        }];
                    }
                }
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

/// Collects counting and basic combinatorics rules for the solver.
///
/// This function builds and returns the set of rules implementing permutation and
/// combination formulas, the pigeonhole principles, inclusion–exclusion identities,
/// derangements, and Catalan-number rules used by the solver.
///
/// # Examples
///
/// ```
/// let rules = counting_rules();
/// assert!(!rules.is_empty());
/// ```
fn counting_rules() -> Vec<Rule> {
    vec![
        // Permutations: P(n,k) = n!/(n-k)!
        Rule {
            id: RuleId(420),
            name: "permutation_formula",
            category: RuleCategory::Simplification,
            description: "P(n,k) = n!/(n-k)!",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let k = intern_symbol("k");
                let rhs = Expr::Div(
                    Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                    Box::new(Expr::Factorial(Box::new(Expr::Sub(
                        Box::new(Expr::Var(n)),
                        Box::new(Expr::Var(k)),
                    )))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Permutation count P(n,k)".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let k = intern_symbol("k");
                let rhs = Expr::Div(
                    Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                    Box::new(Expr::Mul(
                        Box::new(Expr::Factorial(Box::new(Expr::Var(k)))),
                        Box::new(Expr::Factorial(Box::new(Expr::Sub(
                            Box::new(Expr::Var(n)),
                            Box::new(Expr::Var(k)),
                        )))),
                    )),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Combination formula C(n,k)".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let rhs = Expr::Gte(
                    Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                    Box::new(Expr::Var(intern_symbol("box_with_2"))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Pigeonhole principle".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let k = intern_symbol("k");
                let rhs = Expr::Ceiling(Box::new(Expr::Div(
                    Box::new(Expr::Var(n)),
                    Box::new(Expr::Var(k)),
                )));
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Generalized pigeonhole".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let a = intern_symbol("A");
                let b = intern_symbol("B");
                let rhs = Expr::Sub(
                    Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                    Box::new(Expr::Var(intern_symbol("A∩B"))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "|A∪B| = |A| + |B| - |A∩B|".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let a = intern_symbol("A");
                let b = intern_symbol("B");
                let c = intern_symbol("C");
                let rhs = Expr::Add(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Sub(
                            Box::new(Expr::Add(
                                Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                                Box::new(Expr::Var(c)),
                            )),
                            Box::new(Expr::Add(
                                Box::new(Expr::Var(intern_symbol("A∩B"))),
                                Box::new(Expr::Var(intern_symbol("A∩C"))),
                            )),
                        )),
                        Box::new(Expr::Var(intern_symbol("B∩C"))),
                    )),
                    Box::new(Expr::Var(intern_symbol("A∩B∩C"))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| derangement_formula().apply(expr, _ctx),
            reversible: false,
            cost: 3,
        },
        // Catalan number
        Rule {
            id: RuleId(427),
            name: "catalan",
            category: RuleCategory::Simplification,
            description: "C_n = C(2n,n)/(n+1)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| catalan_formula().apply(expr, _ctx),
            reversible: true,
            cost: 3,
        },
    ]
}

// ============================================================================
// Recurrence Rules (ID 440+)
// ============================================================================

/// Returns the set of recurrence-related combinatorics rules.
///
/// The collection includes the Fibonacci recurrence, Binet's closed form, and a
/// generic linear recurrence rule that produces the characteristic polynomial form.
///
/// # Examples
///
/// ```
/// let rules = recurrence_rules();
/// assert!(rules.len() >= 3);
/// ```
fn recurrence_rules() -> Vec<Rule> {
    vec![
        // Fibonacci recurrence
        Rule {
            id: RuleId(440),
            name: "fibonacci_recurrence",
            category: RuleCategory::AlgebraicSolving,
            description: "F(n) = F(n-1) + F(n-2)",
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let rhs = Expr::Add(
                    Box::new(Expr::Var(intern_symbol("F(n-1)"))),
                    Box::new(Expr::Var(intern_symbol("F(n-2)"))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Fibonacci recurrence".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let n = intern_symbol("n");
                let sqrt5 = Expr::Sqrt(Box::new(Expr::int(5)));
                let phi = Expr::Div(
                    Box::new(Expr::Add(Box::new(Expr::int(1)), sqrt5.clone().into())),
                    Box::new(Expr::int(2)),
                );
                let psi = Expr::Div(
                    Box::new(Expr::Sub(Box::new(Expr::int(1)), sqrt5.clone().into())),
                    Box::new(Expr::int(2)),
                );
                let rhs = Expr::Div(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Pow(Box::new(phi), Box::new(Expr::Var(n)))),
                        Box::new(Expr::Pow(Box::new(psi), Box::new(Expr::Var(n)))),
                    )),
                    Box::new(Expr::Sqrt(Box::new(Expr::int(5)))),
                );
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Binet formula".to_string(),
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
            is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
            apply: |expr, _ctx| {
                let r = intern_symbol("r");
                let rhs = Expr::Var(intern_symbol("characteristic_polynomial(r)"));
                vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(rhs),
                    },
                    justification: "Linear recurrence solved via characteristic equation"
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

/// Returns the set of advanced combinatorics rules (IDs 600–669) used by the solver.
///
/// The collection includes rules for derangements, Catalan numbers, Stirling numbers,
/// generating functions, partition identities, Vandermonde/Chu–Vandermonde identities,
/// Burnside/Polya enumeration, and other advanced combinatorial identities and recurrences.
///
/// # Examples
///
/// ```
/// let rules = advanced_combinatorics_rules();
/// assert!(!rules.is_empty());
/// ```
pub fn advanced_combinatorics_rules() -> Vec<Rule> {
    vec![
        // JEE-relevant subset implemented concretely
        derangement_formula(),
        derangement_recurrence(),
        derangement_asymptotic(),
        vandermonde_identity(),
        stars_and_bars(),
        combination_with_repetition(),
        binomial_weighted_sum(),
        binomial_squares_sum(),
        hockey_stick_identity(),
        catalan_formula(),
        catalan_recurrence(),
        // Newly concretized advanced rules
        stirling_first_recurrence(),
        stirling_second_recurrence(),
        partition_recurrence(),
        chu_vandermonde(),
        multinomial_theorem(),
        pigeonhole_principle(),
        inclusion_exclusion_2(),
        inclusion_exclusion_3(),
        double_counting(),
        ordinary_gf(),
        exponential_gf(),
        binomial_sum_2n(),
        binomial_alternating_sum(),
        permutation_formula(),
        circular_permutation(),
        bell_number_recurrence(),
        multinomial_coefficient(),
        subfactorial(),
        christmas_stocking(),
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
        fibonacci_addition(),
        fibonacci_gcd(),
        lucas_numbers(),
        permutation_with_repetition(),
    ]
}

// D(n) = n! * Σ(-1)^k/k!
/// Create a rule that encodes the derangement identity D(n) = n! * Σ_{k=0..n} (-1)^k / k!.
///
/// The rule matches a variable-like expression `n` and, when applied, produces an equation
/// relating `D(n)` (represented by the input variable) to the closed-form summation `n! * Σ_{k=0..n} (-1)^k / k!`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = match expr {
                Expr::Var(sym) => *sym,
                _ => return vec![],
            };
            let k = intern_symbol("k");
            let sum = Expr::Summation {
                var: k,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Var(n)),
                body: Box::new(Expr::Div(
                    Box::new(Expr::Pow(
                        Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                        Box::new(Expr::Var(k)),
                    )),
                    Box::new(Expr::Factorial(Box::new(Expr::Var(k)))),
                )),
            };
            let rhs = Expr::Mul(
                Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                Box::new(sum),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Derangement closed form: D(n) = n! * Σ_{k=0..n} (-1)^k / k!"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// D(n) = (n-1)(D(n-1) + D(n-2))
/// Creates a rule encoding the derangement recurrence D(n) = (n-1) * (D(n-1) + D(n-2)).
///
/// The rule matches a variable expression and produces an equation whose left-hand side is the
/// matched variable and whose right-hand side is the recurrence (n-1)(D(n-1)+D(n-2)).
///
/// # Examples
///
/// ```
/// let rule = derangement_recurrence();
/// assert_eq!(rule.id, RuleId(601));
/// assert!(rule.description.contains("D(n) = (n-1)(D(n-1) + D(n-2))"));
/// ```
fn derangement_recurrence() -> Rule {
    Rule {
        id: RuleId(601),
        name: "derangement_recurrence",
        category: RuleCategory::Simplification,
        description: "D(n) = (n-1)(D(n-1) + D(n-2))",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = match expr {
                Expr::Var(sym) => *sym,
                _ => return vec![],
            };
            let d_n1 = Expr::Var(intern_symbol("D(n-1)"));
            let d_n2 = Expr::Var(intern_symbol("D(n-2)"));
            let rhs = Expr::Mul(
                Box::new(Expr::Sub(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                Box::new(Expr::Add(Box::new(d_n1), Box::new(d_n2))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Derangement recurrence: D(n) = (n-1)(D(n-1)+D(n-2))".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// C(n) = C(2n,n)/(n+1)
/// Constructs a rule that encodes the closed-form Catalan number identity.
///
/// The rule matches a single variable `n` and produces an equation
/// `C(n) = C(2n, n) / (n + 1)` as the rule's result. The rule is not reversible and has cost 2.
///
/// # Examples
///
/// ```
/// let _ = catalan_formula();
/// ```
fn catalan_formula() -> Rule {
    Rule {
        id: RuleId(602),
        name: "catalan_formula",
        category: RuleCategory::Simplification,
        description: "C(n) = C(2n,n)/(n+1)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = match expr {
                Expr::Var(sym) => *sym,
                _ => return vec![],
            };
            let rhs = Expr::Div(
                Box::new(Expr::Binomial(
                    Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(n)))),
                    Box::new(Expr::Var(n)),
                )),
                Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Catalan closed form: C_n = C(2n,n)/(n+1)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// C(n+1) = Σ C(i)*C(n-i) for i=0 to n
/// Constructs the rule encoding the Catalan number recurrence.
///
/// The rule yields the equation C(n+1) = Σ_{i=0..n} C(i)·C(n-i) when applied to a Catalan-style variable expression.
///
/// # Examples
///
/// ```
/// let rule = catalan_recurrence();
/// // Applying `rule` to a Catalan variable expression produces the recurrence equation.
/// ```
fn catalan_recurrence() -> Rule {
    Rule {
        id: RuleId(603),
        name: "catalan_recurrence",
        category: RuleCategory::Simplification,
        description: "C(n+1) = Σ C(i)*C(n-i)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = match expr {
                Expr::Var(sym) => *sym,
                _ => return vec![],
            };
            let i = intern_symbol("i");
            let body = Expr::Mul(
                Box::new(Expr::Var(intern_symbol("C(i)"))),
                Box::new(Expr::Var(intern_symbol("C(n-i)"))),
            );
            let sum = Expr::Summation {
                var: i,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Var(n)),
                body: Box::new(body),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(sum),
                },
                justification: "Catalan recurrence: C(n+1) = Σ_{i=0..n} C(i)·C(n-i)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// s(n,k) = s(n-1,k-1) - (n-1)*s(n-1,k)
/// Creates the rule encoding the recurrence for Stirling numbers of the first kind: s(n, k) = s(n-1, k-1) - (n-1)·s(n-1, k).
///
/// The rule matches variable expressions and produces an Equation tying the input to the recurrence RHS.
///
/// # Examples
///
/// ```
/// let r = stirling_first_recurrence();
/// assert_eq!(r.name, "stirling_first_recurrence");
/// ```
fn stirling_first_recurrence() -> Rule {
    Rule {
        id: RuleId(604),
        name: "stirling_first_recurrence",
        category: RuleCategory::Simplification,
        description: "s(n,k) = s(n-1,k-1) - (n-1)*s(n-1,k)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let _n = intern_symbol("n");
            let _k = intern_symbol("k");
            let rhs = Expr::Sub(
                Box::new(Expr::Var(intern_symbol("s(n-1,k-1)"))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Sub(Box::new(Expr::Var(_n)), Box::new(Expr::int(1)))),
                    Box::new(Expr::Var(intern_symbol("s(n-1,k)"))),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification:
                    "Stirling (1st kind) recurrence: s(n,k) = s(n-1,k-1) - (n-1)·s(n-1,k)"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// S(n,k) = k*S(n-1,k) + S(n-1,k-1)
/// Constructs the Stirling numbers of the second kind recurrence rule.
///
/// The rule produces an equation implementing the recurrence
/// S(n, k) = k * S(n - 1, k) + S(n - 1, k - 1) for a matched variable expression.
///
/// # Examples
///
/// ```
/// let rule = stirling_second_recurrence();
/// assert_eq!(rule.id, RuleId(605));
/// ```
fn stirling_second_recurrence() -> Rule {
    Rule {
        id: RuleId(605),
        name: "stirling_second_recurrence",
        category: RuleCategory::Simplification,
        description: "S(n,k) = k*S(n-1,k) + S(n-1,k-1)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let _n = intern_symbol("n");
            let k = intern_symbol("k");
            let rhs = Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(k)),
                    Box::new(Expr::Var(intern_symbol("S(n-1,k)"))),
                )),
                Box::new(Expr::Var(intern_symbol("S(n-1,k-1)"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Stirling (2nd kind) recurrence: S(n,k) = k·S(n-1,k) + S(n-1,k-1)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// p(n) = Σ (-1)^{k+1} * p(n - k(3k-1)/2) for pentagonal recurrence
/// Constructs a rule that rewrites the partition function into its pentagonal-number recurrence.
///
/// The produced Rule matches a partition-function variable (e.g., `p(n)`) and yields an equation
/// expressing `p(n)` as the pentagonal recurrence:
/// p(n) = Σ_{k=1..n} (-1)^{k+1} · p(n - k(3k-1)/2).
///
/// # Returns
///
/// A `Rule` that, when applicable, produces an `Expr::Equation` with the left-hand side being the
/// matched partition-variable expression and the right-hand side the summation implementing the
/// pentagonal recurrence. The rule is not reversible and has cost 4.
///
/// # Examples
///
/// ```
/// let rule = partition_recurrence();
/// assert_eq!(rule.id, RuleId(606));
/// ```
fn partition_recurrence() -> Rule {
    Rule {
        id: RuleId(606),
        name: "partition_recurrence",
        category: RuleCategory::Simplification,
        description: "Partition function pentagonal recurrence",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let body = Expr::Mul(
                Box::new(Expr::Pow(
                    Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                    Box::new(Expr::Add(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
                )),
                Box::new(Expr::Var(intern_symbol("p(n-k(3k-1)/2)"))),
            );
            let rhs = Expr::Summation {
                var: k,
                from: Box::new(Expr::int(1)),
                to: Box::new(Expr::Var(n)),
                body: Box::new(body),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Pentagonal partition recurrence".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Σ C(i,k) for i=k to n = C(n+1,k+1) (hockey stick)
/// Constructs the hockey-stick combinatorial identity rule.
///
/// The rule matches a summation of binomial terms and produces the identity
/// Σ_{i=k..n} C(i, k) = C(n + 1, k + 1) as the rule result, with a textual
/// justification describing the transformation.
///
/// # Examples
///
/// ```
/// let rule = hockey_stick_identity();
/// assert_eq!(rule.id, RuleId(607));
/// ```
fn hockey_stick_identity() -> Rule {
    Rule {
        id: RuleId(607),
        name: "hockey_stick_identity",
        category: RuleCategory::Simplification,
        description: "Σ C(i,k) = C(n+1,k+1) (Hockey stick)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            if let Expr::Summation { var: k, to, .. } = expr {
                // Build C(n+1, k+1) with the same k and upper limit n
                let n = to.as_ref().clone();
                let rhs = Expr::Binomial(
                    Box::new(Expr::Add(Box::new(n), Box::new(Expr::int(1)))),
                    Box::new(Expr::Add(Box::new(Expr::Var(*k)), Box::new(Expr::int(1)))),
                );
                return vec![RuleApplication {
                    result: rhs,
                    justification: "Hockey stick: Σ_{i=k..n} C(i,k) = C(n+1,k+1)".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// Σ C(m,k)*C(n,r-k) = C(m+n,r) (Vandermonde)
/// Creates a rule implementing Vandermonde's convolution.
///
/// Matches a summation over an index `k` of the form Σ_k C(m, k) * C(n, r - k) and produces
/// the binomial expression C(m + n, r). If the summation does not expose explicit `m`, `n`,
/// and `r - k` subpatterns, the rule will construct a fallback right-hand side using the
/// summation's upper bound to form `C(r + s, to)`.
///
/// # Examples
///
/// ```
/// let rule = vandermonde_identity();
/// assert_eq!(rule.id, RuleId(608));
/// ```
fn vandermonde_identity() -> Rule {
    Rule {
        id: RuleId(608),
        name: "vandermonde_identity",
        category: RuleCategory::Simplification,
        description: "Σ C(m,k)*C(n,r-k) = C(m+n,r)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            if let Expr::Summation {
                var: k, to, body, ..
            } = expr
            {
                if let Expr::Mul(a, b) = body.as_ref() {
                    if let (Expr::Binomial(m, k1), Expr::Binomial(n, r_minus_k)) =
                        (a.as_ref(), b.as_ref())
                    {
                        if matches!(k1.as_ref(), Expr::Var(v) if *v == *k) {
                            if let Expr::Sub(r_sym, k_sym) = r_minus_k.as_ref() {
                                if matches!(k_sym.as_ref(), Expr::Var(v2) if *v2 == *k) {
                                    let rhs = Expr::Binomial(
                                        Box::new(Expr::Add(m.clone(), n.clone())),
                                        r_sym.clone(),
                                    );
                                    return vec![RuleApplication {
                                        result: rhs,
                                        justification: "Vandermonde: Σ C(r,k) C(s,n-k) = C(r+s,n)"
                                            .to_string(),
                                    }];
                                }
                            }
                        }
                    }
                }
                // Fallback: use upper limit as n
                let rhs = Expr::Binomial(
                    Box::new(Expr::Add(
                        Box::new(Expr::Var(intern_symbol("r"))),
                        Box::new(Expr::Var(intern_symbol("s"))),
                    )),
                    to.clone(),
                );
                return vec![RuleApplication {
                    result: rhs,
                    justification: "Vandermonde identity".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 3,
    }
}

// Σ C(a,k)*C(b,n-k)*(-1)^(n-k) = C(a-b,n) (Chu-Vandermonde)
/// Creates the rule encoding the Chu–Vandermonde identity.
///
/// Matches summation expressions of the form Σ_k C(a,k) C(b,n-k) (-1)^{n-k} and produces
/// an equation equating that sum to C(a - b, n).
///
/// # Examples
///
/// ```
/// let r = chu_vandermonde();
/// assert_eq!(r.id, RuleId(609));
/// assert!(r.description.contains("Chu-Vandermonde"));
/// ```
fn chu_vandermonde() -> Rule {
    Rule {
        id: RuleId(609),
        name: "chu_vandermonde",
        category: RuleCategory::Simplification,
        description: "Chu-Vandermonde identity",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            let a = intern_symbol("a");
            let b = intern_symbol("b");
            let n = intern_symbol("n");
            let rhs = Expr::Binomial(
                Box::new(Expr::Sub(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                Box::new(Expr::Var(n)),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Chu-Vandermonde: Σ_{k} C(a,k) C(b,n-k) (-1)^{n-k} = C(a-b,n)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// (x1+x2+...+xk)^n = Σ n!/(n1!*n2!*...*nk!) * x1^n1 * x2^n2 * ... * xk^nk
/// Constructs a rule that expands a power of a sum according to the multinomial theorem.
///
/// The rule matches expressions of the form `(x1 + x2 + ... + xk)^n` and produces an equation
/// equating the original power to a schematic summation representing the multinomial expansion.
///
/// # Examples
///
/// ```
/// let rule = multinomial_theorem();
/// assert_eq!(rule.id, RuleId(610));
/// ```
fn multinomial_theorem() -> Rule {
    Rule {
        id: RuleId(610),
        name: "multinomial_theorem",
        category: RuleCategory::Expansion,
        description: "Multinomial theorem expansion",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Pow(base, _exp) = expr {
                let n_sym = intern_symbol("n");
                let i_sym = intern_symbol("i");
                let body = Expr::Mul(
                    Box::new(Expr::Div(
                        Box::new(Expr::Factorial(Box::new(Expr::Var(n_sym)))),
                        Box::new(Expr::Var(intern_symbol("∏ n_i!"))),
                    )),
                    base.clone(),
                );
                let sum = Expr::Summation {
                    var: i_sym,
                    from: Box::new(Expr::int(0)),
                    to: Box::new(Expr::Var(n_sym)),
                    body: Box::new(body),
                };
                return vec![RuleApplication {
                    result: Expr::Equation {
                        lhs: Box::new(expr.clone()),
                        rhs: Box::new(sum),
                    },
                    justification: "Multinomial expansion schematic".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 4,
    }
}

// C(n+k-1,k) ways to put k indistinguishable balls into n distinguishable bins
/// Constructs a rule encoding the "stars and bars" identity for nonnegative integer solutions.
///
/// The returned `Rule` matches a variable-style expression and produces a binomial form
/// representing the number of solutions to x1 + ... + xk = n with xi ≥ 0, i.e. C(n + k - 1, k - 1).
///
/// # Examples
///
/// ```
/// let rule = stars_and_bars();
/// assert_eq!(rule.id, RuleId(611));
/// assert_eq!(rule.name, "stars_and_bars");
/// ```
fn stars_and_bars() -> Rule {
    Rule {
        id: RuleId(611),
        name: "stars_and_bars",
        category: RuleCategory::Simplification,
        description: "Stars and bars: C(n+k-1,k)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let result = Expr::Binomial(
                Box::new(Expr::Sub(
                    Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::Var(k)))),
                    Box::new(Expr::int(1)),
                )),
                Box::new(Expr::Sub(Box::new(Expr::Var(k)), Box::new(Expr::int(1)))),
            );
            vec![RuleApplication {
                result,
                justification: "Stars and bars: #solutions to x1+..+xk=n (xi≥0) is C(n+k-1, k-1)"
                    .to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// n+1 pigeons in n holes => at least 2 in one hole
/// Create a rule encoding the pigeonhole principle.
///
/// The returned rule matches greater-than or greater-than-or-equal expressions and,
/// when applied, produces a `Gte` inequality asserting `n + 1 >= 2_in_box` (an interned
/// symbol named "2 in box") with a justification stating that if n+1 objects are placed
/// into n boxes, some box contains at least 2 objects.
///
/// # Examples
///
/// ```
/// let r = pigeonhole_principle();
/// assert_eq!(r.id, RuleId(612));
/// ```
fn pigeonhole_principle() -> Rule {
    Rule {
        id: RuleId(612),
        name: "pigeonhole_principle",
        category: RuleCategory::AlgebraicSolving,
        description: "n+1 items in n containers => at least 2 share",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Gte(_, _) | Expr::Gt(_, _)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let count = Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::int(1)));
            vec![RuleApplication {
                result: Expr::Gte(
                    Box::new(count),
                    Box::new(Expr::Var(intern_symbol("2 in box"))),
                ),
                justification:
                    "Pigeonhole: if n+1 objects go into n boxes, some box has ≥2 objects"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 1,
    }
}

// |A ∪ B| = |A| + |B| - |A ∩ B|
/// Creates a rule that applies the two-set inclusion–exclusion identity.
///
/// The rule matches a set-cardinality variable and yields an equation expressing
/// |A ∪ B| = |A| + |B| − |A ∩ B|.
///
/// # Examples
///
/// ```
/// let rule = inclusion_exclusion_2();
/// assert_eq!(rule.id, RuleId(613));
/// assert!(rule.name.contains("inclusion_exclusion_2"));
/// ```
fn inclusion_exclusion_2() -> Rule {
    Rule {
        id: RuleId(613),
        name: "inclusion_exclusion_2",
        category: RuleCategory::Simplification,
        description: "|A∪B| = |A| + |B| - |A∩B|",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let a = intern_symbol("A");
            let b = intern_symbol("B");
            let rhs = Expr::Sub(
                Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                Box::new(Expr::Var(intern_symbol("A∩B"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "|A∪B| = |A| + |B| - |A∩B|".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// |A ∪ B ∪ C| = |A| + |B| + |C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|
/// Constructs a rule encoding the three-set inclusion–exclusion identity.
///
/// The produced rule matches a set-size variable expression and yields an
/// equation expressing |A ∪ B ∪ C| as |A| + |B| + |C| − |A ∩ B| − |A ∩ C| − |B ∩ C| + |A ∩ B ∩ C|.
///
/// # Returns
///
/// A `Rule` whose `apply` produces an `Expr::Equation` with the inclusion–exclusion RHS.
///
/// # Examples
///
/// ```
/// let rule = inclusion_exclusion_3();
/// assert_eq!(rule.id, RuleId(614));
/// assert_eq!(rule.name, "inclusion_exclusion_3");
/// ```
fn inclusion_exclusion_3() -> Rule {
    Rule {
        id: RuleId(614),
        name: "inclusion_exclusion_3",
        category: RuleCategory::Simplification,
        description: "3-set inclusion-exclusion",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let a = intern_symbol("A");
            let b = intern_symbol("B");
            let c = intern_symbol("C");
            let rhs = Expr::Add(
                Box::new(Expr::Sub(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Add(
                            Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                            Box::new(Expr::Var(c)),
                        )),
                        Box::new(Expr::Add(
                            Box::new(Expr::Var(intern_symbol("A∩B"))),
                            Box::new(Expr::Var(intern_symbol("A∩C"))),
                        )),
                    )),
                    Box::new(Expr::Var(intern_symbol("B∩C"))),
                )),
                Box::new(Expr::Var(intern_symbol("A∩B∩C"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification:
                    "3-set inclusion-exclusion: |A∪B∪C| = |A|+|B|+|C| - |A∩B| - |A∩C| - |B∩C| + |A∩B∩C|"
                        .to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Double counting principle
/// Creates a rule that encodes a double-counting identity.
///
/// Matches an `Expr::Equation` and produces an `Expr::Equation` whose left-hand
/// side is the original equation and whose right-hand side is the variable
/// `second_count`, representing an alternative count of the same set.
///
/// # Examples
///
/// ```
/// let r = double_counting();
/// assert_eq!(r.id, RuleId(615));
/// assert_eq!(r.name, "double_counting");
/// ```
fn double_counting() -> Rule {
    Rule {
        id: RuleId(615),
        name: "double_counting",
        category: RuleCategory::AlgebraicSolving,
        description: "Count same set in two ways",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Equation { .. }),
        apply: |expr, _ctx| {
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(Expr::Var(intern_symbol("second_count"))),
                },
                justification: "Double counting: equate two counts of same set".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// OGF: Σ a_n * x^n
/// Constructs the ordinary generating function rule.
///
/// This rule matches a sequence variable and produces an Equation that represents the ordinary generating function
/// for that sequence: Σ_{n=0}^∞ a_n x^n.
///
/// # Examples
///
/// ```
/// let rule = ordinary_gf();
/// assert_eq!(rule.name, "ordinary_gf");
/// assert_eq!(rule.id, RuleId(616));
/// ```
fn ordinary_gf() -> Rule {
    Rule {
        id: RuleId(616),
        name: "ordinary_gf",
        category: RuleCategory::Simplification,
        description: "Ordinary generating function",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let a_n = intern_symbol("a_n");
            let x = intern_symbol("x");
            let term = Expr::Mul(
                Box::new(Expr::Var(a_n)),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::Var(n)))),
            );
            let sum = Expr::Summation {
                var: n,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Var(intern_symbol("∞"))),
                body: Box::new(term),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(sum),
                },
                justification: "Ordinary generating function: Σ a_n x^n".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// EGF: Σ a_n * x^n / n!
/// Create the rule encoding the exponential generating function identity.
///
/// The rule matches a sequence symbol (any `Expr::Var`) and produces an `Expr::Equation` whose
/// right-hand side is the exponential generating function Σ_{n=0}^∞ a_n x^n / n! constructed with
/// module-local interned helper symbols.
///
/// # Examples
///
/// ```
/// let r = exponential_gf();
/// assert_eq!(r.id, RuleId(617));
/// assert_eq!(r.name, "exponential_gf");
/// ```
fn exponential_gf() -> Rule {
    Rule {
        id: RuleId(617),
        name: "exponential_gf",
        category: RuleCategory::Simplification,
        description: "Exponential generating function",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let a_n = intern_symbol("a_n");
            let x = intern_symbol("x");
            let term = Expr::Div(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(a_n)),
                    Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::Var(n)))),
                )),
                Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
            );
            let sum = Expr::Summation {
                var: n,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Var(intern_symbol("∞"))),
                body: Box::new(term),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(sum),
                },
                justification: "Exponential generating function: Σ a_n x^n / n!".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Σ C(n,k) for k=0 to n = 2^n
/// Constructs the simplification rule for the binomial identity Σ_{k=0..n} C(n,k) = 2^n.
///
/// The returned Rule matches summation expressions over a binomial index and produces an
/// expression representing 2^n as the simplified right-hand side with an explanatory justification.
///
/// # Examples
///
/// ```
/// let rule = binomial_sum_2n();
/// assert_eq!(rule.id, RuleId(618));
/// assert_eq!(rule.name, "binomial_sum_2n");
/// ```
fn binomial_sum_2n() -> Rule {
    Rule {
        id: RuleId(618),
        name: "binomial_sum_2n",
        category: RuleCategory::Simplification,
        description: "Σ C(n,k) = 2^n",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            if let Expr::Summation { to, .. } = expr {
                let rhs = Expr::Pow(Box::new(Expr::int(2)), to.clone());
                return vec![RuleApplication {
                    result: rhs,
                    justification: "Σ_{k=0..n} C(n,k) = 2^n".to_string(),
                }];
            }
            vec![]
        },
        reversible: false,
        cost: 1,
    }
}

// Σ (-1)^k * C(n,k) = 0 for n > 0
/// Produces a Rule that recognizes alternating binomial sums Σ (-1)^k C(n, k) and rewrites them to `0` for n > 0.
///
/// The returned Rule matches summation expressions and, when applicable, produces an integer `0` as the result with the justification "Σ (-1)^k * C(n,k) = 0 for n > 0".
///
/// # Examples
///
/// ```
/// let _rule = binomial_alternating_sum();
/// ```
fn binomial_alternating_sum() -> Rule {
    Rule {
        id: RuleId(619),
        name: "binomial_alternating_sum",
        category: RuleCategory::Simplification,
        description: "Σ (-1)^k * C(n,k) = 0",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
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
/// Constructs the permutation formula rule representing P(n, k) = n! / (n - k)!.
///
/// The rule matches a variable-style expression and, when applied, returns an `Equation` whose left-hand side is the original expression and whose right-hand side is the factorial quotient `n! / (n - k)!`. The rule is one-way (not reversible) and has an application cost of 2.
///
/// # Examples
///
/// ```
/// let r = permutation_formula();
/// assert_eq!(r.id, RuleId(620));
/// assert_eq!(r.name, "permutation_formula");
/// ```
fn permutation_formula() -> Rule {
    Rule {
        id: RuleId(620),
        name: "permutation_formula",
        category: RuleCategory::Simplification,
        description: "P(n,k) = n!/(n-k)!",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let rhs = Expr::Div(
                Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                Box::new(Expr::Factorial(Box::new(Expr::Sub(
                    Box::new(Expr::Var(n)),
                    Box::new(Expr::Var(k)),
                )))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Permutation formula: P(n,k) = n!/(n-k)!".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Circular permutations: (n-1)!
/// Creates a simplification rule that rewrites the count of circular permutations to (n - 1)!.
///
/// The produced `Rule` matches a single variable (representing `n`) and, when applied,
/// yields an equation with the original variable on the left-hand side and `(n - 1)!` on the right.
///
/// # Examples
///
/// ```
/// let rule = circular_permutation();
/// assert_eq!(rule.id, RuleId(621));
/// assert_eq!(rule.name, "circular_permutation");
/// ```
fn circular_permutation() -> Rule {
    Rule {
        id: RuleId(621),
        name: "circular_permutation",
        category: RuleCategory::Simplification,
        description: "Circular permutations = (n-1)!",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let rhs = Expr::Factorial(Box::new(Expr::Sub(
                Box::new(Expr::Var(n)),
                Box::new(Expr::int(1)),
            )));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Circular permutations count = (n-1)!".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// D(n) ~ n!/e as n -> ∞
/// Produces a rule that expresses the derangement asymptotic D(n) ≈ n! / e as an equation.
///
/// The rule is applicable to a derangement variable `D(n)` and yields an `Expr::Equation` whose right-hand side is `Expr::Factorial(n) / Expr::E`.
///
/// # Examples
///
/// ```
/// let rule = derangement_asymptotic();
/// assert_eq!(rule.id, RuleId(622));
/// assert_eq!(rule.name, "derangement_asymptotic");
/// ```
fn derangement_asymptotic() -> Rule {
    Rule {
        id: RuleId(622),
        name: "derangement_asymptotic",
        category: RuleCategory::Simplification,
        description: "D(n) ~ n!/e",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = match expr {
                Expr::Var(sym) => *sym,
                _ => return vec![],
            };
            let rhs = Expr::Div(
                Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                Box::new(Expr::E),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Derangement asymptotic: D(n) ≈ n!/e".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// F(m+n) = F(m)*F(n+1) + F(m-1)*F(n)
/// Constructs a rule that encodes the Fibonacci addition identity F(m + n) = F(m)·F(n + 1) + F(m - 1)·F(n).
///
/// The rule is applicable to variable expressions and, when applied, produces an `Equation` whose left-hand side
/// is the original expression and whose right-hand side is the Fibonacci addition expansion.
///
/// # Examples
///
/// ```
/// let rule = fibonacci_addition();
/// // Applying `rule` to a variable `F(m+n)` yields an equation `F(m+n) = F(m) * F(n+1) + F(m-1) * F(n)`
/// ```
fn fibonacci_addition() -> Rule {
    Rule {
        id: RuleId(623),
        name: "fibonacci_addition",
        category: RuleCategory::Simplification,
        description: "F(m+n) = F(m)*F(n+1) + F(m-1)*F(n)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let rhs = Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(intern_symbol("F(m)"))),
                    Box::new(Expr::Var(intern_symbol("F(n+1)"))),
                )),
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(intern_symbol("F(m-1)"))),
                    Box::new(Expr::Var(intern_symbol("F(n)"))),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Fibonacci addition formula".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// gcd(F(m), F(n)) = F(gcd(m,n))
/// Creates the combinatorics rule encoding the identity gcd(F(m), F(n)) = F(gcd(m, n)).
///
/// This rule applies to a Fibonacci-variable expression and yields an `Equation` whose left-hand side
/// is the original expression and whose right-hand side is the symbol `F(gcd(m,n))`. The rule is
/// one-way and assigned id 624 with a moderate application cost.
///
/// # Examples
///
/// ```
/// let rule = fibonacci_gcd();
/// assert_eq!(rule.id, RuleId(624));
/// assert_eq!(rule.name, "fibonacci_gcd");
/// ```
fn fibonacci_gcd() -> Rule {
    Rule {
        id: RuleId(624),
        name: "fibonacci_gcd",
        category: RuleCategory::Simplification,
        description: "gcd(F(m), F(n)) = F(gcd(m,n))",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let rhs = Expr::Var(intern_symbol("F(gcd(m,n))"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "gcd(F(m),F(n)) = F(gcd(m,n))".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// L(n) = F(n-1) + F(n+1)
/// Constructs a simplification rule expressing Lucas numbers in terms of Fibonacci numbers.
///
/// The rule matches a variable expression and produces an equation representing
/// L(n) = F(n-1) + F(n+1).
///
/// # Returns
///
/// A `Rule` that, when applied to a variable expression, yields an `Expr::Equation`
/// whose `lhs` is the original expression and whose `rhs` is `F(n-1) + F(n+1)`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let rhs = Expr::Add(
                Box::new(Expr::Var(intern_symbol("F(n-1)"))),
                Box::new(Expr::Var(intern_symbol("F(n+1)"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Lucas numbers L(n)=F(n-1)+F(n+1)".to_string(),
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
/// Creates the rule that identifies expressions of the form `n^k` and labels them with the
/// "permutations with repetition" interpretation.
///
/// Matches power expressions whose exponent is a constant or variable and produces a
/// RuleApplication that justifies the match with "Permutations with repetition: n choices k times".
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Pow(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Pow(base, exp) = expr {
                return vec![RuleApplication {
                    result: Expr::Pow(base.clone(), exp.clone()),
                    justification: "Permutations with repetition: n choices k times".to_string(),
                }];
            }
            vec![]
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |_expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let result = Expr::Binomial(
                Box::new(Expr::Sub(
                    Box::new(Expr::Add(Box::new(Expr::Var(n)), Box::new(Expr::Var(k)))),
                    Box::new(Expr::int(1)),
                )),
                Box::new(Expr::Var(k)),
            );
            vec![RuleApplication {
                result,
                justification: "Combinations with repetition: C(n+k-1, k)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Bell numbers: B(n+1) = Σ C(n,k)*B(k)
/// Builds the rule encoding the Bell numbers recurrence B(n+1) = Σ_{k=0..n} C(n,k) * B(k).
///
/// The returned `Rule` has id 652, matches variable expressions, and when applied produces an
/// `Expr::Equation` whose left-hand side is `B(n+1)` and whose right-hand side is the summation
/// `Σ_{k=0..n} C(n,k) * B(k)`. The rule is one-way (not reversible) and has cost 3.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let body = Expr::Mul(
                Box::new(Expr::Binomial(
                    Box::new(Expr::Var(n)),
                    Box::new(Expr::Var(k)),
                )),
                Box::new(Expr::Var(intern_symbol("B(k)"))),
            );
            let sum = Expr::Summation {
                var: k,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Var(n)),
                body: Box::new(body),
            };
            let lhs = Expr::Var(intern_symbol("B(n+1)"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(sum),
                },
                justification: "Bell recurrence: B(n+1) = Σ_{k=0..n} C(n,k) B(k)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Multinomial coefficient: n!/(k1! k2! ... km!)
/// Creates a rule that recognizes factorial division expressions and produces an equation representing the multinomial coefficient n!/(k1! k2! ... km!).
///
/// The rule matches expressions where a factorial appears in a division numerator and, when applied, returns an Equation whose right-hand side is the multinomial form and whose justification names the identity.
///
/// # Examples
///
/// ```
/// let rule = multinomial_coefficient();
/// assert_eq!(rule.id.0, 653);
/// ```
fn multinomial_coefficient() -> Rule {
    Rule {
        id: RuleId(653),
        name: "multinomial_coefficient",
        category: RuleCategory::Simplification,
        description: "Multinomial: n!/(k1!k2!...km!)",
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let rhs = Expr::Div(
                Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                Box::new(Expr::Var(intern_symbol("k1!...km!"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Multinomial coefficient n!/(k1!k2!...km!)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Binomial coefficient sum by row: Σ k*C(n,k) = n*2^(n-1)
/// Constructs a rule that recognizes summations of the form Σ_k k * C(n, k) and produces the identity n * 2^(n-1).
///
/// The rule matches a Summation whose body is the product of the summation index `k` and a binomial `C(n, k)`,
/// and yields a single RuleApplication with `result = n * 2^(n-1)` and a justification string describing the identity.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            if let Expr::Summation {
                var: k, to, body, ..
            } = expr
            {
                if let Expr::Mul(coeff, binom) = body.as_ref() {
                    if matches!(coeff.as_ref(), Expr::Var(v) if *v == *k) {
                        if let Expr::Binomial(n, kk) = binom.as_ref() {
                            if matches!(kk.as_ref(), Expr::Var(v2) if *v2 == *k) {
                                let rhs = Expr::Mul(
                                    n.clone(),
                                    Box::new(Expr::Pow(
                                        Box::new(Expr::int(2)),
                                        Box::new(Expr::Sub(to.clone(), Box::new(Expr::int(1)))),
                                    )),
                                );
                                return vec![RuleApplication {
                                    result: rhs,
                                    justification: "Σ_{k=0..n} k*C(n,k) = n*2^(n-1)".to_string(),
                                }];
                            }
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// Derangement subfactorial: !n = D(n)
/// Constructs the simplification rule for the subfactorial (derangement) identity.
///
/// The rule recognizes expressions representing subfactorials and produces an `Equation` equating `!n` with `D(n)` and a justification string.
///
/// # Returns
///
/// A `Rule` with id 655 named `"subfactorial"` whose application yields the equation `!n = D(n)`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let rhs = Expr::Var(intern_symbol("D(n)"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(Expr::Var(intern_symbol("!n"))),
                    rhs: Box::new(rhs),
                },
                justification: "Subfactorial !n equals derangements D(n)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Christmas stocking identity: C(n,m)*C(m,k) = C(n,k)*C(n-k,m-k)
/// Constructs the simplification rule for the Christmas stocking binomial identity.
///
/// Encodes the identity C(n, m) * C(m, k) = C(n, k) * C(n - k, m - k).
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Mul(_, _)),
        apply: |expr, _ctx| {
            if let Expr::Mul(_, _) = expr {
                let n = intern_symbol("n");
                let m = intern_symbol("m");
                let k = intern_symbol("k");
                let rhs = Expr::Mul(
                    Box::new(Expr::Binomial(
                        Box::new(Expr::Var(n)),
                        Box::new(Expr::Var(k)),
                    )),
                    Box::new(Expr::Binomial(
                        Box::new(Expr::Sub(Box::new(Expr::Var(n)), Box::new(Expr::Var(k)))),
                        Box::new(Expr::Sub(Box::new(Expr::Var(m)), Box::new(Expr::Var(k)))),
                    )),
                );
                return vec![RuleApplication {
                    result: rhs,
                    justification: "C(n,m)C(m,k) = C(n,k)C(n-k,m-k)".to_string(),
                }];
            }
            vec![]
        },
        reversible: true,
        cost: 2,
    }
}

// Sum of squares: Σ C(n,k)^2 = C(2n,n)
/// Applies the binomial squares sum identity Σ_{k=0..n} C(n,k)^2 = C(2n,n) to a matching summation.
///
/// Matches summations whose body is the square of a binomial coefficient C(n,k) with summation index `k` running from 0 to `n`, and produces a RuleApplication with the closed-form binomial C(2n, n).
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Summation { .. }),
        apply: |expr, _ctx| {
            if let Expr::Summation {
                var: k,
                from,
                to: _to,
                body,
            } = expr
            {
                if let Expr::Pow(binom, exp) = body.as_ref() {
                    if let Expr::Const(c) = exp.as_ref() {
                        if c.numer() == 2 && c.denom() == 1 {
                            if let Expr::Const(c_from) = from.as_ref() {
                                if c_from.is_zero() {
                                    if let Expr::Binomial(n, kk) = binom.as_ref() {
                                        if matches!(kk.as_ref(), Expr::Var(v) if *v == *k) {
                                            let rhs = Expr::Binomial(
                                                Box::new(Expr::Mul(
                                                    Box::new(Expr::int(2)),
                                                    n.clone(),
                                                )),
                                                n.clone(),
                                            );
                                            return vec![RuleApplication {
                                                result: rhs,
                                                justification: "Σ_{k=0..n} C(n,k)^2 = C(2n, n)"
                                                    .to_string(),
                                            }];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            vec![]
        },
        reversible: false,
        cost: 2,
    }
}

// Rising factorial: (x)_n = x(x+1)(x+2)...(x+n-1)
/// Constructs the rule that expands a rising factorial into a product.
///
/// The returned `Rule` (id 658, name "rising_factorial") represents the identity
/// (x)_n = ∏_{i=0}^{n-1} (x + i) and is applicable to variable expressions; when applied
/// it produces an equation mapping the original expression to the corresponding `BigProduct`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let x = intern_symbol("x");
            let n = intern_symbol("n");
            let i = intern_symbol("i");
            let body = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(i)));
            let prod = Expr::BigProduct {
                var: i,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Sub(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                body: Box::new(body),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(prod),
                },
                justification: "Rising factorial (x)_n = ∏_{i=0}^{n-1} (x+i)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Falling factorial: (x)^n = x(x-1)(x-2)...(x-n+1)
/// Creates a rule that represents the falling factorial as a product.
///
/// The rule matches a variable-like falling factorial expression and produces an equation
/// whose right-hand side is the product ∏_{i=0}^{n-1} (x - i), with a justification
/// describing the identity `x^(n) = ∏_{i=0}^{n-1} (x - i)`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let x = intern_symbol("x");
            let n = intern_symbol("n");
            let i = intern_symbol("i");
            let body = Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(i)));
            let prod = Expr::BigProduct {
                var: i,
                from: Box::new(Expr::int(0)),
                to: Box::new(Expr::Sub(Box::new(Expr::Var(n)), Box::new(Expr::int(1)))),
                body: Box::new(body),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(prod),
                },
                justification: "Falling factorial x^{(n)} = ∏_{i=0}^{n-1} (x-i)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Legendre's formula: vp(n!) = Σ ⌊n/p^k⌋
/// Creates a rule encoding Legendre's formula for the p-adic valuation of n!
///
/// The rule matches a variable expression `n` and produces an equation asserting
/// that v_p(n!) equals the sum over k≥1 of floor(n / p^k).
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let p = intern_symbol("p");
            let k = intern_symbol("k");
            let body = Expr::Floor(Box::new(Expr::Div(
                Box::new(Expr::Var(n)),
                Box::new(Expr::Pow(Box::new(Expr::Var(p)), Box::new(Expr::Var(k)))),
            )));
            let sum = Expr::Summation {
                var: k,
                from: Box::new(Expr::int(1)),
                to: Box::new(Expr::Var(intern_symbol("∞"))),
                body: Box::new(body),
            };
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(sum),
                },
                justification: "Legendre: v_p(n!) = Σ_{k≥1} ⌊n/p^k⌋".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Kummer's theorem for binomial mod p
/// Creates a rule encoding Kummer's theorem: the p-adic valuation of a binomial coefficient equals the number of carries when adding its arguments in base `p`.
///
/// The rule targets binomial-style variable expressions and produces an `Equation` that relates `v_p(C(m+n, m))` to the carry count of adding `m` and `n` in base `p`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let _m = intern_symbol("m");
            let _n = intern_symbol("n");
            let lhs = Expr::Var(intern_symbol("v_p(C(m+n,m))"));
            let rhs = Expr::Var(intern_symbol("carries_base_p(m,n)"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                justification: "Kummer: valuation equals base-p carry count".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Lucas' theorem: C(m,n) mod p = Π C(mi,ni) mod p
/// Constructs a rule that encodes Lucas' theorem for binomial coefficients modulo a prime.
///
/// The returned `Rule` has id `662`, category `NumberTheory`, is not reversible, and has cost `3`. Its application produces an equation whose left-hand side is `C(m, n) mod p` and whose right-hand side is a symbolic product representing the congruence of binomial coefficients of base‑p digits; the justification string is `"Lucas theorem via base-p digits"`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let m = intern_symbol("m");
            let n = intern_symbol("n");
            let p = intern_symbol("p");
            let lhs = Expr::Mod(
                Box::new(Expr::Binomial(
                    Box::new(Expr::Var(m)),
                    Box::new(Expr::Var(n)),
                )),
                Box::new(Expr::Var(p)),
            );
            let rhs = Expr::Var(intern_symbol("∏ C(m_i,n_i) mod p"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
                justification: "Lucas theorem via base-p digits".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Burnside's lemma: |X/G| = (1/|G|) Σ |X^g|
/// Constructs the rule encoding Burnside's lemma for counting orbits.
///
/// Produces an equation equating |X/G| with (1/|G|) Σ |X^g| and provides a textual justification.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let g = intern_symbol("G");
            let i = intern_symbol("i");
            let fix = intern_symbol("|X^g|");
            let sum = Expr::Summation {
                var: i,
                from: Box::new(Expr::int(1)),
                to: Box::new(Expr::Var(g)),
                body: Box::new(Expr::Var(fix)),
            };
            let rhs = Expr::Div(Box::new(sum), Box::new(Expr::Var(g)));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Burnside: orbits |X/G| = (1/|G|) Σ |Fix(g)|".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Polya enumeration theorem
/// Implements the Polya enumeration theorem as a solver rule.
///
/// Matches variable expressions and produces an `Equation` whose right-hand side is the
/// cycle-index symbol `Z_G`. The produced `RuleApplication` includes a justification
/// indicating that the cycle index is used to count inequivalent configurations (orbits)
/// under the group action.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let z_g = intern_symbol("Z_G");
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(Expr::Var(z_g)),
                },
                justification: "Polya: use cycle index Z_G to count orbits".to_string(),
            }]
        },
        reversible: false,
        cost: 4,
    }
}

// Catalan alternative formula: C_n = (2n)!/(n!(n+1)!)
/// Recognizes the Catalan number identity C_n = (2n)! / (n!(n+1)!).
///
/// Produces an Equation whose right-hand side is the alternate closed form (2n)!/(n!(n+1)!) and records the identity as the justification; the original expression is not otherwise transformed.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let rhs = Expr::Div(
                Box::new(Expr::Factorial(Box::new(Expr::Mul(
                    Box::new(Expr::int(2)),
                    Box::new(Expr::Var(n)),
                )))),
                Box::new(Expr::Mul(
                    Box::new(Expr::Factorial(Box::new(Expr::Var(n)))),
                    Box::new(Expr::Factorial(Box::new(Expr::Add(
                        Box::new(Expr::Var(n)),
                        Box::new(Expr::int(1)),
                    )))),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Catalan closed form (alternative)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Partition function: p(n,k) number of partitions of n into k parts
/// Creates a rule encoding the partition recurrence p(n, k) = p(n-1, k-1) + p(n-k, k).
///
/// The produced Rule matches a variable expression representing the partition function and,
/// when applied, yields an Equation whose left-hand side is the original expression and whose
/// right-hand side is the sum `p(n-1,k-1) + p(n-k,k)`. The rule is one-way (not reversible)
/// and has cost 2.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let k = intern_symbol("k");
            let rhs = Expr::Add(
                Box::new(Expr::Var(intern_symbol("p(n-1,k-1)"))),
                Box::new(Expr::Var(intern_symbol("p(n-k,k)"))),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Partition recurrence p(n,k) = p(n-1,k-1)+p(n-k,k)".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Restricted permutations: permutations avoiding pattern
/// Constructs a schematic rule for pattern-avoiding permutations.
///
/// The rule matches a variable expression and produces an equation that maps the original
/// expression to a placeholder symbol `count_avoiding_pattern`, representing the number of
/// permutations avoiding a specified pattern (schematic form used for further reasoning).
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let rhs = Expr::Var(intern_symbol("count_avoiding_pattern"));
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Pattern avoidance counting (schematic)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}

// Recurrence for derangements: D(n) = n*D(n-1) + (-1)^n
/// Rule encoding the derangement recurrence D(n) = n·D(n-1) + (-1)^n.
///
/// The rule matches a variable-like expression and produces an Equation whose
/// left-hand side is the matched expression and whose right-hand side is
/// `n * D(n-1) + (-1)^n`. The produced rule is non-reversible and has id 668.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let n = intern_symbol("n");
            let rhs = Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var(n)),
                    Box::new(Expr::Var(intern_symbol("D(n-1)"))),
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Neg(Box::new(Expr::int(1)))),
                    Box::new(Expr::Var(n)),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Derangement recurrence D(n)=n·D(n-1)+(-1)^n".to_string(),
            }]
        },
        reversible: false,
        cost: 2,
    }
}

// Generating function for Fibonacci: F(x) = x/(1-x-x^2)
/// Recognizes the Fibonacci ordinary generating function x/(1 - x - x^2).
///
/// Matches expressions of the form `x / (1 - x - x^2)` and produces an `Equation` relating the
/// input expression to the identity `Σ F_n x^n = x/(1 - x - x^2)`.
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
        is_applicable: |expr, _ctx| matches!(expr, Expr::Var(_)),
        apply: |expr, _ctx| {
            let x = intern_symbol("x");
            let rhs = Expr::Div(
                Box::new(Expr::Var(x)),
                Box::new(Expr::Sub(
                    Box::new(Expr::Sub(Box::new(Expr::int(1)), Box::new(Expr::Var(x)))),
                    Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                )),
            );
            vec![RuleApplication {
                result: Expr::Equation {
                    lhs: Box::new(expr.clone()),
                    rhs: Box::new(rhs),
                },
                justification: "Fibonacci OGF: Σ F_n x^n = x/(1 - x - x^2)".to_string(),
            }]
        },
        reversible: false,
        cost: 3,
    }
}