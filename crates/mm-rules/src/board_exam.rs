// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Board Exam Algebra Module
//!
//! Provides functions for solving common board exam algebra problems:
//! - Quadratic equations (roots, discriminant, sum/product of roots)
//! - Arithmetic Progressions (nth term, sum)
//! - Geometric Progressions (nth term, sum)

use mm_core::{Expr, Rational, Symbol};

// ============================================================================
// Quadratic Equations: ax² + bx + c = 0
// ============================================================================

/// Result of solving a quadratic equation
#[derive(Debug, Clone, PartialEq)]
pub enum QuadraticRoots {
    /// Two distinct real roots
    TwoReal(Rational, Rational),
    /// One repeated real root (discriminant = 0)
    OneReal(Rational),
    /// Complex roots (discriminant < 0) - we store the real and imaginary parts
    Complex {
        real: Rational,
        imaginary_squared: Rational,
    },
    /// Cannot solve (a = 0, not quadratic)
    NotQuadratic,
}

/// Compute the discriminant of a quadratic equation ax² + bx + c.
///
/// The discriminant determines the nature of the roots of the quadratic:
/// D = b² - 4ac.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let a = Rational::from(1);
/// let b = Rational::from(-5);
/// let c = Rational::from(6);
/// let d = discriminant(a, b, c);
/// assert_eq!(d, Rational::from(1)); // 25 - 24 = 1
/// ```
///
/// # Returns
///
/// The discriminant D = b² - 4ac as a `Rational`.
pub fn discriminant(a: Rational, b: Rational, c: Rational) -> Rational {
    b * b - Rational::from(4) * a * c
}

/// Solves the quadratic equation ax² + bx + c = 0 and returns its roots.
///
/// The function returns one of the `QuadraticRoots` variants:
/// - `QuadraticRoots::NotQuadratic` when `a == 0`.
/// - `QuadraticRoots::OneReal(r)` when the discriminant is zero (repeated real root).
/// - `QuadraticRoots::TwoReal(r1, r2)` when there are two distinct real roots. If the discriminant is a perfect square the roots are returned exactly as rationals; otherwise approximate rationals are used.
/// - `QuadraticRoots::Complex { real, imaginary_squared }` when the discriminant is negative; `real` is the real part and `imaginary_squared` equals the square of the imaginary part.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let roots = solve_quadratic(Rational::from(1), Rational::from(-5), Rational::from(6));
/// match roots {
///     QuadraticRoots::TwoReal(r1, r2) => {
///         let s1 = r1.to_f64();
///         let s2 = r2.to_f64();
///         assert!((s1 - 2.0).abs() < 1e-9 || (s2 - 2.0).abs() < 1e-9);
///         assert!((s1 - 3.0).abs() < 1e-9 || (s2 - 3.0).abs() < 1e-9);
///     }
///     _ => panic!("expected two real roots"),
/// }
/// ```
pub fn solve_quadratic(a: Rational, b: Rational, c: Rational) -> QuadraticRoots {
    if a.is_zero() {
        return QuadraticRoots::NotQuadratic;
    }

    let d = discriminant(a, b, c);
    let two_a = Rational::from(2) * a;
    let neg_b = Rational::from(0) - b;

    if d.is_zero() {
        // D = 0: One repeated root
        let root = neg_b / two_a;
        QuadraticRoots::OneReal(root)
    } else if d.is_positive() {
        // D > 0: Two distinct real roots
        // We need to check if D is a perfect square for rational roots
        let d_numer = d.numer();
        let d_denom = d.denom();

        // Try to find integer square root
        if let (Some(sqrt_numer), Some(sqrt_denom)) =
            (int_sqrt(d_numer.abs()), int_sqrt(d_denom.abs()))
        {
            let sqrt_d = Rational::new(sqrt_numer, sqrt_denom);
            let root1 = (neg_b + sqrt_d) / two_a;
            let root2 = (neg_b - sqrt_d) / two_a;
            QuadraticRoots::TwoReal(root1, root2)
        } else {
            // D is not a perfect square - roots are irrational
            // For board exams, we still report them symbolically
            // Store as approximate for now
            let sqrt_d_approx = Rational::new((d.to_f64().sqrt() * 1000.0) as i64, 1000);
            let root1 = (neg_b + sqrt_d_approx) / two_a;
            let root2 = (neg_b - sqrt_d_approx) / two_a;
            QuadraticRoots::TwoReal(root1, root2)
        }
    } else {
        // D < 0: Complex roots
        QuadraticRoots::Complex {
            real: neg_b / two_a,
            imaginary_squared: (Rational::from(0) - d) / (two_a * two_a),
        }
    }
}

