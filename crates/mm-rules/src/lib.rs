// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-rules
//!
//! Mathematical transformation rules for the LEMMA system.
//!
//! This crate provides:
//! - [`Rule`] - The core rule structure
//! - [`RuleSet`] - A collection of rules
//! - Pre-defined rules for algebra, calculus, trigonometry, and integration

pub mod algebra;
pub mod calculus;
pub mod equations;
pub mod integration;
pub mod rule;
pub mod trig;

pub use rule::{Rule, RuleApplication, RuleCategory, RuleContext, RuleId, RuleSet};
