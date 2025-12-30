//! Neural network architecture for mathematical reasoning.

use candle_core::{DType, Device, IndexOp, Module, Result, Tensor};
use candle_nn::{embedding, linear, Embedding, Linear, VarBuilder, VarMap};

/// Configuration for the network.
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Vocabulary size for embedding.
    pub vocab_size: usize,
    /// Embedding dimension.
    pub embed_dim: usize,
    /// Hidden dimension.
    pub hidden_dim: usize,
    /// Number of attention heads.
    pub num_heads: usize,
    /// Number of transformer layers.
    pub num_layers: usize,
    /// Maximum sequence length.
    pub max_seq_len: usize,
    /// Number of rules (output dimension for policy).
    pub num_rules: usize,
    /// Dropout rate.
    pub dropout: f64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            vocab_size: 64,
            embed_dim: 64,
            hidden_dim: 128,
            num_heads: 4,
            max_seq_len: 64,
            num_rules: 25, // Number of rules we have
            num_layers: 2,
            dropout: 0.1,
        }
    }
}

/// Simple self-attention layer.
struct SelfAttention {
    query: Linear,
    key: Linear,
    value: Linear,
    out: Linear,
    num_heads: usize,
    head_dim: usize,
}

impl SelfAttention {
    fn new(embed_dim: usize, num_heads: usize, vb: VarBuilder) -> Result<Self> {
        let head_dim = embed_dim / num_heads;
        Ok(Self {
            query: linear(embed_dim, embed_dim, vb.pp("query"))?,
            key: linear(embed_dim, embed_dim, vb.pp("key"))?,
            value: linear(embed_dim, embed_dim, vb.pp("value"))?,
            out: linear(embed_dim, embed_dim, vb.pp("out"))?,
            num_heads,
            head_dim,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let (batch_size, seq_len, _) = x.dims3()?;

        // Compute Q, K, V
        let q = self.query.forward(x)?;
        let k = self.key.forward(x)?;
        let v = self.value.forward(x)?;

        // Reshape for multi-head attention: (batch, seq, heads, head_dim)
        let q = q.reshape((batch_size, seq_len, self.num_heads, self.head_dim))?;
        let k = k.reshape((batch_size, seq_len, self.num_heads, self.head_dim))?;
        let v = v.reshape((batch_size, seq_len, self.num_heads, self.head_dim))?;

        // Transpose to (batch, heads, seq, head_dim) and make contiguous
        let q = q.transpose(1, 2)?.contiguous()?;
        let k = k.transpose(1, 2)?.contiguous()?;
        let v = v.transpose(1, 2)?.contiguous()?;

        // Attention scores: (batch, heads, seq, seq)
        let scale = (self.head_dim as f64).sqrt();
        let k_t = k.transpose(2, 3)?.contiguous()?;
        let scores = q.matmul(&k_t)?;
        let scores = (scores / scale)?;

        // Softmax
        let attn = candle_nn::ops::softmax(&scores, candle_core::D::Minus1)?;

        // Apply attention to values
        let out = attn.matmul(&v)?;

        // Reshape back: (batch, seq, embed_dim)
        let out = out.transpose(1, 2)?.contiguous()?;
        let out = out.reshape((batch_size, seq_len, self.num_heads * self.head_dim))?;

        // Output projection
        self.out.forward(&out)
    }
}

/// Feed-forward network.
struct FeedForward {
    fc1: Linear,
    fc2: Linear,
}

impl FeedForward {
    fn new(embed_dim: usize, hidden_dim: usize, vb: VarBuilder) -> Result<Self> {
        Ok(Self {
            fc1: linear(embed_dim, hidden_dim, vb.pp("fc1"))?,
            fc2: linear(hidden_dim, embed_dim, vb.pp("fc2"))?,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.fc1.forward(x)?;
        let x = x.gelu()?;
        self.fc2.forward(&x)
    }
}

/// Transformer block.
struct TransformerBlock {
    attention: SelfAttention,
    ff: FeedForward,
    ln1: candle_nn::LayerNorm,
    ln2: candle_nn::LayerNorm,
}

impl TransformerBlock {
    fn new(config: &NetworkConfig, vb: VarBuilder) -> Result<Self> {
        let attention = SelfAttention::new(config.embed_dim, config.num_heads, vb.pp("attn"))?;
        let ff = FeedForward::new(config.embed_dim, config.hidden_dim, vb.pp("ff"))?;
        let ln1 = candle_nn::layer_norm(config.embed_dim, 1e-5, vb.pp("ln1"))?;
        let ln2 = candle_nn::layer_norm(config.embed_dim, 1e-5, vb.pp("ln2"))?;

        Ok(Self {
            attention,
            ff,
            ln1,
            ln2,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        // Pre-norm architecture
        let normed = self.ln1.forward(x)?;
        let attn_out = self.attention.forward(&normed)?;
        let x = (x + attn_out)?;

        let normed = self.ln2.forward(&x)?;
        let ff_out = self.ff.forward(&normed)?;
        (x + ff_out)
    }
}

/// The main neural network for mathematical reasoning.
///
/// Architecture:
/// - Token embedding
/// - Positional encoding
/// - Transformer blocks
/// - Mean pooling
/// - Policy head (rule probabilities)
/// - Value head (state evaluation)
pub struct MathNetwork {
    embedding: Embedding,
    pos_encoding: Tensor,
    transformer_blocks: Vec<TransformerBlock>,
    policy_head: Linear,
    value_head: Linear,
    config: NetworkConfig,
}

impl MathNetwork {
    /// Create a new network with random weights.
    pub fn new(config: NetworkConfig, device: &Device) -> Result<Self> {
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);

        Self::new_with_vb(config, vb)
    }

    /// Create a network with a specific VarBuilder.
    pub fn new_with_vb(config: NetworkConfig, vb: VarBuilder) -> Result<Self> {
        let embedding = embedding(config.vocab_size, config.embed_dim, vb.pp("embed"))?;

        // Create positional encoding
        let pos_encoding =
            Self::create_pos_encoding(config.max_seq_len, config.embed_dim, vb.device())?;

        // Create transformer blocks
        let mut transformer_blocks = Vec::new();
        for i in 0..config.num_layers {
            let block = TransformerBlock::new(&config, vb.pp(format!("layer_{}", i)))?;
            transformer_blocks.push(block);
        }

        let policy_head = linear(config.embed_dim, config.num_rules, vb.pp("policy"))?;
        let value_head = linear(config.embed_dim, 1, vb.pp("value"))?;

        Ok(Self {
            embedding,
            pos_encoding,
            transformer_blocks,
            policy_head,
            value_head,
            config,
        })
    }

    /// Create sinusoidal positional encoding.
    fn create_pos_encoding(max_len: usize, embed_dim: usize, device: &Device) -> Result<Tensor> {
        let mut pe = vec![0f32; max_len * embed_dim];

        for pos in 0..max_len {
            for i in 0..embed_dim {
                let angle = pos as f32 / 10000f32.powf((2 * (i / 2)) as f32 / embed_dim as f32);
                pe[pos * embed_dim + i] = if i % 2 == 0 { angle.sin() } else { angle.cos() };
            }
        }

        Tensor::new(pe.as_slice(), device)?.reshape((1, max_len, embed_dim))
    }

    /// Forward pass through the network.
    ///
    /// # Arguments
    /// * `tokens` - Token IDs tensor of shape (batch_size, seq_len)
    ///
    /// # Returns
    /// * Policy logits (batch_size, num_rules)
    /// * Value estimate (batch_size, 1)
    pub fn forward(&self, tokens: &Tensor) -> Result<(Tensor, Tensor)> {
        let (batch_size, seq_len) = tokens.dims2()?;

        // Embed tokens
        let mut x = self.embedding.forward(tokens)?;

        // Add positional encoding
        let pos = self.pos_encoding.i((.., ..seq_len, ..))?;
        let pos = pos.broadcast_as((batch_size, seq_len, self.config.embed_dim))?;
        x = x.add(&pos)?;

        // Pass through transformer blocks
        for block in &self.transformer_blocks {
            x = block.forward(&x)?;
        }

        // Mean pooling over sequence dimension
        let pooled = x.mean(1)?;

        // Policy and value heads
        let policy_logits = self.policy_head.forward(&pooled)?;
        let value = self.value_head.forward(&pooled)?;

        Ok((policy_logits, value))
    }

    /// Get policy probabilities (softmax over logits).
    pub fn get_policy(&self, tokens: &Tensor) -> Result<Tensor> {
        let (logits, _) = self.forward(tokens)?;
        candle_nn::ops::softmax(&logits, candle_core::D::Minus1)
    }

    /// Get value estimate.
    pub fn get_value(&self, tokens: &Tensor) -> Result<Tensor> {
        let (_, value) = self.forward(tokens)?;
        // Squeeze to scalar if needed
        value.tanh()
    }

    /// Get configuration.
    pub fn config(&self) -> &NetworkConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_forward() {
        let config = NetworkConfig::default();
        let device = Device::Cpu;

        let network = MathNetwork::new(config.clone(), &device).unwrap();

        // Create dummy input: batch_size=2, seq_len=32
        let tokens = Tensor::zeros((2, 32), DType::U32, &device).unwrap();

        let (policy, value) = network.forward(&tokens).unwrap();

        assert_eq!(policy.dims(), &[2, config.num_rules]);
        assert_eq!(value.dims(), &[2, 1]);
    }

    #[test]
    fn test_policy_sums_to_one() {
        let config = NetworkConfig::default();
        let device = Device::Cpu;

        let network = MathNetwork::new(config, &device).unwrap();
        let tokens = Tensor::zeros((1, 32), DType::U32, &device).unwrap();

        let policy = network.get_policy(&tokens).unwrap();
        let sum: f32 = policy.sum_all().unwrap().to_scalar().unwrap();

        assert!((sum - 1.0).abs() < 1e-5);
    }
}
