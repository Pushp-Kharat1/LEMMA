// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-rules
//!
//! Mathematical transformation rules for the LEMMA system.
//! Contains 500+ rules for IMO-level problem solving.
//!
//! ## Rule Categories
//! - Basic algebra and simplification
//! - Calculus (derivatives and integration)
//! - Trigonometry (identities and special values)
//! - Number theory (divisibility, modular arithmetic)
//! - Inequalities (AM-GM, Cauchy-Schwarz)
//! - Combinatorics (binomial, counting)
//! - Polynomials (Vieta's, symmetric functions)

pub mod algebra;
pub mod calculus;
pub mod combinatorics;
pub mod equations;
pub mod inequalities;
pub mod integration;
pub mod number_theory;
pub mod polynomials;
pub mod rule;
pub mod trig;

pub use rule::{Rule, RuleApplication, RuleCategory, RuleContext, RuleId, RuleSet};
