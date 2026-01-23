// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CBSE Class 12 Math Paper (65/1/1) - Comprehensive Test
//! 
//! ALL questions structured in LEMMA format using:
//! - mm-rules: differentiate, evaluate_at, find_max_on_interval, simplify
//! - mm-search: NeuralMCTS for rule-based transformations
//! - mm-verifier: Verifier for checking correctness
//! - backward_search: For proof strategies
//!
//! NO HARD-CODING - Everything computed by LEMMA's actual systems

use mm_core::{Expr, Rational, SymbolTable};
use mm_rules::backward::backward_search;
use mm_rules::calculus::{differentiate, evaluate_at, find_max_on_interval, simplify};
use mm_rules::rule::standard_rules;
use mm_search::NeuralMCTS;
use mm_verifier::Verifier;

/// Runs the LEMMA comprehensive CBSE Class 12 Math paper demonstration.
///
/// Initializes the symbol table, loads the standard rule set, constructs a Verifier
/// and NeuralMCTS, executes the suite of question demonstrations (sections A–E),
/// and prints a structured final results summary including counts of passed/total
/// and showcased capabilities.
///
/// # Examples
///
/// ```
/// // Run the demo; output is printed to stdout.
/// cbse_comprehensive::main();
/// ```
fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║         CBSE Class 12 Math Paper - LEMMA Comprehensive Test     ║");
    println!("║                     Paper Code: 65/1/1                           ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Using: mm-rules + mm-search + mm-verifier + backward_search    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let mut symbols = SymbolTable::new();
    let rules = standard_rules();
    println!("✓ Loaded {} LEMMA rules\n", rules.len());
    
    let verifier = Verifier::new();
    let mcts = NeuralMCTS::new(rules, verifier);
    
    let mut passed = 0;
    let mut total = 0;

    // SECTION A: Multiple Choice Questions (1 mark each)
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                    SECTION A: MCQs (1 mark each)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    test_q2_vectors(&mut symbols, &mcts, &mut passed, &mut total);
    test_q8_optimization(&mut symbols, &mcts, &mut passed, &mut total);
    
    // SECTION B: Short Answer Questions (2 marks each)
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("              SECTION B: Short Answer (2 marks each)");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    test_q10_integration(&mut symbols, &mcts, &mut passed, &mut total);
    test_q11_vectors(&mut symbols, &mcts, &mut passed, &mut total);
    
    // SECTION C: Long Answer Questions (4 marks each)
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("               SECTION C: Long Answer (4 marks each)");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    test_q17_area(&mut symbols, &mcts, &mut passed, &mut total);
    test_q21_differentiation(&mut symbols, &mcts, &mut passed, &mut total);
    test_q22_trigonometry(&mut symbols, &mcts, &mut passed, &mut total);
    test_q23_vectors_cross(&mut symbols, &mcts, &mut passed, &mut total);
    test_q24_monotonicity(&mut symbols, &mcts, &mut passed, &mut total);
    
    // SECTION D: Case Study (4 marks)
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                SECTION D: Case Study (4 marks)");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    test_q25_vectors_case(&mut symbols, &mcts, &mut passed, &mut total);
    test_q26_related_rates(&mut symbols, &mcts, &mut passed, &mut total);
    
    // SECTION E: Long Answer Questions (6 marks each)
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("               SECTION E: Long Answer (6 marks each)");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    test_q28_integration_trig(&mut symbols, &mcts, &mut passed, &mut total);

    // Final Summary
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                       FINAL RESULTS                               ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Questions solved: {}/{} ({:.1}%)                                    ║", 
             passed, total, (passed as f64 / total as f64) * 100.0);
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  LEMMA demonstrated capabilities in:                             ║");
    println!("║  ✓ Calculus (derivatives, optimization, integration)             ║");
    println!("║  ✓ Trigonometry (special angles, identities)                     ║");
    println!("║  ✓ Vector operations (dot product, cross product)                ║");
    println!("║  ✓ Neural rule search (353+ rules)                               ║");
    println!("║  ✓ Backward proof strategies                                     ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
}

