//! ONNX Model Inference for LEMMA MathBERT
//!
//! Loads the trained BERT model for mathematical problem analysis.
//!
//! Note: Full ONNX integration pending. Uses heuristic-based prediction.

use std::path::Path;

/// MathBERT ONNX Model for problem classification  
pub struct MathBertModel {
    vocab: Vec<String>,
    max_length: usize,
}

impl MathBertModel {
    /// Load model from vocab file
    pub fn load(_model_path: &Path, vocab_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if !vocab_path.exists() {
            return Err(format!("Vocab not found: {:?}", vocab_path).into());
        }

        let vocab_text = std::fs::read_to_string(vocab_path)?;
        let vocab: Vec<String> = vocab_text.lines().map(|s| s.to_string()).collect();

        println!("✓ MathBERT loaded: {} vocab tokens", vocab.len());

        Ok(Self {
            vocab,
            max_length: 128,
        })
    }

    /// Tokenize text
    fn tokenize(&self, text: &str) -> Vec<i64> {
        let mut input_ids = vec![101i64]; // [CLS]
        for word in text.split_whitespace() {
            let word_lower = word.to_lowercase();
            if let Some(idx) = self.vocab.iter().position(|v| v == &word_lower) {
                input_ids.push(idx as i64);
            } else {
                input_ids.push(100); // [UNK]
            }
            if input_ids.len() >= self.max_length - 1 {
                break;
            }
        }
        input_ids.push(102); // [SEP]
        input_ids
    }

    /// Predict using heuristics (ONNX integration pending)
    pub fn predict(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let _tokens = self.tokenize(text);
        let mut scores = vec![0.0f32; 20];
        let text_lower = text.to_lowercase();

        if text_lower.contains("prove") {
            scores[0] += 2.0;
        }
        if text_lower.contains("find") {
            scores[1] += 2.0;
        }
        if text_lower.contains("inequality") || text_lower.contains(">=") {
            scores[2] += 2.0;
        }
        if text_lower.contains("equation") {
            scores[3] += 2.0;
        }
        if text_lower.contains("prime") {
            scores[4] += 2.0;
        }
        if text_lower.contains("triangle") {
            scores[5] += 2.0;
        }
        if text_lower.contains("function") {
            scores[6] += 2.0;
        }
        if text_lower.contains("count") {
            scores[7] += 2.0;
        }
        if text_lower.contains("polynomial") {
            scores[8] += 2.0;
        }
        if text_lower.contains("sequence") {
            scores[9] += 2.0;
        }

        Ok(scores)
    }

    /// Get top-k predictions
    pub fn predict_top_k(
        &self,
        text: &str,
        k: usize,
    ) -> Result<Vec<(usize, f32)>, Box<dyn std::error::Error>> {
        let logits = self.predict(text)?;
        let max_l = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp: Vec<f32> = logits.iter().map(|&x| (x - max_l).exp()).collect();
        let sum: f32 = exp.iter().sum();
        let probs: Vec<f32> = exp.iter().map(|&x| x / sum).collect();

        let mut indexed: Vec<(usize, f32)> = probs.into_iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(indexed.into_iter().take(k).collect())
    }

    /// Get class name
    pub fn class_name(idx: usize) -> &'static str {
        const CLASSES: [&str; 20] = [
            "Proof",
            "FindValue",
            "Inequality",
            "Equation",
            "NumberTheory",
            "Geometry",
            "FunctionalEq",
            "Combinatorics",
            "Polynomials",
            "Sequences",
            "Algebra",
            "Calculus",
            "Probability",
            "GameTheory",
            "Optimization",
            "Modular",
            "Divisibility",
            "Construction",
            "Existence",
            "Other",
        ];
        CLASSES.get(idx).unwrap_or(&"Unknown")
    }
}
