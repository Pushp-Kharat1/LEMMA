use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200,
        exploration_weight: 1.41,
        max_depth: 20,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    // Test 2*(3+4)
    let expr = Expr::Mul(
        Box::new(Expr::int(2)),
        Box::new(Expr::Add(Box::new(Expr::int(3)), Box::new(Expr::int(4)))),
    );

    println!("Input: 2*(3+4)");
    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    println!("\nStep details:");
    for (i, step) in result.steps.iter().enumerate() {
        println!("  {}: {} -> {:?}", i + 1, step.rule_name, step.after);
    }
}
