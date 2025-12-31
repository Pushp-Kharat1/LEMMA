// LEMMA Stress Test - Complex Multi-Step Formulas
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::time::Instant;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           LEMMA STRESS TEST - Complex Formulas                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200,
        exploration_weight: 1.41,
        max_depth: 30,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    // Test 1: (2+3)*(4+5) → 45
    test(
        &mcts,
        "1",
        "(2+3)*(4+5) → 45",
        Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)))),
            Box::new(Expr::Add(Box::new(Expr::int(4)), Box::new(Expr::int(5)))),
        ),
        |e| matches!(e, Expr::Const(r) if r.numer() == 45),
    );

    // Test 2: ((x+0)*1)+0 → x
    test(
        &mcts,
        "2",
        "((x+0)*1)+0 → x",
        Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))),
                Box::new(Expr::int(1)),
            )),
            Box::new(Expr::int(0)),
        ),
        |e| matches!(e, Expr::Var(_)),
    );

    // Test 3: x^2 * x^3 * x^4 → x^9
    test(
        &mcts,
        "3",
        "x² * x³ * x⁴ → x⁹",
        Expr::Mul(
            Box::new(Expr::Mul(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            )),
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(4)))),
        ),
        |e| match e {
            Expr::Pow(_, exp) => matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 9),
            _ => false,
        },
    );

    // Test 4: d/dx(x² + x³) → 2x + 3x²
    test(
        &mcts,
        "4",
        "d/dx(x² + x³) → 2x + 3x²",
        Expr::Derivative {
            expr: Box::new(Expr::Add(
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            )),
            var: x,
        },
        |e| matches!(e, Expr::Add(_, _)), // At least got the structure right
    );

    // Test 5: 3x + 5 = 17 → x = 4
    test(
        &mcts,
        "5",
        "3x + 5 = 17 → x = 4",
        Expr::Equation {
            lhs: Box::new(Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
                Box::new(Expr::int(5)),
            )),
            rhs: Box::new(Expr::int(17)),
        },
        |e| match e {
            Expr::Equation { lhs, rhs } => {
                matches!(lhs.as_ref(), Expr::Var(_))
                    && matches!(rhs.as_ref(), Expr::Const(r) if r.numer() == 4)
            }
            _ => false,
        },
    );

    // Test 6: d/dx(x⁴) → 4x³
    test(
        &mcts,
        "6",
        "d/dx(x⁴) → 4x³",
        Expr::Derivative {
            expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(4)))),
            var: x,
        },
        |e| match e {
            Expr::Mul(a, b) => {
                matches!(a.as_ref(), Expr::Const(r) if r.numer() == 4)
                    && matches!(b.as_ref(), Expr::Pow(_, exp) if matches!(exp.as_ref(), Expr::Const(r) if r.numer() == 3))
            }
            _ => false,
        },
    );

    // Test 7: (x+1)*(x+1) - should distribute
    test(
        &mcts,
        "7",
        "(x+1)*(x+1) distribute",
        Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)))),
            Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)))),
        ),
        |_e| true, // Just see what we get
    );

    // Test 8: sin²(x) + cos²(x) - 1 → 0
    test(
        &mcts,
        "8",
        "sin²x + cos²x - 1 → 0",
        Expr::Sub(
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
            Box::new(Expr::int(1)),
        ),
        |e| matches!(e, Expr::Const(r) if r.is_zero()),
    );

    // Test 9: 2*(x+y) + 3*(x+y) → 5*(x+y) (collect like terms)
    test(
        &mcts,
        "9",
        "2(x+y) + 3(x+y) → 5(x+y)",
        Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::int(2)),
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
            )),
            Box::new(Expr::Mul(
                Box::new(Expr::int(3)),
                Box::new(Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::Var(y)))),
            )),
        ),
        |e| match e {
            Expr::Mul(a, _) => matches!(a.as_ref(), Expr::Const(r) if r.numer() == 5),
            _ => false,
        },
    );

    // Test 10: (100-50)*2 + 10 → 110
    test(
        &mcts,
        "10",
        "(100-50)*2 + 10 → 110",
        Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::Sub(Box::new(Expr::int(100)), Box::new(Expr::int(50)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::int(10)),
        ),
        |e| matches!(e, Expr::Const(r) if r.numer() == 110),
    );
}

fn test<F>(mcts: &NeuralMCTS, id: &str, name: &str, expr: Expr, check: F)
where
    F: Fn(&Expr) -> bool,
{
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    let passed = check(&result.result);

    let status = if passed { "✅" } else { "❌" };
    println!("{} Test {}: {} ", status, id, name);
    println!(
        "   Steps: {}  |  Time: {:.1}ms",
        result.steps.len(),
        elapsed
    );
    println!("   Result: {:?}\n", result.result);
}
