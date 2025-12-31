//! Example: Solving quadratic equations.
//!
//! This demonstrates LEMMA's ability to solve equations
//! with verified step-by-step reasoning.

use mm_solver::LemmaSolver;

fn main() {
    println!("=== LEMMA: Quadratic Equation Solver ===\n");

    let mut solver = LemmaSolver::new();
    println!("Loaded {} mathematical rules\n", solver.num_rules());

    // Example 1: Simplify an expression
    println!("Example 1: Simplify 2 + 3 * 4");
    println!("{}", "-".repeat(40));

    match solver.simplify("2 + 3") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 2: Simplify x + 0
    println!("Example 2: Simplify x + 0");
    println!("{}", "-".repeat(40));

    match solver.simplify("x + 0") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 3: Simplify x * 1
    println!("Example 3: Simplify x * 1");
    println!("{}", "-".repeat(40));

    match solver.simplify("x * 1") {
        Ok(result) => {
            println!("{}", result.format(solver.symbols()));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!();

    // Example 4: Verify a solution
    println!("Example 4: Verify that x = 2 solves x + 1 = 3");
    println!("{}", "-".repeat(40));

    match solver.verify_solution("x + 1 = 3", "x", "2") {
        Ok(result) => {
            println!("Verification result: {:?}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\n=== Done ===");
}
