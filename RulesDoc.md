# LEMMA Rules Documentation

## Derivative Rules

### Rule 408: `constant_base_exp_simple`
**Formula:** `d/dx(a^x) = a^x·ln(a)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:725-765`  
**Purpose:** Differentiates exponential with constant base and variable exponent  
**Example:** `d/dx(2^x) = 2^x·ln(2)`  

### Rule 409: `constant_base_exp_chain`
**Formula:** `d/dx(a^f(x)) = a^f(x)·ln(a)·f'(x)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:770-820`  
**Purpose:** Differentiates exponential with constant base and composite exponent  
**Example:** `d/dx(2^(cos²x)) = 2^(cos²x)·ln(2)·d/dx(cos²x)`  


### Rule 476: `sqrt_chain_rule`
**Formula:** `d/dx(√f(x)) = f'(x)/(2√f(x))`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:830-867`  
**Purpose:** Square root derivative with chain rule  
**Example:** `d/dx(√(x²+1)) = x/√(x²+1)`  


### Rule 475: `general_power_rule`
**Formula:** `d/dx(f(x)^n) = n·f(x)^(n-1)·f'(x)`  
**Added:** 2026-01-14  
**File:** `crates/mm-rules/src/calculus.rs:873-923`  
**Purpose:** Power rule for composite functions with constant exponent  
**Example:** `d/dx((x²+1)³) = 3(x²+1)²·2x`  


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

### Rule 472: `sec_derivative`
**Formula:** `d/dx(sec(f)) = f'·sec(f)·tan(f) = f'·sin(f)/cos²(f)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1030-1079`  
**Purpose:** Secant derivative with chain rule  
**Example:** `d/dx(sec(x²)) = 2x·sec(x²)·tan(x²)`  
**Note:** Matches pattern `1/cos(f)`

### Rule 473: `csc_derivative`
**Formula:** `d/dx(csc(f)) = -f'·csc(f)·cot(f) = -f'·cos(f)/sin²(f)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1085-1134`  
**Purpose:** Cosecant derivative with chain rule  
**Example:** `d/dx(csc(x²)) = -2x·csc(x²)·cot(x²)`  
**Note:** Matches pattern `1/sin(f)`

### Rule 474: `cot_derivative`
**Formula:** `d/dx(cot(f)) = -f'/sin²(f)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1140-1186`  
**Purpose:** Cotangent derivative with chain rule  
**Example:** `d/dx(cot(x²)) = -2x/sin²(x²)`  
**Note:** Matches pattern `cos(f)/sin(f)`

### Rule 413: `arcsin_derivative`
**Formula:** `d/dx(arcsin(f)) = f'/√(1-f²)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1195-1241`  
**Purpose:** Inverse sine derivative with chain rule  
**Example:** `d/dx(arcsin(x²)) = 2x/√(1-x⁴)`  
**Note:** Requires mm-core Expr enum update (Arcsin variant added)

### Rule 414: `arccos_derivative`
**Formula:** `d/dx(arccos(f)) = -f'/√(1-f²)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1247-1296`  
**Purpose:** Inverse cosine derivative with chain rule  
**Example:** `d/dx(arccos(x²)) = -2x/√(1-x⁴)`  
**Note:** Requires mm-core Expr enum update (Arccos variant added)

### Rule 415: `arctan_derivative`
**Formula:** `d/dx(arctan(f)) = f'/(1+f²)`  
**Added:** 2026-01-15  
**File:** `crates/mm-rules/src/calculus.rs:1302-1345`  
**Purpose:** Inverse tangent derivative with chain rule  
**Example:** `d/dx(arctan(x²)) = 2x/(1+x⁴)`  
**Note:** Requires mm-core Expr enum update (Arctan variant added)

## Integration Rules

### Rule 420: `integral_power`
**Formula:** `∫x^n dx = x^(n+1)/(n+1) + C` (n ≠ -1)  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1491-1550`  
**Purpose:** Power rule for integration  
**Example:** `∫x² dx = x³/3`, `∫x dx = x²/2`  
**Note:** Excludes n=-1 (handled by integral_ln)

### Rule 421: `integral_constant`
**Formula:** `∫k dx = kx + C`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1551-1582`  
**Purpose:** Integration of constants  
**Example:** `∫5 dx = 5x`  

### Rule 422: `integral_sum`
**Formula:** `∫(f+g) dx = ∫f dx + ∫g dx`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1583-1621`  
**Purpose:** Sum rule for integration  
**Example:** `∫(x²+3x) dx = ∫x² dx + ∫3x dx`  

### Rule 423: `integral_exp`
**Formula:** `∫e^x dx = e^x + C`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1622-1650`  
**Purpose:** Exponential integration  
**Example:** `∫e^x dx = e^x`  

### Rule 424: `integral_ln`
**Formula:** `∫1/x dx = ln|x| + C`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1651-1683`  
**Purpose:** Reciprocal integration  
**Example:** `∫1/x dx = ln|x|`  

### Rule 425: `integral_sin`
**Formula:** `∫sin(x) dx = -cos(x) + C`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1684-1714`  
**Purpose:** Sine integration  
**Example:** `∫sin(x) dx = -cos(x)`  

### Rule 426: `integral_cos`
**Formula:** `∫cos(x) dx = sin(x) + C`  
**Added:** 2026-01-16  
**File:** `crates/mm-rules/src/calculus.rs:1715-1745`  
**Purpose:** Cosine integration  
**Example:** `∫cos(x) dx = sin(x)`  

---

**Total Rules Added:** 19 (12 derivatives + 7 integrals)  
**Total Derivative Rules:** 25  
**Total Integration Rules:** 7 (basic rules now functional)  
**Next Available ID:** 427  
**Build Status:** ✅ Compiles successfully  
**Tests:** ✅ All mm-rules tests pass (5/5)  
**Coverage:** ~95% derivative, ~40% basic integration  
**Core Changes:** ✅ Added Arcsin, Arccos, Arctan to mm-core Expr enum

## Files Updated (2026-01-15)
**For inverse trig derivative implementation:**
- `mm-core/src/expr.rs` - Added Arcsin/Arccos/Arctan enum variants + trait implementations
- `mm-core/src/eval.rs` - Added asin/acos/atan evaluation support
- `mm-core/src/canon.rs` - Added canonicalization for inverse trig functions
- `mm-rules/src/calculus.rs` - Implemented Rules 413-415 + updated contains_var helper
- `mm-rules/src/case_analysis.rs` - Updated collect_vars_recursive pattern matching
- `mm-rules/src/quantifier.rs` - Updated substitute pattern matching
- `mm-macro/src/lib.rs` - Added inverse trig to expr_to_token_stream
- `mm-verifier/src/lib.rs` - Updated is_calculus_expr and substitute functions
- `mm-brain/src/encoder.rs` - Added arcsin/arccos/arctan tokenization
- `RulesDoc.md` - Documented new rules and file changes

## Files Updated (2026-01-16)
**For basic integration rules implementation:**
- `mm-rules/src/calculus.rs` - Implemented 7 integration rules (420-426)
- `RulesDoc.md` - Documented integration rules