/// Return the integer square root when `n` is a perfect square.
///
/// Returns `Some(k)` where `k * k == n` for `n >= 0`, or `None` if `n` is negative or not a perfect square.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// assert_eq!(int_sqrt(16), Some(4));
/// assert_eq!(int_sqrt(15), None);
/// assert_eq!(int_sqrt(0), Some(0));
/// assert_eq!(int_sqrt(-1), None);
/// ```
fn int_sqrt(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }
    if n == 0 {
        return Some(0);
    }
    let sqrt = (n as f64).sqrt() as i64;
    if sqrt * sqrt == n {
        Some(sqrt)
    } else if (sqrt + 1) * (sqrt + 1) == n {
        Some(sqrt + 1)
    } else {
        None
    }
}

/// Compute the sum of the roots of a quadratic equation using Vieta's formula.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let s = sum_of_roots(Rational::from(1), Rational::from(-5), Rational::from(6)); // roots 2 and 3
/// assert_eq!(s, Rational::from(5));
/// ```
///
/// # Returns
///
/// `-b / a` if `a` ≠ 0; `0` if `a` == 0.
pub fn sum_of_roots(a: Rational, b: Rational, _c: Rational) -> Rational {
    if a.is_zero() {
        return Rational::from(0);
    }
    (Rational::from(0) - b) / a
}

/// Compute the product of the roots of the quadratic equation `ax² + bx + c = 0`.
///
/// # Returns
///
/// The product `αβ` of the roots: `c / a` when `a != 0`, otherwise `0`.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let prod = product_of_roots(Rational::from(1), Rational::from(-5), Rational::from(6));
/// assert_eq!(prod, Rational::from(6)); // roots 2 and 3 -> product 6
/// ```
pub fn product_of_roots(a: Rational, _b: Rational, c: Rational) -> Rational {
    if a.is_zero() {
        return Rational::from(0);
    }
    c / a
}

// ============================================================================
// Arithmetic Progression (AP)
// ============================================================================

/// Compute the nth term of an arithmetic progression (1-indexed).
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let a = Rational::from(2); // first term
/// let d = Rational::from(3); // common difference
/// let t4 = ap_nth_term(a, d, 4); // 2 + (4-1)*3 = 11
/// assert_eq!(t4, Rational::from(11));
/// ```
pub fn ap_nth_term(a: Rational, d: Rational, n: i64) -> Rational {
    a + Rational::from(n - 1) * d
}

/// Compute the sum of the first n terms of an arithmetic progression.
///
/// Formula: Sₙ = n/2 × [2a + (n - 1)d]
///
/// Returns the sum as a `Rational`.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let a = Rational::from(1);
/// let d = Rational::from(1);
/// let sum = ap_sum(a, d, 10);
/// assert_eq!(sum, Rational::from(55));
/// ```
pub fn ap_sum(a: Rational, d: Rational, n: i64) -> Rational {
    let n_rat = Rational::from(n);
    n_rat / Rational::from(2) * (Rational::from(2) * a + Rational::from(n - 1) * d)
}

/// Alternative sum formula: Sₙ = n/2 × (first + last)
pub fn ap_sum_with_last(first: Rational, last: Rational, n: i64) -> Rational {
    Rational::from(n) / Rational::from(2) * (first + last)
}

