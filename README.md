# LEMMA

**Logical Engine for Multi-domain Mathematical Analysis.**

LEMMA is a neuro-symbolic system for mathematical reasoning that combines verified transformation rules with neural network guidance. Unlike language models that predict text statistically, LEMMA applies rigorous mathematical rules and verifies each step of its reasoning process.

---

## Overview

Mathematical reasoning requires precision. A single incorrect step invalidates an entire derivation. LEMMA addresses this by separating the concerns of *what transformations are valid* (handled by explicit rules) from *which transformation to apply* (learned by a neural network).

The system consists six modular components:

| Component | Purpose |
|-----------|---------|
| `mm-core` | Expression representation, parsing, canonicalization |
| `mm-rules` | Mathematical transformation rules (29 implementations) |
| `mm-verifier` | Numerical and symbolic verification of rule applications |
| `mm-search` | Tree search algorithms (Beam search, Neural MCTS) |
| `mm-brain` | Transformer neural network for strategy selection |
| `mm-solver` | Unified API combining all components |

---

## Architecture

```
                    ┌─────────────────────────────┐
                    │       Neural Network        │
                    │   (Policy + Value heads)    │
                    └──────────────┬──────────────┘
                                   │ suggests rule
                                   ▼
┌───────────┐     ┌─────────────────────────────────────┐     ┌───────────┐
│  Problem  │────▶│           MCTS Search               │────▶│ Solution  │
└───────────┘     │  (AlphaZero-style tree search)      │     │ + Proof   │
                  └──────────────┬──────────────────────┘     └───────────┘
                                 │ applies rule
                                 ▼
                    ┌─────────────────────────────┐
                    │         Verifier            │
                    │  (Numerical + Symbolic)     │
                    └─────────────────────────────┘
```

The neural network learns which rules tend to lead toward solutions. The rule library ensures only valid mathematical operations are performed. The verifier confirms each step is correct.

---

## Mathematical Rules

LEMMA implements 29 transformation rules across four categories:

### Algebraic Rules
- Constant folding: `2 + 3 → 5`
- Identity elimination: `x + 0 → x`, `x * 1 → x`
- Zero multiplication: `x * 0 → 0`
- Distribution: `a(b + c) → ab + ac`
- Factoring: `ab + ac → a(b + c)`
- Difference of squares: `a² - b² → (a + b)(a - b)`
- Like term collection: `ax + bx → (a + b)x`

### Calculus Rules
- Power rule: `d/dx(x^n) → n·x^(n-1)`
- Constant rule: `d/dx(c) → 0`
- Sum rule: `d/dx(f + g) → f' + g'`
- Product rule: `d/dx(fg) → f'g + fg'`
- Quotient rule: `d/dx(f/g) → (f'g - fg') / g²`
- Trigonometric derivatives: `d/dx(sin x) → cos x`, `d/dx(cos x) → -sin x`
- Exponential and logarithmic: `d/dx(e^x) → e^x`, `d/dx(ln x) → 1/x`

### Equation Solving Rules
- Addition/subtraction cancellation: `x + a = b → x = b - a`
- Multiplication/division cancellation: `ax = b → x = b/a`
- Linear equation solving: `ax + b = c → x = (c - b)/a`
- Quadratic formula application

### Trigonometric Identities
- Pythagorean identity: `sin²x + cos²x → 1`

---

## Neural Network

The neural network is a Transformer architecture implemented using the Candle framework:

**Architecture:**
- Token embedding (vocabulary size: 64)
- Positional encoding
- 3 Transformer blocks with multi-head self-attention
- Policy head (outputs rule probabilities)
- Value head (estimates solution likelihood)

**Training:**
- 16,968 synthetic training examples
- AdamW optimizer with weight decay
- Cross-entropy loss for policy, MSE for value
- Trained on CPU (GPU support available via Candle features)

---

## Installation

Requirements:
- Rust 1.75 or later
- Cargo package manager

```bash
git clone https://github.com/Pushp-Kharat1/LEMMA.git
cd LEMMA/math-monster

# Build all crates
cargo build --release --workspace

# Run tests
cargo test --workspace
```

