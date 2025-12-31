# Contributing to LEMMA

Thank you for considering contributing to LEMMA! This document provides guidelines and templates to help you get started.

---

## Ways to Contribute

### 1. Add New Mathematical Rules (Most Needed!)

LEMMA's power comes from its rule library. Adding rules is the easiest way to contribute.

**High-Priority Rules We Need:**
- Chain rule: `d/dx(f(g(x))) -> f'(g(x)) * g'(x)`
- Integration rules: `integral(x^n dx) -> x^(n+1)/(n+1)`
- More trig identities: `tan^2(x) + 1 -> sec^2(x)`
- Logarithm rules: `log(ab) -> log(a) + log(b)`
- Exponential rules: `e^a * e^b -> e^(a+b)`

### 2. Report Bugs

Found an expression that LEMMA simplifies incorrectly? Open an issue with:
- Input expression
- Expected output
- Actual output
- Steps printed (if any)

### 3. Add Test Cases

More test coverage = more confidence. Add cases to:
- `examples/benchmark.rs` - basic tests
- `examples/benchmark_advanced.rs` - multi-step tests
- `examples/stress_test.rs` - complex cases

### 4. Improve Documentation

- Fix typos
- Add examples
- Clarify confusing sections

---

## Development Setup

```bash
# Clone
git clone https://github.com/Pushp-Kharat1/LEMMA.git
cd LEMMA

# Build
cargo build --release

# Test everything
cargo test --workspace

# Run benchmarks
cargo run --release --example benchmark_advanced
cargo run --release --example stress_test
```

---

## Adding a New Rule

### Step 1: Choose the Right File

| Rule Type | File |
|-----------|------|
| Algebraic | `crates/mm-rules/src/algebra.rs` |
| Calculus | `crates/mm-rules/src/calculus.rs` |
| Trigonometry | `crates/mm-rules/src/trig.rs` |
| Equation Solving | `crates/mm-rules/src/equations.rs` |

### Step 2: Copy This Template

```rust
// ============================================================================
// Rule XX: Your Rule Name
// ============================================================================

fn your_rule_name() -> Rule {
    Rule {
        id: RuleId(XX),  // Pick next available ID
        name: "your_rule_name",
        category: RuleCategory::Simplification,  // or Expansion, EquationSolving, etc.
        description: "Human readable: pattern -> result",
        
        is_applicable: |expr, _ctx| {
            // Return true if this rule can apply to `expr`
            // Use pattern matching on the Expr enum
            match expr {
                Expr::Add(a, b) => {
                    // Your condition here
                    false
                }
                _ => false,
            }
        },
        
        apply: |expr, _ctx| {
            // Transform the expression
            // Return Vec<RuleApplication> - usually just one
            if let Expr::Add(a, b) = expr {
                return vec![RuleApplication {
                    result: /* your transformed expression */,
                    justification: "explanation".to_string(),
                }];
            }
            vec![]
        },
        
        reversible: false,  // true if rule can be applied backwards
        cost: 1,  // 1-3, lower = preferred
    }
}
```

### Step 3: Register the Rule

Add your rule to the appropriate function:

```rust
// In algebra.rs
pub fn algebra_rules() -> Vec<Rule> {
    vec![
        constant_fold(),
        identity_add_zero(),
        // ... existing rules ...
        your_rule_name(),  // ADD HERE
    ]
}
```

### Step 4: Add a Test

```rust
// In examples/benchmark.rs or stress_test.rs
test(&mcts, "N", "your_rule description",
    Expr::...,  // input expression
    |e| matches!(e, Expr::...)  // expected output pattern
);
```

### Step 5: Run Tests

```bash
cargo test --workspace
cargo run --release --example benchmark_advanced
```

---

## Rule Examples

### Simple Rule: x^0 -> 1

