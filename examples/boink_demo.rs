//! BOINK Demo: Budget-Optimized Inference with Neural Knowledge
//!
//! This example demonstrates LEMMA's self-regulating supervisor layer:
//! - Budget allocation based on problem complexity
//! - Cost tracking per rule application
//! - Reward/penalty feedback loop
//! - Bank persistence across runs
//!
//! Run: cargo run --example boink_demo

use mm_boink::{analyze, Bank};
use mm_core::{Expr, SymbolTable};
use mm_rules::standard_rules;
use mm_search::BoinkMCTS;
use mm_verifier::Verifier;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    BOINK DEMO                              ║");
    println!("║    Budget-Optimized Inference with Neural Knowledge        ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize components
    let mut symbols = SymbolTable::new();
    let rules = standard_rules();
    let verifier = Verifier::default();

    // Create BOINK-enhanced MCTS
    let boink = BoinkMCTS::from_parts(rules, verifier);

    // Show initial bank status
    println!("=== Initial Bank Status ===");
    print_bank(&boink.bank());

    // Load any saved bank state
    let bank_path = Bank::default_path();
    if bank_path.exists() {
        println!("📂 Loading saved bank from {:?}", bank_path);
    }

    println!();
    println!("=== Running Test Problems ===");
    println!();

    // Test Problem 1: Simple algebra
    println!("📝 Problem 1: (x + 0)");
    let x = symbols.intern("x");
    let expr1 = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)));

    run_problem(&boink, &expr1, "x + 0");

    // Test Problem 2: Pythagorean distance (geometry pattern)
    println!("📝 Problem 2: √(3² + 4²) [Geometry - Distance Formula]");
    let distance = Expr::Sqrt(Box::new(Expr::Add(
        Box::new(Expr::Pow(
            Box::new(Expr::Const(3.into())),
            Box::new(Expr::Const(2.into())),
        )),
        Box::new(Expr::Pow(
            Box::new(Expr::Const(4.into())),
            Box::new(Expr::Const(2.into())),
        )),
    )));

    run_problem(&boink, &distance, "√(3² + 4²)");

    // Test Problem 3: Trigonometric
    println!("📝 Problem 3: sin²(x) + cos²(x) [Trigonometry]");
    let sin_sq = Expr::Pow(
        Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
        Box::new(Expr::Const(2.into())),
    );
    let cos_sq = Expr::Pow(
        Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
        Box::new(Expr::Const(2.into())),
    );
    let trig_identity = Expr::Add(Box::new(sin_sq), Box::new(cos_sq));

    run_problem(&boink, &trig_identity, "sin²(x) + cos²(x)");

    // Test Problem 4: Simple derivative (calculus)
    println!("📝 Problem 4: d/dx(x²) [Calculus]");
    let derivative = Expr::Derivative {
        expr: Box::new(Expr::Pow(
            Box::new(Expr::Var(x)),
            Box::new(Expr::Const(2.into())),
        )),
        var: x,
    };

    run_problem(&boink, &derivative, "d/dx(x²)");

    // Show final bank status
    println!();
    println!("=== Final Bank Status ===");
    print_bank(&boink.bank());

    // Show accumulated statistics
    println!();
    println!("🎯 BOINK Feedback: Bank credits will be saved for premium features!");
    println!("   Reach 20,000 credits to unlock advanced solving modes.");
}

fn run_problem(boink: &BoinkMCTS, expr: &Expr, _description: &str) {
    // Analyze problem to see what domains were detected
    let profile = analyze(expr);
    println!("   Domains detected: {:?}", profile.domains);
    println!("   Complexity: {}", profile.complexity);

    // Run BOINK-tracked simplification
    let (solution, stats) = boink.simplify_tracked(expr.clone());

    // Print solution
    if stats.solved {
        println!("   ✅ Solved in {} steps", stats.rules_applied);
        if let Some(step) = solution.steps.last() {
            println!("   Result: {:?}", step.after);
        }
    } else {
        println!("   ❌ Could not simplify further");
    }

    // Print BOINK stats
    println!(
        "   Budget: {} | Cost: {} | Saved: {}",
        stats.budget_allocated, stats.cost_spent, stats.credits_remaining
    );
    println!();
}

fn print_bank(bank: &Bank) {
    println!("   Credits: {}", bank.total_credits());
    println!("   Problems solved: {}", bank.problems_solved());
    println!("   Total savings: {}", bank.total_savings());
    // Check if premium is unlocked
    if bank.total_credits() >= 20_000 {
        println!("   🌟 PREMIUM UNLOCKED!");
    } else {
        println!(
            "   Progress to premium: {:.1}%",
            (bank.total_credits() as f64 / 200.0)
        );
    }
}
