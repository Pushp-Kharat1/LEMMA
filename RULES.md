# LEMMA Rule Reference

Complete reference of all mathematical transformation rules implemented in LEMMA.

## Quick Stats

| Category | Count |
|----------|-------|
| Algebra | 13 |
| Calculus (Derivatives) | 8 |
| Calculus (Integration) | 9 |
| Trigonometry | 26 |
| Equation Solving | 7 |
| **Total** | **63** |

---

## Algebra Rules

### Basic Identities

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 1 | `add_identity` | x + 0 = x | Additive identity |
| 2 | `mul_identity` | x × 1 = x | Multiplicative identity |
| 3 | `mul_zero` | x × 0 = 0 | Zero property |
| 4 | `double` | x + x = 2x | Combine like terms |
| 5 | `collect_like` | ax + bx = (a+b)x | Collect like terms |

### Expansion & Factoring

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 6 | `distribute` | a(b + c) = ab + ac | Distributive property |
| 7 | `factor_common` | ab + ac = a(b + c) | Factor common term |
| 8 | `diff_of_squares` | a² - b² = (a+b)(a-b) | Difference of squares |

### Power Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 9 | `power_of_one` | x¹ = x | Power of one |
| 10 | `power_of_zero` | x⁰ = 1 | Power of zero |
| 11 | `power_add` | xᵃ × xᵇ = xᵃ⁺ᵇ | Product of powers |
| 12 | `power_mul` | (xᵃ)ᵇ = xᵃᵇ | Power of a power |
| 13 | `neg_neg` | -(-x) = x | Double negation |

---

## Calculus Rules (Derivatives)

### Basic Derivative Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 14 | `const_derivative` | d/dx(c) = 0 | Constant rule |
| 13 | `var_derivative` | d/dx(x) = 1 | Variable rule |

### Sum & Product Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 14 | `sum_derivative` | d/dx(f + g) = f' + g' | Sum rule |
| - | `diff_derivative` | d/dx(f - g) = f' - g' | Difference rule |

### Power & Chain Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 17 | `power_derivative` | d/dx(xⁿ) = n·xⁿ⁻¹ | Power rule |
| 15 | `chain_rule_sin` | d/dx(sin(g)) = cos(g)·g' | Chain rule (sin) |
| 16 | `chain_rule_cos` | d/dx(cos(g)) = -sin(g)·g' | Chain rule (cos) |

### Trigonometric Derivatives

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 18 | `exp_derivative` | d/dx(eˣ) = eˣ | Exponential rule |

---

## Calculus Rules (Integration)

### Basic Integration Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 30 | `power_integral` | ∫xⁿ dx = xⁿ⁺¹/(n+1) | Power rule (n ≠ -1) |
| 31 | `constant_integral` | ∫c dx = cx | Constant rule |
| 37 | `one_over_x_integral` | ∫(1/x) dx = ln\|x\| | Natural log rule |

### Sum & Difference Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 32 | `sum_integral` | ∫(f + g) dx = ∫f dx + ∫g dx | Sum rule |
| 33 | `difference_integral` | ∫(f - g) dx = ∫f dx - ∫g dx | Difference rule |
| 38 | `constant_multiple` | ∫c·f dx = c·∫f dx | Constant multiple |

### Trigonometric Integrals

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 34 | `sin_integral` | ∫sin(x) dx = -cos(x) | Sine integral |
| 35 | `cos_integral` | ∫cos(x) dx = sin(x) | Cosine integral |
| 36 | `exp_integral` | ∫eˣ dx = eˣ | Exponential integral |

---

## Trigonometry Rules

### Pythagorean Identity

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 19 | `pythagorean` | sin²(x) + cos²(x) = 1 | Pythagorean identity |

### Double Angle Formulas

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 20 | `sin_double` | sin(2x) = 2·sin(x)·cos(x) | Sine double angle |
| 57 | `sin_sum_formula` | 2·sin(x)·cos(x) = sin(2x) | Reverse form |
| 58 | `cos_sum_formula` | cos²(x) - sin²(x) = cos(2x) | Cosine double angle |

### Zero Values

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| - | `sin_zero` | sin(0) = 0 | Sine of zero |
| - | `cos_zero` | cos(0) = 1 | Cosine of zero |
| - | `tan_zero` | tan(0) = 0 | Tangent of zero |

### Pi Values

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 40 | `sin_pi` | sin(π) = 0 | Sine of pi |
| 41 | `cos_pi` | cos(π) = -1 | Cosine of pi |
| 42 | `sin_pi_over_2` | sin(π/2) = 1 | Sine of pi/2 |
| 43 | `cos_pi_over_2` | cos(π/2) = 0 | Cosine of pi/2 |
| 44 | `sin_pi_over_4` | sin(π/4) = √2/2 | Sine of pi/4 |
| 45 | `cos_pi_over_4` | cos(π/4) = √2/2 | Cosine of pi/4 |
| 46 | `sin_pi_over_6` | sin(π/6) = 1/2 | Sine of pi/6 |
| 47 | `cos_pi_over_6` | cos(π/6) = √3/2 | Cosine of pi/6 |
| 48 | `sin_pi_over_3` | sin(π/3) = √3/2 | Sine of pi/3 |
| 49 | `cos_pi_over_3` | cos(π/3) = 1/2 | Cosine of pi/3 |

### Quotient Identities

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 50 | `tan_identity` | tan(x) = sin(x)/cos(x) | Tangent definition |
| 51 | `sec_identity` | 1/cos(x) = sec(x) | Secant definition |
| 52 | `csc_identity` | 1/sin(x) = csc(x) | Cosecant definition |
| 53 | `cot_identity` | cos(x)/sin(x) = cot(x) | Cotangent definition |

### Negative Angle Identities

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 54 | `sin_neg` | sin(-x) = -sin(x) | Sine is odd |
| 55 | `cos_neg` | cos(-x) = cos(x) | Cosine is even |
| 56 | `tan_neg` | tan(-x) = -tan(x) | Tangent is odd |

---

## Equation Solving Rules

### Isolation Rules

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 21 | `isolate_variable` | ax + b = c → ax = c - b | Move constant |
| 22 | `cancel_addition` | x + a = b → x = b - a | Cancel addition |
| 23 | `cancel_subtraction` | x - a = b → x = b + a | Cancel subtraction |
| 24 | `cancel_multiplication` | ax = b → x = b/a | Cancel multiplication |
| 25 | `cancel_division` | x/a = b → x = ab | Cancel division |

### Polynomial Solving

| ID | Name | Formula | Description |
|----|------|---------|-------------|
| 26 | `linear_solve` | ax + b = 0 → x = -b/a | Linear equations |
| 27 | `quadratic_formula` | ax² + bx + c = 0 → x = (-b ± √(b²-4ac))/2a | Quadratic formula |

---

## Mathematical Constants

| Constant | Symbol | Value |
|----------|--------|-------|
| Pi | π | 3.14159... |
| Euler's number | e | 2.71828... |

---

## Usage Examples

```bash
# Start the LEMMA demo
cargo run --release --example demo

# Example commands:
lemma> simplify x^2 * x^3
Result: x^5

lemma> deriv sin(x^2)
d/dx(sin(x^2)) = cos(x^2) * 2 * x

lemma> solve 2*x + 3 = 7
Solution: x = 2
```

---

## Adding New Rules

See [CONTRIBUTING.md](CONTRIBUTING.md) for the rule template and guidelines.

Each rule requires:
1. Unique `RuleId`
2. Pattern matching in `is_applicable`
3. Transformation in `apply`
4. Category classification
5. Cost estimation

---

*Last updated: January 1, 2026*
