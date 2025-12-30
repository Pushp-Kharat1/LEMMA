// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Symbol interning for efficient variable handling.
//!
//! Symbols are interned strings, meaning each unique string is stored only once,
//! and symbols can be compared by simple integer comparison.

use serde::{Deserialize, Serialize};
use string_interner::{DefaultBackend, DefaultSymbol, StringInterner};

/// An interned symbol representing a variable name.
///
/// Symbols are cheap to copy and compare (just an integer under the hood).
pub type Symbol = DefaultSymbol;

/// A table for interning variable symbols.
///
/// Use a single `SymbolTable` throughout your application to ensure
/// symbols can be compared correctly.
///
/// # Example
///
/// ```rust
/// use mm_core::SymbolTable;
///
/// let mut symbols = SymbolTable::new();
/// let x1 = symbols.intern("x");
/// let x2 = symbols.intern("x");
/// let y = symbols.intern("y");
///
/// assert_eq!(x1, x2);  // Same symbol
/// assert_ne!(x1, y);   // Different symbols
/// ```
#[derive(Debug, Default)]
pub struct SymbolTable {
    interner: StringInterner<DefaultBackend>,
}

impl SymbolTable {
    /// Create a new empty symbol table.
    pub fn new() -> Self {
        Self {
            interner: StringInterner::new(),
        }
    }

    /// Intern a string, returning its symbol.
    ///
    /// If the string has been interned before, returns the existing symbol.
    pub fn intern(&mut self, s: &str) -> Symbol {
        self.interner.get_or_intern(s)
    }

    /// Get the string for a symbol, if it exists.
    pub fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.interner.resolve(symbol)
    }

    /// Get the string for a symbol, panicking if not found.
    pub fn resolve_unchecked(&self, symbol: Symbol) -> &str {
        self.interner
            .resolve(symbol)
            .expect("Symbol not found in table")
    }

    /// Check if a string has been interned.
    pub fn contains(&self, s: &str) -> bool {
        self.interner.get(s).is_some()
    }

    /// Get the symbol for a string if it exists.
    pub fn get(&self, s: &str) -> Option<Symbol> {
        self.interner.get(s)
    }

    /// Get the number of interned symbols.
    pub fn len(&self) -> usize {
        self.interner.len()
    }

    /// Check if the table is empty.
    pub fn is_empty(&self) -> bool {
        self.interner.is_empty()
    }
}

/// Wrapper for serializing symbols with their string representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSymbol {
    name: String,
}

impl SerializableSymbol {
    /// Create from a symbol and table.
    pub fn from_symbol(symbol: Symbol, table: &SymbolTable) -> Self {
        Self {
            name: table.resolve_unchecked(symbol).to_string(),
        }
    }

    /// Convert back to a symbol using a table.
    pub fn to_symbol(&self, table: &mut SymbolTable) -> Symbol {
        table.intern(&self.name)
    }

    /// Get the name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_interning() {
        let mut table = SymbolTable::new();

        let x1 = table.intern("x");
        let x2 = table.intern("x");
        let y = table.intern("y");

        assert_eq!(x1, x2);
        assert_ne!(x1, y);
    }

    #[test]
    fn test_symbol_resolution() {
        let mut table = SymbolTable::new();
        let x = table.intern("x");

        assert_eq!(table.resolve(x), Some("x"));
        assert_eq!(table.resolve_unchecked(x), "x");
    }

    #[test]
    fn test_table_operations() {
        let mut table = SymbolTable::new();

        assert!(table.is_empty());
        assert_eq!(table.len(), 0);

        table.intern("x");
        table.intern("y");
        table.intern("x"); // Duplicate

        assert!(!table.is_empty());
        assert_eq!(table.len(), 2);

        assert!(table.contains("x"));
        assert!(table.contains("y"));
        assert!(!table.contains("z"));
    }
}
