// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Test with EXACT expressions the model was trained on
//! These are the same patterns from data.rs DataGenerator

use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   LEMMA: Testing with EXACT Training Data Patterns         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

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

    let mut passed = 0;
    let mut total = 0;

    // ═══════════════════════════════════════════════════════════════
    // CONSTANT FOLDING (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("━━━ Constant Folding (trained pattern) ━━━");

    let tests_const = vec![
        (
            "5 + 7",
            Expr::Add(Box::new(Expr::int(5)), Box::new(Expr::int(7))),
        ),
        (
            "12 - 4",
            Expr::Sub(Box::new(Expr::int(12)), Box::new(Expr::int(4))),
        ),
        (
            "3 * 8",
            Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::int(8))),
        ),
        (
            "20 / 5",
            Expr::Div(Box::new(Expr::int(20)), Box::new(Expr::int(5))),
        ),
        (
            "2^4",
            Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(4))),
        ),
    ];

    for (name, expr) in tests_const {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let is_const = matches!(result.result, Expr::Const(_));
        if is_const {
            passed += 1;
        }
        let status = if is_const { "✅" } else { "❌" };
        println!("  {} {} → {:?}", status, name, result.result);
    }

    // ═══════════════════════════════════════════════════════════════
    // IDENTITY RULES (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Identity Rules (trained pattern) ━━━");

    let tests_identity = vec![
        (
            "x + 0",
            Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        ),
        (
            "0 + x",
            Expr::Add(Box::new(Expr::int(0)), Box::new(Expr::Var(x))),
        ),
        (
            "x * 1",
            Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(1))),
        ),
        (
            "1 * x",
            Expr::Mul(Box::new(Expr::int(1)), Box::new(Expr::Var(x))),
        ),
        (
            "x * 0",
            Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0))),
        ),
        (
            "0 * x",
            Expr::Mul(Box::new(Expr::int(0)), Box::new(Expr::Var(x))),
        ),
    ];

    for (name, expr) in tests_identity {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let simplified = result.result != expr;
        if simplified || result.verified {
            passed += 1;
        }
        let status = if simplified || result.verified {
            "✅"
        } else {
            "❌"
        };
        let rule = result.steps.first().map(|s| s.rule_name).unwrap_or("-");
        println!("  {} {} → {:?} ({})", status, name, result.result, rule);
    }

    // ═══════════════════════════════════════════════════════════════
    // DISTRIBUTION (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Distribution (trained pattern) ━━━");

    let tests_dist = vec![
        (
            "3 * (x + y)",
            Expr::Mul(
                Box::new(Expr::int(3)),
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
            ),
        ),
        (
            "(x + y) * 5",
            Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
                Box::new(Expr::int(5)),
            ),
        ),
    ];

    for (name, expr) in tests_dist {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let simplified = result.steps.len() > 0;
        if simplified {
            passed += 1;
        }
        let status = if simplified { "✅" } else { "⚠️" };
        let rule = result.steps.first().map(|s| s.rule_name).unwrap_or("-");
        println!("  {} {} → {:?} ({})", status, name, result.result, rule);
    }

    // ═══════════════════════════════════════════════════════════════
    // DIFFERENCE OF SQUARES (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Difference of Squares (trained pattern) ━━━");

    let tests_diff_sq = vec![
        (
            "x² - y²",
            Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
            ),
        ),
        (
            "x² - 4",
            Expr::Sub(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(2)))),
            ),
        ),
    ];

    for (name, expr) in tests_diff_sq {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let did_something = result.steps.len() > 0 || result.result != expr;
        if did_something {
            passed += 1;
        }
        let status = if did_something { "✅" } else { "⚠️" };
        let rule = result.steps.first().map(|s| s.rule_name).unwrap_or("-");
        println!("  {} {} → {:?} ({})", status, name, result.result, rule);
    }

    // ═══════════════════════════════════════════════════════════════
    // PYTHAGOREAN IDENTITY (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Trig Identities (trained pattern) ━━━");

    let tests_trig = vec![
        (
            "sin²(x) + cos²(x)",
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
        ),
        (
            "cos²(x) + sin²(x)",
            Expr::Add(
                Box::new(Expr::Pow(
                    Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2)),
                )),
                Box::new(Expr::Pow(
                    Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                    Box::new(Expr::int(2)),
                )),
            ),
        ),
    ];

    for (name, expr) in tests_trig {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let is_one = matches!(&result.result, Expr::Const(r) if r.numer() == 1 && r.denom() == 1);
        if is_one {
            passed += 1;
        }
        let status = if is_one { "✅" } else { "❌" };
        let rule = result.steps.first().map(|s| s.rule_name).unwrap_or("-");
        println!("  {} {} → {:?} ({})", status, name, result.result, rule);
    }

    // ═══════════════════════════════════════════════════════════════
    // POWER RULES (exactly as trained)
    // ═══════════════════════════════════════════════════════════════
    println!("\n━━━ Power Rules (trained pattern) ━━━");

    let tests_power = vec![
        (
            "x^2 * x^3",
            Expr::Mul(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            ),
        ),
        (
            "(x^2)^3",
            Expr::Pow(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::int(3)),
            ),
        ),
    ];

    for (name, expr) in tests_power {
        total += 1;
        let result = mcts.simplify(expr.clone());
        let did_something = result.steps.len() > 0;
        if did_something {
            passed += 1;
        }
        let status = if did_something { "✅" } else { "⚠️" };
        let rule = result.steps.first().map(|s| s.rule_name).unwrap_or("-");
        println!("  {} {} → {:?} ({})", status, name, result.result, rule);
    }

    // ═══════════════════════════════════════════════════════════════
    // RESULTS
    // ═══════════════════════════════════════════════════════════════
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    RESULTS                                  ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!(
        "║  Passed: {}/{} ({:.1}%)                                    ║",
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    );
    println!("╚════════════════════════════════════════════════════════════╝");
}
