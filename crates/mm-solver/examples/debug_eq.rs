use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_rules::RuleContext;

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let rules = standard_rules();
    let ctx = RuleContext::default();

    // Test 2x = 10
    let expr = Expr::Equation {
        lhs: Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
        rhs: Box::new(Expr::int(10)),
    };

    println!("Expression: 2x = 10");
    println!("Checking applicable rules...");

    let applicable = rules.applicable(&expr, &ctx);
    println!("Found {} applicable rules:", applicable.len());

    for rule in &applicable {
        let results = (rule.apply)(&expr, &ctx);
        println!(
            "  - {}: {:?}",
            rule.name,
            results.first().map(|r| &r.result)
        );
    }
}