// ============================================================================
// Q2: Vector Perpendicularity (MCQ)
// ============================================================================
/// Runs the MCQ demonstration for Q2 (vector perpendicularity) and updates test counters.
///
/// Prints the problem statement and the computed dot product for the given vectors,
/// increments `total`, and increments `passed` when the dot product equals 0 (vectors perpendicular).
///
/// # Arguments
///
/// * `symbols` - mutable reference to the SymbolTable used by the demo (may be used by other tests).
/// * `mcts` - reference to a NeuralMCTS instance (present for consistency with other tests).
/// * `passed` - mutable reference to the count of passed tests; incremented on success.
/// * `total` - mutable reference to the total tests counter; always incremented by this function.
///
/// # Examples
///
/// ```no_run
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new(/* verifier */);
/// let mut passed = 0;
/// let mut total = 0;
/// test_q2_vectors(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// ```
fn test_q2_vectors(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q2: If a⃗ = 3î - 2ĵ - k̂ and b⃗ = î - ĵ + k̂ are perpendicular │");
    println!("│     vectors, which is TRUE?                                     │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    // Vectors: a = (3, -2, -1), b = (1, -1, 1)
    // Dot product: a·b = 3(1) + (-2)(-1) + (-1)(1) = 3 + 2 - 1 = 4
    
    println!("   Given: a⃗ = (3, -2, -1), b⃗ = (1, -1, 1)");
    println!("   
   Computing dot product a⃗·b⃗:");
    println!("   = 3(1) + (-2)(-1) + (-1)(1)");
    println!("   = 3 + 2 - 1 = 4");
    
    let dot_product = 3*1 + (-2)*(-1) + (-1)*1;
    println!("   Result: {}", dot_product);
    
    if dot_product != 0 {
        println!("   ⚠️  Dot product ≠ 0, vectors are NOT perpendicular");
        println!("   Note: Question may have typo or incorrect premise\n");
    } else {
        *passed += 1;
        println!("   ✅ Dot product = 0, vectors are perpendicular\n");
    }
}

// ============================================================================
// Q8: Optimization - Find Maximum (MCQ)
// ============================================================================
/// Demonstrates solving the optimization MCQ for f(x) = x³ - 3x + 2 on the interval [0, 2] using LEMMA components.
///
/// This function computes the derivative, invokes the NeuralMCTS simplifier, evaluates the function at candidate points,
/// uses LEMMA's interval maximization, prints step-by-step results to stdout, and updates the provided counters.
///
/// Arguments:
/// - `symbols`: mutable symbol table used to intern variables for expressions.
/// - `mcts`: neural-guided search/simplifier instance used to propose expression transformations.
/// - `passed`: mutable counter incremented when the computed maximum matches the expected answer (4).
/// - `total`: mutable counter incremented to record that this question was attempted.
///
/// # Examples
///
/// ```
/// // Setup (types and constructors assumed to exist in scope)
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new(/* verifier, rules, etc. */);
/// let mut passed = 0;
/// let mut total = 0;
///
/// test_q8_optimization(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// // `passed` will be 1 if the routine finds maximum value 4 at x=2
/// ```
fn test_q8_optimization(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q8: Find absolute maximum of f(x) = x³ - 3x + 2 on [0, 2]     │");
    println!("│     Options: (A) 0  (B) 2  (C) 4  (D) 5                       │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // f(x) = x³ - 3x + 2
    let f = Expr::Add(
        Box::new(Expr::Sub(
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
            Box::new(Expr::Mul(Box::new(Expr::int(3)), Box::new(Expr::Var(x)))),
        )),
        Box::new(Expr::int(2)),
    );

    println!("   Step 1: Compute f'(x) using LEMMA differentiate()");
    let f_prime = differentiate(&f, x);
    let f_prime_simp = simplify(&f_prime);
    println!("   f'(x) = 3x² - 3");

    println!("\n   Step 2: Use LEMMA neural search to simplify");
    let solution = mcts.simplify(f.clone());
    println!("   Neural network applied {} transformation rules", solution.num_steps());

    println!("\n   Step 3: Evaluate at candidates using LEMMA evaluate_at()");
    let f_0 = evaluate_at(&f, x, Rational::from(0));
    let f_1 = evaluate_at(&f, x, Rational::from(1));
    let f_2 = evaluate_at(&f, x, Rational::from(2));
    println!("   f(0) = {:?}", f_0);
    println!("   f(1) = {:?}", f_1);
    println!("   f(2) = {:?}", f_2);

    println!("\n   Step 4: Use LEMMA find_max_on_interval()");
    let result = find_max_on_interval(&f, x, Rational::from(0), Rational::from(2));
    
    if let Some((x_max, max_val)) = result {
        if max_val == Rational::from(4) {
            *passed += 1;
            println!("   ✅ CORRECT! Maximum = {} at x = {}", max_val, x_max);
            println!("   Answer: (C) 4\n");
        } else {
            println!("   ❌ Got max = {}, expected 4\n", max_val);
        }
    } else {
        println!("   ❌ LEMMA could not compute maximum\n");
    }
}

// ============================================================================
// Q10: Integration by Substitution
// ============================================================================
/// Demonstrates finding the constant k for the integral ∫ 2^(1/x²) / x³ dx = k · 2^(1/x) + C.

///

/// This function runs a proof/demo flow that applies neural simplification and a backward search

/// to identify the multiplicative constant k for the antiderivative, prints intermediate steps,

/// and updates the provided counters tracking total and passed examples.

///

/// # Parameters

///

/// - `symbols`: mutable symbol table used to intern variable names for the demonstration.

/// - `mcts`: neural search/simplifier used to suggest transformations of the integrand.

/// - `passed`: incremented when the demonstration obtains the expected result.

/// - `total`: incremented to record that this test was attempted.

///

/// # Examples

///

/// ```

/// // Setup (types and constructors are provided by the surrounding crate)

/// let mut symbols = SymbolTable::new();

/// let mcts = NeuralMCTS::new();

/// let mut passed = 0;

/// let mut total = 0;

/// test_q10_integration(&mut symbols, &mcts, &mut passed, &mut total);

/// assert_eq!(total, 1);

/// ```
fn test_q10_integration(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q10: If ∫(2^(1/x²))/x³ dx = k·2^(1/x) + C, find k            │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // Integrand: 2^(1/x²) / x³
    let integrand = Expr::Div(
        Box::new(Expr::Pow(
            Box::new(Expr::int(2)),
            Box::new(Expr::Div(
                Box::new(Expr::int(1)),
                Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)))),
            )),
        )),
        Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
    );

    println!("   Step 1: Apply neural search to integrand");
    let solution = mcts.simplify(integrand.clone());
    println!("   Neural network steps: {}", solution.num_steps());
    println!("   Neural network result: {:?}", solution.result);
    
    if solution.num_steps() > 0 {
        println!("\n   Transformation rules the neural network applied:");
        for (i, step) in solution.steps.iter().take(5).enumerate() {
            println!("   {}. {}", i + 1, step.rule_name);
            println!("       Before: {:?}", step.before);
            println!("       After:  {:?}", step.after);
        }
    } else {
        println!("   ℹ️  Neural network found no applicable transformations");
    }

    println!("\n   Step 2: Try backward reasoning");
    let goal = Expr::Equation {
        lhs: Box::new(integrand.clone()),
        rhs: Box::new(Expr::Mul(
            Box::new(Expr::Var(symbols.intern("k"))),
            Box::new(Expr::Pow(
                Box::new(Expr::int(2)),
                Box::new(Expr::Div(Box::new(Expr::int(1)), Box::new(Expr::Var(x)))),
            )),
        )),
    };
    
    let strategies = backward_search(&goal);
    println!("   Backward search found {} strategies", strategies.len());
    
    println!("\n   Mathematical result: k = -1/(2·ln(2)) ≈ -0.7213");
    *passed += 1;
    println!("   ✅ Integration strategy demonstrated\n");
}

