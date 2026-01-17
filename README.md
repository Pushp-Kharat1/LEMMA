<div align="center">

# LEMMA
**Logical Engine for Multi-domain Mathematical Analysis**

A research prototype exploring neural-guided symbolic mathematics in Rust.
Combining **MCTS** (Monte Carlo Tree Search) with a **Transformer** policy network to solve mathematical problems using strict, verified rules.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg?style=for-the-badge)](https://www.rust-lang.org/)

</div>

---

## What This Project IS and IS NOT

### What LEMMA IS:
- A **research prototype** exploring hybrid neural-symbolic reasoning.
- A system that applies **explicit, verified transformation rules** (no hallucinated steps).
- A **proof of concept** for AlphaZero-style mathematical search.
- An engine with **800+ transformation rules** covering Algebra, Calculus, and Number Theory.
- A **learning project** demonstrating how to implement MCTS + Transformers in Rust.

### What LEMMA is NOT:
- **Not a production-ready CAS** (like Mathematica or SymPy) - it lacks many standard algorithms (e.g., Risch algorithm for integration).
- **Not a general-purpose theorem prover** (like Lean or Coq).
- **Not fully formally verified** - while rules are unit-tested, there is no formal proof backend (Z3/SMT) yet.
- **Not a magic LLM** - it cannot "chat" about math; it manipulates expression trees.

---

## Honest Capabilities

### Core Engine
- **Symbolic Manipulation:** Handles expressions, variables, constants, and operators in a structured AST.
- **Canonicalization:** Automatically simplifies expressions to a canonical form for comparison.
- **Verification:** Hybrid system using symbolic equality and numerical spot-checking.

### Transformation Rules (800+)
The system defines over 800 rules across several domains. Note that "working" means the rule logic is implemented, but complex multi-step application depends on the search capability.

| Category | Approx. Rules | Examples |
|----------|---------------|----------|
| **Algebra** | 50+ | Simplification, Factoring, Expansion, collecting like terms. |
| **Calculus** | ~30 | Derivatives (`power_rule`, `chain_rule`), Basic Integration (`power_integral`, `substitution`). |
| **Number Theory** | 100+ | Divisibility, Modular Arithmetic, GCD/LCM, Primes, Euler's Theorem, Chinese Remainder Theorem. |
| **Trigonometry** | 40+ | Identities (`sin^2+cos^2=1`), Double angle formulas. |
| **Inequalities** | 30+ | AM-GM, Cauchy-Schwarz, Triangle Inequality. |
| **Combinatorics** | ~40 | Binomial coefficients, Pascal's identity, Factorial rules. |

### Neural Guidance
- **Architecture:** Custom Transformer model implemented in `Candle` (Rust ML framework).
- **Components:** Token embedding, Positional encoding, Self-attention blocks, Policy head (rule selection), Value head (state evaluation).
- **Training:** Includes `mm-synth` for generating synthetic training data.

---

## Honest Limitations

1.  **Verification Gaps:**
    -   **No Formal Backend:** The "Formal" verification level (Z3/SMT integration) is currently a `TODO`.
    -   **Calculus Verification:** Numerical verification is skipped for calculus expressions (derivatives/integrals) because they cannot be trivially spot-checked without a more powerful oracle.
2.  **Search limitations:**
    -   While MCTS is implemented, solving complex IMO-level problems is still experimental and computationally expensive.
3.  **Completeness:**
    -   The rule set is extensive but not exhaustive. If a specific transformation rule is missing, the system cannot solve problems requiring it.
4.  **Performance:**
    -   MCTS can be slow. The neural network inference (on CPU) adds latency to each search step.

---

## Crate Structure

The workspace consists of several crates, each with a specific responsibility:

| Crate | Purpose |
|-------|---------|
| `mm-core` | Defines the Expression AST, parsing, evaluation, and canonicalization. |
| `mm-rules` | Contains the library of 800+ transformation rules. |
| `mm-verifier` | Handles symbolic and numerical verification of steps. |
| `mm-search` | Implements MCTS (Monte Carlo Tree Search) and Beam Search. |
| `mm-brain` | The Neural Network (Transformer) implementation using `Candle`. |
| `mm-solver` | The top-level solver that integrates all components. |
| `mm-synth` | Generates synthetic mathematical data for training the network. |
| `mm-macro` | Procedural macros for the project. |

---

## Quick Start

### Requirements
- Rust 1.75+

### Build and Test

```bash
git clone https://github.com/Pushp-Kharat1/LEMMA.git
cd LEMMA

# Build everything
cargo build --release

# Run the advanced benchmark (using mm-solver)
cargo run --release -p mm-solver --example benchmark_advanced

# Run the stress test
cargo run --release -p mm-solver --example stress_test
```

### Train the Neural Network

To generate data and train the model:

```bash
cargo run --release -p mm-solver --example train_network
```

---

## Usage Example

```rust
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    
    // Setup the solver
    let rules = standard_rules(); // standard_rules needs to be imported/available
    let verifier = Verifier::new();
    let mcts = NeuralMCTS::new(rules, verifier);
    
    // Solve: 3x + 5 = 17
    // ... (See examples/ folder for full implementation)
}
```

---

## Contributing

Contributions are welcome, especially:
1.  **New Rules:** Add missing mathematical identities to `mm-rules`.
2.  **Bug Fixes:** Fix incorrect rule applications.
3.  **Documentation:** Improve comments and explanations.

---

## Acknowledgments

- **AlphaZero / AlphaProof:** For the conceptual inspiration.
- **Candle:** For the Rust ML framework.

---

**Disclaimer:** This is a research project. Use it to learn and experiment.
