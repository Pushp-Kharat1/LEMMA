// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Global credit bank for LEMMA.
//!
//! Tracks accumulated savings and enables trading for upgrades.

use mm_rules::RuleId;
use serde::{Deserialize, Serialize};

/// Trading options when bank balance is sufficient.
#[derive(Debug, Clone)]
pub enum TradeOption {
    /// Unlock a premium rule (cost: 5,000 credits)
    UnlockRule(RuleId),
    /// Increase MCTS search depth by 5 (cost: 10,000 credits)
    DepthUpgrade,
    /// Increase retry limit by 1 (cost: 3,000 credits)
    ExtraRetry,
}

/// Global credit bank state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    /// Current credit balance
    pub credits: u64,
    /// Rules unlocked through trading
    pub unlocked_rules: Vec<u32>,
    /// Bonus MCTS depth purchased
    pub extra_depth: u32,
    /// Bonus retries purchased
    pub extra_retries: u32,
    /// Total credits ever earned
    pub lifetime_earnings: u64,
    /// Total credits ever spent (penalties + trades)
    pub lifetime_spent: u64,
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            credits: 0,
            unlocked_rules: Vec::new(),
            extra_depth: 0,
            extra_retries: 0,
            lifetime_earnings: 0,
            lifetime_spent: 0,
        }
    }
}

impl Bank {
    /// Cost thresholds
    pub const RULE_UNLOCK_COST: u64 = 5_000;
    pub const DEPTH_UPGRADE_COST: u64 = 10_000;
    pub const RETRY_UPGRADE_COST: u64 = 3_000;
    pub const AUTO_TRADE_THRESHOLD: u64 = 20_000;

    /// Create a new empty bank.
    pub fn new() -> Self {
        Self::default()
    }

    /// Deposit credits (from successful problem solving).
    pub fn deposit(&mut self, amount: u64) {
        self.credits += amount;
        self.lifetime_earnings += amount;
    }

    /// Withdraw credits (for penalties).
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.credits >= amount {
            self.credits -= amount;
            self.lifetime_spent += amount;
            true
        } else {
            false
        }
    }

    /// Check if trading is available.
    pub fn can_trade(&self) -> bool {
        self.credits >= Self::AUTO_TRADE_THRESHOLD
    }

    /// Get available trade options based on current balance.
    pub fn available_trades(&self) -> Vec<TradeOption> {
        let mut options = Vec::new();

        if self.credits >= Self::RETRY_UPGRADE_COST {
            options.push(TradeOption::ExtraRetry);
        }
        if self.credits >= Self::RULE_UNLOCK_COST {
            // Could add specific rule suggestions here
            options.push(TradeOption::UnlockRule(RuleId(0)));
        }
        if self.credits >= Self::DEPTH_UPGRADE_COST {
            options.push(TradeOption::DepthUpgrade);
        }

        options
    }

    /// Execute a trade.
    pub fn execute_trade(&mut self, option: TradeOption) -> Result<(), &'static str> {
        match option {
            TradeOption::UnlockRule(rule_id) => {
                if self.credits < Self::RULE_UNLOCK_COST {
                    return Err("Insufficient credits for rule unlock");
                }
                self.credits -= Self::RULE_UNLOCK_COST;
                self.lifetime_spent += Self::RULE_UNLOCK_COST;
                self.unlocked_rules.push(rule_id.0);
                Ok(())
            }
            TradeOption::DepthUpgrade => {
                if self.credits < Self::DEPTH_UPGRADE_COST {
                    return Err("Insufficient credits for depth upgrade");
                }
                self.credits -= Self::DEPTH_UPGRADE_COST;
                self.lifetime_spent += Self::DEPTH_UPGRADE_COST;
                self.extra_depth += 5;
                Ok(())
            }
            TradeOption::ExtraRetry => {
                if self.credits < Self::RETRY_UPGRADE_COST {
                    return Err("Insufficient credits for retry upgrade");
                }
                self.credits -= Self::RETRY_UPGRADE_COST;
                self.lifetime_spent += Self::RETRY_UPGRADE_COST;
                self.extra_retries += 1;
                Ok(())
            }
        }
    }

    /// Save bank state to JSON.
    pub fn save(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Load bank state from JSON.
    pub fn load(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    /// Save bank state to a file.
    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = self.save();
        std::fs::write(path, json)
    }

    /// Load bank state from a file.
    pub fn load_from_file(path: &std::path::Path) -> Option<Self> {
        let json = std::fs::read_to_string(path).ok()?;
        Self::load(&json)
    }

    /// Get the default bank file path (in user's home directory).
    pub fn default_path() -> std::path::PathBuf {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());
        std::path::PathBuf::from(home).join(".lemma_bank.json")
    }
}
