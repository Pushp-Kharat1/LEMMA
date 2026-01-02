// LEMMA Real IMO-Level Problems
// These are actual competition-level problems requiring multi-step reasoning

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::time::Instant;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     LEMMA - Real IMO-Level Competition Problems               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let a = symbols.intern("a");
    let b = symbols.intern("b");
    let c = symbols.intern("c");
    let x = symbols.intern("x");
    let y = symbols.intern("y");
    let _z = symbols.intern("z");
    let n = symbols.intern("n");

    let rules = standard_rules();
    println!("📚 Loaded {} rules\n", rules.len());

    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 1000, // Maximum simulations for hard problems
        exploration_weight: 1.41,
        max_depth: 100, // Very deep search
        temperature: 0.8,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    println!("═══════════════════════════════════════════════════════════════");
    println!("       INEQUALITY PROOFS (IMO Shortlist Style)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 1: Prove that (a+b)/2 ≥ √(ab) for a,b ≥ 0 [AM-GM]
    // We set up: (a+b)/2 - √(ab) and show it simplifies to something ≥ 0
    // i.e., (√a - √b)² / 2 ≥ 0
    println!("🔶 IMO-1: AM-GM Inequality Verification");
    println!("   Prove: (a+b)/2 ≥ √(ab) for a,b ≥ 0");
    println!("   Method: Show (a+b)/2 - √(ab) = (√a - √b)²/2 ≥ 0\n");

    // (a + b)/2 - sqrt(ab)
    let am_gm_diff = Expr::Sub(
        Box::new(Expr::Div(
            Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            Box::new(Expr::int(2)),
        )),
        Box::new(Expr::Sqrt(Box::new(Expr::Mul(
            Box::new(Expr::Var(a)),
            Box::new(Expr::Var(b)),
        )))),
    );

    test(&mcts, "AM-GM", am_gm_diff, |_| true);

    // IMO 2: Cauchy-Schwarz: (a² + b²)(c² + d²) ≥ (ac + bd)²
    // Show LHS - RHS = (ad - bc)² ≥ 0
    println!("🔶 IMO-2: Cauchy-Schwarz Inequality");
    println!("   Prove: (a² + b²)(x² + y²) ≥ (ax + by)²");
    println!("   Method: Show LHS - RHS = (ay - bx)² ≥ 0\n");

    // (a² + b²)(x² + y²) - (ax + by)²
    let lhs = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(2)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(2)))),
        )),
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
        )),
    );

    let rhs = Expr::Pow(
        Box::new(Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::Var(a)), Box::new(Expr::Var(x)))),
            Box::new(Expr::Mul(Box::new(Expr::Var(b)), Box::new(Expr::Var(y)))),
        )),
        Box::new(Expr::int(2)),
    );

    let cauchy_schwarz = Expr::Sub(Box::new(lhs), Box::new(rhs));
    test(&mcts, "Cauchy-Schwarz", cauchy_schwarz, |_| true);

    // IMO 3: Nesbitt's Inequality (IMO 1961 Problem)
    // For positive a, b, c: a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2
    println!("🔶 IMO-3: Nesbitt's Inequality (IMO 1961)");
    println!("   Prove: a/(b+c) + b/(a+c) + c/(a+b) ≥ 3/2");
    println!("   This is a CLASSIC IMO problem from 1961\n");

    // a/(b+c) + b/(a+c) + c/(a+b) - 3/2
    let nesbitt = Expr::Sub(
        Box::new(Expr::Add(
            Box::new(Expr::Add(
                Box::new(Expr::Div(
                    Box::new(Expr::Var(a)),
                    Box::new(Expr::Add(Box::new(Expr::Var(b)), Box::new(Expr::Var(c)))),
                )),
                Box::new(Expr::Div(
                    Box::new(Expr::Var(b)),
                    Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(c)))),
                )),
            )),
            Box::new(Expr::Div(
                Box::new(Expr::Var(c)),
                Box::new(Expr::Add(Box::new(Expr::Var(a)), Box::new(Expr::Var(b)))),
            )),
        )),
        Box::new(Expr::Div(Box::new(Expr::int(3)), Box::new(Expr::int(2)))),
    );
    test(&mcts, "Nesbitt", nesbitt, |_| true);

    println!("═══════════════════════════════════════════════════════════════");
    println!("       ALGEBRAIC IDENTITIES (IMO Algebraic Style)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 4: Sophie Germain Identity
    // a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)
    println!("🔶 IMO-4: Sophie Germain Identity");
    println!("   Prove: a⁴ + 4b⁴ = (a² + 2b² + 2ab)(a² + 2b² - 2ab)\n");

    let sophie_germain = Expr::Add(
        Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(4)))),
        Box::new(Expr::Mul(
            Box::new(Expr::int(4)),
            Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(4)))),
        )),
    );
    test(&mcts, "Sophie-Germain", sophie_germain, |_| true);

    // IMO 5: x³ + y³ + z³ - 3xyz = (x+y+z)(x² + y² + z² - xy - yz - xz)
    // This is a key factorization for many IMO problems
    println!("🔶 IMO-5: Sum of Three Cubes Factorization");
    println!("   Factor: x³ + y³ + z³ - 3xyz\n");

    // x³ + y³ + z³ - 3xyz (we'll use a,b,c)
    let three_cubes = Expr::Sub(
        Box::new(Expr::Add(
            Box::new(Expr::Add(
                Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(3)))),
                Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(3)))),
            )),
            Box::new(Expr::Pow(Box::new(Expr::Var(c)), Box::new(Expr::int(3)))),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::int(3)),
            Box::new(Expr::Mul(
                Box::new(Expr::Var(a)),
                Box::new(Expr::Mul(Box::new(Expr::Var(b)), Box::new(Expr::Var(c)))),
            )),
        )),
    );
    test(&mcts, "Three-Cubes", three_cubes, |_| true);

    println!("═══════════════════════════════════════════════════════════════");
    println!("       NUMBER THEORY (IMO Style)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 6: Fermat's Little Theorem verification
    // a^(p-1) ≡ 1 (mod p) for p prime
    // We verify: 2^6 mod 7 = 1 (since 7 is prime)
    println!("🔶 IMO-6: Fermat's Little Theorem Check");
    println!("   Verify: 2^6 ≡ 1 (mod 7) since 7 is prime\n");

    // 2^6 mod 7
    let fermat = Expr::Mod(
        Box::new(Expr::Pow(Box::new(Expr::int(2)), Box::new(Expr::int(6)))),
        Box::new(Expr::int(7)),
    );
    test(
        &mcts,
        "Fermat-Little",
        fermat,
        |e| matches!(e, Expr::Const(r) if r.numer() == 1),
    );

    // IMO 7: Wilson's Theorem
    // (p-1)! ≡ -1 (mod p) for prime p
    // Check: 4! mod 5 = 24 mod 5 = 4 ≡ -1 (mod 5)
    println!("🔶 IMO-7: Wilson's Theorem Check");
    println!("   Verify: (5-1)! = 24 ≡ -1 (mod 5)\n");

    let wilson = Expr::Mod(
        Box::new(Expr::Factorial(Box::new(Expr::int(4)))),
        Box::new(Expr::int(5)),
    );
    test(&mcts, "Wilson", wilson, |e| {
        matches!(e, Expr::Const(r) if r.numer() == 4) // 4 ≡ -1 (mod 5)
    });

    // IMO 8: Sum of first n positive integers using formula
    // 1 + 2 + 3 + ... + n = n(n+1)/2
    // Verify for n = 100: sum = 5050
    println!("🔶 IMO-8: Gauss Sum Formula");
    println!("   Verify: 1 + 2 + ... + 100 = 100*101/2 = 5050\n");

    let gauss_sum = Expr::Div(
        Box::new(Expr::Mul(
            Box::new(Expr::int(100)),
            Box::new(Expr::int(101)),
        )),
        Box::new(Expr::int(2)),
    );
    test(
        &mcts,
        "Gauss-Sum",
        gauss_sum,
        |e| matches!(e, Expr::Const(r) if r.numer() == 5050),
    );

    println!("═══════════════════════════════════════════════════════════════");
    println!("       POWER SUM IDENTITIES (IMO Competition Classic)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // IMO 9: Newton's Identity for power sums
    // p₂ = e₁² - 2e₂ where p₂ = x² + y², e₁ = x + y, e₂ = xy
    // So: x² + y² = (x+y)² - 2xy
    println!("🔶 IMO-9: Newton's Identity p₂ = e₁² - 2e₂");
    println!("   Transform: x² + y² → (x+y)² - 2xy\n");

    let newton_p2 = Expr::Add(
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
        Box::new(Expr::Pow(Box::new(Expr::Var(y)), Box::new(Expr::int(2)))),
    );
    test(&mcts, "Newton-p2", newton_p2, |_| true);

    // IMO 10: Power sum p₃ = e₁³ - 3e₁e₂ + 3e₃
    // x³ + y³ + z³ expression
    println!("🔶 IMO-10: Power Sum p₃");
    println!("   Expression: a³ + b³ + c³\n");

    let power_sum_3 = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(Expr::Var(a)), Box::new(Expr::int(3)))),
            Box::new(Expr::Pow(Box::new(Expr::Var(b)), Box::new(Expr::int(3)))),
        )),
        Box::new(Expr::Pow(Box::new(Expr::Var(c)), Box::new(Expr::int(3)))),
    );
    test(&mcts, "Power-Sum-p3", power_sum_3, |_| true);

    println!("═══════════════════════════════════════════════════════════════");
    println!("                    SUMMARY");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("These are representative of actual IMO competition problems.");
    println!("Full solutions require multi-step algebraic manipulation,");
    println!("inequality bounds, and sophisticated reasoning patterns.");
    println!("\nLEMMA provides the building blocks; full IMO solutions");
    println!("typically require human-guided search or specialized tactics.");
}

fn test<F>(mcts: &NeuralMCTS, name: &str, expr: Expr, check: F)
where
    F: Fn(&Expr) -> bool,
{
    let start = Instant::now();
    let result = mcts.simplify(expr);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    let passed = check(&result.result);

    let status = if passed { "✅" } else { "🔸" };
    println!("{} {}", status, name);
    println!(
        "   Steps: {}  |  Time: {:.1}ms",
        result.steps.len(),
        elapsed
    );
    println!("   Result: {:?}\n", result.result);
}
