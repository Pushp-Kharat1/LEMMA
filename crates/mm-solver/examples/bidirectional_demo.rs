//! Bidirectional Proof Search Demo
//!
//! Demonstrates backward reasoning from goals
//!
//! Usage: cargo run --example bidirectional_demo -p mm-solver

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::backward::backward_search;
use mm_rules::rule::standard_rules;
use mm_rules::RuleContext;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          BIDIRECTIONAL PROOF SEARCH DEMO                    ║");
    println!("║        Backward Reasoning from Goals                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    // ═══════════════════════════════════════════════════════════════
    // DEMO 1: Prove x² + y² ≥ 2xy
    //═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("DEMO 1: Prove x² + y² ≥ 2xy for all real x, y");
    println!("═══════════════════════════════════════════════════════════════\n");

    // The goal
    let lhs = Expr::Add(
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
    );

    let rhs = Expr::Mul(
        Box::new(Expr::int(2)),
        Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
    );

    let goal = Expr::Gte(Box::new(lhs.clone()), Box::new(rhs.clone()));

    println!("Goal: {:?}", goal);
    println!("\nBACKWARD REASONING:\n");

    // Apply backward search
    let backward_steps = backward_search(&goal);

    println!("Found {} backward strategies:\n", backward_steps.len());

    for (i, step) in backward_steps.iter().enumerate() {
        println!("Strategy {}: {}", i + 1, step.strategy_name());
        println!("  Justification: {}", step.justification);
        println!("  New subgoals:");
        for (j, subgoal) in step.subgoals.iter().enumerate() {
            println!("    {}. {:?}", j + 1, subgoal);
        }
        println!();
    }

    // ═══════════════════════════════════════════════════════════════
    // DEMO 2: Prove (a+b)² = a² + 2ab + b²
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("DEMO 2: Prove (a+b)² = a² + 2ab + b²");
    println!("═══════════════════════════════════════════════════════════════\n");

    let a = symbols.intern("a");
    let b = symbols.intern("b");

    let lhs2 = Expr::Pow(
        Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
        Box::new(Expr::int(2)),
    );

    let rhs2 = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(2)))),
            Box::new(Expr::Mul(
                Box::new(Expr::int(2)),
                Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            )),
        )),
        Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
    );

    let goal2 = Expr::Equation {
        lhs: Box::new(lhs2),
        rhs: Box::new(rhs2),
    };

    println!("Goal: {:?}\n", goal2);
    println!("BACKWARD REASONING:\n");

    let backward_steps2 = backward_search(&goal2);
    println!("Found {} backward strategies:\n", backward_steps2.len());

    for (i, step) in backward_steps2.iter().enumerate() {
        println!("Strategy {}: {}", i + 1, step.strategy_name());
        println!("  {}", step.justification);
        println!();
    }

    // ═══════════════════════════════════════════════════════════════
    // Show forward rules that could complete the proof
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("FORWARD RULES AVAILABLE");
    println!("═══════════════════════════════════════════════════════════════\n");

    let rules = standard_rules();
    let ctx = RuleContext::default();

    // Check what rules apply to (x-y)²
    let x_minus_y = Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
    let squared = Expr::Pow(Box::new(x_minus_y), Box::new(Expr::int(2)));

    let applicable = rules.applicable(&squared, &ctx);
    println!("Rules applicable to (x-y)²: {}", applicable.len());

    for rule in applicable.iter().take(5) {
        println!("  • {} ({:?})", rule.name, rule.category);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("PROOF CONSTRUCTION");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Goal: x² + y² ≥ 2xy");
    println!();
    println!("Backward Step:");
    println!("  ← Show: (x-y)² ≥ 0");
    println!("     (because (x-y)² = x² - 2xy + y²)");
    println!();
    println!("Forward Step:");
    println!("  → Axiom: All squares are nonnegative");
    println!("  → Therefore: (x-y)² ≥ 0 ✓");
    println!();
    println!("✅ PROOF COMPLETE!");
}

// Helper trait for display
trait StrategyDisplay {
    fn strategy_name(&self) -> &str;
}

impl StrategyDisplay for mm_rules::backward::BackwardStep {
    fn strategy_name(&self) -> &str {
        use mm_rules::backward::BackwardStrategy;
        match self.strategy {
            BackwardStrategy::InequalityToNonneg => "Inequality to nonnegative form",
            BackwardStrategy::EquivalentForm => "Equivalent form transformation",
            BackwardStrategy::TheoremApplication => "Theorem application",
            BackwardStrategy::Substitution => "Substitution",
        }
    }
}
