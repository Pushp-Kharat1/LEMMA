//! Generate synthetic training data for LEMMA
//!
//! This example generates a large dataset of synthetic problems
//! for training the neural networks.
//!
//! Usage: cargo run --example generate_data --release -p mm-synth -- --count 100000

use mm_synth::{GeneratorConfig, ProblemCategory, ProblemGenerator};
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let count: usize = args
        .iter()
        .position(|a| a == "--count")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║       LEMMA Synthetic Data Generator                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Generating {} problems...", count);

    let config = GeneratorConfig {
        num_problems: count,
        seed: 42,
        max_depth: 5,
        categories: vec![
            ProblemCategory::FunctionalEquation,
            ProblemCategory::Algebra,
            ProblemCategory::Inequality,
            ProblemCategory::NumberTheory,
        ],
    };

    let mut generator = ProblemGenerator::new(config);
    let start = std::time::Instant::now();
    let problems = generator.generate_all();
    let elapsed = start.elapsed();

    println!(
        "Generated {} problems in {:.2}s",
        problems.len(),
        elapsed.as_secs_f64()
    );
    println!();

    // Count by category
    let mut counts = std::collections::HashMap::new();
    for p in &problems {
        *counts.entry(&p.category).or_insert(0) += 1;
    }

    println!("By category:");
    for (cat, count) in &counts {
        println!("  {:?}: {}", cat, count);
    }

    // Save to JSON
    let output_path = format!("data/synthetic_{}.json", problems.len());
    std::fs::create_dir_all("data").ok();

    let file = File::create(&output_path).expect("Failed to create output file");
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &problems).expect("Failed to write JSON");

    println!();
    println!("Saved to {}", output_path);

    // Show samples
    println!();
    println!("Sample problems:");
    println!("─────────────────────────────────────────────────────────────────");
    for (i, p) in problems.iter().take(5).enumerate() {
        println!(
            "{}. [{}] {}",
            i + 1,
            format!("{:?}", p.category),
            &p.statement[..p.statement.len().min(70)]
        );
        println!("   → {:?}", p.substitutions);
    }
}
