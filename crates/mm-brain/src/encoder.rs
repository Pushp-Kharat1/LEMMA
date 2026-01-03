// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Expression encoding for neural network input.

use candle_core::{Device, Result, Tensor};
use mm_core::Expr;
use std::collections::HashMap;
use string_interner::Symbol; // For to_usize() method on symbols

/// Token IDs for special tokens
pub const PAD_TOKEN: u32 = 0;
pub const START_TOKEN: u32 = 1;
pub const END_TOKEN: u32 = 2;
pub const UNK_TOKEN: u32 = 3;

/// Vocabulary for expression encoding.
#[derive(Debug, Clone)]
pub struct Vocabulary {
    token_to_id: HashMap<String, u32>,
    id_to_token: Vec<String>,
}

impl Default for Vocabulary {
    fn default() -> Self {
        Self::new()
    }
}

impl Vocabulary {
    /// Create a new vocabulary with standard mathematical tokens.
    pub fn new() -> Self {
        let tokens = vec![
            // Special tokens (indices 0-3)
            "<PAD>", "<START>", "<END>", "<UNK>", // Operators (4-13)
            "+", "-", "*", "/", "^", "=", "(", ")", ",", ".", // Functions (14-25)
            "sin", "cos", "tan", "ln", "exp", "sqrt", "abs", "d/dx", "∫", "lim", "sum", "prod",
            // Numbers (26-35)
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", // Variables (36-45)
            "x", "y", "z", "a", "b", "c", "n", "t", "u", "v", // Keywords (46-50)
            "pi", "e", "inf", "neg", "frac",
        ];

        let token_to_id: HashMap<String, u32> = tokens
            .iter()
            .enumerate()
            .map(|(i, &t)| (t.to_string(), i as u32))
            .collect();

        let id_to_token: Vec<String> = tokens.iter().map(|&t| t.to_string()).collect();

        Self {
            token_to_id,
            id_to_token,
        }
    }

    /// Get the ID for a token.
    pub fn get_id(&self, token: &str) -> u32 {
        *self.token_to_id.get(token).unwrap_or(&UNK_TOKEN)
    }

    /// Get the token for an ID.
    pub fn get_token(&self, id: u32) -> &str {
        self.id_to_token
            .get(id as usize)
            .map(|s| s.as_str())
            .unwrap_or("<UNK>")
    }

    /// Get vocabulary size.
    pub fn size(&self) -> usize {
        self.id_to_token.len()
    }
}

/// Encoder for converting expressions to tensors.
pub struct ExpressionEncoder {
    vocab: Vocabulary,
    max_length: usize,
    device: Device,
}

impl ExpressionEncoder {
    /// Create a new expression encoder.
    pub fn new(device: Device) -> Self {
        Self {
            vocab: Vocabulary::new(),
            max_length: 64,
            device,
        }
    }

    /// Set maximum sequence length.
    pub fn with_max_length(mut self, len: usize) -> Self {
        self.max_length = len;
        self
    }

    /// Tokenize an expression into a sequence of token strings.
    pub fn tokenize(&self, expr: &Expr) -> Vec<String> {
        let mut tokens = Vec::new();
        self.tokenize_recursive(expr, &mut tokens);
        tokens
    }

