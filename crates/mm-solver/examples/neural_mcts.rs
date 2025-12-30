//! Example: Neural MCTS solving.
//!
//! Demonstrates using neural-guided Monte Carlo Tree Search
//! to simplify expressions.

use candle_core::Device;
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    println!("=== Math Monster: Neural MCTS Demo ===\n");

    // Create MCTS with custom config
    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 50, // Reduced for demo
        exploration_weight: 1.41,
        max_depth: 10,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    println!("Created NeuralMCTS with 50 simulations\n");

    // Test 1: Constant folding
    println!("--- Test 1: Constant Folding ---");
    let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
    println!("Input: 2 + 3");

    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    println!("Verified: {}", result.verified);

    // Test 2: Identity elimination
    println!("\n--- Test 2: Identity Elimination ---");
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
    println!("Input: x + 0");

    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for (i, step) in result.steps.iter().enumerate() {
        println!(
            "  Step {}: {} - {}",
            i + 1,
            step.rule_name,
            step.justification
        );
    }

    // Test 3: Zero multiplication
    println!("\n--- Test 3: Zero Multiplication ---");
    let expr = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
    println!("Input: x * 0");

    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());

    // Test 4: More complex expression
    println!("\n--- Test 4: Nested Expression ---");
    let expr = Expr::Add(
        Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::int(4)))),
        Box::new(Expr::int(0)),
    );
    println!("Input: (3 * 4) + 0");

    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for (i, step) in result.steps.iter().enumerate() {
        println!(
            "  Step {}: {} - {}",
            i + 1,
            step.rule_name,
            step.justification
        );
    }

    println!("\n=== Done ===");
}
