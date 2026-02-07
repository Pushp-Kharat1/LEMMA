// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-rules
//!
//! Mathematical transformation rules for the LEMMA system.
//!
//! ## Rule Categories (162 working rules)
//! - Basic algebra and simplification (36 rules)
//! - Trigonometry (43 rules)
//! - Number theory (28 working, 56 need implementation)
//! - Inequalities (20 working, 12 need implementation)
//! - Calculus derivatives (15 working)
//! - Integration (9 rules)
//! - Equations (7 rules)
//! - Combinatorics (1 working, 45 need implementation)
//! - Polynomials (3 working, 36 need implementation)

pub mod algebra;
pub mod backward;
pub mod board_exam;
pub mod calculus;
pub mod case_analysis;
pub mod combinatorics;
pub mod equations;
pub mod induction;
pub mod inequalities;
pub mod inequality_chain;
pub mod integration;
pub mod number_theory;
pub mod polynomial;
pub mod polynomials;
pub mod quantifier;
pub mod rule;
pub mod trig;

pub use rule::{Rule, RuleApplication, RuleCategory, RuleContext, RuleId, RuleSet};
// Re-export commonly used helpers so doctest examples work with the default `use mm_rules::*;`
pub use algebra::*;
pub use board_exam::*;
pub use calculus::*;
pub use case_analysis::*;
pub use combinatorics::*;
pub use equations::*;
pub use induction::*;
pub use inequalities::*;
pub use inequality_chain::*;
pub use integration::*;
pub use mm_core::{Expr, Rational, Symbol};
pub use number_theory::*;
pub use polynomial::*;
pub use polynomials::*;
pub use quantifier::*;
pub use trig::*;
