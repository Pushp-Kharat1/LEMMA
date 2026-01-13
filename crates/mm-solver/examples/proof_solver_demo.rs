// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! End-to-End Proof Solver Demo
//!
//! This demo shows LEMMA solving real mathematical problems with
//! step-by-step reasoning, using:
//! - Mathematical induction
//! - Case analysis
//! - Backward reasoning
//! - Forward transformation rules

use mm_core::Expr;
use mm_solver::orchestrator::ProofOrchestrator;

fn main() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           LEMMA: End-to-End Mathematical Proof Solver            ║");
    println!("║                   Demonstrating Real Proofs                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create the proof orchestrator
    let mut orchestrator = ProofOrchestrator::new();

    // ═══════════════════════════════════════════════════════════════════════
    // Problem 1: x² ≥ 0 (Direct proof - trivially true)
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  PROBLEM 1: Prove x² ≥ 0 for all real x                        │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    let x = orchestrator.symbols_mut().intern("x");
    let goal1 = Expr::Gte(
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        Box::new(Expr::int(0)),
    );

    let result1 = orchestrator.prove(&goal1);
    println!("\n{}", result1.summary);
    if result1.success {
        println!("🏆 Result: PROVEN ✓");
    } else {
        println!("⚠️ Result: Could not complete proof");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Problem 2: ∀n. n² ≥ 0 (Induction proof)
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  PROBLEM 2: Prove ∀n. n² ≥ 0 by induction                      │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    let n = orchestrator.symbols_mut().intern("n");
    let goal2 = Expr::ForAll {
        var: n,
        domain: None,
        body: Box::new(Expr::Gte(
            Box::new(Expr::Pow(Box::new(Expr::Var(n)), Box::new(Expr::int(2)))),
            Box::new(Expr::int(0)),
        )),
    };

    let result2 = orchestrator.prove(&goal2);
    println!("\n{}", result2.summary);
    if result2.success {
        println!("🏆 Result: PROVEN ✓");
    } else {
        println!("⚠️ Result: Could not complete proof");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Problem 3: x² + y² ≥ 0 (Case analysis - sum of squares)
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│  PROBLEM 3: Prove x² + y² ≥ 0 by case analysis                 │");
    println!("└─────────────────────────────────────────────────────────────────┘");

    let y = orchestrator.symbols_mut().intern("y");
    let goal3 = Expr::Gte(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
        )),
        Box::new(Expr::int(0)),
    );

    let result3 = orchestrator.prove(&goal3);
    println!("\n{}", result3.summary);
    if result3.success {
        println!("🏆 Result: PROVEN ✓");
    } else {
        println!("⚠️ Result: Could not complete proof");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Summary
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                         SUMMARY                                  ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!(
        "║  Problem 1 (x² ≥ 0):       {} │",
        if result1.success {
            "PROVEN ✓  "
        } else {
            "INCOMPLETE"
        }
    );
    println!(
        "║  Problem 2 (∀n. n² ≥ 0):   {} │",
        if result2.success {
            "PROVEN ✓  "
        } else {
            "INCOMPLETE"
        }
    );
    println!(
        "║  Problem 3 (x² + y² ≥ 0):  {} │",
        if result3.success {
            "PROVEN ✓  "
        } else {
            "INCOMPLETE"
        }
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let total_proven = [result1.success, result2.success, result3.success]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("📊 Total: {}/3 problems proven\n", total_proven);
}
