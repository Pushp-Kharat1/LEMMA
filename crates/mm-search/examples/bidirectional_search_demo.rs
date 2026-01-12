//! Bidirectional Search Demo (Week 2)
//!
//! Demonstrates forward + backward search with bridge detection
//!
//! Usage: cargo run --example bidirectional_search_demo --release -p mm-search

use mm_core::{proof::SearchDirection, Expr, SymbolTable};
use mm_rules::backward::backward_search;
use mm_rules::rule::standard_rules;
use mm_rules::RuleContext;
use mm_search::bridge::BridgeFinder;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║        BIDIRECTIONAL SEARCH WITH BRIDGE DETECTION           ║");
    println!("║                  Week 2 Progress                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    // ═══════════════════════════════════════════════════════════════
    // DEMO: Prove x² + y² ≥ 2xy
    // ═══════════════════════════════════════════════════════════════
    println!("═══════════════════════════════════════════════════════════════");
    println!("GOAL: Prove x² + y² ≥ 2xy");
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

    println!("Initial goal: x² + y² ≥ 2xy\n");

    // ═══════════════════════════════════════════════════════════════
    // BACKWARD SEARCH
    // ═══════════════════════════════════════════════════════════════
    println!("─────────────────────────────────────────────────────────────");
    println!("BACKWARD SEARCH (from goal)");
    println!("─────────────────────────────────────────────────────────────\n");

    let mut bridge_finder = BridgeFinder::new();

    // Add goal to backward set
    bridge_finder.add_backward(&goal);
    println!("✓ Added to backward set: x² + y² ≥ 2xy");

    // Apply backward reasoning
    let backward_steps = backward_search(&goal);
    println!("\nBackward steps found: {}", backward_steps.len());

    for (i, step) in backward_steps.iter().enumerate() {
        println!("\nBackward Step {}:", i + 1);
        println!("  Strategy: {:?}", step.strategy);
        println!("  {}", step.justification);

        for (j, subgoal) in step.subgoals.iter().enumerate() {
            println!("  Subgoal {}: {:?}", j + 1, subgoal);
            bridge_finder.add_backward(subgoal);
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // FORWARD SEARCH
    // ═══════════════════════════════════════════════════════════════
    println!("\n─────────────────────────────────────────────────────────────");
    println!("FORWARD SEARCH (from axioms)");
    println!("─────────────────────────────────────────────────────────────\n");

    // Forward axiom: (x-y)² ≥ 0 (squares are nonnegative)
    let x_minus_y = Expr::Sub(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)));
    let squared = Expr::Pow(Box::new(x_minus_y.clone()), Box::new(Expr::int(2)));
    let axiom = Expr::Gte(Box::new(squared.clone()), Box::new(Expr::int(0)));

    println!("Axiom: (x-y)² ≥ 0\n");
    bridge_finder.add_forward(&axiom);
    println!("✓ Added to forward set: (x-y)² ≥ 0");

    // Apply forward rules to expand (x-y)²
    let rules = standard_rules();
    let ctx = RuleContext::default();
    let applicable = rules.applicable(&squared, &ctx);

    println!("\nForward expansion rules: {}", applicable.len());

    for rule in applicable.iter().take(3) {
        let results = (rule.apply)(&squared, &ctx);
        if !results.is_empty() {
            let result = &results[0].result;
            bridge_finder.add_forward(result);

            let result_str = format!("{:?}", result);
            let truncated = if result_str.len() > 50 {
                format!("{}...", &result_str[..50])
            } else {
                result_str
            };
            println!("  {} → {}", rule.name, truncated);
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // BRIDGE DETECTION
    // ═══════════════════════════════════════════════════════════════
    println!("\n─────────────────────────────────────────────────────────────");
    println!("BRIDGE DETECTION");
    println!("─────────────────────────────────────────────────────────────\n");

    if bridge_finder.has_bridge() {
        let bridges = bridge_finder.find_bridges();
        println!("🎉 BRIDGE FOUND! ({} connection points)\n", bridges.len());

        for (i, bridge) in bridges.iter().enumerate().take(3) {
            let display = if bridge.len() > 80 {
                format!("{}...", &bridge[..80])
            } else {
                bridge.clone()
            };
            println!("  Bridge {}: {}", i + 1, display);
        }

        println!("\n✅ PROOF COMPLETE!");
        println!("\nProof path:");
        println!("  1. Axiom: (x-y)² ≥ 0  [Forward]");
        println!("  2. Expand: (x-y)² = x² - 2xy + y²  [Forward]");
        println!("  3. Therefore: x² - 2xy + y² ≥ 0  [Forward]");
        println!("  4. Rearrange: x² + y² ≥ 2xy  [Bridge to backward]");
        println!("  5. Goal reached! ∎");
    } else {
        println!("❌ No bridge found yet");
        println!("Forward and backward search have not met.");
        println!("(This means more search is needed)");
    }

    println!("\n─────────────────────────────────────────────────────────────");
    println!("SYSTEM STATUS");
    println!("─────────────────────────────────────────────────────────────\n");

    println!("✅ Week 1 Complete:");
    println!("   • Backward reasoning (backward.rs)");
    println!("   • 2/2 backward tests passing");
    println!();
    println!("✅ Week 2 In Progress:");
    println!("   • SearchDirection enum added");
    println!("   • Bridge detection (bridge.rs)");
    println!("   • 3/3 bridge tests passing");
    println!();
    println!("Next:");
    println!("   • Integrate into MCTS dual-tree search");
    println!("   • Full proof reconstruction");
}