/// Computes the common difference of an arithmetic progression from two terms.
///
/// `term_m` and `term_n` are the terms at positions `m` and `n` (indices), respectively.
/// Returns the `Rational` common difference d = (term_n - term_m) / (n - m).
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let d = ap_common_difference(Rational::from(2), Rational::from(8), 1, 3);
/// assert_eq!(d, Rational::from(3)); // sequence: 2, 5, 8,...
/// ```
pub fn ap_common_difference(term_m: Rational, term_n: Rational, m: i64, n: i64) -> Rational {
    if m == n {
        return Rational::from(0);
    }
    (term_n - term_m) / Rational::from(n - m)
}

// ============================================================================
// Geometric Progression (GP)
// ============================================================================

/// Computes the nth term of a geometric progression.
///
/// The first term is `a`, the common ratio is `r`, and `n` is 1-indexed.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let term = gp_nth_term(2.into(), 3.into(), 4);
/// assert_eq!(term, 54.into()); // 2 * 3^(4-1) = 54
/// ```
pub fn gp_nth_term(a: Rational, r: Rational, n: i64) -> Rational {
    a * r.pow((n - 1) as i32)
}

/// Computes the sum of the first `n` terms of a geometric progression.
///
/// If `r == 1`, returns `n * a`. Otherwise returns `a * (r.pow(n) - 1) / (r - 1)`.
/// Returns `None` if the computation would require division by zero.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let a = Rational::from(1);
/// let r = Rational::from(2);
/// assert_eq!(gp_sum(a, r, 4), Some(Rational::from(15))); // 1 + 2 + 4 + 8 = 15
///
/// let a = Rational::from(3);
/// let r = Rational::from(1);
/// assert_eq!(gp_sum(a, r, 5), Some(Rational::from(15))); // 5 * 3 = 15
/// ```
pub fn gp_sum(a: Rational, r: Rational, n: i64) -> Option<Rational> {
    if r.is_one() {
        // If r = 1, sum is just n × a
        return Some(Rational::from(n) * a);
    }

    let r_n = r.pow(n as i32);
    let numerator = a * (r_n - Rational::from(1));
    let denominator = r - Rational::from(1);

    if denominator.is_zero() {
        None
    } else {
        Some(numerator / denominator)
    }
}

/// Compute the sum of an infinite geometric progression when it converges.
///
/// Returns `Some(sum)` where `sum = a / (1 - r)` if |r| < 1 and the denominator `1 - r` is non-zero; returns `None` if the series diverges or the denominator is zero.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let sum = gp_sum_infinite(Rational::from(1), Rational::from(1) / Rational::from(2));
/// assert_eq!(sum, Some(Rational::from(2)));
/// ```
pub fn gp_sum_infinite(a: Rational, r: Rational) -> Option<Rational> {
    // Check if |r| < 1
    let abs_r = r.abs();
    if abs_r >= Rational::from(1) {
        return None; // Diverges
    }

    let denominator = Rational::from(1) - r;
    if denominator.is_zero() {
        None
    } else {
        Some(a / denominator)
    }
}

