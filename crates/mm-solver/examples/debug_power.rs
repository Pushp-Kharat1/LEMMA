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

    // Test x^2 * x^3 * x^4 = (x^2 * x^3) * x^4
    let expr = Expr::Mul(
        Box::new(Expr::Mul(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
        )),
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(4)))),
    );

    println!("Input: x^2 * x^3 * x^4");
    println!("Structure: {:?}\n", expr);

    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for (i, step) in result.steps.iter().enumerate() {
        println!("  {}: {} -> {:?}", i + 1, step.rule_name, step.after);
    }
}
