//! Debug example that logs EVERY node searched to prove the search is real
//!
//! Usage: cargo run --example deep_mcts_debug --release -p mm-search

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{DeepMCTS, DeepMCTSConfig};
use mm_verifier::Verifier;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     DeepMCTS DEBUG - Logging Every Node Searched             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Create a simple expression: (a + b)^2
    let mut symbols = SymbolTable::new();
    let a = symbols.intern("a");
    let b = symbols.intern("b");

    let expr = Expr::Pow(
        Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
        Box::new(Expr::Const(Rational::from(2))),
    );

    println!("Starting expression: {:?}\n", expr);
    println!("Goal: Simplify to constant or variable\n");
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    NODE SEARCH LOG");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Use very small config so we can log everything
    let config = DeepMCTSConfig {
        max_nodes: 50, // Only 50 nodes so we can see each one
        time_limit_secs: 10,
        num_workers: 1, // Single worker for clear sequential logging
        max_depth: 10,
        exploration_weight: 1.41,
        virtual_loss: 3.0,
        progress_interval: 10,
    };

    let rules = standard_rules();
    println!("Loaded {} rules\n", rules.len());

    let verifier = Verifier::new();
    let mcts = DeepMCTS::with_config(rules, verifier, config);

    // Custom search with logging
    let node_counter = AtomicU64::new(0);

    let goal = |e: &Expr| {
        let count = node_counter.fetch_add(1, Ordering::Relaxed) + 1;

        // Log every node
        let expr_str = format!("{:?}", e);
        let truncated = if expr_str.len() > 80 {
            format!("{}...", &expr_str[..80])
        } else {
            expr_str
        };

        println!("Node {:03}: {}", count, truncated);

        // Goal check
        matches!(e, Expr::Const(_) | Expr::Var(_))
    };

    let (solution, stats) = mcts.search(expr, goal);

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    SEARCH COMPLETE");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!(
        "Total nodes logged: {}",
        node_counter.load(Ordering::Relaxed)
    );
    println!("Stats nodes explored: {}", stats.nodes_explored);
    println!("Time: {:.4}s", stats.elapsed_seconds);
    println!("Solution found: {}", solution.is_some());

    if let Some(path) = solution {
        println!("\nSolution path:");
        for (i, step) in path.iter().enumerate() {
            println!("  Step {}: {:?}", i, step);
        }
    }
}
