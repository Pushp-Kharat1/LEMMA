// Test equation solving fix for x*2 + 60 = 100
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");

    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200,
        exploration_weight: 1.41,
        max_depth: 30,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    // x*2 + 60 = 100  should give x = 20
    println!("Test: x*2 + 60 = 100");
    let expr = Expr::Equation {
        lhs: Box::new(Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::int(60)),
        )),
        rhs: Box::new(Expr::int(100)),
    };

    let result = mcts.simplify(expr);
    println!("Result: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for (i, step) in result.steps.iter().enumerate() {
        println!("  {}: {} -> {:?}", i + 1, step.rule_name, step.after);
    }
}
