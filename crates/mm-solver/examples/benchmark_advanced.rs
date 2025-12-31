// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! LEMMA Advanced Benchmark - Multi-Step Problems
//!
//! Tests real math problems that students struggle with.

use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::time::Instant;

struct TestResult {
    name: String,
    passed: bool,
    steps: usize,
    time_ms: f64,
    input: String,
    output: String,
    expected: String,
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           LEMMA ADVANCED BENCHMARK - Multi-Step               ║");
    println!("║                   Real Math Problems                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200, // More simulations for complex problems
        exploration_weight: 1.41,
        max_depth: 20, // Deeper search for multi-step
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    let mut results: Vec<TestResult> = Vec::new();

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 1: MULTI-STEP ALGEBRA
    // ═══════════════════════════════════════════════════════════════
    println!("━━━ Category 1: Multi-Step Algebra ━━━");

    // Test 1: (x + 0) * 1 + 0 → x (3 steps)
    let expr = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
            Box::new(Expr::int(1)),
        )),
        Box::new(Expr::int(0)),
    );
    let start = Instant::now();
    let result = mcts.simplify(expr);
    results.push(TestResult {
        name: "(x+0)*1+0 → x".to_string(),
        passed: matches!(result.result, Expr::Var(_)),
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "(x+0)*1+0".to_string(),
        output: format!("{:?}", result.result),
        expected: "x".to_string(),
    });

    // Test 2: 2 * (3 + 4) → 14 (distribute then fold, or fold first)
    let expr = Expr::Mul(
        Box::new(Expr::int(2)),
        Box::new(Expr::Add(Box::new(Expr::int(3)), Box::new(Expr::int(4)))),
    );
    let start = Instant::now();
    let result = mcts.simplify(expr);
    results.push(TestResult {
        name: "2*(3+4) → 14".to_string(),
        passed: matches!(&result.result, Expr::Const(r) if r.numer() == 14),
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "2*(3+4)".to_string(),
        output: format!("{:?}", result.result),
        expected: "14".to_string(),
    });

    // Test 3: x^2 * x^3 → x^5 (power_add then const_fold)
    let expr = Expr::Mul(
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
    );
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let passed = match &result.result {
        Expr::Pow(_, exp) => matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 5),
        _ => false,
    };
    results.push(TestResult {
        name: "x^2 * x^3 → x^5".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "x^2 * x^3".to_string(),
        output: format!("{:?}", result.result),
        expected: "x^5".to_string(),
    });

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 2: CHAIN RULE / PRODUCT RULE
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 2: Calculus Multi-Step ━━━");

    // Test 4: d/dx(x^3) → 3x^2 (needs power_rule then potentially more)
    let expr = Expr::Derivative {
        expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
        var: x,
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    // Expect: 3 * x^2
    let passed = match &result.result {
        Expr::Mul(a, b) => {
            matches!(a.as_ref(), Expr::Const(r) if r.numer() == 3)
                && matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 2))
        }
        _ => false,
    };
    results.push(TestResult {
        name: "d/dx(x³) → 3x²".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "d/dx(x³)".to_string(),
        output: format!("{:?}", result.result),
        expected: "3x²".to_string(),
    });

    // Test 5: d/dx(x + 5) → 1 (sum rule then constant/power rule)
    let expr = Expr::Derivative {
        expr: Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(5)))),
        var: x,
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    // Could be Const(1) or Add(Const(1), Const(0)) simplified
    let passed = matches!(&result.result, Expr::Const(r) if r.numer() == 1);
    results.push(TestResult {
        name: "d/dx(x+5) → 1".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "d/dx(x+5)".to_string(),
        output: format!("{:?}", result.result),
        expected: "1".to_string(),
    });

    // Test 6: d/dx(2x) → 2 (constant times x)
    let expr = Expr::Derivative {
        expr: Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
        var: x,
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let passed = matches!(&result.result, Expr::Const(r) if r.numer() == 2);
    results.push(TestResult {
        name: "d/dx(2x) → 2".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "d/dx(2x)".to_string(),
        output: format!("{:?}", result.result),
        expected: "2".to_string(),
    });

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 3: EQUATION SOLVING
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 3: Equation Solving ━━━");

    // Test 7: x + 3 = 7 → x = 4
    let expr = Expr::Equation {
        lhs: Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
        rhs: Box::new(Expr::int(7)),
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    // Check if we get x = 4
    let passed = match &result.result {
        Expr::Equation { lhs, rhs } => {
            matches!(lhs.as_ref(), Expr::Var(_))
                && matches!(rhs.as_ref(), Expr::Const(r) if r.numer() == 4)
        }
        _ => false,
    };
    results.push(TestResult {
        name: "x + 3 = 7 → x = 4".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "x + 3 = 7".to_string(),
        output: format!("{:?}", result.result),
        expected: "x = 4".to_string(),
    });

    // Test 8: 2x = 10 → x = 5
    let expr = Expr::Equation {
        lhs: Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::Var(x)))),
        rhs: Box::new(Expr::int(10)),
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let passed = match &result.result {
        Expr::Equation { lhs, rhs } => {
            matches!(lhs.as_ref(), Expr::Var(_))
                && matches!(rhs.as_ref(), Expr::Const(r) if r.numer() == 5)
        }
        _ => false,
    };
    results.push(TestResult {
        name: "2x = 10 → x = 5".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "2x = 10".to_string(),
        output: format!("{:?}", result.result),
        expected: "x = 5".to_string(),
    });

    // ═══════════════════════════════════════════════════════════════
    // CATEGORY 4: TRIG MULTI-STEP
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Category 4: Trig Multi-Step ━━━");

    // Test 9: sin²(x) + cos²(x) + sin(0) → 1 (pythagorean + sin_zero)
    let expr = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(
                Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
        )),
        Box::new(Expr::Sin(Box::new(Expr::int(0)))),
    );
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let passed = matches!(&result.result, Expr::Const(r) if r.numer() == 1);
    results.push(TestResult {
        name: "sin²x + cos²x + sin(0) → 1".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "sin²x + cos²x + sin(0)".to_string(),
        output: format!("{:?}", result.result),
        expected: "1".to_string(),
    });

    // Test 10: d/dx(sin(x) + cos(x)) → cos(x) - sin(x)
    let expr = Expr::Derivative {
        expr: Box::new(Expr::Add(
            Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
            Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
        )),
        var: x,
    };
    let start = Instant::now();
    let result = mcts.simplify(expr);
    // Should get cos(x) + (-sin(x)) or equivalent
    let passed = match &result.result {
        Expr::Add(a, b) => {
            (matches!(a.as_ref(), Expr::Cos(_)) || matches!(b.as_ref(), Expr::Cos(_)))
                && (matches!(a.as_ref(), Expr::Neg(_)) || matches!(b.as_ref(), Expr::Neg(_)))
        }
        _ => false,
    };
    results.push(TestResult {
        name: "d/dx(sin(x)+cos(x)) → cos(x)-sin(x)".to_string(),
        passed,
        steps: result.steps.len(),
        time_ms: start.elapsed().as_secs_f64() * 1000.0,
        input: "d/dx(sin(x)+cos(x))".to_string(),
        output: format!("{:?}", result.result),
        expected: "cos(x) - sin(x)".to_string(),
    });

    // ═══════════════════════════════════════════════════════════════
    // RESULTS SUMMARY
    // ═══════════════════════════════════════════════════════════════
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                 ADVANCED BENCHMARK RESULTS                     ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");

    for r in &results {
        let status = if r.passed { "✅" } else { "❌" };
        println!(
            "║ {} {:35} {:2} steps  {:7.1}ms",
            status, r.name, r.steps, r.time_ms
        );
        if !r.passed {
            println!("║    Got: {}", truncate(&r.output, 55));
        }
    }

    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();
    let pct = (passed as f64 / total as f64) * 100.0;

    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!(
        "║ TOTAL: {}/{} passed ({:.1}%)                                  ║",
        passed, total, pct
    );
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Category breakdown
    println!("\n📊 Category Breakdown:");
    println!(
        "  Multi-Step Algebra: {}/3",
        results[0..3].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Calculus Multi-Step: {}/3",
        results[3..6].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Equation Solving: {}/2",
        results[6..8].iter().filter(|r| r.passed).count()
    );
    println!(
        "  Trig Multi-Step: {}/2",
        results[8..10].iter().filter(|r| r.passed).count()
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
