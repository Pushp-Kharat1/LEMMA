//! Example: Computing derivatives.
//!
//! This demonstrates the Math Monster's ability to compute symbolic
//! derivatives with step-by-step reasoning.

use mm_solver::MathMonster;

fn main() {
    println!("=== Math Monster: Derivative Calculator ===\n");

    let mut monster = MathMonster::new();
    println!("Loaded {} mathematical rules\n", monster.num_rules());

    // Example 1: d/dx(x^2)
    println!("Example 1: d/dx(x^2)");
    println!("{}", "-".repeat(40));

    match monster.differentiate("x^2", "x") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 2: d/dx(x^3)
    println!("Example 2: d/dx(x^3)");
    println!("{}", "-".repeat(40));

    match monster.differentiate("x^3", "x") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 3: d/dx(sin(x))
    println!("Example 3: d/dx(sin(x))");
    println!("{}", "-".repeat(40));

    match monster.differentiate("sin(x)", "x") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 4: d/dx(5) - constant
    println!("Example 4: d/dx(5)");
    println!("{}", "-".repeat(40));

    match monster.differentiate("5", "x") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n=== Done ===");
}
