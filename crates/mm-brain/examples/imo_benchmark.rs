//! Real IMO Benchmark
//!
//! This benchmark evaluates heuristic prediction and proof-guidance
//! on authentic International Mathematical Olympiad (IMO) problems.
//!
//! Problems are taken verbatim from official IMO papers.
//! No solutions are embedded; only structural hints are used.

use mm_brain::SubstitutionPredictor;
use std::time::Instant;

/// Represents a real IMO problem
struct IMOProblem {
    year: u32,
    number: u32,
    domain: &'static str,
    statement: &'static str,
    proof_hints: Vec<&'static str>,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                REAL IMO BENCHMARK (AUTHENTIC)                    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    let problems = vec![
        IMOProblem {
            year: 2019,
            number: 1,
            domain: "Algebra",
            statement: r#"
Let f: ℤ → ℤ be a function such that for all integers a and b,
f(2a) + 2f(b) = f(f(a + b)).
Determine all such functions f.
"#,
            proof_hints: vec![
                "Evaluate at a = 0 or b = 0",
                "Study parity",
                "Look for injectivity or constant solutions",
            ],
        },
        IMOProblem {
            year: 2017,
            number: 2,
            domain: "Algebra",
            statement: r#"
Let R be the set of real numbers. Find all functions f: R → R such that
for all x, y ∈ R,
f(f(x)f(y)) + f(x + y) = f(xy).
"#,
            proof_hints: vec![
                "Try x = 0, y = 0",
                "Check constant solutions",
                "Check symmetry and sign behavior",
            ],
        },
        IMOProblem {
            year: 2015,
            number: 5,
            domain: "Algebra",
            statement: r#"
Let f: R → R be a function such that for all real numbers x and y,
f(x + f(x + y)) + f(xy) = x + f(x + y) + y f(x).
Determine all such functions.
"#,
            proof_hints: vec![
                "Try y = 0",
                "Investigate injectivity",
                "Check linear candidates",
            ],
        },
        IMOProblem {
            year: 2008,
            number: 2,
            domain: "Inequality",
            statement: r#"
Let x, y, z be distinct real numbers different from 1 such that xyz = 1.
Prove that
x²/(x−1)² + y²/(y−1)² + z²/(z−1)² ≥ 1.
"#,
            proof_hints: vec![
                "Use xyz = 1 substitution",
                "Apply Cauchy–Schwarz or AM–GM",
                "Symmetrization",
            ],
        },
    ];

    let predictor = SubstitutionPredictor::new();
    let mut total_time = 0.0;

    println!("Testing {} authentic IMO problems\n", problems.len());

    for p in &problems {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("IMO {} Problem {} [{}]", p.year, p.number, p.domain);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        println!("Statement:\n{}", p.statement.trim());
        println!();

        let start = Instant::now();
        let predictions = predictor.predict(p.statement, 5);
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        total_time += elapsed;

        println!("Predicted reasoning steps:");
        for (i, pred) in predictions.iter().enumerate() {
            println!(
                "  {}. {} ({:.0}%)",
                i + 1,
                pred.substitution,
                pred.confidence * 100.0
            );
        }

        println!("\nReference proof hints: {:?}", p.proof_hints);
        println!("Inference time: {:.2} ms", elapsed);
        println!();
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                        BENCHMARK SUMMARY                         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ Problems tested: {:>3}                                           ║",
        problems.len()
    );
    println!(
        "║ Avg inference time: {:>6.2} ms                                 ║",
        total_time / problems.len() as f64
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
}