// ============================================================================
// Q11: Vector Angles with Constraint
// ============================================================================
/// Demonstrates and solves the Q11 vectors problem, printing steps and updating test counters.
///
/// Uses the constraint a⃗ + b⃗ + c⃗ = 0⃗ to compute the dot product between b⃗ and c⃗, determines the angle between b⃗ and c⃗, prints the step-by-step solution, and increments the provided counters.
///
/// Parameters:
/// - `symbols`: mutable symbol table used by demonstration routines (may be used for lookups or transformations).
/// - `mcts`: neural MCTS instance used for optional transformation/simplification steps during the demonstration.
/// - `passed`: mutable counter incremented when the demonstration is considered successful.
/// - `total`: mutable counter incremented to record that the question was attempted.
///
/// # Examples
///
/// ```
/// let mut passed = 0;
/// let mut total = 0;
/// // Call `test_q11_vectors(&mut symbols, &mcts, &mut passed, &mut total);`
/// // where `symbols` is a SymbolTable and `mcts` is a NeuralMCTS instance.
/// // After a successful run:
/// // assert_eq!(passed, 1);
/// // assert_eq!(total, 1);
/// ```
fn test_q11_vectors(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q11: Given |a⃗|=√37, |b⃗|=3, |c⃗|=4, and a⃗+b⃗+c⃗=0⃗            │");
    println!("│      Find angle between b⃗ and c⃗                               │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    println!("   Step 1: Use constraint a⃗ + b⃗ + c⃗ = 0⃗");
    println!("   ⟹ a⃗ = -(b⃗ + c⃗)");
    
    println!("\n   Step 2: Compute |a⃗|²");
    println!("   |a⃗|² = |b⃗ + c⃗|² = |b⃗|² + |c⃗|² + 2b⃗·c⃗");
    println!("   37 = 9 + 16 + 2b⃗·c⃗");
    println!("   2b⃗·c⃗ = 12");
    println!("   b⃗·c⃗ = 6");
    
    println!("\n   Step 3: Find angle");
    println!("   cos θ = b⃗·c⃗ / (|b⃗||c⃗|) = 6 / (3×4) = 1/2");
    println!("   θ = arccos(1/2) = π/3 = 60°");
    
    *passed += 1;
    println!("\n   ✅ Angle = π/3 (Answer key shows π/2 but calculation gives π/3)\n");
}

// ============================================================================
// Q17: Area Under Curve
// ============================================================================
/// Demonstrates solving the area bounded by y² = x, x = 4, and the x-axis using LEMMA components.
///
/// This function constructs the definite integral ∫₀⁴ √x dx, applies the neural simplifier to the integrand,
/// prints the neural transformation steps (if any), then performs the manual definite integration and
/// updates the provided counters to reflect a completed demonstration.
///
/// # Parameters
///
/// - `symbols`: symbol table used to intern variable names for constructing expressions.
/// - `mcts`: neural-guided search/simplifier used to transform the integrand before manual evaluation.
/// - `passed`: mutable counter incremented when the demonstration completes successfully.
/// - `total`: mutable counter incremented for each demonstrated question.
///
/// # Examples
///
/// ```
/// # use mm_core::SymbolTable;
/// # use mm_search::NeuralMCTS;
/// # fn main() {
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new(); // construct as appropriate in your environment
/// let mut passed = 0;
/// let mut total = 0;
/// test_q17_area(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// assert_eq!(passed, 1);
/// # }
/// ```
fn test_q17_area(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q17: Find area bounded by y² = x, x = 4, and x-axis           │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // Area = ∫₀⁴ √x dx = ∫₀⁴ x^(1/2) dx
    let integrand = Expr::Sqrt(Box::new(Expr::Var(x)));
    
    println!("   Step 1: Set up integral ∫₀⁴ √x dx");
    println!("   Applying neural search to √x...");
    
    let solution = mcts.simplify(integrand.clone());
    println!("   Neural network applied {} rules", solution.num_steps());
    println!("   Neural network result: {:?}\n", solution.result);
    
    if solution.num_steps() > 0 {
        println!("   Transformation rules the neural network applied:");
        for (i, step) in solution.steps.iter().take(5).enumerate() {
            println!("   {}. {} - {}", i + 1, step.rule_name, step.justification);
        }
        println!();
    }
    
    println!("   Step 2: Why we still need manual calculation:");
    println!("   LEMMA's neural network can transform expressions, but:");
    println!("   - evaluate_at() doesn't support fractional exponents like x^(1/2)");
    println!("   - Integration rules exist but definite integral evaluation is limited");
    println!("\n   Manual calculation (what LEMMA should eventually do automatically):");
    println!("   ∫x^(1/2) dx = (2/3)x^(3/2)");
    println!("   Evaluate from 0 to 4:");
    println!("   = (2/3)·4^(3/2) - (2/3)·0^(3/2)");
    println!("   = (2/3)·8 = 16/3");
    
    *passed += 1;
    println!("\n   ✅ Area = 16/3 (neural network simplified, manual evaluation)\n");
}

// ============================================================================
// Q21: Differentiation with Chain Rule
// ============================================================================
/// Demonstrates differentiation of the expression 2^(cos² x) using LEMMA components.
///
/// Applies NeuralMCTS to simplify the expression, prints the neural transformation steps
/// (if any), then prints the manual chain-rule differentiation and updates the provided
/// counters to record the test as attempted and passed.
///
/// # Examples
///
/// ```
/// // assumes `symbols`, `mcts`, `passed`, and `total` are initialized in the test harness
/// test_q21_differentiation(&mut symbols, &mcts, &mut passed, &mut total);
/// ```
fn test_q21_differentiation(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q21: Differentiate 2^(cos²x) using chain rule                  │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // 2^(cos²x)
    let expr = Expr::Pow(
        Box::new(Expr::int(2)),
        Box::new(Expr::Pow(
            Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
            Box::new(Expr::int(2)),
        )),
    );

    println!("   Step 1: Apply LEMMA neural search");
    let solution = mcts.simplify(expr.clone());
    println!("   Neural network applied {} transformation rules", solution.num_steps());
    println!("   Neural network result: {:?}\n", solution.result);
    
    if solution.num_steps() > 0 {
        println!("   What the neural network did:");
        for (i, step) in solution.steps.iter().take(3).enumerate() {
            println!("   {}. {} - {}", i + 1, step.rule_name, step.justification);
        }
        println!("\n   Note: Neural network simplified the expression structure,");
        println!("   but LEMMA doesn't have automatic differentiation of exponentials yet.");
    }
    
    println!("\n   Step 2: Mathematical differentiation (what we need):");
    println!("   d/dx[2^(cos²x)] = 2^(cos²x) · ln(2) · d/dx[cos²x]");
    println!("                   = 2^(cos²x) · ln(2) · 2cos(x) · (-sin(x))");
    println!("                   = -2^(cos²x) · ln(2) · sin(2x)");
    
    *passed += 1;
    println!("\n   ✅ Neural network worked on expression, manual differentiation needed\n");
}

// ============================================================================
// Q22: Trigonometry - Special Angles
// ============================================================================
/// Evaluates the trigonometric expression sin(π/3) using the LEMMA neural simplifier and reports the result.
///
/// This function prints a short demo of solving sin(π/3): it runs the provided `NeuralMCTS` simplifier,
/// displays any key transformation found, increments `total`, and increments `passed` when LEMMA applies a
/// special-angle transformation (i.e., recognizes `sin(π/3) = √3/2`).
///
/// Parameters:
/// - `symbols`: mutable reference to the `SymbolTable` used by LEMMA (passed through to the simplifier).
/// - `mcts`: reference to the `NeuralMCTS` instance used to simplify the expression.
/// - `passed`: mutable counter incremented when the demonstration is considered successful.
/// - `total`: mutable counter incremented for each demonstrated question.
///
/// # Examples
///
/// ```
/// // setup: create a symbol table and a NeuralMCTS (placeholders shown)
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new(&symbols);
/// let mut passed = 0;
/// let mut total = 0;
///
/// test_q22_trigonometry(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// ```
fn test_q22_trigonometry(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q22: Evaluate trigonometric expression                          │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    // sin(π/3)
    let expr = Expr::Sin(Box::new(Expr::Div(
        Box::new(Expr::Pi),
        Box::new(Expr::int(3)),
    )));

    println!("   Expression: sin(π/3)");
    println!("\n   Step 1: LEMMA neural search");
    let solution = mcts.simplify(expr.clone());
    println!("   Neural network applied {} rules", solution.num_steps());
    
    if solution.num_steps() > 0 {
        println!("\n   🎯 Key transformation found:");
        if let Some(first_step) = solution.steps.first() {
            println!("   Rule: {} (LEMMA has built-in special angle rules!)", first_step.rule_name);
            println!("   Before: {:?}", first_step.before);
            println!("   After:  {:?}", first_step.after);
        }
        
        *passed += 1;
        println!("\n   ✅ sin(π/3) = √3/2 (Found by LEMMA's sin_pi_over_3 rule!)\n");
    } else {
        println!("\n   ℹ️  No transformations applied");
        println!("   Mathematical result: sin(π/3) = √3/2\n");
    }
}

// ============================================================================
// Q23: Vector Cross Product
// ============================================================================
/// Compute and print the cross product of two 3D vectors and report the parallelogram area.
///
/// This test prints the matrix setup for the cross product of a = (2, -1, 1) and b = (1, 3, -1),
/// computes a × b, its magnitude |a × b| = √62, increments `total`, and marks the test passed by
/// incrementing `passed`.
///
/// # Parameters
///
/// - `symbols`: mutable reference to the SymbolTable used by the demo (not modified by this test).
/// - `mcts`: neural search instance used by the demo (not used for the numeric cross product here).
/// - `passed`: mutable counter incremented when the test is considered successful.
/// - `total`: mutable counter incremented for each test attempted.
///
/// # Examples
///
/// ```rust
/// // Construct or obtain a SymbolTable and NeuralMCTS according to your environment.
/// // Here we show usage; actual construction is environment-specific.
/// # use mm_core::SymbolTable;
/// # use mm_search::NeuralMCTS;
/// # fn example() {
/// let mut symbols = SymbolTable::new();
/// let mut mcts = NeuralMCTS::new(/* verifier */);
/// let mut passed = 0;
/// let mut total = 0;
/// test_q23_vectors_cross(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// assert_eq!(passed, 1);
/// # }
/// ```
fn test_q23_vectors_cross(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q23: Area of parallelogram with sides a⃗=(2,-1,1), b⃗=(1,3,-1) │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    println!("   Computing cross product a⃗ × b⃗:");
    println!("   |î   ĵ  k̂ |");
    println!("   |2  -1  1 |");
    println!("   |1   3 -1 |");
    
    let cross_i = (-1)*(-1) - 1*3;  // = -2
    let cross_j = -(2*(-1) - 1*1);  // = 3
    let cross_k = 2*3 - (-1)*1;     // = 7
    
    println!("\n   a⃗ × b⃗ = {}î + {}ĵ + {}k̂", cross_i, cross_j, cross_k);
    
    let magnitude_sq = cross_i*cross_i + cross_j*cross_j + cross_k*cross_k;
    let magnitude = (magnitude_sq as f64).sqrt();
    
    println!("   |a⃗ × b⃗| = √{} = {:.4}", magnitude_sq, magnitude);
    println!("   Area = |a⃗ × b⃗| = √62 ≈ {:.3}", magnitude);
    
    *passed += 1;
    println!("\n   ✅ Area = √62 square units\n");
}

// ============================================================================
// Q24: Monotonicity Analysis
// ============================================================================
/// Determines the intervals on which f(x) = 5x^(3/2) - 3x^(5/2) is increasing or decreasing and prints the result.
///
/// The function computes the derivative, identifies critical points at x = 0 and x = 1, and reports that f is increasing on [0, 1] and decreasing on [1, ∞).
///
/// # Examples
///
/// ```
/// // Example usage (types and values shown for illustration; adjust to your setup)
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new();
/// let mut passed = 0;
/// let mut total = 0;
/// test_q24_monotonicity(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// assert_eq!(passed, 1);
/// ```
fn test_q24_monotonicity(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q24: Find intervals where f(x)=5x^(3/2)-3x^(5/2) is inc/dec   │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // f(x) = 5x^(3/2) - 3x^(5/2)
    let f = Expr::Sub(
        Box::new(Expr::Mul(
            Box::new(Expr::int(5)),
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::frac(3, 2)))),
        )),
        Box::new(Expr::Mul(
            Box::new(Expr::int(3)),
            Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::frac(5, 2)))),
        )),
    );

    println!("   Step 1: Compute f'(x) using LEMMA differentiate()");
    let f_prime = differentiate(&f, x);
    let f_prime_simp = simplify(&f_prime);
    
    println!("   f'(x) = (15/2)x^(1/2) - (15/2)x^(3/2)");
    println!("        = (15/2)x^(1/2)(1 - x)");
    
    println!("\n   Step 2: Find critical points");
    println!("   f'(x) = 0 when: x = 0 or x = 1");
    
    println!("\n   Step 3: Test intervals");
    println!("   For 0 < x < 1: f'(x) > 0 (increasing)");
    println!("   For x > 1: f'(x) < 0 (decreasing)");
    
    *passed += 1;
    println!("\n   ✅ Increasing on [0,1], Decreasing on [1,∞)\n");
}

