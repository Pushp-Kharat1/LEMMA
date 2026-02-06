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
pub mod geometry;
pub mod guardrail;
pub mod induction;
pub mod inequalities;
pub mod inequality_chain;
pub mod integration;
pub mod number_theory;
pub mod patterns;
pub mod polynomial;
pub mod polynomials;
pub mod quantifier;
pub mod rule;
pub mod trig;

pub use guardrail::{
    analyze, decompose_additive, filter_rules, is_rule_applicable, solvability_score,
    ProblemProfile,
};
pub use patterns::match_integral_pattern;
pub use rule::{
    Domain, Feature, Rule, RuleApplication, RuleCategory, RuleContext, RuleId, RuleSet,
};
