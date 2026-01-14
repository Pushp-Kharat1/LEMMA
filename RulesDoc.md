# LEMMA Rules Documentation

## Derivative Rules

### Rule 408: `constant_base_exp_simple`
**Formula:** `d/dx(a^x) = a^x·ln(a)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:725-765`  
**Purpose:** Differentiates exponential with constant base and variable exponent  
**Example:** `d/dx(2^x) = 2^x·ln(2)`  
**Fixes:** CBSE Q21 (partial)

### Rule 409: `constant_base_exp_chain`
**Formula:** `d/dx(a^f(x)) = a^f(x)·ln(a)·f'(x)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:770-820`  
**Purpose:** Differentiates exponential with constant base and composite exponent  
**Example:** `d/dx(2^(cos²x)) = 2^(cos²x)·ln(2)·d/dx(cos²x)`  
**Fixes:** CBSE Q21 (complete)

### Rule 476: `sqrt_chain_rule`
**Formula:** `d/dx(√f(x)) = f'(x)/(2√f(x))`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:830-867`  
**Purpose:** Square root derivative with chain rule  
**Example:** `d/dx(√(x²+1)) = x/√(x²+1)`  
**Coverage:** Very high frequency in CBSE/JEE

### Rule 475: `general_power_rule`
**Formula:** `d/dx(f(x)^n) = n·f(x)^(n-1)·f'(x)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:873-923`  
**Purpose:** Power rule for composite functions with constant exponent  
**Example:** `d/dx((x²+1)³) = 3(x²+1)²·2x`  
**Coverage:** Extremely high frequency in CBSE/JEE

### Rule 411: `log_base_simple`
**Formula:** `d/dx(log_a(x)) = 1/(x·ln(a))`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:929-971`  
**Purpose:** Logarithm derivative with arbitrary base  
**Example:** `d/dx(log₂(x)) = 1/(x·ln(2))`  
**Note:** Matches pattern `ln(x)/ln(a)`

### Rule 412: `log_base_chain`
**Formula:** `d/dx(log_a(f(x))) = f'(x)/(f(x)·ln(a))`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:977-1021`  
**Purpose:** Logarithm derivative with chain rule  
**Example:** `d/dx(log₁₀(sin(x))) = cos(x)/(sin(x)·ln(10))`  
**Note:** Matches pattern `ln(f)/ln(a)`

---

**Total Rules Added:** 6  
**Total Derivative Rules:** 19 (was 13)  
**Next Available ID:** 413  
**Build Status:** ✅ Compiles successfully  
**Tests:** ✅ All mm-rules tests pass
