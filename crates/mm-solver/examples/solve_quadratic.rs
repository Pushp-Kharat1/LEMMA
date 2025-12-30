//! Example: Solving quadratic equations.
//!
//! This demonstrates the Math Monster's ability to solve equations
//! with verified step-by-step reasoning.

use mm_solver::MathMonster;

fn main() {
    println!("=== Math Monster: Quadratic Equation Solver ===\n");

    let mut monster = MathMonster::new();
    println!("Loaded {} mathematical rules\n", monster.num_rules());

    // Example 1: Simplify an expression
    println!("Example 1: Simplify 2 + 3 * 4");
    println!("{}", "-".repeat(40));

    match monster.simplify("2 + 3") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 2: Simplify x + 0
    println!("Example 2: Simplify x + 0");
    println!("{}", "-".repeat(40));

    match monster.simplify("x + 0") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 3: Simplify x * 1
    println!("Example 3: Simplify x * 1");
    println!("{}", "-".repeat(40));

    match monster.simplify("x * 1") {
        Ok(result) => {
            println!("{}", result.format(monster.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 4: Verify a solution
    println!("Example 4: Verify that x = 2 solves x + 1 = 3");
    println!("{}", "-".repeat(40));

    match monster.verify_solution("x + 1 = 3", "x", "2") {
        Ok(result) => {
            println!("Verification result: {:?}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n=== Done ===");
}
