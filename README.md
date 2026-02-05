<div align="center">

# LEMMA
**Logical Engine for Multi-domain Mathematical Analysis**

A high-performance **Neuro-Symbolic Theorem Prover** in Rust.
Implements an **AlphaZero-style** architecture combining **Deep MCTS** (Monte Carlo Tree Search) with a **Transformer** policy network to solve competition-level mathematical problems.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg?style=for-the-badge)](https://www.rust-lang.org/)
[![AI](https://img.shields.io/badge/AI-Candle%20Transformer-blueviolet.svg?style=for-the-badge)](https://github.com/huggingface/candle)

</div>

---

## Overview

LEMMA is a comprehensive research system designed to solve complex mathematical problems (Algebra, Calculus, Number Theory) by combining the intuition of neural networks with the rigor of symbolic logic.

Unlike LLMs which "hallucinate" answers, LEMMA uses a **Deep Search** engine that explores millions of logical states, guided by a neural network, but constrained by **800+ strictly verified mathematical rules**. It produces not just an answer, but a **verifiable proof trace**.

### Key Technologies
*   **Deep MCTS:** A parallelized, lock-free Monte Carlo Tree Search engine capable of exploring **10M+ nodes** per problem, utilizing virtual loss and PUCT (Polynomial Upper Confidence Trees).
*   **Neural Guidance:** A custom **Transformer** model (implemented in `Candle`) that predicts the most promising rules to apply (Policy Head) and estimates the solubility of the current state (Value Head).
*   **Symbolic Core:** A high-performance AST engine with canonicalization, pattern matching, and a library of **807 verified transformation rules**.
*   **Synthetic Curriculum:** The `mm-synth` crate generates millions of synthetic training examples ("Forward" and "Backward" synthesis) to bootstrap the neural network, similar to **AlphaProof**.

---

## Capabilities

The system is designed to tackle problems at the level of the **International Mathematical Olympiad (IMO)**.

| Domain | Capabilities |
|--------|--------------|
| **Number Theory** | **100+ Rules**: Modular arithmetic, Fermat's Little Theorem, Euler's Totient, Diophantine equations, Prime counting bounds, Chinese Remainder Theorem. |
| **Calculus** | **Deep Integration**: Symbolic integration via substitution, parts, and partial fractions. Derivatives including Chain Rule. |
| **Algebra** | **High-School to Olympiad**: Advanced factorization, inequalities (AM-GM, Cauchy-Schwarz), functional equations, polynomial roots (Vieta's formulas). |
| **Combinatorics** | **Symbolic Counting**: Binomial identities, Pascal's triangle properties, factorial manipulation. |

### Performance
*   **Speed:** Written in pure Rust for maximum performance.
*   **Scale:** The `DeepMCTS` engine supports multi-threaded search across all available cores.
*   **Verification:** Hybrid verification system (Symbolic + Numerical) ensures 0% hallucination rate on verified steps.

---

## Architecture & Crates

The workspace is organized into specialized components:

| Crate | Description |
|-------|-------------|
| **`mm-search`** | **The Engine.** Implements `DeepMCTS` (industrial-strength, parallel search) and `NeuralMCTS`. |
| **`mm-brain`** | **The Intuition.** A complete Transformer neural network using `Candle`. Handles embedding, attention, and policy/value estimation. |
| **`mm-rules`** | **The Laws.** A library of **807** strictly defined mathematical transformation rules. |
| **`mm-synth`** | **The Teacher.** Generates synthetic training data via forward/backward chaining to train the brain. |
| **`mm-core`** | **The Foundation.** Expression AST, interning (`SymbolTable`), canonicalization, and parsing. |
| **`mm-verifier`** | **The Judge.** Verifies every step using symbolic equivalence and numerical spot-checking. |
| **`mm-solver`** | **The Application.** Unified CLI and API for running benchmarks and solving problems. |

---

## Limitations

While LEMMA is a powerful research engine, it is designed for *search* and *discovery*, not general-purpose computation:

1.  **Computation vs. Search:** It is not a substitute for Mathematica/WolframAlpha. It is better at "proving x is an integer" than "computing the integral of e^(x^2)".
2.  **Formal Verification:** While rules are unit-tested and steps are verified numerically/symbolically, a full backend connection to a formal proof assistant (like Lean or Coq) or SMT solver (Z3) is currently in development (TODO).
3.  **Neural Latency:** The search speed is bound by the inference latency of the Transformer model (currently CPU-bound, though Candle supports CUDA).

---

## Quick Start

### Prerequisites
*   Rust 1.75+
*   (Optional) CUDA toolkit for GPU acceleration (via Candle)

### Installation

```bash
git clone https://github.com/Pushp-Kharat1/LEMMA.git
cd LEMMA
cargo build --release
```

### Running the Advanced Solver

To see the `DeepMCTS` engine in action on complex problems:

```bash
# Run the advanced benchmark suite
cargo run --release -p mm-solver --example benchmark_advanced
```

### Simulating IMO Problems

To attempt the 2024 IMO problem set (experimental):

```bash
cargo run --release -p mm-solver --example imo_2024_solve
```

### Training the Brain

To generate synthetic data and train the policy network:

```bash
cargo run --release -p mm-solver --example train_network
```

---

## Contributing

We welcome researchers and engineers interested in Neuro-Symbolic AI.
*   **Add Rules:** See `crates/mm-rules/src/` for examples of adding new mathematical identities.
*   **Optimize Search:** Improvements to the MCTS algorithm in `crates/mm-search`.
*   **Verification:** Help implement the Z3/SMT bridge in `crates/mm-verifier`.

---

## Acknowledgments

*   **AlphaZero & AlphaProof:** The architectural inspiration for MCTS + Neural Guidance.
*   **Candle:** The efficient Rust ML framework by Hugging Face.
*   **Rust:** For enabling safety without sacrificing performance.

---

<div align="center">
<i>"Mathematics is the art of giving the same name to different things." — Henri Poincaré</i>
</div>
