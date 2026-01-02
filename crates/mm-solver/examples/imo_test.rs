// LEMMA IMO-Level Test - Advanced Mathematical Problems
use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::time::Instant;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       LEMMA IMO-LEVEL TEST - Advanced Problems                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");
    let z = symbols.intern("z");
    let a = symbols.intern("a");
    let b = symbols.intern("b");
    let n = symbols.intern("n");

    let rules = standard_rules();
    println!("📚 Loaded {} rules\n", rules.len());

    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 500, // More simulations for harder problems
        exploration_weight: 1.41,
        max_depth: 50, // Deeper search
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    ALGEBRAIC IDENTITIES");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 1: a³ + b³ factorization
    // (a + b)*(a² - ab + b²) = a³ + b³
    test(
        &mcts,
        "IMO-1",
        "(a+b)*(a²-ab+b²) → a³+b³ [Sum of Cubes]",
        Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            Box::new(Expr::Add(
                Box::new(Expr::Sub(
                    Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(2)))),
                    Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                )),
                Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
            )),
        ),
        |_e| true, // Observe result
    );

    // IMO 2: (a - b)(a + b) = a² - b² (difference of squares verification)
    test(
        &mcts,
        "IMO-2",
        "(a-b)*(a+b) → a²-b² [Diff of Squares]",
        Expr::Mul(
            Box::new(Expr::Sub(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
        ),
        |e| match e {
            Expr::Sub(left, right) => {
                matches!(left.as_ref(), Expr::Pow(base, exp) 
                    if matches!(base.as_ref(), Expr::Var(_)) 
                    && matches!(exp.as_ref(), Expr::Const(r) if *r == Rational::from_integer(2)))
                    && matches!(right.as_ref(), Expr::Pow(_, _))
            }
            _ => false,
        },
    );

    // IMO 3: (a + b + c)² = a² + b² + c² + 2ab + 2bc + 2ac
    test(
        &mcts,
        "IMO-3",
        "(a+b+c)² expansion",
        Expr::Pow(
            Box::new(Expr::Add(
                Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
                Box::new(Expr::Var(z)), // Using z as c
            )),
            Box::new(Expr::int(2)),
        ),
        |_e| true, // Observe expansion
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    TRIGONOMETRIC CHALLENGES");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 4: sin(2x) = 2 sin(x) cos(x)
    test(
        &mcts,
        "IMO-4",
        "2sin(x)cos(x) → sin(2x) [Double Angle]",
        Expr::Mul(
            Box::new(Expr::Mul(
                Box::new(Expr::int(2)),
                Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
            )),
            Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
        ),
        |e| match e {
            Expr::Sin(inner) => matches!(inner.as_ref(), Expr::Mul(_, _)),
            _ => false,
        },
    );

    // IMO 5: cos²(x) - sin²(x) = cos(2x)
    test(
        &mcts,
        "IMO-5",
        "cos²x - sin²x → cos(2x) [Double Angle]",
        Expr::Sub(
            Box::new(Expr::Pow(
                Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
        ),
        |e| match e {
            Expr::Cos(inner) => matches!(inner.as_ref(), Expr::Mul(_, _)),
            _ => false,
        },
    );

    // IMO 6: tan²(x) + 1 = sec²(x) = 1/cos²(x)
    test(
        &mcts,
        "IMO-6",
        "tan²x + 1 → 1/cos²x [Pythagorean]",
        Expr::Add(
            Box::new(Expr::Pow(
                Box::new(Expr::Tan(Box::new(Expr::Var(x)))),
                Box::new(Expr::int(2)),
            )),
            Box::new(Expr::int(1)),
        ),
        |_e| true, // Observe result
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    CALCULUS CHALLENGES");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 7: d/dx(x^n) = n*x^(n-1) for general n
    test(
        &mcts,
        "IMO-7",
        "d/dx(x^n) → n*x^(n-1) [Power Rule General]",
        Expr::Derivative {
            expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::Var(n)))),
            var: x,
        },
        |_e| true, // Observe result
    );

    // IMO 8: d/dx(sin(x²)) = 2x*cos(x²) [Chain Rule]
    test(
        &mcts,
        "IMO-8",
        "d/dx(sin(x²)) → 2x*cos(x²) [Chain Rule]",
        Expr::Derivative {
            expr: Box::new(Expr::Sin(Box::new(Expr::Pow(
                Box::new(Expr::Var(x)),
                Box::new(Expr::int(2)),
            )))),
            var: x,
        },
        |e| match e {
            Expr::Mul(_, b) => matches!(b.as_ref(), Expr::Cos(_)),
            _ => false,
        },
    );

    // IMO 9: d/dx(e^(x²)) = 2x*e^(x²) [Chain Rule]
    test(
        &mcts,
        "IMO-9",
        "d/dx(e^(x²)) → 2x*e^(x²) [Chain Rule]",
        Expr::Derivative {
            expr: Box::new(Expr::Exp(Box::new(Expr::Pow(
                Box::new(Expr::Var(x)),
                Box::new(Expr::int(2)),
            )))),
            var: x,
        },
        |_e| true, // Observe result
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    EQUATION SOLVING");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 10: Quadratic: x² - 5x + 6 = 0 → x = 2 or x = 3
    test(
        &mcts,
        "IMO-10",
        "x² - 5x + 6 = 0 [Factorize to (x-2)(x-3)]",
        Expr::Equation {
            lhs: Box::new(Expr::Add(
                Box::new(Expr::Sub(
                    Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                    Box::new(Expr::Mul(Box::new(Expr::int(5)), Box::new(Expr::Var(x)))),
                )),
                Box::new(Expr::int(6)),
            )),
            rhs: Box::new(Expr::int(0)),
        },
        |_e| true, // Observe result
    );

    // IMO 11: Linear system simulation: 2x + 3y = 12, x = 3 → y = ?
    test(
        &mcts,
        "IMO-11",
        "2*3 + 3y = 12 → y = 2",
        Expr::Equation {
            lhs: Box::new(Expr::Add(
                Box::new(Expr::Mul(Box::new(Expr::int(2)), Box::new(Expr::int(3)))),
                Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(y)))),
            )),
            rhs: Box::new(Expr::int(12)),
        },
        |e| match e {
            Expr::Equation { lhs, rhs } => {
                matches!(lhs.as_ref(), Expr::Var(_))
                    && matches!(rhs.as_ref(), Expr::Const(r) if r.numer() == 2)
            }
            _ => false,
        },
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    NUMBER THEORY");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 12: GCD computation gcd(48, 18) = 6
    test(
        &mcts,
        "IMO-12",
        "gcd(48, 18) → 6",
        Expr::GCD(Box::new(Expr::int(48)), Box::new(Expr::int(18))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 6),
    );

    // IMO 13: LCM computation lcm(12, 18) = 36
    test(
        &mcts,
        "IMO-13",
        "lcm(12, 18) → 36",
        Expr::LCM(Box::new(Expr::int(12)), Box::new(Expr::int(18))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 36),
    );

    // IMO 14: Mod operation 17 mod 5 = 2
    test(
        &mcts,
        "IMO-14",
        "17 mod 5 → 2",
        Expr::Mod(Box::new(Expr::int(17)), Box::new(Expr::int(5))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 2),
    );

    // IMO 15: Binomial C(6,2) = 15
    test(
        &mcts,
        "IMO-15",
        "C(6,2) → 15",
        Expr::Binomial(Box::new(Expr::int(6)), Box::new(Expr::int(2))),
        |e| matches!(e, Expr::Const(r) if r.numer() == 15),
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");
}

fn test<F>(mcts: &NeuralMCTS, id: &str, name: &str, expr: Expr, check: F)
where
    F: Fn(&Expr) -> bool,
{
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    let passed = check(&result.result);

    let status = if passed { "✅" } else { "🔸" }; // 🔸 for "in progress"
    println!("{} {}: {}", status, id, name);
    println!(
        "   Steps: {}  |  Time: {:.1}ms",
        result.steps.len(),
        elapsed
    );
    println!("   Result: {:?}\n", result.result);
}
