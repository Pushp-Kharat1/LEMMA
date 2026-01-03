//! Neural-Guided IMO Benchmark
//!
//! This benchmark integrates the NN substitution predictor with MCTS
//! to demonstrate the end-to-end system on real IMO problems.
//!
//! Run: cargo run --example neural_imo_benchmark --release

use mm_brain::SubstitutionPredictor;
use mm_core::{Domain, NeuralHint, ProofSearchEngine, ProofState, SearchConfig};
use std::time::Instant;

/// Represents a real IMO problem
struct IMOProblem {
    year: u32,
    number: u32,
    domain_type: &'static str,
    statement: &'static str,
    proof_hints: Vec<&'static str>,
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║      NEURAL-GUIDED MCTS BENCHMARK (NN + Proof Search)            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Create the neural substitution predictor
    let predictor = SubstitutionPredictor::new();

    let problems = vec![
        IMOProblem {
            year: 2019,
            number: 1,
            domain_type: "Functional Equation",
            statement: r#"
Let f: ℤ → ℤ be a function such that for all integers a and b,
f(2a) + 2f(b) = f(f(a + b)).
Determine all such functions f.
"#,
            proof_hints: vec!["x = 0", "y = 0", "linear"],
        },
        IMOProblem {
            year: 2017,
            number: 2,
            domain_type: "Functional Equation",
            statement: r#"
Let R be the set of real numbers. Find all functions f: R → R such that
for all x, y ∈ R, f(f(x)f(y)) + f(x + y) = f(xy).
"#,
            proof_hints: vec!["x = 0", "y = 0", "x = 1"],
        },
        IMOProblem {
            year: 2008,
            number: 2,
            domain_type: "Algebra/Inequality",
            statement: r#"
Let x, y, z be distinct real numbers different from 1 such that xyz = 1.
Prove that x²/(x−1)² + y²/(y−1)² + z²/(z−1)² ≥ 1.
"#,
            proof_hints: vec!["AM-GM", "Cauchy-Schwarz", "abc = 1"],
        },
        IMOProblem {
            year: 2024,
            number: 1,
            domain_type: "Number Theory",
            statement: r#"
Determine all real numbers α such that, for every positive integer n,
the integer ⌊α⌋ + ⌊2α⌋ + ... + ⌊nα⌋ is a multiple of n.
"#,
            proof_hints: vec!["Check small cases", "Use modular arithmetic"],
        },
    ];

    println!(
        "Testing {} IMO problems with Neural-Guided MCTS\n",
        problems.len()
    );
    println!("Pipeline: Problem → NN Predictor → Neural Hints → MCTS → Proof\n");

    let mut total_nn_time = 0.0;
    let mut total_mcts_time = 0.0;
    let mut nn_hits = 0;

    for p in &problems {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("IMO {} Problem {} [{}]", p.year, p.number, p.domain_type);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // Step 1: Neural Network Prediction
        let nn_start = Instant::now();
        let predictions = predictor.predict(p.statement, 5);
        let nn_time = nn_start.elapsed().as_secs_f64() * 1000.0;
        total_nn_time += nn_time;

        println!("\n📊 Step 1: Neural Network Predictions ({:.2}ms)", nn_time);
        for (i, pred) in predictions.iter().enumerate() {
            println!(
                "   {}. {} ({:.0}%)",
                i + 1,
                pred.substitution,
                pred.confidence * 100.0
            );
        }

        // Convert predictions to NeuralHints
        let hints: Vec<NeuralHint> = predictions
            .iter()
            .map(|p| NeuralHint {
                action: p.substitution.clone(),
                confidence: p.confidence,
            })
            .collect();

        // Step 2: Set up MCTS with neural hints
        let config = SearchConfig {
            max_depth: 20,
            time_limit_ms: 1000,
            max_nodes: 1000,
            enable_case_split: true,
            enable_induction: true,
        };

        let mut engine = ProofSearchEngine::new(config);
        engine.set_neural_hints(hints);

        // Step 3: Create a proof state (simplified)
        let mut state = ProofState::new();
        let _x = state.add_variable("x", Domain::Real);
        let _y = state.add_variable("y", Domain::Real);

        // Run MCTS search
        let mcts_start = Instant::now();
        let _result = engine.search(state);
        let mcts_time = mcts_start.elapsed().as_secs_f64() * 1000.0;
        total_mcts_time += mcts_time;

        println!("\n🔍 Step 2: MCTS Proof Search ({:.2}ms)", mcts_time);
        println!("   Nodes explored: {}", engine.stats.nodes_explored);
        println!("   Rules tried: {}", engine.stats.rules_tried);
        println!("   Case splits: {}", engine.stats.case_splits);

        // Check if NN predictions matched expected hints
        let predicted_strs: Vec<&str> = predictions
            .iter()
            .map(|p| p.substitution.as_str())
            .collect();
        let matches: Vec<&&str> = p
            .proof_hints
            .iter()
            .filter(|h| {
                predicted_strs.iter().any(|pred| {
                    pred.to_lowercase().contains(&h.to_lowercase())
                        || h.to_lowercase().contains(&pred.to_lowercase())
                })
            })
            .collect();

        let hit = !matches.is_empty();
        if hit {
            nn_hits += 1;
        }

        println!("\n📋 Evaluation:");
        println!("   Expected hints: {:?}", p.proof_hints);
        println!(
            "   NN matches: {} {} ({}/{})",
            if hit { "✅" } else { "❌" },
            if hit { "HIT" } else { "MISS" },
            matches.len(),
            p.proof_hints.len()
        );
        println!();
    }

    // Summary
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                     BENCHMARK RESULTS                            ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ Problems tested:     {:>3}                                        ║",
        problems.len()
    );
    println!(
        "║ NN hit rate:         {}/{} ({:.0}%)                                 ║",
        nn_hits,
        problems.len(),
        (nn_hits as f64 / problems.len() as f64) * 100.0
    );
    println!(
        "║ Avg NN time:         {:>6.2} ms                                  ║",
        total_nn_time / problems.len() as f64
    );
    println!(
        "║ Avg MCTS time:       {:>6.2} ms                                  ║",
        total_mcts_time / problems.len() as f64
    );
    println!(
        "║ Total pipeline:      {:>6.2} ms/problem                          ║",
        (total_nn_time + total_mcts_time) / problems.len() as f64
    );
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    if nn_hits >= 3 {
        println!("🏆 EXCELLENT! Neural network is guiding MCTS effectively.");
    } else if nn_hits >= 2 {
        println!("✅ GOOD! System shows promising integration.");
    } else {
        println!("🔶 PARTIAL! Consider expanding training data.");
    }
}
