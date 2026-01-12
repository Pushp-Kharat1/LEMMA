//! Synthetic Problem Generator for LEMMA
//!
//! Generates millions of training problems using:
//! - Forward synthesis: Start with answer, derive problem
//! - Backward synthesis: Start with problem, trace solution
//!
//! This is the core of AlphaProof-style training.

pub mod algebra;
pub mod functional;
pub mod generator;
pub mod number_theory;

pub use generator::{
    GeneratorConfig, ProblemCategory, ProblemGenerator, SolutionStep, SyntheticProblem,
};
