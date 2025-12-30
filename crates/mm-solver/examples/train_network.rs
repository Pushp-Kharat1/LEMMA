//! Example: Training the neural network on synthetic data.
//!
//! This demonstrates the complete training pipeline:
//! 1. Generate synthetic training data
//! 2. Create and train the network
//! 3. Test inference

use candle_core::Device;
use mm_brain::{
    data::DataGenerator,
    network::NetworkConfig,
    training::{Trainer, TrainingConfig},
};
use mm_core::Expr;

fn main() {
    println!("=== Math Monster: Neural Network Training ===\n");

    // Use CPU for now
    let device = Device::Cpu;
    println!("Using device: {:?}", device);

    // Generate synthetic training data - 10K examples
    println!("\n--- Generating Training Data (10K) ---");
    let mut generator = DataGenerator::new(device.clone());
    let examples = generator.generate_dataset(500); // ~10K total examples
    println!("Generated {} training examples", examples.len());

    // Configure the network (tuned for better accuracy)
    let network_config = NetworkConfig {
        vocab_size: 64,
        embed_dim: 128,  // Increased from 64
        hidden_dim: 256, // Increased from 128
        num_heads: 8,    // Increased from 4
        num_layers: 3,   // Increased from 2
        max_seq_len: 64,
        num_rules: 30, // 29 rules + 1 no-op
        dropout: 0.1,
    };

    // Configure training (tuned)
    let training_config = TrainingConfig {
        learning_rate: 5e-4, // Reduced from 1e-3 for stability
        weight_decay: 0.01,
        batch_size: 32, // Reduced from 64 for better gradients
        epochs: 50,     // Increased from 30
        value_weight: 0.5,
    };

    // Create trainer
    println!("\n--- Creating Trainer ---");
    let mut trainer = match Trainer::new(network_config.clone(), training_config, device.clone()) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create trainer: {}", e);
            return;
        }
    };

    // Train
    println!("\n--- Training ---");
    match trainer.train(&examples) {
        Ok(history) => {
            println!("\nTraining complete!");
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

    // Save the trained model
    println!("\n--- Saving Model ---");
    let model_path = "lemma_model.safetensors";
    match trainer.save(model_path) {
        Ok(()) => println!("Model saved to: {}", model_path),
        Err(e) => eprintln!("Failed to save model: {}", e),
    }

    // Test inference using the TRAINED network
    println!("\n--- Testing Inference (using trained weights) ---");

    // Get the trained network and encoder from trainer
    let network = trainer.network();
    let encoder = trainer.encoder();

    // Test on some expressions
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
        (
            "5 * 1",
            Expr::Mul(Box::new(Expr::int(5)), Box::new(Expr::int(1))),
        ),
        ("x * 0", {
            let mut symbols = mm_core::SymbolTable::new();
            let x = symbols.intern("x");
            Expr::Mul(Box::new(Expr::Var(x)), Box::new(Expr::int(0)))
        }),
        ("d/dx(x^2)", {
            let mut symbols = mm_core::SymbolTable::new();
            let x = symbols.intern("x");
            Expr::Derivative {
                expr: Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
                var: x,
            }
        }),
        ("3x = 12 (equation)", {
            let mut symbols = mm_core::SymbolTable::new();
            let x = symbols.intern("x");
            Expr::Equation {
                lhs: Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
                rhs: Box::new(Expr::int(12)),
            }
        }),
    ];

    for (name, expr) in test_exprs {
        println!("\nExpression: {}", name);

        // Tokenize and encode
        let tokens = encoder.encode_tokens(&encoder.tokenize(&expr));
        let seq_len = encoder.max_length();
        let mut padded_tokens = tokens.clone();
        padded_tokens.resize(seq_len, 0);

        // Create tensor
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

        // Forward pass with trained network
        match network.forward(&tokens_tensor) {
            Ok((policy_logits, value)) => {
                // Get probabilities via softmax
                let probs = match candle_nn::ops::softmax(&policy_logits, 1) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("  Softmax error: {}", e);
                        continue;
                    }
                };

                // Get probs as vec
                let probs_vec: Vec<f32> = match probs.squeeze(0) {
                    Ok(p) => match p.to_vec1() {
                        Ok(v) => v,
                        Err(e) => {
                            println!("  Vec error: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("  Squeeze error: {}", e);
                        continue;
                    }
                };

                // Find top 3 rules
                let mut indexed: Vec<(usize, f32)> = probs_vec.into_iter().enumerate().collect();
                indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                println!("  Top rules:");
                for (i, (rule_idx, prob)) in indexed.iter().take(3).enumerate() {
                    println!("    {}. Rule {} (prob: {:.3})", i + 1, rule_idx, prob);
                }

                // Get value
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

    println!("\n=== Done ===");
}
