<div align="center">

# LEMMA

**Logical Engine for Multi-domain Mathematical Analysis**

A research prototype exploring neural-guided symbolic mathematics in Rust. Inspired by AlphaProof and AlphaZero.

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg?style=for-the-badge)](https://www.rust-lang.org/)

</div>

---

## What This Project IS and IS NOT

### What LEMMA IS:
- A **research prototype** exploring hybrid neural-symbolic reasoning
- A **proof of concept** for AlphaProof-style mathematical search
- **450+ verified transformation rules** for IMO-level mathematics
- An **MCTS engine** guided by a neural policy network
- A **learning project** for anyone interested in symbolic AI

### What LEMMA is NOT:
- **Not a Wolfram Alpha replacement** - we handle basic calculus, not arbitrary math
- **Not production-ready** - this is research code
- **Not a complete CAS** - missing integration, limits, ODEs, and advanced features
- **Not magic** - it applies explicit rules, nothing more

---

## Current Capabilities

### Working Features (Tested)

| Category | Examples | Status |
|----------|----------|--------|
| **Arithmetic** | `(2+3)*(4+5) -> 45` | Working |
| **Identities** | `((x+0)*1)+0 -> x` | Working |
| **Power Rules** | `x^2 * x^3 * x^4 -> x^9` | Working |
| **Basic Derivatives** | `d/dx(x^3) -> 3x^2` | Working |
| **Sum Rule** | `d/dx(x^2 + x^3) -> 2x + 3x^2` | Working |
| **Trig Derivatives** | `d/dx(sin x) -> cos x` | Working |
| **Linear Equations** | `3x + 5 = 17 -> x = 4` | Working |
| **Pythagorean** | `sin^2(x) + cos^2(x) -> 1` | Working |
| **Like Terms** | `2(x+y) + 3(x+y) -> 5(x+y)` | Working |

### New Features (Added Jan 2026)

| Category | Count | Examples |
|----------|-------|----------|
| **Integration** | 9 | `∫x^n dx → x^(n+1)/(n+1)` |
| **Number Theory** | 50+ | Divisibility, GCD, parity |
| **Inequalities** | 30+ | AM-GM, Cauchy-Schwarz |
| **Combinatorics** | 30+ | Binomial, Pascal, Catalan |
| **Polynomials** | 30+ | Vieta's, symmetric polys |

---

## Architecture

```
+------------------------------------------------------------------+
|                      Neural Policy Network                       |
|              (Transformer, suggests which rule to try)           |
+------------------------------+-----------------------------------+
                               |
                               v
+-----------+     +-----------------------------+     +-----------+
|  Problem  |---->|      MCTS Search Engine     |---->| Solution  |
|   Expr    |     |   (AlphaZero-style UCB)     |     | + Proof   |
+-----------+     +--------------+--------------+     +-----------+
                                 |
                                 v
                  +-----------------------------+
                  |       Rule Library          |
                  |  (450+ verified transforms) |
                  +--------------+--------------+
                                 |
                                 v
                  +-----------------------------+
                  |         Verifier            |
                  |  (Numerical + Symbolic)     |
                  +-----------------------------+
```

### Key Innovation

Unlike LLMs that predict text statistically, LEMMA:
1. **Only applies verified mathematical rules** - no hallucination
2. **Provides complete proof traces** - every step is justified
3. **Uses neural guidance for search** - learns which rules to try first

---

## Crate Structure

| Crate | Purpose | Lines of Code |
|-------|---------|---------------|
| `mm-core` | Expression AST, parsing, evaluation | ~1,200 |
| `mm-rules` | 220+ transformation rules | ~5,000 |
| `mm-verifier` | Numerical and symbolic verification | ~400 |
| `mm-search` | Beam search, Neural MCTS | ~800 |
| `mm-brain` | Transformer network (Candle) | ~1,000 |
| `mm-solver` | Unified API | ~300 |

---

## Quick Start

### Requirements
- Rust 1.75+
- ~500MB disk space for dependencies

### Build and Test

```bash
git clone https://github.com/Pushp-Kharat1/LEMMA.git
cd LEMMA

# Build everything
cargo build --release

# Run the benchmark suite
cargo run --release --example benchmark_advanced

# Run stress tests
cargo run --release --example stress_test
```

### Train the Neural Network

```bash
# Generates ~17k synthetic examples, trains for 50 epochs
cargo run --release --example train_network
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
    let rules = standard_rules();
    let verifier = Verifier::new();
    let mcts = NeuralMCTS::new(rules, verifier);
    
    // Solve: 3x + 5 = 17
    let equation = Expr::Equation {
        lhs: Box::new(Expr::Add(
            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
            Box::new(Expr::int(5)),
        )),
        rhs: Box::new(Expr::int(17)),
    };
    
    let solution = mcts.simplify(equation);
    // Result: x = 4
    // Steps: ["isolate_variable", "cancel_multiplication"]
    // Verified: true
}
```

---

## Rule Categories (220+)

### Algebra (14 rules)
| Rule | Transformation |
|------|---------------|
| `constant_fold` | `2 + 3 -> 5` |
| `identity_add_zero` | `x + 0 -> x` |
| `identity_mul_one` | `x * 1 -> x` |
| `zero_mul` | `x * 0 -> 0` |
| `collect_like_terms` | `2x + 3x -> 5x` |
| `distribute` | `a(b + c) -> ab + ac` |
| `factor_common` | `ab + ac -> a(b + c)` |
| `difference_of_squares` | `a^2 - b^2 -> (a+b)(a-b)` |
| `perfect_square_sum` | `a^2 + 2ab + b^2 -> (a+b)^2` |
| `perfect_square_diff` | `a^2 - 2ab + b^2 -> (a-b)^2` |
| `power_of_one` | `x^1 -> x` |
| `power_of_zero` | `x^0 -> 1` |
| `power_add` | `x^a * x^b -> x^(a+b)` |
| `power_mul` | `(x^a)^b -> x^(ab)` |

### Calculus (9 rules)
| Rule | Transformation |
|------|---------------|
| `power_rule` | `d/dx(x^n) -> n*x^(n-1)` |
| `constant_rule` | `d/dx(c) -> 0` |
| `sum_rule` | `d/dx(f+g) -> f' + g'` |
| `product_rule` | `d/dx(fg) -> f'g + fg'` |
| `quotient_rule` | `d/dx(f/g) -> (f'g - fg')/g^2` |
| `sin_derivative` | `d/dx(sin x) -> cos x` |
| `cos_derivative` | `d/dx(cos x) -> -sin x` |
| `exp_rule` | `d/dx(e^x) -> e^x` |
| `ln_rule` | `d/dx(ln x) -> 1/x` |

### Trigonometry (6 rules)
| Rule | Transformation |
|------|---------------|
| `pythagorean_identity` | `sin^2(x) + cos^2(x) -> 1` |
| `sin_double_angle` | `2*sin(x)*cos(x) -> sin(2x)` |
| `cos_double_angle` | `cos^2(x) - sin^2(x) -> cos(2x)` |
| `sin_zero` | `sin(0) -> 0` |
| `cos_zero` | `cos(0) -> 1` |
| `tan_zero` | `tan(0) -> 0` |

### Equation Solving (4 rules)
| Rule | Transformation |
|------|---------------|
| `cancel_addition` | `x + a = b -> x = b - a` |
| `cancel_multiplication` | `ax = b -> x = b/a` |
| `linear_solve` | `ax + b = 0 -> x = -b/a` |
| `quadratic_formula` | `ax^2 + bx + c = 0 -> ...` |

---

## Benchmark Results

### Basic Benchmark (21 tests)
```
Algebraic Identities: 5/6
Constant Folding: 5/5
Trigonometry: 3/3
Derivatives: 5/5
Multi-Variable: 2/2
--------------------------
TOTAL: 20/21 (95.2%)
```

### Advanced Benchmark (10 tests)
```
Multi-Step Algebra: 3/3
Calculus Multi-Step: 3/3
Equation Solving: 2/2
Trig Multi-Step: 2/2
--------------------------
TOTAL: 10/10 (100%)
```

### Stress Test (10 complex problems)
```
All 10 passing
```

---

## Design Philosophy

### Why Not Just Use an LLM?

LLMs are amazing at many tasks, but they can:
- Produce plausible-looking but incorrect derivations
- Skip steps or make sign errors
- Not explain why a transformation is valid

LEMMA trades generality for **reliability**:
- Every step is a provable transformation
- The verifier catches errors
- Complete proof traces for debugging

### Why Rust?

1. **Performance** - MCTS explores thousands of nodes; speed matters
2. **Memory Safety** - No GC pauses during search
3. **Type System** - Expression trees are naturally typed
4. **Ecosystem** - Candle for neural networks, excellent tooling

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

**Quick version:**
1. Fork the repo
2. Add a rule to `mm-rules/src/algebra.rs`
3. Add a test case
4. Submit a PR

We especially welcome:
- New mathematical rules
- Bug reports with reproducible examples
- Documentation improvements
- Benchmark problems that fail

---

## License

Mozilla Public License 2.0 - see [LICENSE](LICENSE)

---

## Acknowledgments

- [AlphaZero](https://www.nature.com/articles/nature24270) - MCTS + Neural guidance
- [AlphaProof](https://deepmind.google/discover/blog/ai-solves-imo-problems-at-silver-medal-level/) - Inspiration for math reasoning
- [Candle](https://github.com/huggingface/candle) - Rust ML framework
- [SimSIMD](https://github.com/ashvardanian/SimSIMD) / [USearch](https://github.com/unum-cloud/usearch) - Ash Vardanian's ecosystem

---

## Contact

- **Author**: Pushp Kharat
- **Email**: kharatpushp16@outlook.com
- **GitHub**: [@Pushp-Kharat1](https://github.com/Pushp-Kharat1)

---

LEMMA is a research project. Use it to learn, experiment, and contribute - not as your only source of mathematical truth.