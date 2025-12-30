//! Example: Testing model with new rules.
//!
//! This tests the neural network and MCTS on:
//! 1. New equation solving rules
//! 2. Quotient rule for derivatives
//! 3. Linear and polynomial expressions

use mm_core::{Expr, SymbolTable};
use mm_rules::{rule::standard_rules, RuleContext};
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    println!("=== Math Monster: Testing New Rules ===\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let rules = standard_rules();
    let ctx = RuleContext::default();

    println!("Total rules loaded: {}\n", rules.len());

    // =========================================================================
    // Test 1: Equation Solving Rules
    // =========================================================================
    println!("--- Test 1: Equation Solving Rules ---\n");

    // Test: x + 5 = 12
    let eq1 = Expr::Equation {
        lhs: Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(5)))),
        rhs: Box::new(Expr::int(12)),
    };
    println!("Equation: x + 5 = 12");
    test_applicable_rules(&rules, &eq1, &ctx);

    // Test: 3x = 15
    let eq2 = Expr::Equation {
        lhs: Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
        rhs: Box::new(Expr::int(15)),
    };
    println!("\nEquation: 3x = 15");
    test_applicable_rules(&rules, &eq2, &ctx);

    // Test: 2x + 4 = 10 (linear solve)
    let eq3 = Expr::Equation {
        lhs: Box::new(Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
            Box::new(Expr::int(4)),
        )),
        rhs: Box::new(Expr::int(10)),
    };
    println!("\nEquation: 2x + 4 = 10");
    test_applicable_rules(&rules, &eq3, &ctx);

    // =========================================================================
    // Test 2: Quotient Rule
    // =========================================================================
    println!("\n--- Test 2: Quotient Rule ---\n");

    // d/dx(x / (x+1))
    let quotient_deriv = Expr::Derivative {
        expr: Box::new(Expr::Div(
            Box::new(Expr::Var(x)),
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)))),
        )),
        var: x,
    };
    println!("Expression: d/dx(x / (x+1))");
    test_applicable_rules(&rules, &quotient_deriv, &ctx);

    // =========================================================================
    // Test 3: Neural MCTS on New Expressions
    // =========================================================================
    println!("\n--- Test 3: Neural MCTS Simplification ---\n");

    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 30,
        exploration_weight: 1.41,
        max_depth: 5,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(standard_rules(), verifier, config);

    // Test simplification
    let test_exprs = vec![
        (
            "(x + 0) * 1",
            Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
                Box::new(Expr::int(1)),
            ),
        ),
        (
            "0 + (5 * 1)",
            Expr::Add(
                Box::new(Expr::int(0)),
                Box::new(Expr::Mul(Box::new(Expr::int(5)), Box::new(Expr::int(1)))),
            ),
        ),
        (
            "2 * 3 + 4",
            Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::int(3)))),
                Box::new(Expr::int(4)),
            ),
        ),
    ];

    for (name, expr) in test_exprs {
        println!("Input: {}", name);
        let result = mcts.simplify(expr.clone());
        println!("  Output: {:?}", result.result);
        println!("  Steps: {}", result.steps.len());
        println!("  Verified: {}", result.verified);
        if !result.steps.is_empty() {
            for (i, step) in result.steps.iter().enumerate() {
                println!("    Step {}: {}", i + 1, step.rule_name);
            }
        }
        println!();
    }

    // =========================================================================
    // Test 4: Rule Application Results
    // =========================================================================
    println!("--- Test 4: Direct Rule Application ---\n");

    // Apply cancel_addition to x + 5 = 12
    println!("Applying cancel_addition to: x + 5 = 12");
    for rule in rules.applicable(&eq1, &ctx) {
        let results = rule.apply(&eq1, &ctx);
        for app in results {
            println!("  {} → {:?}", rule.name, app.result);
            println!("    Justification: {}", app.justification);
        }
    }

    // Apply cancel_multiplication to 3x = 15
    println!("\nApplying cancel_multiplication to: 3x = 15");
    for rule in rules.applicable(&eq2, &ctx) {
        let results = rule.apply(&eq2, &ctx);
        for app in results {
            println!("  {} → {:?}", rule.name, app.result);
            println!("    Justification: {}", app.justification);
        }
    }

    // Apply linear_solve to 2x + 4 = 10
    println!("\nApplying linear_solve to: 2x + 4 = 10");
    for rule in rules.applicable(&eq3, &ctx) {
        let results = rule.apply(&eq3, &ctx);
        for app in results {
            println!("  {} → {:?}", rule.name, app.result);
            println!("    Justification: {}", app.justification);
        }
    }

    println!("\n=== All Tests Complete ===");
}

fn test_applicable_rules(rules: &mm_rules::RuleSet, expr: &Expr, ctx: &RuleContext) {
    let applicable = rules.applicable(expr, ctx);
    if applicable.is_empty() {
        println!("  No rules applicable");
    } else {
        println!("  Applicable rules:");
        for rule in applicable {
            println!("    - {} ({})", rule.name, rule.description);
        }
    }
}
