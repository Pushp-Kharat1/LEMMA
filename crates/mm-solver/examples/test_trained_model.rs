// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Test the trained model with complex expressions

use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    println!("=== LEMMA: Complex Expression Testing ===\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    // Create MCTS with more simulations for complex cases
    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 100,
        exploration_weight: 1.41,
        max_depth: 15,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    println!("Created NeuralMCTS with 100 simulations, max_depth 15\n");

    // Test cases with increasing complexity
    let tests: Vec<(&str, Expr)> = vec![
        // Algebra - Identity rules
        (
            "(x + 0) * 1",
            Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
                Box::new(Expr::int(1)),
            ),
        ),
        (
            "x * 1 + 0",
            Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(1)))),
                Box::new(Expr::int(0)),
            ),
        ),
        (
            "x^1",
            Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(1))),
        ),
        (
            "x^0",
            Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        ),
        // Constant folding
        (
            "2 + 3",
            Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3))),
        ),
        (
            "2 * 3 + 4 * 5",
            Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::int(3)))),
                Box::new(Expr::Mul(Box::new(Expr::int(4)), Box::new(Expr::int(5)))),
            ),
        ),
        // Calculus - Derivatives
        (
            "d/dx(x)",
            Expr::Derivative {
                expr: Box::new(Expr::Var(x)),
                var: x,
            },
        ),
        (
            "d/dx(x^2)",
            Expr::Derivative {
                expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                var: x,
            },
        ),
        (
            "d/dx(x^3)",
            Expr::Derivative {
                expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
                var: x,
            },
        ),
        (
            "d/dx(5)",
            Expr::Derivative {
                expr: Box::new(Expr::int(5)),
                var: x,
            },
        ),
        (
            "d/dx(sin(x))",
            Expr::Derivative {
                expr: Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                var: x,
            },
        ),
        (
            "d/dx(cos(x))",
            Expr::Derivative {
                expr: Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                var: x,
            },
        ),
        // Trig identities
        (
            "sin(x)^2 + cos(x)^2",
            Expr::Add(
                Box::new(Expr::Pow(
                    Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2)),
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2)),
                )),
            ),
        ),
        // Multi-variable
        (
            "x + y",
            Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y))),
        ),
        (
            "x * y + 0",
            Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                Box::new(Expr::int(0)),
            ),
        ),
    ];

    let mut verified_count = 0;
    let mut transformed_count = 0;
    let total = tests.len();

    for (name, expr) in tests {
        println!("--- {} ---", name);

        let result = mcts.simplify(expr.clone());

        println!("  Input:  {:?}", expr);
        println!("  Output: {:?}", result.result);
        println!("  Steps:  {}", result.steps.len());

        for (i, step) in result.steps.iter().enumerate() {
            println!("    {}: {}", i + 1, step.rule_name);
        }

        if result.verified {
            println!("  ✅ Verified");
            verified_count += 1;
        } else if result.result != expr {
            println!("  ⚠️ Transformed (not verified)");
            transformed_count += 1;
        } else {
            println!("  ⏭️ No change");
        }
        println!();
    }

    println!("=== Results ===");
    println!("  Verified: {}/{}", verified_count, total);
    println!("  Transformed: {}/{}", transformed_count, total);
    println!(
        "  Total success: {}/{}",
        verified_count + transformed_count,
        total
    );
}
