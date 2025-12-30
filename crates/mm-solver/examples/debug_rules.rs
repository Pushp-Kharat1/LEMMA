//! Debug test to see what's happening with rule application

use mm_core::{Expr, SymbolTable};
use mm_rules::{rule::standard_rules, RuleContext};

fn main() {
    println!("=== Math Monster Debug Test ===\n");

    let rules = standard_rules();
    let ctx = RuleContext::default();

    println!("Total rules loaded: {}\n", rules.len());

    // Test 1: 2 + 3 - should match constant folding
    println!("Test 1: Constant Folding (2 + 3)");
    println!("{}", "-".repeat(40));
    let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
    println!("Expression: {:?}", expr);

    let applicable = rules.applicable(&expr, &ctx);
    println!("Applicable rules: {}", applicable.len());
    for rule in &applicable {
        println!("  - {} ({})", rule.name, rule.id);
        let results = rule.apply(&expr, &ctx);
        for r in results {
            println!("    → {:?}", r.result);
        }
    }

    println!();

    // Test 2: x + 0 - should match identity
    println!("Test 2: Identity (x + 0)");
    println!("{}", "-".repeat(40));
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
    println!("Expression: {:?}", expr);

    let applicable = rules.applicable(&expr, &ctx);
    println!("Applicable rules: {}", applicable.len());
    for rule in &applicable {
        println!("  - {} ({})", rule.name, rule.id);
        let results = rule.apply(&expr, &ctx);
        for r in results {
            println!("    → {:?}", r.result);
        }
    }

    println!();

    // Test 3: x * 0 - should simplify to 0
    println!("Test 3: Zero Multiplication (x * 0)");
    println!("{}", "-".repeat(40));
    let expr = Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));
    println!("Expression: {:?}", expr);

    let applicable = rules.applicable(&expr, &ctx);
    println!("Applicable rules: {}", applicable.len());
    for rule in &applicable {
        println!("  - {} ({})", rule.name, rule.id);
        let results = rule.apply(&expr, &ctx);
        for r in results {
            println!("    → {:?}", r.result);
        }
    }

    println!();

    // Test 4: d/dx(x^2) - should apply power rule
    println!("Test 4: Power Rule d/dx(x^2)");
    println!("{}", "-".repeat(40));
    let expr = Expr::Derivative {
        expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        var: x,
    };
    println!("Expression: {:?}", expr);

    let applicable = rules.applicable(&expr, &ctx);
    println!("Applicable rules: {}", applicable.len());
    for rule in &applicable {
        println!("  - {} ({})", rule.name, rule.id);
        let results = rule.apply(&expr, &ctx);
        for r in results {
            println!("    → {:?}", r.result);
        }
    }

    println!("\n=== Done ===");
}