/// Determines the common ratio r of a geometric progression given two terms.
///
/// The function returns `Some(r)` when the ratio can be recovered as a rational number:
/// - if the terms are one step apart (n - m = 1), returns `term_n / term_m`;
/// - if they are two steps apart (n - m = 2) and the quotient is a perfect rational square, returns its rational square root.
/// Returns `None` when `m == n`, `term_m` is zero, the gap is not 1 or 2, or a rational root cannot be determined.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// // For GP 2, 6, 18,... term_1 = 2, term_3 = 18 so r = 3
/// let r = gp_common_ratio(Rational::new(2, 1), Rational::new(18, 1), 1, 3);
/// assert_eq!(r, Some(Rational::new(3, 1)));
///
/// // For adjacent terms ratio is direct
/// let r2 = gp_common_ratio(Rational::new(5, 1), Rational::new(15, 1), 1, 2);
/// assert_eq!(r2, Some(Rational::new(3, 1)));
///
/// // Non-computable gap > 2 returns None
/// let r3 = gp_common_ratio(Rational::new(1, 1), Rational::new(8, 1), 1, 4);
/// assert_eq!(r3, None);
/// ```
pub fn gp_common_ratio(term_m: Rational, term_n: Rational, m: i64, n: i64) -> Option<Rational> {
    if m == n || term_m.is_zero() {
        return None;
    }
    // term_n / term_m = r^(n-m)
    // For rational result, only works when (n-m) divides evenly
    let ratio = term_n / term_m;
    let power = n - m;

    if power == 1 {
        Some(ratio)
    } else if power == 2 {
        // Need square root of ratio
        let r_numer = ratio.numer();
        let r_denom = ratio.denom();
        match (int_sqrt(r_numer.abs()), int_sqrt(r_denom.abs())) {
            (Some(sn), Some(sd)) if r_numer >= 0 => Some(Rational::new(sn, sd)),
            _ => None,
        }
    } else {
        None // Can't compute rational nth root
    }
}

// ============================================================================
// Integration (Antiderivatives)
// ============================================================================

/// Compute the antiderivative of an expression with respect to a variable.
///
/// The function returns an expression whose derivative (with respect to `var`) is `expr`.
/// The constant of integration is omitted. Returns `Some(antiderivative)` when `expr` can be
/// integrated by the implemented elementary rules, or `None` when the integrand is not handled.
///
/// Supported forms (not an exhaustive list):
/// - Constants and variables (constants are treated as constant factors; a different variable is treated as constant).
/// - Negation, addition, and subtraction (integrates termwise).
/// - Multiplication when exactly one factor is constant with respect to `var`.
/// - Powers `x^n` where base is the integration variable and `n` is a rational constant (returns `x^(n+1)/(n+1)` for `n != -1`).
/// - Returns `None` for forms not covered (e.g., products where both factors contain `var`, `x^-1`, or other non-elementary cases).
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let x = Symbol::from("x");
/// // ∫ x^2 dx = x^3 / 3
/// let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));
/// let res = integrate(&expr, x).unwrap();
/// let expected = Expr::Div(
///     Box::new(Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(3)))),
///     Box::new(Expr::int(3)),
/// );
/// assert_eq!(res, expected);
/// ```
pub fn integrate(expr: &Expr, var: Symbol) -> Option<Expr> {
    match expr {
        // ∫c dx = cx
        Expr::Const(c) => Some(Expr::Mul(
            Box::new(Expr::Const(*c)),
            Box::new(Expr::Var(var)),
        )),

        // ∫x dx = x²/2
        Expr::Var(v) if *v == var => Some(Expr::Div(
            Box::new(Expr::Pow(Box::new(Expr::Var(var)), Box::new(Expr::int(2)))),
            Box::new(Expr::int(2)),
        )),

        // ∫y dx = yx (y is a different variable, treated as constant)
        Expr::Var(_) => Some(Expr::Mul(Box::new(expr.clone()), Box::new(Expr::Var(var)))),

        // ∫-f dx = -∫f dx
        Expr::Neg(inner) => {
            let inner_int = integrate(inner, var)?;
            Some(Expr::Neg(Box::new(inner_int)))
        }

        // ∫(f + g) dx = ∫f dx + ∫g dx
        Expr::Add(a, b) => {
            let a_int = integrate(a, var)?;
            let b_int = integrate(b, var)?;
            Some(Expr::Add(Box::new(a_int), Box::new(b_int)))
        }

        // ∫(f - g) dx = ∫f dx - ∫g dx
        Expr::Sub(a, b) => {
            let a_int = integrate(a, var)?;
            let b_int = integrate(b, var)?;
            Some(Expr::Sub(Box::new(a_int), Box::new(b_int)))
        }

        // ∫c*f dx = c * ∫f dx (if c is constant w.r.t. var)
        Expr::Mul(a, b) => {
            // Check if one factor is constant
            if !contains_var(a, var) {
                let b_int = integrate(b, var)?;
                Some(Expr::Mul(a.clone(), Box::new(b_int)))
            } else if !contains_var(b, var) {
                let a_int = integrate(a, var)?;
                Some(Expr::Mul(Box::new(a_int), b.clone()))
            } else {
                // Both contain var - can't integrate simply
                None
            }
        }

        // ∫x^n dx = x^(n+1)/(n+1) for n ≠ -1
        Expr::Pow(base, exp) => {
            if let (Expr::Var(v), Expr::Const(n)) = (base.as_ref(), exp.as_ref()) {
                if *v == var {
                    let n_plus_1 = *n + Rational::from(1);
                    if n_plus_1.is_zero() {
                        // ∫x^(-1) dx = ln|x| - can't express in Rational
                        None
                    } else {
                        Some(Expr::Div(
                            Box::new(Expr::Pow(
                                Box::new(Expr::Var(var)),
                                Box::new(Expr::Const(n_plus_1)),
                            )),
                            Box::new(Expr::Const(n_plus_1)),
                        ))
                    }
                } else {
                    // base is different var, treat as constant
                    Some(Expr::Mul(Box::new(expr.clone()), Box::new(Expr::Var(var))))
                }
            } else {
                None
            }
        }

        _ => None,
    }
}

