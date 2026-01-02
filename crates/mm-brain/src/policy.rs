// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Policy network for rule selection.

use candle_core::{Device, Result};
use mm_core::Expr;
use mm_rules::RuleId;

use crate::encoder::ExpressionEncoder;
use crate::network::{MathNetwork, NetworkConfig};

/// Policy network for selecting which rule to apply.
///
/// This wraps the neural network and provides a high-level API
/// for getting rule probabilities given an expression.
pub struct PolicyNetwork {
    network: MathNetwork,
    encoder: ExpressionEncoder,
    device: Device,
}

impl PolicyNetwork {
    /// Create a new policy network with default configuration.
    pub fn new() -> Result<Self> {
        Self::with_config(NetworkConfig::default(), Device::Cpu)
    }

    /// Create with custom configuration.
    pub fn with_config(config: NetworkConfig, device: Device) -> Result<Self> {
        let network = MathNetwork::new(config, &device)?;
        let encoder = ExpressionEncoder::new(device.clone());

        Ok(Self {
            network,
            encoder,
            device,
        })
    }

    /// Get rule probabilities for an expression.
    ///
    /// Returns a vector of probabilities, one per rule.
    pub fn forward(&self, expr: &Expr) -> Result<Vec<f32>> {
        // Encode expression to tensor
        let tokens = self.encoder.encode(expr)?;
        let tokens = tokens.unsqueeze(0)?; // Add batch dimension

        // Get policy probabilities
        let policy = self.network.get_policy(&tokens)?;

        // Convert to Vec<f32>
        let policy = policy.squeeze(0)?; // Remove batch dimension
        let policy: Vec<f32> = policy.to_vec1()?;

        Ok(policy)
    }

    /// Get value estimate for an expression.
    ///
    /// Returns a value between -1 (bad state) and 1 (good state).
    pub fn get_value(&self, expr: &Expr) -> Result<f32> {
        let tokens = self.encoder.encode(expr)?;
        let tokens = tokens.unsqueeze(0)?;

        let value = self.network.get_value(&tokens)?;
        let value: f32 = value.squeeze(0)?.squeeze(0)?.to_scalar()?;

        Ok(value)
    }

    /// Get the top-k most likely rules.
    pub fn top_k(&self, expr: &Expr, k: usize) -> Result<Vec<(RuleId, f32)>> {
        let probs = self.forward(expr)?;

        // Create (index, probability) pairs and sort by probability
        let mut indexed: Vec<(usize, f32)> = probs.into_iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top k
        Ok(indexed
            .into_iter()
            .take(k)
            .map(|(idx, prob)| (RuleId(idx as u32 + 1), prob))
            .collect())
    }

    /// Get the network for training.
    pub fn network(&self) -> &MathNetwork {
        &self.network
    }

    /// Get mutable network for training.
    pub fn network_mut(&mut self) -> &mut MathNetwork {
        &mut self.network
    }

    /// Get the encoder.
    pub fn encoder(&self) -> &ExpressionEncoder {
        &self.encoder
    }

    /// Get the device.
    pub fn device(&self) -> &Device {
        &self.device
    }
}

impl Default for PolicyNetwork {
    fn default() -> Self {
        Self::new().expect("Failed to create default PolicyNetwork")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_network() {
        let policy = PolicyNetwork::new().unwrap();

        // Test with a simple expression
        let expr = Expr::Add(Box::new(Expr::int(2)), Box::new(Expr::int(3)));
        let probs = policy.forward(&expr).unwrap();

        // Should have one probability per rule
        assert_eq!(probs.len(), policy.network().config().num_rules);

        // Probabilities should sum to 1
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_top_k() {
        let policy = PolicyNetwork::new().unwrap();
        let expr = Expr::int(5);

        let top = policy.top_k(&expr, 3).unwrap();
        assert_eq!(top.len(), 3);

        // Check probabilities are in descending order
        assert!(top[0].1 >= top[1].1);
        assert!(top[1].1 >= top[2].1);
    }

    #[test]
    fn test_value_estimate() {
        let policy = PolicyNetwork::new().unwrap();
        let expr = Expr::int(5);

        let value = policy.get_value(&expr).unwrap();

        // Value should be between -1 and 1 (tanh output)
        assert!(value >= -1.0 && value <= 1.0);
    }
}