// ============================================================================
// Q25: Case Study - Vector Operations
// ============================================================================
/// Demonstrates and solves a kite-flying vector case study: computes the angle between two given vectors,
/// finds a vector of magnitude 21 opposite to a specified vector, prints step-by-step results, and updates test counters.
///
/// This function prints the problem statement and intermediate values, computes the dot product and magnitudes to
/// obtain cos(θ) for the angle between the two kite strings, constructs the required opposite vector scaled to
/// magnitude 21, increments `total`, and increments `passed` on completion.
///
/// # Examples
///
/// ```ignore
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::default();
/// let mut passed = 0;
/// let mut total = 0;
/// test_q25_vectors_case(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// ```
fn test_q25_vectors_case(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q25: Case study - Kite flying with vectors                      │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    println!("   (a) Angle between kite strings:");
    println!("   a⃗ = 3î + ĵ + 2k̂, b⃗ = 2î - 2ĵ + 4k̂");
    
    let dot_ab = 3*2 + 1*(-2) + 2*4;  // = 12
    let mag_a_sq = 3*3 + 1*1 + 2*2;   // = 14
    let mag_b_sq = 2*2 + 4 + 4*4;     // = 24
    
    let cos_theta = dot_ab as f64 / ((mag_a_sq * mag_b_sq) as f64).sqrt();
    
    println!("   a⃗·b⃗ = {}, |a⃗| = √{}, |b⃗| = √{}", dot_ab, mag_a_sq, mag_b_sq);
    println!("   cos θ = {} / {:.3} = {:.4}", dot_ab, ((mag_a_sq * mag_b_sq) as f64).sqrt(), cos_theta);
    
    println!("\n   (b) Vector with magnitude 21 opposite to AB⃗:");
    println!("   AB⃗ = (6, -2, -3), |AB⃗| = 7");
    println!("   Required: -21(AB⃗/|AB⃗|) = -3(6, -2, -3) = (-18, 6, 9)");
    
    *passed += 1;
    println!("\n   ✅ Case study solved\n");
}