```rust
fn power_of_zero() -> Rule {
    Rule {
        id: RuleId(12),
        name: "power_of_zero",
        category: RuleCategory::Simplification,
        description: "x^0 = 1 (where x != 0)",
        
        is_applicable: |expr, _ctx| {
            matches!(expr, Expr::Pow(_, exp) 
                if matches!(exp.as_ref(), Expr::Const(r) if r.is_zero()))
        },
        
        apply: |expr, _ctx| {
            if let Expr::Pow(_, _) = expr {
                vec![RuleApplication {
                    result: Expr::int(1),
                    justification: "x^0 = 1".to_string(),
                }]
            } else {
                vec![]
            }
        },
        
        reversible: false,
        cost: 1,
    }
}
```

### Derivative Rule: d/dx(sin x) -> cos x

```rust
fn sin_derivative() -> Rule {
    Rule {
        id: RuleId(15),
        name: "sin_derivative",
        category: RuleCategory::Simplification,
        description: "d/dx(sin(x)) = cos(x)",
        
        is_applicable: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sin(arg) = inner.as_ref() {
                    // Check that sin's argument is just the variable
                    return matches!(arg.as_ref(), Expr::Var(v) if *v == *var);
                }
            }
            false
        },
        
        apply: |expr, _ctx| {
            if let Expr::Derivative { expr: inner, var } = expr {
                if let Expr::Sin(arg) = inner.as_ref() {
                    return vec![RuleApplication {
                        result: Expr::Cos(arg.clone()),
                        justification: "d/dx(sin(x)) = cos(x)".to_string(),
                    }];
                }
            }
            vec![]
        },
        
        reversible: false,
        cost: 1,
    }
}
```

---

## Expression Types

```rust
pub enum Expr {
    // Constants and variables
    Const(Rational),      // Exact rational number
    Var(Symbol),          // Variable like x, y
    
    // Basic operations
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    
    // Functions
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Exp(Box<Expr>),
    Ln(Box<Expr>),
    Log(Box<Expr>, Box<Expr>),  // log_base(arg)
    Sqrt(Box<Expr>),
    Abs(Box<Expr>),
    
    // Calculus
    Derivative { expr: Box<Expr>, var: Symbol },
    Integral { expr: Box<Expr>, var: Symbol },
    
    // Equations
    Equation { lhs: Box<Expr>, rhs: Box<Expr> },
}
```

---

## Rule Categories

```rust
pub enum RuleCategory {
    Simplification,   // Makes expression simpler: x+0 -> x
    Expansion,        // Expands: a(b+c) -> ab+ac
    Factoring,        // Factors: ab+ac -> a(b+c)
    EquationSolving,  // Solves: x+3=7 -> x=4
    TrigIdentity,     // Trig: sin^2+cos^2 -> 1
    Calculus,         // Derivatives, integrals
}
```

---

## Common Pitfalls

### 1. Infinite Loops

If your rule's output can trigger another rule that produces the original input:
```
distribute: a(b+c) -> ab+ac
factor_common: ab+ac -> a(b+c)
```

LEMMA has loop detection, but try to avoid this by:
- Making rules prefer simpler outputs
- Setting appropriate `cost` values

### 2. Forgetting Box

Expressions are boxed for recursion:
```rust
// Wrong
Expr::Add(a, b)

// Right
Expr::Add(Box::new(a), Box::new(b))
```

### 3. Not Cloning

Expressions need to be cloned when reused:
```rust
// Wrong
result: base

// Right  
result: base.clone()
```

---

## Quality Checklist

Before submitting a PR:

- [ ] `cargo test --workspace` passes
- [ ] `cargo run --release --example benchmark_advanced` shows 100%
- [ ] New rule has at least one test case
- [ ] Rule description is clear and accurate
- [ ] Rule ID is unique (check existing IDs)

---

## Questions?

- Open an issue with the `question` label
- Check existing issues for similar questions

---

## Code of Conduct

Be respectful. We're all here to learn and build something useful.

---

## Contact

For questions or discussions:
- **Email**: kharatpushp16@outlook.com
- **GitHub Issues**: Open an issue with the `question` label

---

Thank you for contributing to LEMMA!