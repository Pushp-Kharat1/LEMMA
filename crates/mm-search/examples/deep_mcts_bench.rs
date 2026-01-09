//! Deep MCTS Benchmark
//!
//! Tests the DeepMCTS configuration and node counting.
//!
//! Usage: cargo run --example deep_mcts_bench --release -p mm-search

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::RuleSet;
use mm_search::{DeepMCTS, DeepMCTSConfig};
use mm_verifier::Verifier;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           LEMMA Deep MCTS Benchmark                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Show config presets
    println!("Available DeepMCTS Configurations:");
    println!("─────────────────────────────────────────────────────────────────");

    let quick = DeepMCTSConfig::quick();
    println!(
        "  QUICK:   {:>10} nodes, {:>4}s timeout, {} workers",
        quick.max_nodes, quick.time_limit_secs, quick.num_workers
    );

    let medium = DeepMCTSConfig::medium();
    println!(
        "  MEDIUM:  {:>10} nodes, {:>4}s timeout, {} workers",
        medium.max_nodes, medium.time_limit_secs, medium.num_workers
    );

    let deep = DeepMCTSConfig::deep();
    println!(
        "  DEEP:    {:>10} nodes, {:>4}s timeout, {} workers",
        deep.max_nodes, deep.time_limit_secs, deep.num_workers
    );

    let maximum = DeepMCTSConfig::maximum();
    println!(
        "  MAXIMUM: {:>10} nodes, {:>4}s timeout, {} workers",
        maximum.max_nodes, maximum.time_limit_secs, maximum.num_workers
    );

    println!();

    // Create solver with quick config
    println!("Testing with QUICK config:");
    println!("─────────────────────────────────────────────────────────────────");

    let rules = RuleSet::new();
    let verifier = Verifier::new();
    let config = DeepMCTSConfig::quick();
    let solver = DeepMCTS::with_config(rules, verifier, config);

    // Create test expression: x + 0
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let expr = Expr::Add(
        Box::new(Expr::Var(x)),
        Box::new(Expr::Const(Rational::from(0))),
    );

    println!("  Problem: x + 0");
    println!("  Input: {:?}", expr);

    // Goal: simplified to single variable or constant
    let goal = |e: &Expr| matches!(e, Expr::Var(_) | Expr::Const(_));

    let (solution, stats) = solver.search(expr.clone(), goal);

    println!("  Nodes explored: {}", stats.nodes_explored);
    println!("  Time: {:.3}s", stats.elapsed_seconds);
    if stats.elapsed_seconds > 0.0 {
        println!("  Rate: {:.0} nodes/sec", stats.nodes_per_second);
    }

    if let Some(path) = solution {
        println!("  ✓ Solution found: {} steps", path.len() - 1);
        println!("  Result: {:?}", path.last().unwrap());
    } else {
        println!("  ✗ No solution found (may need more rules)");
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           DeepMCTS Ready for 10M+ Node Search                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
