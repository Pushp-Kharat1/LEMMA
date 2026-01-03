# LEMMA Rule Reference

Complete reference of all mathematical transformation rules implemented in LEMMA.

## Quick Stats

| Category | Count | Description |
|----------|-------|-------------|
| Algebra | 15 | Basic identities, expansion, factoring |
| Calculus (Derivatives) | 10 | Power, chain, product rules |
| Calculus (Integration) | 9 | Power, trig, exponential integrals |
| Trigonometry | 30 | Identities, special values, formulas |
| Number Theory | 100+ | Divisibility, GCD/LCM, parity |
| Inequalities | 30+ | AM-GM, Cauchy-Schwarz, triangle |
| Combinatorics | 30+ | Binomial, counting, recurrences |
| Polynomials | 30+ | Vieta's, symmetric, factoring |
| Equation Solving | 10 | Linear, quadratic, isolation |
| Calculus (Phase 4) | 50 | Integration, limits, Taylor, vector |
| **Total** | **450+** | |

---

## Algebra Rules

| ID | Name | Formula |
|----|------|---------|
| 1 | `add_identity` | x + 0 = x |
| 2 | `mul_identity` | x × 1 = x |
| 3 | `mul_zero` | x × 0 = 0 |
| 4 | `double` | x + x = 2x |
| 5 | `collect_like` | ax + bx = (a+b)x |
| 6 | `distribute` | a(b + c) = ab + ac |
| 7 | `factor_common` | ab + ac = a(b + c) |
| 8 | `diff_of_squares` | a² - b² = (a+b)(a-b) |
| 9 | `power_of_one` | x¹ = x |
| 10 | `power_of_zero` | x⁰ = 1 |
| 11 | `power_add` | xᵃ × xᵇ = xᵃ⁺ᵇ |
| 12 | `power_mul` | (xᵃ)ᵇ = xᵃᵇ |
| 13 | `neg_neg` | -(-x) = x |

---

## Calculus Rules (Derivatives)

| ID | Name | Formula |
|----|------|---------|
| 13 | `var_derivative` | d/dx(x) = 1 |
| 14 | `const_derivative` | d/dx(c) = 0 |
| 14 | `sum_derivative` | d/dx(f + g) = f' + g' |
| 15 | `chain_rule_sin` | d/dx(sin(g)) = cos(g)·g' |
| 16 | `chain_rule_cos` | d/dx(cos(g)) = -sin(g)·g' |
| 17 | `power_derivative` | d/dx(xⁿ) = n·xⁿ⁻¹ |
| 18 | `exp_derivative` | d/dx(eˣ) = eˣ |

---

## Integration Rules

| ID | Name | Formula |
|----|------|---------|
| 30 | `power_integral` | ∫xⁿ dx = xⁿ⁺¹/(n+1) |
| 31 | `constant_integral` | ∫c dx = cx |
| 32 | `sum_integral` | ∫(f + g) dx = ∫f + ∫g |
| 33 | `difference_integral` | ∫(f - g) dx = ∫f - ∫g |
| 34 | `sin_integral` | ∫sin(x) dx = -cos(x) |
| 35 | `cos_integral` | ∫cos(x) dx = sin(x) |
| 36 | `exp_integral` | ∫eˣ dx = eˣ |
| 37 | `one_over_x` | ∫(1/x) dx = ln|x| |
| 38 | `constant_multiple` | ∫c·f dx = c·∫f |

---

## Trigonometry Rules

### Core Identities
| ID | Name | Formula |
|----|------|---------|
| 19 | `pythagorean` | sin²x + cos²x = 1 |
| 20 | `sin_double` | sin(2x) = 2sin(x)cos(x) |
| 50 | `tan_identity` | tan(x) = sin(x)/cos(x) |

### Special Values
| Rule | Formula |
|------|---------|
| `sin_pi_over_6` | sin(π/6) = 1/2 |
| `cos_pi_over_6` | cos(π/6) = √3/2 |
| `sin_pi_over_4` | sin(π/4) = √2/2 |
| `cos_pi_over_4` | cos(π/4) = √2/2 |
| `sin_pi_over_3` | sin(π/3) = √3/2 |
| `cos_pi_over_3` | cos(π/3) = 1/2 |
| `sin_pi_over_2` | sin(π/2) = 1 |
| `cos_pi_over_2` | cos(π/2) = 0 |

### Negative Angles
| Rule | Formula |
|------|---------|
| `sin_neg` | sin(-x) = -sin(x) |
| `cos_neg` | cos(-x) = cos(x) |
| `tan_neg` | tan(-x) = -tan(x) |

---

## Number Theory Rules

### Divisibility
| ID | Name | Formula |
|----|------|---------|
| 100 | `divides_zero` | n divides 0 |
| 101 | `divides_self` | n/n = 1 |
| 104 | `cancel_common` | (a·b)/a = b |
| 106 | `diff_squares` | a² - b² = (a+b)(a-b) |
| 107 | `diff_cubes` | a³ - b³ = (a-b)(a² + ab + b²) |
| 108 | `sum_cubes` | a³ + b³ = (a+b)(a² - ab + b²) |
| 109 | `square_binomial` | (a+b)² = a² + 2ab + b² |
| 110 | `square_binomial_sub` | (a-b)² = a² - 2ab + b² |