---

## Usage

### Training the Neural Network

```bash
cargo run --release --example train_network
```

This generates 16,968 synthetic examples and trains for 50 epochs. The trained model is saved to `lemma_model.safetensors`.

### Testing Rules

```bash
cargo run --release --example test_new_rules
```

Demonstrates rule application for algebraic simplification, derivatives, and equation solving.

### Neural MCTS Simplification

```bash
cargo run --release --example neural_mcts
```

Shows the neural-guided search simplifying expressions like `2 + 3`, `x + 0`, and `(3*4) + 0`.

---

## API Example

```rust
use mm_core::{Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::NeuralMCTS;
use mm_verifier::Verifier;

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    
    let rules = standard_rules();
    let verifier = Verifier::new();
    let mcts = NeuralMCTS::new(rules, verifier);
    
    // Simplify: x + 0
    let expr = Expr::Add(
        Box::new(Expr::Var(x)),
        Box::new(Expr::int(0))
    );
    
    let result = mcts.simplify(expr);
    // Result: Var(x)
    // Steps: [identity_add_zero]
    // Verified: true
}
```

---

## Project Structure

```
math-monster/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── mm-core/               # Core expression types
│   │   ├── expr.rs            # AST definition
│   │   ├── parse.rs           # Expression parser
│   │   ├── canon.rs           # Canonicalization
│   │   ├── eval.rs            # Numerical evaluation
│   │   ├── rational.rs        # Exact rational arithmetic
│   │   └── symbol.rs          # Symbol table (interned strings)
│   │
│   ├── mm-rules/              # Transformation rules
│   │   ├── algebra.rs         # Algebraic rules (10)
│   │   ├── calculus.rs        # Derivative rules (9)
│   │   ├── equations.rs       # Equation solving (7)
│   │   ├── trig.rs            # Trigonometric identities (2)
│   │   └── rule.rs            # Rule infrastructure
│   │
│   ├── mm-verifier/           # Step verification
│   │   ├── numerical.rs       # Point sampling verification
│   │   └── symbolic.rs        # Symbolic equality checking
│   │
│   ├── mm-search/             # Search algorithms
│   │   ├── beam.rs            # Beam search
│   │   └── mcts.rs            # Neural MCTS (AlphaZero-style)
│   │
│   ├── mm-brain/              # Neural network
│   │   ├── encoder.rs         # Expression tokenization
│   │   ├── network.rs         # Transformer architecture
│   │   ├── policy.rs          # Policy network API
│   │   ├── training.rs        # Training loop
│   │   └── data.rs            # Synthetic data generation
│   │
│   └── mm-solver/             # Unified API
│       ├── lib.rs             # MathMonster interface
│       └── examples/          # Usage examples
```

---

## Design Decisions

**Why not just use an LLM?**

Language models excel at many tasks, but mathematical reasoning exposes their limitations. They can produce plausible-looking derivations that contain subtle errors. LEMMA instead:
- Applies only verified transformation rules
- Checks every step numerically and symbolically
- Provides a complete proof trace

**Why a hybrid approach?**

Pure rule-based systems require hand-crafted heuristics to choose between applicable rules. Pure neural approaches lack guarantees. The hybrid approach uses neural learning for intuition and explicit rules for correctness.

**Why Rust?**

Performance matters for tree search algorithms that evaluate many candidates. Rust provides zero-cost abstractions and memory safety without garbage collection pauses.

---

## Limitations

- Currently supports single-variable expressions
- Training runs on CPU (slower than GPU)
- Perfect square factoring patterns not yet implemented
- Integration rules not yet implemented

---

## Future Work

- Multi-variable calculus support
- Integration rule library
- GPU-accelerated training
- Series and limit evaluation
- Ordinary differential equations

---

## License

This project is licensed under the Mozilla Public License 2.0. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

This project draws inspiration from:
- AlphaZero's approach to combining neural networks with tree search
- The Lean theorem prover's emphasis on verified reasoning
- Symbolic mathematics systems like Mathematica and SymPy
