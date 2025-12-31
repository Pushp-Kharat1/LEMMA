//! Example: Computing derivatives.
//!
//! This demonstrates LEMMA's ability to compute symbolic
//! derivatives with step-by-step reasoning.

use mm_solver::LemmaSolver;

fn main() {
    println!("=== LEMMA: Derivative Calculator ===\n");

    let mut solver = LemmaSolver::new();
    println!("Loaded {} mathematical rules\n", solver.num_rules());

    // Example 1: d/dx(x^2)
    println!("Example 1: d/dx(x^2)");
    println!("{}", "-".repeat(40));

    match solver.differentiate("x^2", "x") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 2: d/dx(x^3)
    println!("Example 2: d/dx(x^3)");
    println!("{}", "-".repeat(40));

    match solver.differentiate("x^3", "x") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 3: d/dx(sin(x))
    println!("Example 3: d/dx(sin(x))");
    println!("{}", "-".repeat(40));

    match solver.differentiate("sin(x)", "x") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 4: d/dx(5) - constant
    println!("Example 4: d/dx(5)");
    println!("{}", "-".repeat(40));

    match solver.differentiate("5", "x") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n=== Done ===");
}
