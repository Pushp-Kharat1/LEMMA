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
        simulations: 100,
        exploration_weight: 1.41,
        max_depth: 15,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    // Test x^0
    let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
    println!("Input: x^0");
    let result = mcts.simplify(expr);
    println!("Output: {:?}", result.result);
    println!("Steps: {}", result.steps.len());
    for step in &result.steps {
        println!("  - {} : {:?}", step.rule_name, step.after);
    }

    // Test d/dx(x^2)
    let expr2 = Expr::Derivative {
        expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        var: x,
    };
    println!("\nInput: d/dx(x^2)");
    let result2 = mcts.simplify(expr2);
    println!("Output: {:?}", result2.result);
    println!("Steps: {}", result2.steps.len());
    for step in &result2.steps {
        println!("  - {} : {:?}", step.rule_name, step.after);
    }
}
