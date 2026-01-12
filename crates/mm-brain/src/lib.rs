// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

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
pub mod onnx_inference;
pub mod policy;
pub mod substitution;
pub mod training;

pub use data::DataGenerator;
pub use encoder::ExpressionEncoder;
pub use network::MathNetwork;
pub use onnx_inference::MathBertModel;
pub use policy::PolicyNetwork;
pub use substitution::{SearchHint, SubstitutionPrediction, SubstitutionPredictor};
pub use training::{Trainer, TrainingConfig};
