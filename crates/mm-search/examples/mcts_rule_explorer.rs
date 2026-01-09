//! Enhanced MCTS Debug - Shows WHICH RULES are being applied
//!
//! Usage: cargo run --example mcts_rule_explorer --release -p mm-search

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_rules::{RuleCategory, RuleContext};
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       MCTS Rule Explorer - See Which Rules Apply             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Create a simple expression: (a + b)^2
    let mut symbols = SymbolTable::new();
    let a = symbols.intern("a");
    let b = symbols.intern("b");
    let x = symbols.intern("x");

    let expr = Expr::Pow(
        Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
        Box::new(Expr::Const(Rational::from(2))),
    );

    println!("Starting expression: (a + b)²\n");
    println!("═══════════════════════════════════════════════════════════════");
    println!("                   AVAILABLE RULES");
    println!("═══════════════════════════════════════════════════════════════\n");

    let rules = standard_rules();
    let all_rules = rules.all();
    println!("Total rules loaded: {}\n", all_rules.len());

    // Group by category
    let mut by_category: HashMap<RuleCategory, usize> = HashMap::new();
    for rule in all_rules {
        *by_category.entry(rule.category).or_insert(0) += 1;
    }

    for (cat, count) in &by_category {
        println!("[{:?}] {} rules", cat, count);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                RULES APPLICABLE TO (a+b)²");
    println!("═══════════════════════════════════════════════════════════════\n");

    let ctx = RuleContext::default();
    let applicable = rules.applicable(&expr, &ctx);

    println!("Found {} applicable rules:\n", applicable.len());

    for rule in &applicable {
        println!("✓ {} (ID: {:?})", rule.name, rule.id);
        println!("  Category: {:?}", rule.category);
        println!("  Description: {}", rule.description);

        // Try to apply and show result
        let results = (rule.apply)(&expr, &ctx);
        for (i, result) in results.iter().take(2).enumerate() {
            let result_str = format!("{:?}", result.result);
            let truncated = if result_str.len() > 60 {
                format!("{}...", &result_str[..60])
            } else {
                result_str
            };
            println!("  → Result {}: {}", i + 1, truncated);
            println!("    Justification: {}", result.justification);
        }
        println!();
    }

    // Now test a few more expressions
    println!("═══════════════════════════════════════════════════════════════");
    println!("            RULES APPLICABLE TO OTHER EXPRESSIONS");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Test x^3 - 1
    let expr2 = Expr::Sub(
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
        Box::new(Expr::int(1)),
    );
    test_expression("x³ - 1", &expr2, &rules);

    // Test x^2 + 2x + 1
    let expr3 = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
        )),
        Box::new(Expr::int(1)),
    );
    test_expression("x² + 2x + 1", &expr3, &rules);

    // Test a + b (simple)
    let expr4 = Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)));
    test_expression("a + b", &expr4, &rules);

    // Test factorial
    let expr5 = Expr::Factorial(Box::new(Expr::int(5)));
    test_expression("5!", &expr5, &rules);

    // Test GCD
    let expr6 = Expr::GCD(Box::new(Expr::int(12)), Box::new(Expr::int(18)));
    test_expression("gcd(12, 18)", &expr6, &rules);

    // Test sin²(x) + cos²(x)
    let expr7 = Expr::Add(
        Box::new(Expr::Pow(
            Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
            Box::new(Expr::int(2)),
        )),
        Box::new(Expr::Pow(
            Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
            Box::new(Expr::int(2)),
        )),
    );
    test_expression("sin²(x) + cos²(x)", &expr7, &rules);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                   RULE CATEGORY SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    for (cat, count) in by_category {
        let cat_rules = rules.by_category(cat);
        let sample_names: Vec<_> = cat_rules.iter().take(5).map(|r| r.name).collect();
        println!(
            "{:20?} {:3} rules | samples: {}",
            cat,
            count,
            sample_names.join(", ")
        );
    }

    println!("\n✅ MCTS Rule Exploration Complete!");
}

fn test_expression(name: &str, expr: &Expr, rules: &mm_rules::RuleSet) {
    let ctx = RuleContext::default();
    let applicable = rules.applicable(expr, &ctx);
    let names: Vec<_> = applicable.iter().map(|r| r.name).collect();
    println!(
        "[{}] {} applicable: {}",
        name,
        applicable.len(),
        if names.is_empty() {
            "none".to_string()
        } else {
            names.join(", ")
        }
    );
}
