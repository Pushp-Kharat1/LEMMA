// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! BEAST MODE Training - 100K+ examples, large transformer
//!
//! Designed for GPU training (RunPod, Lambda Labs, etc.)
//! Expected training time: ~30 min on T4, ~15 min on A40

use candle_core::Device;
use mm_brain::{
    data::DataGenerator,
    network::NetworkConfig,
    training::{Trainer, TrainingConfig},
};
use mm_core::Expr;

fn main() {
    println!("=== LEMMA: BEAST MODE Training ===\n");
    println!("WARNING: This is designed for GPU training!");
    println!("On CPU, this will take DAYS. Use RunPod or similar.\n");

    // Detect device - prefer CUDA if available
    let device = if cfg!(feature = "cuda") {
        match Device::new_cuda(0) {
            Ok(d) => {
                println!("Using device: CUDA GPU 0");
                d
            }
            Err(_) => {
                println!("CUDA not available, falling back to CPU");
                Device::Cpu
            }
        }
    } else {
        println!("Using device: CPU (compile with --features cuda for GPU)");
        Device::Cpu
    };

    // =========================================================================
    // BEAST MODE: 100K+ Dataset
    // =========================================================================
    println!("\n--- Generating BEAST MODE Dataset (100K+) ---");
    let mut generator = DataGenerator::new(device.clone());

    // 3000 samples per category = ~100K total examples
    let examples = generator.generate_dataset(3000);
    println!("Generated {} training examples", examples.len());
    println!(
        "Dataset size: {:.2} MB (approx)",
        (examples.len() * std::mem::size_of::<mm_brain::training::TrainingExample>()) as f64
            / 1_000_000.0
    );

    // =========================================================================
    // BEAST MODE: Large Transformer
    // =========================================================================
    let network_config = NetworkConfig {
        vocab_size: 128,  // Larger vocabulary
        embed_dim: 256,   // 256 embedding dimension
        hidden_dim: 512,  // 512 hidden dimension
        num_heads: 16,    // 16 attention heads
        num_layers: 6,    // 6 transformer layers
        max_seq_len: 128, // Longer sequences
        num_rules: 30,    // 29 rules + 1 no-op
        dropout: 0.1,
    };

    println!("\n--- Model Configuration ---");
    println!("  Embedding dim: {}", network_config.embed_dim);
    println!("  Hidden dim: {}", network_config.hidden_dim);
    println!("  Attention heads: {}", network_config.num_heads);
    println!("  Transformer layers: {}", network_config.num_layers);
    println!("  Max sequence length: {}", network_config.max_seq_len);

    // Estimate parameters
    let params_estimate = network_config.vocab_size * network_config.embed_dim +  // Token embedding
        network_config.max_seq_len * network_config.embed_dim + // Position embedding
        network_config.num_layers * (
            4 * network_config.embed_dim * network_config.embed_dim + // Attention
            2 * network_config.embed_dim * network_config.hidden_dim  // FFN
        ) +
        network_config.embed_dim * network_config.num_rules +   // Policy head
        network_config.embed_dim; // Value head
    println!(
        "  Estimated parameters: ~{:.2}M",
        params_estimate as f64 / 1_000_000.0
    );

    // =========================================================================
    // Training Configuration
    // =========================================================================
    let training_config = TrainingConfig {
        learning_rate: 3e-4, // AdamW default for transformers
        weight_decay: 0.01,
        batch_size: 128, // Larger batch for GPU
        epochs: 100,     // More epochs
        value_weight: 0.5,
    };

    println!("\n--- Training Configuration ---");
    println!("  Learning rate: {}", training_config.learning_rate);
    println!("  Batch size: {}", training_config.batch_size);
    println!("  Epochs: {}", training_config.epochs);
    println!(
        "  Total batches: ~{}",
        examples.len() / training_config.batch_size * training_config.epochs
    );

    // =========================================================================
    // Create Trainer
    // =========================================================================
    println!("\n--- Creating Trainer ---");
    let mut trainer = match Trainer::new(network_config.clone(), training_config, device.clone()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create trainer: {}", e);
            return;
        }
    };

    // =========================================================================
    // Train!
    // =========================================================================
    println!("\n--- BEAST MODE Training ---");
    println!("Estimated time: ~30 min on T4 GPU, ~15 min on A40");
    println!("On CPU: DON'T. Just don't.\n");

    match trainer.train(&examples) {
        Ok(history) => {
            println!("\n=== Training Complete! ===");
            if let Some((final_policy, final_value)) = history.last() {
                println!(
                    "Final losses: policy={:.4}, value={:.4}",
                    final_policy, final_value
                );
            }
        }
        Err(e) => {
            eprintln!("Training failed: {}", e);
            return;
        }
    }

    // =========================================================================
    // Save Model
    // =========================================================================
    println!("\n--- Saving BEAST Model ---");
    let model_path = "lemma_beast_model.safetensors";
    match trainer.save(model_path) {
        Ok(()) => println!("Model saved to: {}", model_path),
        Err(e) => eprintln!("Failed to save model: {}", e),
    }

    // =========================================================================
    // Quick Inference Test
    // =========================================================================
    println!("\n--- Quick Inference Test ---");

    let network = trainer.network();
    let encoder = trainer.encoder();

    let test_exprs = vec![
        (
            "2 + 3",
            Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3))),
        ),
        ("x + 0", {
            let mut symbols = mm_core::SymbolTable::new();
            let x = symbols.intern("x");
            Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))
        }),
        ("d/dx(x^3)", {
            let mut symbols = mm_core::SymbolTable::new();
            let x = symbols.intern("x");
            Expr::Derivative {
                expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
                var: x,
            }
        }),
    ];

    for (name, expr) in test_exprs {
        println!("\nExpression: {}", name);

        let tokens = encoder.encode_tokens(&encoder.tokenize(&expr));
        let seq_len = encoder.max_length();
        let mut padded_tokens = tokens.clone();
        padded_tokens.resize(seq_len, 0);

        let tokens_tensor = match candle_core::Tensor::new(&padded_tokens[..], &device) {
            Ok(t) => match t.reshape((1, seq_len)) {
                Ok(t) => t,
                Err(e) => {
                    println!("  Reshape error: {}", e);
                    continue;
                }
            },
            Err(e) => {
                println!("  Tensor error: {}", e);
                continue;
            }
        };

        match network.forward(&tokens_tensor) {
            Ok((policy_logits, value)) => {
                let probs = match candle_nn::ops::softmax(&policy_logits, 1) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("  Softmax error: {}", e);
                        continue;
                    }
                };

                let probs_vec: Vec<f32> = match probs.squeeze(0) {
                    Ok(p) => match p.to_vec1() {
                        Ok(v) => v,
                        Err(_) => continue,
                    },
                    Err(_) => continue,
                };

                let mut indexed: Vec<(usize, f32)> = probs_vec.into_iter().enumerate().collect();
                indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                println!("  Top 3 rules:");
                for (i, (rule_idx, prob)) in indexed.iter().take(3).enumerate() {
                    println!("    {}. Rule {} (prob: {:.3})", i + 1, rule_idx, prob);
                }

                let value_scalar: f32 = match value.squeeze(0) {
                    Ok(v) => match v.to_scalar() {
                        Ok(s) => s,
                        Err(_) => 0.0,
                    },
                    Err(_) => 0.0,
                };
                println!("  Value estimate: {:.3}", value_scalar);
            }
            Err(e) => {
                println!("  Forward error: {}", e);
            }
        }
    }

    println!("\n=== BEAST MODE Complete! ===");
    println!("Model saved to: {}", model_path);
}