    /// Recursively tokenize an expression.
    fn tokenize_recursive(&self, expr: &Expr, tokens: &mut Vec<String>) {
        match expr {
            Expr::Const(r) => {
                if r.is_integer() {
                    let n = r.numer();
                    if n < 0 {
                        tokens.push("neg".to_string());
                    }
                    for c in n.abs().to_string().chars() {
                        tokens.push(c.to_string());
                    }
                } else {
                    tokens.push("frac".to_string());
                    for c in r.numer().to_string().chars() {
                        tokens.push(c.to_string());
                    }
                    tokens.push("/".to_string());
                    for c in r.denom().to_string().chars() {
                        tokens.push(c.to_string());
                    }
                }
            }
            Expr::Var(sym) => {
                // Map symbol index to variable tokens (x, y, z, a, b, c, n, t, u, v)
                // This preserves variable identity - different symbols get different tokens
                let var_tokens = ["x", "y", "z", "a", "b", "c", "n", "t", "u", "v"];
                let idx = sym.to_usize() % var_tokens.len();
                tokens.push(var_tokens[idx].to_string());
            }
            Expr::Pi => {
                tokens.push("pi".to_string());
            }
            Expr::E => {
                tokens.push("e".to_string());
            }
            Expr::Neg(e) => {
                tokens.push("neg".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Add(a, b) => {
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push("+".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Sub(a, b) => {
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push("-".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Mul(a, b) => {
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push("*".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Div(a, b) => {
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push("/".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Pow(a, b) => {
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push("^".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Sqrt(e) => {
                tokens.push("sqrt".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Sin(e) => {
                tokens.push("sin".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Cos(e) => {
                tokens.push("cos".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Tan(e) => {
                tokens.push("tan".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Ln(e) => {
                tokens.push("ln".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Exp(e) => {
                tokens.push("exp".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Abs(e) => {
                tokens.push("abs".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Derivative { expr: e, .. } => {
                tokens.push("d/dx".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Integral { expr: e, .. } => {
                tokens.push("∫".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Sum(terms) => {
                tokens.push("sum".to_string());
                tokens.push("(".to_string());
                for (i, term) in terms.iter().enumerate() {
                    if i > 0 {
                        tokens.push("+".to_string());
                    }
                    self.tokenize_recursive(&term.expr, tokens);
                }
                tokens.push(")".to_string());
            }
            Expr::Product(factors) => {
                tokens.push("prod".to_string());
                tokens.push("(".to_string());
                for (i, factor) in factors.iter().enumerate() {
                    if i > 0 {
                        tokens.push("*".to_string());
                    }
                    self.tokenize_recursive(&factor.base, tokens);
                }
                tokens.push(")".to_string());
            }
            Expr::Equation { lhs, rhs } => {
                self.tokenize_recursive(lhs, tokens);
                tokens.push("=".to_string());
                self.tokenize_recursive(rhs, tokens);
            }
            // Number theory
            Expr::GCD(a, b) => {
                tokens.push("gcd".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::LCM(a, b) => {
                tokens.push("lcm".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Mod(a, b) => {
                tokens.push("mod".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(a, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(b, tokens);
                tokens.push(")".to_string());
            }
            Expr::Binomial(n, k) => {
                tokens.push("C".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(n, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(k, tokens);
                tokens.push(")".to_string());
            }
            Expr::Floor(e) => {
                tokens.push("floor".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Ceiling(e) => {
                tokens.push("ceil".to_string());
                tokens.push("(".to_string());
                self.tokenize_recursive(e, tokens);
                tokens.push(")".to_string());
            }
            Expr::Factorial(e) => {
                self.tokenize_recursive(e, tokens);
                tokens.push("!".to_string());
            }
            Expr::Gte(a, b) => {
                self.tokenize_recursive(a, tokens);
                tokens.push(">=".to_string());
                self.tokenize_recursive(b, tokens);
            }
            Expr::Gt(a, b) => {
                self.tokenize_recursive(a, tokens);
                tokens.push(">".to_string());
                self.tokenize_recursive(b, tokens);
            }
            Expr::Lte(a, b) => {
                self.tokenize_recursive(a, tokens);
                tokens.push("<=".to_string());
                self.tokenize_recursive(b, tokens);
            }
            Expr::Lt(a, b) => {
                self.tokenize_recursive(a, tokens);
                tokens.push("<".to_string());
                self.tokenize_recursive(b, tokens);
            }
            Expr::Summation {
                var,
                from,
                to,
                body,
            } => {
                tokens.push("sum".to_string());
                tokens.push("(".to_string());
                let var_tokens = ["i", "j", "k", "m", "n"];
                let idx = var.to_usize() % var_tokens.len();
                tokens.push(format!("_{}", var_tokens[idx]));
                tokens.push("=".to_string());
                self.tokenize_recursive(from, tokens);
                tokens.push("^".to_string());
                self.tokenize_recursive(to, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(body, tokens);
                tokens.push(")".to_string());
            }
            Expr::BigProduct {
                var,
                from,
                to,
                body,
            } => {
                tokens.push("prod".to_string());
                tokens.push("(".to_string());
                let var_tokens = ["i", "j", "k", "m", "n"];
                let idx = var.to_usize() % var_tokens.len();
                tokens.push(format!("_{}", var_tokens[idx]));
                tokens.push("=".to_string());
                self.tokenize_recursive(from, tokens);
                tokens.push("^".to_string());
                self.tokenize_recursive(to, tokens);
                tokens.push(",".to_string());
                self.tokenize_recursive(body, tokens);
                tokens.push(")".to_string());
            }
        }
    }

    /// Convert tokens to IDs with padding.
    pub fn encode_tokens(&self, tokens: &[String]) -> Vec<u32> {
        let mut ids = vec![START_TOKEN];

        for token in tokens.iter().take(self.max_length - 2) {
            ids.push(self.vocab.get_id(token));
        }

        ids.push(END_TOKEN);

        // Pad to max length
        while ids.len() < self.max_length {
            ids.push(PAD_TOKEN);
        }

        ids
    }

    /// Encode an expression to a tensor.
    pub fn encode(&self, expr: &Expr) -> Result<Tensor> {
        let tokens = self.tokenize(expr);
        let ids = self.encode_tokens(&tokens);

        Tensor::new(ids.as_slice(), &self.device)
    }

    /// Encode a batch of expressions.
    pub fn encode_batch(&self, exprs: &[Expr]) -> Result<Tensor> {
        let batch: Vec<Vec<u32>> = exprs
            .iter()
            .map(|e| {
                let tokens = self.tokenize(e);
                self.encode_tokens(&tokens)
            })
            .collect();

        let flat: Vec<u32> = batch.into_iter().flatten().collect();
        let batch_size = exprs.len();

        Tensor::new(flat.as_slice(), &self.device)?.reshape((batch_size, self.max_length))
    }

    /// Get vocabulary size.
    pub fn vocab_size(&self) -> usize {
        self.vocab.size()
    }

    /// Get max sequence length.
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    /// Get device.
    pub fn device(&self) -> &Device {
        &self.device
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let encoder = ExpressionEncoder::new(Device::Cpu);

        // x + 1
        let mut symbols = mm_core::SymbolTable::new();
        let x = symbols.intern("x");
        let expr = Expr::Add(Box::new(Expr::Var(x)), Box::new(Expr::int(1)));

        let tokens = encoder.tokenize(&expr);
        assert!(tokens.contains(&"(".to_string()));
        assert!(tokens.contains(&"+".to_string()));
        assert!(tokens.contains(&"1".to_string()));
    }

    #[test]
    fn test_encode_to_tensor() {
        let encoder = ExpressionEncoder::new(Device::Cpu);
        let expr = Expr::int(42);

        let tensor = encoder.encode(&expr).unwrap();
        assert_eq!(tensor.dims(), &[encoder.max_length()]);
    }
}