/// Determines whether an expression contains a given variable.
///
/// Returns `true` if the variable appears anywhere inside `expr`, `false` otherwise.
/// The check recurses into common expression forms (negation, addition, subtraction,
/// multiplication, division, and power) and conservatively returns `true` for any
/// expression variants not explicitly handled.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// let x = Symbol::from("x");
/// let expr = Expr::Add(Box::new(Expr::Var(x.clone())), Box::new(Expr::Const(Rational::from(1))));
/// assert!(contains_var(&expr, x));
/// ```
fn contains_var(expr: &Expr, var: Symbol) -> bool {
    match expr {
        Expr::Const(_) => false,
        Expr::Var(v) => *v == var,
        Expr::Neg(e) => contains_var(e, var),
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            contains_var(a, var) || contains_var(b, var)
        }
        Expr::Pow(base, exp) => contains_var(base, var) || contains_var(exp, var),
        _ => true, // Assume complex expressions may contain the var
    }
}

/// Computes the definite integral of `expr` with respect to `var` over [lower, upper].
///
/// Returns the value F(upper) - F(lower) where F is an antiderivative of `expr`.
/// Returns `None` if an antiderivative cannot be found or if evaluation at the bounds fails.
///
/// # Examples
///
/// ```
/// # use mm_rules::*;
/// // ∫_0^2 x dx = 2
/// let x = Symbol::from("x");
/// let expr = Expr::Var(x.clone());
/// let res = definite_integral(&expr, x, Rational::from(0), Rational::from(2));
/// assert_eq!(res, Some(Rational::from(2)));
/// ```
pub fn definite_integral(
    expr: &Expr,
    var: Symbol,
    lower: Rational,
    upper: Rational,
) -> Option<Rational> {
    use crate::calculus::evaluate_at;

    let antideriv = integrate(expr, var)?;
    let f_upper = evaluate_at(&antideriv, var, upper)?;
    let f_lower = evaluate_at(&antideriv, var, lower)?;
    Some(f_upper - f_lower)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mm_core::SymbolTable;

    #[test]
    fn test_quadratic_two_roots() {
        // x² - 5x + 6 = 0 → (x-2)(x-3) = 0 → x = 2, 3
        let roots = solve_quadratic(Rational::from(1), Rational::from(-5), Rational::from(6));

        match roots {
            QuadraticRoots::TwoReal(r1, r2) => {
                assert!(
                    (r1 == Rational::from(3) && r2 == Rational::from(2))
                        || (r1 == Rational::from(2) && r2 == Rational::from(3))
                );
            }
            _ => panic!("Expected two real roots"),
        }
    }

    #[test]
    fn test_quadratic_one_root() {
        // x² - 4x + 4 = 0 → (x-2)² = 0 → x = 2
        let roots = solve_quadratic(Rational::from(1), Rational::from(-4), Rational::from(4));

        assert_eq!(roots, QuadraticRoots::OneReal(Rational::from(2)));
    }

    #[test]
    fn test_quadratic_complex() {
        // x² + 1 = 0 → x = ±i
        let roots = solve_quadratic(Rational::from(1), Rational::from(0), Rational::from(1));

        match roots {
            QuadraticRoots::Complex { real, .. } => {
                assert!(real.is_zero());
            }
            _ => panic!("Expected complex roots"),
        }
    }

    #[test]
    fn test_sum_product_of_roots() {
        // x² - 5x + 6 = 0
        let a = Rational::from(1);
        let b = Rational::from(-5);
        let c = Rational::from(6);

        assert_eq!(sum_of_roots(a, b, c), Rational::from(5)); // 2 + 3 = 5
        assert_eq!(product_of_roots(a, b, c), Rational::from(6)); // 2 × 3 = 6
    }

    #[test]
    fn test_ap_nth_term() {
        // AP: 2, 5, 8, 11... (a=2, d=3)
        let a = Rational::from(2);
        let d = Rational::from(3);

        assert_eq!(ap_nth_term(a, d, 1), Rational::from(2));
        assert_eq!(ap_nth_term(a, d, 2), Rational::from(5));
        assert_eq!(ap_nth_term(a, d, 4), Rational::from(11));
        assert_eq!(ap_nth_term(a, d, 10), Rational::from(29)); // 2 + 9×3 = 29
    }

    #[test]
    fn test_ap_sum() {
        // Sum of 1+2+3+...+10 = 55
        let a = Rational::from(1);
        let d = Rational::from(1);

        assert_eq!(ap_sum(a, d, 10), Rational::from(55));
    }

    #[test]
    fn test_gp_nth_term() {
        // GP: 2, 6, 18, 54... (a=2, r=3)
        let a = Rational::from(2);
        let r = Rational::from(3);

        assert_eq!(gp_nth_term(a, r, 1), Rational::from(2));
        assert_eq!(gp_nth_term(a, r, 2), Rational::from(6));
        assert_eq!(gp_nth_term(a, r, 4), Rational::from(54));
    }

    #[test]
    fn test_gp_sum() {
        // Sum of GP: 1 + 2 + 4 + 8 = 15
        let a = Rational::from(1);
        let r = Rational::from(2);

        assert_eq!(gp_sum(a, r, 4), Some(Rational::from(15)));
    }

    #[test]
    fn test_gp_infinite_sum() {
        // Sum of 1 + 1/2 + 1/4 + ... = 2
        let a = Rational::from(1);
        let r = Rational::new(1, 2);

        assert_eq!(gp_sum_infinite(a, r), Some(Rational::from(2)));
    }

    #[test]
    fn test_integrate_polynomial() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // ∫x² dx = x³/3
        let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));
        let result = integrate(&expr, x);

        assert!(result.is_some());
    }

    #[test]
    fn test_definite_integral() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // ∫[0,2] x dx = [x²/2]₀² = 2 - 0 = 2
        let expr = Expr::Var(x);
        let result = definite_integral(&expr, x, Rational::from(0), Rational::from(2));

        assert_eq!(result, Some(Rational::from(2)));
    }

    #[test]
    fn test_area_under_parabola() {
        let mut symbols = SymbolTable::new();
        let x = symbols.intern("x");

        // ∫[0,3] x² dx = [x³/3]₀³ = 27/3 - 0 = 9
        let expr = Expr::Pow(Box::new(Expr::Var(x)), Box::new(Expr::int(2)));
        let result = definite_integral(&expr, x, Rational::from(0), Rational::from(3));

        assert_eq!(result, Some(Rational::from(9)));
    }
}
