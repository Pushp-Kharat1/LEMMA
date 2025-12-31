// Quick test of derivative rules
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_rules::RuleContext;

fn main() {
    println!("=== Testing Derivative Rules Directly ===\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");

    let rules = standard_rules();
    let ctx = RuleContext::default();

    // Test 1: d/dx(5) should give 0 via constant_rule
    let d_const = Expr::Derivative {
        expr: Box::new(Expr::int(5)),
        var: x,
    };

    println!("Test: d/dx(5)");
    println!("  Expression: {:?}", d_const);
    let applicable = rules.applicable(&d_const, &ctx);
    println!("  Applicable rules: {}", applicable.len());
    for rule in &applicable {
        let results = (rule.apply)(&d_const, &ctx);
        println!(
            "    - {} -> {:?}",
            rule.name,
            results.first().map(|r| &r.result)
        );
    }

    // Test 2: d/dx(x) should give 1 via power_rule (x = x^1)
    let d_x = Expr::Derivative {
        expr: Box::new(Expr::Var(x)),
        var: x,
    };

    println!("\nTest: d/dx(x)");
    println!("  Expression: {:?}", d_x);
    let applicable = rules.applicable(&d_x, &ctx);
    println!("  Applicable rules: {}", applicable.len());
    for rule in &applicable {
        let results = (rule.apply)(&d_x, &ctx);
        println!(
            "    - {} -> {:?}",
            rule.name,
            results.first().map(|r| &r.result)
        );
    }

    // Test 3: d/dx(x^2)
    let d_x2 = Expr::Derivative {
        expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        var: x,
    };

    println!("\nTest: d/dx(x^2)");
    println!("  Expression: {:?}", d_x2);
    let applicable = rules.applicable(&d_x2, &ctx);
    println!("  Applicable rules: {}", applicable.len());
    for rule in &applicable {
        let results = (rule.apply)(&d_x2, &ctx);
        println!(
            "    - {} -> {:?}",
            rule.name,
            results.first().map(|r| &r.result)
        );
    }

    // Test 4: d/dx(sin(x))
    let d_sin = Expr::Derivative {
        expr: Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
        var: x,
    };

    println!("\nTest: d/dx(sin(x))");
    println!("  Expression: {:?}", d_sin);
    let applicable = rules.applicable(&d_sin, &ctx);
    println!("  Applicable rules: {}", applicable.len());
    for rule in &applicable {
        let results = (rule.apply)(&d_sin, &ctx);
        println!(
            "    - {} -> {:?}",
            rule.name,
            results.first().map(|r| &r.result)
        );
    }
}