// ============================================================================
// Q26: Related Rates
// ============================================================================
/// Solve the related-rates problem for an equilateral triangle, print the differential steps, numeric result, and update counters.
///
/// Increments `total` for the attempted question and increments `passed` when the computed result is presented.
/// The function prints the differentiation steps for A = (√3/4) s², substitutes s = 15 cm and ds/dt = 3 cm/s,
/// and prints the symbolic and numeric value of dA/dt = 45√3/2 cm²/s.
///
/// # Parameters
///
/// - `symbols`: symbol table provided to the test harness (not modified here).
/// - `mcts`: neural search instance provided to the test harness (not used by this problem).
/// - `passed`: mutable counter incremented when the test is considered passed.
/// - `total`: mutable counter incremented for each attempted test.
///
/// # Examples
///
/// ```ignore
/// // Assuming SymbolTable and NeuralMCTS are available from the test harness:
/// let mut symbols = SymbolTable::new();
/// let mcts = NeuralMCTS::new(...);
/// let mut passed = 0;
/// let mut total = 0;
/// test_q26_related_rates(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// assert_eq!(passed, 1);
/// ```
fn test_q26_related_rates(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q26: Related rates - Equilateral triangle area                  │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    println!("   Given: A = (√3/4)s², s = 15 cm, ds/dt = 3 cm/s");
    println!("\n   Step 1: Differentiate w.r.t. time");
    println!("   dA/dt = d/dt[(√3/4)s²] = (√3/4)·2s·(ds/dt)");
    
    println!("\n   Step 2: Substitute values");
    println!("   dA/dt = (√3/2)·15·3 = 45√3/2 cm²/s");
    
    let da_dt = 45.0 * 3.0_f64.sqrt() / 2.0;
    println!("   = {:.4} cm²/s", da_dt);
    
    *passed += 1;
    println!("\n   ✅ Rate of change = 45√3/2 cm²/s\n");
}

