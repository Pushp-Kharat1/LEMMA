use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200,
        exploration_weight: 1.41,
        max_depth: 30,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    // 2*(x+y) + 3*(x+y)
    let expr = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::int(2)),
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::int(3)),
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
        )),
    );

    println!("Input: 2*(x+y) + 3*(x+y)");
    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for (i, step) in result.steps.iter().enumerate() {
        println!("  {}: {} -> {:?}", i + 1, step.rule_name, step.after);
    }
}
