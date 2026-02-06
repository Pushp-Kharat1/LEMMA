// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! # mm-boink
//!
//! **BOINK** - Budget-Optimized Inference with Neural Knowledge
//!
//! This crate implements LEMMA's self-regulating supervisor layer that:
//! - Analyzes problems and allocates credit budgets
//! - Tracks rule application costs
//! - Manages a global credit bank with savings/penalties
//! - Provides guardrails to filter irrelevant rules by domain
//! - Pattern matching for fast-path solutions
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                   BOINK SUPERVISOR                           │
//! │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
//! │  │ Analyzer │→ │ Budget   │→ │ Tracker  │→ │ Bank     │    │
//! │  │          │  │ Allocator│  │          │  │          │    │
//! │  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Credit System
//!
//! - Each rule has a `cost: u32`
//! - Problems get budgets based on detected domains
//! - Unused credits go to a global bank
//! - At 20,000 credits: unlock premium features

pub mod bank;
pub mod budget;
pub mod guardrail;
pub mod patterns;
pub mod supervisor;

pub use bank::{Bank, TradeOption};
pub use budget::{Budget, Difficulty};
pub use guardrail::{analyze, filter_rules, is_rule_applicable, ProblemProfile};
pub use patterns::{match_integral_pattern, IntegralForm};
pub use supervisor::{BoinkSupervisor, RunResult};
