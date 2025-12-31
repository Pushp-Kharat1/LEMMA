// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! LEMMA Benchmark Suite
//!
//! Measures LEMMA's performance across different mathematical domains.

use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::time::Instant;

/// Benchmark result for a single test
struct BenchmarkResult {
    name: String,
    passed: bool,
    steps: usize,
    time_ms: f64,
    rule_used: Option<String>,
}

/// Run a single benchmark
fn run_benchmark(
    mcts: &NeuralMCTS,
    name: &str,
    expr: Expr,
    expected_check: impl Fn(&Expr) -> bool,
) -> BenchmarkResult {
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let time_ms = start.elapsed().as_secs_f64() * 1000.0;

    let passed = result.verified && expected_check(&result.result);
    let rule_used = result.steps.first().map(|s| s.rule_name.to_string());

    BenchmarkResult {
        name: name.to_string(),
        passed,
        steps: result.steps.len(),
        time_ms,
        rule_used,
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║              LEMMA BENCHMARK SUITE v0.1                   ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 100,
        exploration_weight: 1.41,
        max_depth: 15,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    let mut results: Vec<BenchmarkResult> = Vec::new();

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 1: ALGEBRAIC IDENTITIES
    // ═══════════════════════════════════════════════════════════════
    println!("━━━ Category 1: Algebraic Identities ━━━");

    // 1.1 Additive Identity: x + 0 = x
    results.push(run_benchmark(
        &mcts,
        "x + 0 → x",
        Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        |e| matches!(e, Expr::Var(_)),
    ));

    // 1.2 Multiplicative Identity: x * 1 = x
    results.push(run_benchmark(
        &mcts,
        "x * 1 → x",
        Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(1))),
        |e| matches!(e, Expr::Var(_)),
    ));

    // 1.3 Zero Multiplication: x * 0 = 0
    results.push(run_benchmark(
        &mcts,
        "x * 0 → 0",
        Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        |e| matches!(e, Expr::Const(r) if r.is_zero()),
    ));

    // 1.4 Power of One: x^1 = x
    results.push(run_benchmark(
        &mcts,
        "x^1 → x",
        Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(1))),
        |e| matches!(e, Expr::Var(_)),
    ));

    // 1.5 Power of Zero: x^0 = 1
    results.push(run_benchmark(
        &mcts,
        "x^0 → 1",
        Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        |e| matches!(e, Expr::Const(r) if *r == mm_core::Rational::from_integer(1)),
    ));

    // 1.6 Nested Identity: (x + 0) * 1 = x
    results.push(run_benchmark(
        &mcts,
        "(x + 0) * 1 → x",
        Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
            Box::new(Expr::int(1)),
        ),
        |e| matches!(e, Expr::Var(_)),
    ));

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 2: CONSTANT FOLDING
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 2: Constant Folding ━━━");

    results.push(run_benchmark(
        &mcts,
        "2 + 3 → 5",
        Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 5),
    ));

    results.push(run_benchmark(
        &mcts,
        "7 * 8 → 56",
        Expr::Mul(Box::new(Expr::int(7)), Box::new(Expr::int(8))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 56),
    ));

    results.push(run_benchmark(
        &mcts,
        "10 - 4 → 6",
        Expr::Sub(Box::new(Expr::int(10)), Box::new(Expr::int(4))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 6),
    ));

    results.push(run_benchmark(
        &mcts,
        "12 / 4 → 3",
        Expr::Div(Box::new(Expr::int(12)), Box::new(Expr::int(4))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 3),
    ));

    results.push(run_benchmark(
        &mcts,
        "2^3 → 8",
        Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(3))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 8),
    ));

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 3: TRIGONOMETRIC IDENTITIES
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 3: Trigonometric Identities ━━━");

    // Pythagorean: sin²(x) + cos²(x) = 1
    results.push(run_benchmark(
        &mcts,
        "sin²(x) + cos²(x) → 1",
        Expr::Add(
            Box::new(Expr::Pow(
                Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
        ),
        |e| matches!(e, Expr::Const(r) if *r == mm_core::Rational::from_integer(1)),
    ));

    // sin(0) = 0
    results.push(run_benchmark(
        &mcts,
        "sin(0) → 0",
        Expr::Sin(Box::new(Expr::int(0))),
        |e| matches!(e, Expr::Const(r) if r.is_zero()),
    ));

    // cos(0) = 1
    results.push(run_benchmark(
        &mcts,
        "cos(0) → 1",
        Expr::Cos(Box::new(Expr::int(0))),
        |e| matches!(e, Expr::Const(r) if *r == mm_core::Rational::from_integer(1)),
    ));

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 4: DERIVATIVES
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 4: Derivatives ━━━");

    // d/dx(c) = 0
    results.push(run_benchmark(
        &mcts,
        "d/dx(5) → 0",
        Expr::Derivative {
            expr: Box::new(Expr::int(5)),
            var: x,
        },
        |e| matches!(e, Expr::Const(r) if r.is_zero()),
    ));

    // d/dx(x) = 1
    results.push(run_benchmark(
        &mcts,
        "d/dx(x) → 1",
        Expr::Derivative {
            expr: Box::new(Expr::Var(x)),
            var: x,
        },
        |e| matches!(e, Expr::Const(r) if *r == mm_core::Rational::from_integer(1)),
    ));

    // d/dx(x²) = 2x
    results.push(run_benchmark(
        &mcts,
        "d/dx(x²) → 2x",
        Expr::Derivative {
            expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            var: x,
        },
        |e| match e {
            Expr::Mul(a, b) => {
                matches!(a.as_ref(), Expr::Const(r) if r.numer() == 2)
                    && matches!(b.as_ref(), Expr::Var(_))
            }
            _ => false,
        },
    ));

    // d/dx(sin(x)) = cos(x)
    results.push(run_benchmark(
        &mcts,
        "d/dx(sin(x)) → cos(x)",
        Expr::Derivative {
            expr: Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
            var: x,
        },
        |e| matches!(e, Expr::Cos(_)),
    ));

    // d/dx(cos(x)) = -sin(x)
    results.push(run_benchmark(
        &mcts,
        "d/dx(cos(x)) → -sin(x)",
        Expr::Derivative {
            expr: Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
            var: x,
        },
        |e| matches!(e, Expr::Neg(inner) if matches!(inner.as_ref(), Expr::Sin(_))),
    ));

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 5: MULTI-VARIABLE
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 5: Multi-Variable ━━━");

    // x + y + 0 → x + y
    results.push(run_benchmark(
        &mcts,
        "x + y + 0 → x + y",
        Expr::Add(
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
            Box::new(Expr::int(0)),
        ),
        |e| matches!(e, Expr::Add(_, _)),
    ));

    // x * y * 1 → x * y
    results.push(run_benchmark(
        &mcts,
        "x * y * 1 → x * y",
        Expr::Mul(
            Box::new(Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
            Box::new(Expr::int(1)),
        ),
        |e| matches!(e, Expr::Mul(_, _)),
    ));

    // ═══════════════════════════════════════════════════════════════
    // RESULTS SUMMARY
    // ═══════════════════════════════════════════════════════════════
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    BENCHMARK RESULTS                       ║");
    println!("╠═══════════════════════════════════════════════════════════╣");

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let total_time: f64 = results.iter().map(|r| r.time_ms).sum();
    let avg_time = total_time / total as f64;

    for r in &results {
        let status = if r.passed { "✅" } else { "❌" };
        let rule = r.rule_used.as_deref().unwrap_or("-");
        println!("║ {} {:30} {:8.2}ms  {}", status, r.name, r.time_ms, rule);
    }

    println!("╠═══════════════════════════════════════════════════════════╣");
    println!(
        "║ TOTAL: {}/{} passed ({:.1}%)                               ║",
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    );
    println!(
        "║ Average time: {:.2}ms                                      ║",
        avg_time
    );
    println!(
        "║ Total time: {:.2}ms                                        ║",
        total_time
    );
    println!("╚═══════════════════════════════════════════════════════════╝");

    // Category breakdown
    println!("\n📊 Category Breakdown:");
    println!(
        "  Algebraic Identities: {}/6",
        results[0..6].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Constant Folding: {}/5",
        results[6..11].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Trigonometry: {}/3",
        results[11..14].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Derivatives: {}/5",
        results[14..19].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Multi-Variable: {}/2",
        results[19..21].iter().filter(|r| r.passed).count()
    );
}
