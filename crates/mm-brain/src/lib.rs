//! # mm-brain
//!
//! Neural network for strategy learning in the Math Monster system.
//!
//! This crate provides:
//! - [`ExpressionEncoder`] - Convert expressions to neural network inputs
//! - [`MathNetwork`] - The actual neural network for policy/value prediction
//! - [`PolicyNetwork`] - High-level API for rule selection
//! - [`Trainer`] - Training loop for the network
//! - [`DataGenerator`] - Synthetic training data generation
//!
//! ## Architecture
//!
//! ```text
//! Expression → Tokenize → Embed → Transformer → Policy Head → Rule Probs
//!                                             → Value Head  → State Value
//! ```

pub mod data;
pub mod encoder;
pub mod network;
pub mod policy;
pub mod training;

pub use data::DataGenerator;
pub use encoder::ExpressionEncoder;
pub use network::MathNetwork;
pub use policy::PolicyNetwork;
pub use training::{Trainer, TrainingConfig};
