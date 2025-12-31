// Test what simplify() returns for derivatives
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    println!("=== Testing MCTS simplify on derivatives ===\n");

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

    // Test d/dx(5)
    let d_const = Expr::Derivative {
        expr: Box::new(Expr::int(5)),
        var: x,
    };

    println!("Test: d/dx(5)");
    println!("  Input: {:?}", d_const);
    let result = mcts.simplify(d_const);
    println!("  Output: {:?}", result.result);
    println!("  Steps: {}", result.steps.len());
    for step in &result.steps {
        println!(
            "    - {} : {:?} -> {:?}",
            step.rule_name, step.before, step.after
        );
    }
    println!("  Verified: {}", result.verified);

    // Test d/dx(x)
    let d_x = Expr::Derivative {
        expr: Box::new(Expr::Var(x)),
        var: x,
    };

    println!("\nTest: d/dx(x)");
    println!("  Input: {:?}", d_x);
    let result = mcts.simplify(d_x);
    println!("  Output: {:?}", result.result);
    println!("  Steps: {}", result.steps.len());
    for step in &result.steps {
        println!(
            "    - {} : {:?} -> {:?}",
            step.rule_name, step.before, step.after
        );
    }
    println!("  Verified: {}", result.verified);
}
