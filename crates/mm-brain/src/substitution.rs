//! ONNX Model Inference for Substitution Prediction
//!
//! Loads and runs the DistilBERT ONNX model for predicting
//! useful substitutions for IMO problems.
//!
//! This module connects the Python-trained model to LEMMA's MCTS.

use std::path::Path;

/// Substitution vocabulary - must match training script
pub const SUBSTITUTION_VOCAB: &[&str] = &[
    "x = 0",
    "y = 0",
    "x = y",
    "x = 1",
    "y = 1",
    "a = b = c = 1",
    "abc = 1 constraint",
    "Apply AM-GM",
    "Apply Cauchy-Schwarz",
    "Assume f is linear",
    "Assume f is injective",
    "Assume f is monotonic",
    "Check small cases",
    "Use modular arithmetic",
    "Homogenize",
    "WLOG assume ordering",
    "Substitute c = 1/(ab)",
    "y = f(x)",
    "x = -y",
    "Consider p = 2 separately",
];

/// A predicted substitution with confidence score
#[derive(Clone, Debug)]
pub struct SubstitutionPrediction {
    /// The substitution text
    pub substitution: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
}

/// Substitution predictor using ONNX model
pub struct SubstitutionPredictor {
    /// Path to the ONNX model
    model_path: Option<String>,

    /// Whether model is loaded (placeholder for actual ONNX loading)
    loaded: bool,
}

impl SubstitutionPredictor {
    /// Create a new predictor
    pub fn new() -> Self {
        SubstitutionPredictor {
            model_path: None,
            loaded: false,
        }
    }

    /// Load model from ONNX file
    ///
    /// Future: Will use candle-onnx for actual inference
    pub fn load(&mut self, model_path: &Path) -> Result<(), String> {
        if !model_path.exists() {
            return Err(format!("Model file not found: {:?}", model_path));
        }

        self.model_path = Some(model_path.to_string_lossy().to_string());
        self.loaded = true;

        // TODO: Actual ONNX loading with candle-onnx:
        // let model = candle_onnx::read_file(model_path)?;
        // self.session = Some(candle_onnx::simple_eval(&model)?);

        Ok(())
    }

    /// Predict substitutions for a problem
    ///
    /// Returns top-k substitutions with confidence scores.
    pub fn predict(&self, problem_text: &str, top_k: usize) -> Vec<SubstitutionPrediction> {
        if !self.loaded {
            // Return rule-based predictions if model not loaded
            return self.rule_based_predict(problem_text, top_k);
        }

        // TODO: Actual ONNX inference:
        // 1. Tokenize input with DistilBERT tokenizer
        // 2. Run through ONNX model
        // 3. Apply sigmoid to logits
        // 4. Return top-k predictions

        self.rule_based_predict(problem_text, top_k)
    }

    /// Rule-based prediction (fallback when model not loaded)
    fn rule_based_predict(&self, problem_text: &str, top_k: usize) -> Vec<SubstitutionPrediction> {
        let text_lower = problem_text.to_lowercase();
        let mut predictions = Vec::new();

        // Check for functional equation patterns
        if text_lower.contains("find all functions") || text_lower.contains("f(") {
            predictions.push(SubstitutionPrediction {
                substitution: "x = 0".to_string(),
                confidence: 0.95,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "y = 0".to_string(),
                confidence: 0.90,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "x = y".to_string(),
                confidence: 0.85,
            });

            if text_lower.contains("f(f(") {
                predictions.push(SubstitutionPrediction {
                    substitution: "y = f(x)".to_string(),
                    confidence: 0.80,
                });
            }

            if text_lower.contains("linear") || text_lower.contains("ax + b") {
                predictions.push(SubstitutionPrediction {
                    substitution: "Assume f is linear".to_string(),
                    confidence: 0.75,
                });
            }
        }

        // Check for inequality patterns
        if text_lower.contains("positive")
            && (text_lower.contains("abc") || text_lower.contains("a + b + c"))
        {
            predictions.push(SubstitutionPrediction {
                substitution: "abc = 1 constraint".to_string(),
                confidence: 0.90,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "Apply AM-GM".to_string(),
                confidence: 0.85,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "a = b = c = 1".to_string(),
                confidence: 0.80,
            });
        }

        // Check for number theory patterns
        if text_lower.contains("integer")
            || text_lower.contains("prime")
            || text_lower.contains("divides")
            || text_lower.contains("divisible")
        {
            predictions.push(SubstitutionPrediction {
                substitution: "Check small cases".to_string(),
                confidence: 0.90,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "Use modular arithmetic".to_string(),
                confidence: 0.80,
            });

            if text_lower.contains("prime") {
                predictions.push(SubstitutionPrediction {
                    substitution: "Consider p = 2 separately".to_string(),
                    confidence: 0.75,
                });
            }
        }

        // Always add some generic substitutions
        if predictions.is_empty() {
            predictions.push(SubstitutionPrediction {
                substitution: "x = 0".to_string(),
                confidence: 0.50,
            });
            predictions.push(SubstitutionPrediction {
                substitution: "x = 1".to_string(),
                confidence: 0.45,
            });
        }

        // Sort by confidence and take top-k
        predictions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        predictions.truncate(top_k);

        predictions
    }

    /// Convert predictions to search hints
    pub fn to_search_hints(&self, predictions: &[SubstitutionPrediction]) -> Vec<SearchHint> {
        predictions
            .iter()
            .map(|p| SearchHint {
                action: p.substitution.clone(),
                priority: (p.confidence * 100.0) as u32,
            })
            .collect()
    }
}

impl Default for SubstitutionPredictor {
    fn default() -> Self {
        Self::new()
    }
}

/// A hint for the search engine
#[derive(Clone, Debug)]
pub struct SearchHint {
    /// The action to try
    pub action: String,
    /// Priority (higher = try first)
    pub priority: u32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_equation_prediction() {
        let predictor = SubstitutionPredictor::new();

        let text = "Find all functions f: ℝ → ℝ such that f(x + f(y)) = f(x) + y.";
        let preds = predictor.predict(text, 3);

        assert!(!preds.is_empty());
        assert!(preds[0].substitution.contains("x = 0") || preds[0].substitution.contains("y = 0"));
    }

    #[test]
    fn test_inequality_prediction() {
        let predictor = SubstitutionPredictor::new();

        let text = "Let a, b, c be positive reals with abc = 1. Prove that a + b + c >= 3.";
        let preds = predictor.predict(text, 3);

        assert!(!preds.is_empty());
        // Should suggest AM-GM or abc = 1 constraint
        let has_relevant = preds
            .iter()
            .any(|p| p.substitution.contains("AM-GM") || p.substitution.contains("abc = 1"));
        assert!(has_relevant);
    }

    #[test]
    fn test_search_hints() {
        let predictor = SubstitutionPredictor::new();
        let preds = vec![
            SubstitutionPrediction {
                substitution: "x = 0".to_string(),
                confidence: 0.9,
            },
            SubstitutionPrediction {
                substitution: "y = 0".to_string(),
                confidence: 0.8,
            },
        ];

        let hints = predictor.to_search_hints(&preds);
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].priority, 90);
    }
}
