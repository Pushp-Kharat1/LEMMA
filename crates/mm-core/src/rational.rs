// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Exact rational number arithmetic.
//!
//! We use rational numbers instead of floating-point to avoid rounding errors.
//! `1/3 * 3 = 1` exactly, no floating-point surprises.

use num_rational::Ratio;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A rational number (fraction) for exact arithmetic.
///
/// Internally uses `num_rational::Ratio<i64>` but with additional
/// conveniences for mathematical expression handling.
#[derive(Clone, Copy)]
pub struct Rational(Ratio<i64>);

// Custom serde implementation since Ratio doesn't have serde by default
impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as (numerator, denominator) tuple
        (self.numer(), self.denom()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Rational {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (numer, denom) = <(i64, i64)>::deserialize(deserializer)?;
        if denom == 0 {
            return Err(serde::de::Error::custom("denominator cannot be zero"));
        }
        Ok(Rational::new(numer, denom))
    }
}

impl Rational {
    /// Create a new rational from numerator and denominator.
    ///
    /// The fraction is automatically reduced to lowest terms.
    ///
    /// # Panics
    ///
    /// Panics if denominator is zero.
    pub fn new(numer: i64, denom: i64) -> Self {
        assert!(denom != 0, "Denominator cannot be zero");
        Rational(Ratio::new(numer, denom))
    }

    /// Create a rational from an integer.
    pub fn from_integer(n: i64) -> Self {
        Rational(Ratio::from_integer(n))
    }

    /// Get the numerator.
    pub fn numer(&self) -> i64 {
        *self.0.numer()
    }

    /// Get the denominator.
    pub fn denom(&self) -> i64 {
        *self.0.denom()
    }

    /// Check if this is zero.
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Check if this is one.
    pub fn is_one(&self) -> bool {
        self.0.is_one()
    }

    /// Check if this is a negative number.
    pub fn is_negative(&self) -> bool {
        self.0 < Ratio::zero()
    }

    /// Check if this is a positive number.
    pub fn is_positive(&self) -> bool {
        self.0 > Ratio::zero()
    }

    /// Check if this is an integer (denominator is 1).
    pub fn is_integer(&self) -> bool {
        self.0.is_integer()
    }

    /// Get the absolute value.
    pub fn abs(&self) -> Self {
        Rational(self.0.abs())
    }

    /// Get the reciprocal (1/x).
    ///
    /// # Panics
    ///
    /// Panics if self is zero.
    pub fn recip(&self) -> Self {
        Rational(self.0.recip())
    }

    /// Convert to f64 (lossy).
    pub fn to_f64(&self) -> f64 {
        self.numer() as f64 / self.denom() as f64
    }

    /// Raise to an integer power.
    pub fn pow(&self, exp: i32) -> Self {
        if exp >= 0 {
            Rational(self.0.pow(exp))
        } else {
            Rational(self.0.recip().pow(-exp))
        }
    }
}

// ============================================================================
// Trait implementations
// ============================================================================

impl Default for Rational {
    fn default() -> Self {
        Rational::from_integer(0)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_integer() {
            write!(f, "{}", self.numer())
        } else {
            write!(f, "{}/{}", self.numer(), self.denom())
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_integer() {
            write!(f, "{}", self.numer())
        } else {
            write!(f, "{}/{}", self.numer(), self.denom())
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Rational {}

impl Hash for Rational {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Rationals are always in reduced form, so this is fine
        self.numer().hash(state);
        self.denom().hash(state);
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

// ============================================================================
// Arithmetic operations
// ============================================================================

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Rational(self.0 + rhs.0)
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Rational(self.0 - rhs.0)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational(self.0 * rhs.0)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Rational(self.0 / rhs.0)
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Rational(-self.0)
    }
}

// ============================================================================
// From implementations
// ============================================================================

impl From<i64> for Rational {
    fn from(n: i64) -> Self {
        Rational::from_integer(n)
    }
}

impl From<i32> for Rational {
    fn from(n: i32) -> Self {
        Rational::from_integer(n as i64)
    }
}

impl From<(i64, i64)> for Rational {
    fn from((n, d): (i64, i64)) -> Self {
        Rational::new(n, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 3);

        // 1/2 + 1/3 = 5/6
        let sum = a + b;
        assert_eq!(sum, Rational::new(5, 6));

        // 1/2 * 1/3 = 1/6
        let prod = a * b;
        assert_eq!(prod, Rational::new(1, 6));

        // 1/3 * 3 = 1 (exact!)
        let c = Rational::new(1, 3);
        let three = Rational::from_integer(3);
        assert_eq!(c * three, Rational::from_integer(1));
    }

    #[test]
    fn test_reduction() {
        // 2/4 should reduce to 1/2
        let a = Rational::new(2, 4);
        assert_eq!(a.numer(), 1);
        assert_eq!(a.denom(), 2);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Rational::from_integer(5)), "5");
        assert_eq!(format!("{}", Rational::new(1, 2)), "1/2");
    }

    #[test]
    fn test_power() {
        let half = Rational::new(1, 2);
        assert_eq!(half.pow(2), Rational::new(1, 4));
        assert_eq!(half.pow(-1), Rational::from_integer(2));
    }
}
