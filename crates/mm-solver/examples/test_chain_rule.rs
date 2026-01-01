// Test chain rule implementation
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

    println!("=== Chain Rule Tests ===\n");

    // Test 1: d/dx(sin(x^2)) should give cos(x^2) * 2x
    println!("Test 1: d/dx(sin(x^2))");
    let expr1 = Expr::Derivative {
        expr: Box::new(Expr::Sin(Box::new(Expr::Pow(
            Box::new(Expr::Var(x)),
            Box::new(Expr::int(2)),
        )))),
        var: x,
    };
    let result1 = mcts.simplify(expr1);
    println!("Result: {:?}", result1.result);
    println!("Steps: {}\n", result1.steps.len());

    // Test 2: d/dx(cos(x^3)) should give -sin(x^3) * 3x^2
    println!("Test 2: d/dx(cos(x^3))");
    let expr2 = Expr::Derivative {
        expr: Box::new(Expr::Cos(Box::new(Expr::Pow(
            Box::new(Expr::Var(x)),
            Box::new(Expr::int(3)),
        )))),
        var: x,
    };
    let result2 = mcts.simplify(expr2);
    println!("Result: {:?}", result2.result);
    println!("Steps: {}\n", result2.steps.len());

    // Test 3: Simple sin(x) should still work
    println!("Test 3: d/dx(sin(x))");
    let expr3 = Expr::Derivative {
        expr: Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
        var: x,
    };
    let result3 = mcts.simplify(expr3);
    println!("Result: {:?}", result3.result);
    println!("Steps: {}\n", result3.steps.len());

    // Test 4: d/dx(sin(2x)) should give cos(2x) * 2
    println!("Test 4: d/dx(sin(2x))");
    let expr4 = Expr::Derivative {
        expr: Box::new(Expr::Sin(Box::new(Expr::Mul(
            Box::new(Expr::int(2)),
            Box::new(Expr::Var(x)),
        )))),
        var: x,
    };
    let result4 = mcts.simplify(expr4);
    println!("Result: {:?}", result4.result);
    println!("Steps: {}", result4.steps.len());
}