### Perfect Powers
| ID | Name | Formula |
|----|------|---------|
| 160 | `sqrt_square` | √(a²) = |a| |
| 161 | `square_sqrt` | (√a)² = a |
| 162 | `sqrt_product` | √a · √b = √(ab) |
| 163 | `sqrt_quotient` | √(a/b) = √a/√b |
| 164 | `half_power` | a^(1/2) = √a |

### Parity
| ID | Name | Formula |
|----|------|---------|
| 180 | `neg_one_even` | (-1)^(2n) = 1 |
| 181 | `neg_one_odd` | (-1)^(2n+1) = -1 |
| 182 | `neg_squared` | (-a)² = a² |

---

## Inequality Rules

### AM-GM
| ID | Name | Formula |
|----|------|---------|
| 300 | `am_gm_2` | a + b ≥ 2√(ab) |
| 301 | `sum_squares` | a² + b² ≥ 2ab |
| 303 | `reciprocal_sum` | a/b + b/a ≥ 2 |

### Cauchy-Schwarz
| ID | Name | Formula |
|----|------|---------|
| 320 | `cauchy_schwarz` | (a²+b²)(c²+d²) ≥ (ac+bd)² |
| 321 | `titus_lemma` | a²/x + b²/y ≥ (a+b)²/(x+y) |

### Triangle Inequality
| ID | Name | Formula |
|----|------|---------|
| 340 | `triangle` | \|a + b\| ≤ \|a\| + \|b\| |
| 341 | `reverse_triangle` | \|a - b\| ≥ \|\|a\| - \|b\|\| |

### Absolute Value
| ID | Name | Formula |
|----|------|---------|
| 361 | `abs_product` | \|a·b\| = \|a\|·\|b\| |
| 362 | `abs_quotient` | \|a/b\| = \|a\|/\|b\| |
| 363 | `abs_neg` | \|-a\| = \|a\| |
| 364 | `abs_abs` | \|\|a\|\| = \|a\| |
| 365 | `abs_squared` | \|a\|² = a² |

---

## Combinatorics Rules

### Binomial Coefficients
| ID | Name | Formula |
|----|------|---------|
| 400 | `binomial_zero` | C(n,0) = 1 |
| 401 | `binomial_full` | C(n,n) = 1 |
| 402 | `binomial_one` | C(n,1) = n |
| 403 | `binomial_symmetry` | C(n,k) = C(n,n-k) |
| 404 | `pascal` | C(n,k) = C(n-1,k-1) + C(n-1,k) |
| 405 | `hockey_stick` | ΣC(i,k) = C(n+1,k+1) |
| 407 | `binomial_sum` | Σ C(n,k) = 2ⁿ |
| 408 | `binomial_theorem` | (a+b)ⁿ = Σ C(n,k) aᵏbⁿ⁻ᵏ |

### Counting
| ID | Name | Formula |
|----|------|---------|
| 420 | `permutation` | P(n,k) = n!/(n-k)! |
| 421 | `combination` | C(n,k) = n!/(k!(n-k)!) |
| 424 | `inclusion_exclusion` | \|A ∪ B\| = \|A\| + \|B\| - \|A ∩ B\| |
| 427 | `catalan` | Cₙ = C(2n,n)/(n+1) |

---

## Polynomial Rules

### Vieta's Formulas
| ID | Name | Formula |
|----|------|---------|
| 500 | `vieta_sum_quad` | r₁ + r₂ = -b/a |
| 501 | `vieta_product_quad` | r₁ · r₂ = c/a |
| 502 | `vieta_sum_cubic` | r₁ + r₂ + r₃ = -b/a |
| 504 | `vieta_product_cubic` | r₁ · r₂ · r₃ = -d/a |

### Symmetric Polynomials
| ID | Name | Formula |
|----|------|---------|
| 523 | `newton_2` | p₂ = e₁² - 2e₂ |
| 525 | `sum_squares_sym` | x² + y² = (x+y)² - 2xy |
| 527 | `sum_three_cubes` | x³+y³+z³-3xyz = (x+y+z)(x²+y²+z²-xy-yz-zx) |

### Factoring
| ID | Name | Formula |
|----|------|---------|
| 540 | `factor_theorem` | (x-a) \| P(x) ⟺ P(a) = 0 |
| 541 | `remainder_theorem` | P(a) = remainder of P(x)/(x-a) |
| 543 | `complete_square` | x² + bx = (x + b/2)² - b²/4 |
| 560 | `rational_root` | Rational roots = ±(factors of a₀)/(factors of aₙ) |

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
d/dx(sin(x²)) = cos(x²) · 2x

lemma> solve 2*x + 3 = 7
Solution: x = 2
```

---

*Last updated: January 3, 2026*
*Total rules: 450+ (targeting 500+ for IMO-level)*
