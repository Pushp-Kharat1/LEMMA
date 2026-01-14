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

/// Compute the discriminant of ax² + bx + c
/// D = b² - 4ac
pub fn discriminant(a: Rational, b: Rational, c: Rational) -> Rational {
    b * b - Rational::from(4) * a * c
}

/// Solve the quadratic equation ax² + bx + c = 0
/// Returns the roots using the quadratic formula: x = (-b ± √D) / 2a
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

/// Check if an integer is a perfect square and return its square root
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

/// Get the sum of roots: α + β = -b/a
pub fn sum_of_roots(a: Rational, b: Rational, _c: Rational) -> Rational {
    if a.is_zero() {
        return Rational::from(0);
    }
    (Rational::from(0) - b) / a
}

/// Get the product of roots: αβ = c/a
pub fn product_of_roots(a: Rational, _b: Rational, c: Rational) -> Rational {
    if a.is_zero() {
        return Rational::from(0);
    }
    c / a
}

// ============================================================================
// Arithmetic Progression (AP)
// ============================================================================

/// Get the nth term of an AP: aₙ = a + (n-1)d
/// where a = first term, d = common difference, n = term number (1-indexed)
pub fn ap_nth_term(a: Rational, d: Rational, n: i64) -> Rational {
    a + Rational::from(n - 1) * d
}

/// Get the sum of first n terms of an AP: Sₙ = n/2 × [2a + (n-1)d]
pub fn ap_sum(a: Rational, d: Rational, n: i64) -> Rational {
    let n_rat = Rational::from(n);
    n_rat / Rational::from(2) * (Rational::from(2) * a + Rational::from(n - 1) * d)
}

/// Alternative sum formula: Sₙ = n/2 × (first + last)
pub fn ap_sum_with_last(first: Rational, last: Rational, n: i64) -> Rational {
    Rational::from(n) / Rational::from(2) * (first + last)
}

/// Find the common difference given two terms
pub fn ap_common_difference(term_m: Rational, term_n: Rational, m: i64, n: i64) -> Rational {
    if m == n {
        return Rational::from(0);
    }
    (term_n - term_m) / Rational::from(n - m)
}

// ============================================================================
// Geometric Progression (GP)
// ============================================================================

/// Get the nth term of a GP: aₙ = a × r^(n-1)
/// where a = first term, r = common ratio, n = term number (1-indexed)
pub fn gp_nth_term(a: Rational, r: Rational, n: i64) -> Rational {
    a * r.pow((n - 1) as i32)
}

/// Get the sum of first n terms of a GP: Sₙ = a(rⁿ - 1)/(r - 1) for r ≠ 1
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

/// Sum of infinite GP: S∞ = a/(1-r) for |r| < 1
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

/// Find the common ratio given two terms
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

/// Compute the indefinite integral of an expression with respect to a variable.
/// Returns the antiderivative (without the +C constant).
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

/// Check if expression contains a variable
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

/// Compute the definite integral: ∫[a,b] f(x) dx = F(b) - F(a)
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