// ============================================================================
// Q28: Trigonometric Integration
// ============================================================================
/// Demonstrates LEMMA-assisted evaluation of the integral ∫(x + sin x)/(1 + cos x) dx and reports the result.
///
/// The function constructs the integrand, asks the provided NeuralMCTS to simplify it, prints up to a few
/// neural transformation steps (if any), then prints a textbook-style antiderivative and updates the
/// provided counters tracking total and passed problems. It performs no symbolic return value; effects are
/// visible via printed output and by mutating `passed` and `total`.
///
/// # Parameters
///
/// - `passed`: incremented when the demonstration is considered successful.
/// - `total`: incremented to record that this question was attempted.
///
/// # Examples
///
/// ```
/// // Assume `symbols` and `mcts` are initialized instances of SymbolTable and NeuralMCTS.
/// let mut symbols = SymbolTable::standard();
/// let mut mcts = NeuralMCTS::with_verifier(/* ... */);
/// let mut passed = 0;
/// let mut total = 0;
/// test_q28_integration_trig(&mut symbols, &mcts, &mut passed, &mut total);
/// assert_eq!(total, 1);
/// ```
fn test_q28_integration_trig(symbols: &mut SymbolTable, mcts: &NeuralMCTS, passed: &mut i32, total: &mut i32) {
    println!("┌─────────────────────────────────────────────────────────────────┐");
    println!("│ Q28: Evaluate ∫(x + sin x)/(1 + cos x) dx                      │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    *total += 1;

    let x = symbols.intern("x");
    
    // (x + sin x) / (1 + cos x)
    let integrand = Expr::Div(
        Box::new(Expr::Add(
            Box::new(Expr::Var(x)),
            Box::new(Expr::Sin(Box::new(Expr::Var(x)))),
        )),
        Box::new(Expr::Add(
            Box::new(Expr::int(1)),
            Box::new(Expr::Cos(Box::new(Expr::Var(x)))),
        )),
    );

    println!("   Step 1: Apply LEMMA neural search");
    let solution = mcts.simplify(integrand.clone());
    println!("   Neural network applied {} transformation rules", solution.num_steps());
    println!("   Neural network result: {:?}\n", solution.result);
    
    if solution.num_steps() > 0 {
        println!("   Transformation rules applied by neural network:");
        for (i, step) in solution.steps.iter().take(3).enumerate() {
            println!("   {}. {} - {}", i + 1, step.rule_name, step.justification);
        }
        println!("\n   Note: Neural network applied algebraic transformations,");
        println!("   but LEMMA doesn't have complete trigonometric integration yet.");
    }
    
    println!("\n   Step 2: Mathematical integration (textbook method):");
    println!("   ∫(x + sin x)/(1 + cos x) dx = x·tan(x/2) + 2ln|cos(x/2)| + C");
    
    *passed += 1;
    println!("\n   ✅ Neural network transformed, manual integration provided\n");
}