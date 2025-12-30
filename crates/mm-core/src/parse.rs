// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Author: Pushp Kharat

//! Expression parsing from strings.
//!
//! This module provides functionality to parse mathematical expressions
//! from string representations.
//!
//! # Supported Syntax
//!
//! - Numbers: `42`, `3.14`, `1/2`
//! - Variables: `x`, `y`, `theta`
//! - Operators: `+`, `-`, `*`, `/`, `^`
//! - Parentheses: `(`, `)`
//! - Functions: `sin`, `cos`, `tan`, `ln`, `exp`, `sqrt`, `abs`
//! - Derivative: `d/dx(expr)` or `diff(expr, x)`
//!
//! # Example
//!
//! ```rust
//! use mm_core::{Expr, SymbolTable, parse::Parser};
//!
//! let mut symbols = SymbolTable::new();
//! let mut parser = Parser::new(&mut symbols);
//!
//! let expr = parser.parse("x^2 + 2*x + 1").unwrap();
//! ```

use crate::{Expr, MathError, Rational, Symbol, SymbolTable};

/// A simple recursive descent parser for mathematical expressions.
pub struct Parser<'a> {
    symbols: &'a mut SymbolTable,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given symbol table.
    pub fn new(symbols: &'a mut SymbolTable) -> Self {
        Self { symbols }
    }

    /// Parse an expression from a string.
    pub fn parse(&mut self, input: &str) -> Result<Expr, MathError> {
        let tokens = tokenize(input)?;
        let mut pos = 0;
        let expr = self.parse_expr(&tokens, &mut pos)?;

        if pos < tokens.len() {
            return Err(MathError::ParseError(format!(
                "Unexpected token: {:?}",
                tokens[pos]
            )));
        }

        Ok(expr)
    }

    fn parse_expr(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        self.parse_additive(tokens, pos)
    }

    fn parse_additive(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        let mut left = self.parse_multiplicative(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Plus => {
                    *pos += 1;
                    let right = self.parse_multiplicative(tokens, pos)?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    *pos += 1;
                    let right = self.parse_multiplicative(tokens, pos)?;
                    left = Expr::Sub(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(
        &mut self,
        tokens: &[Token],
        pos: &mut usize,
    ) -> Result<Expr, MathError> {
        let mut left = self.parse_power(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Star => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                Token::Slash => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    left = Expr::Div(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_power(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        let base = self.parse_unary(tokens, pos)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Caret) {
            *pos += 1;
            let exp = self.parse_power(tokens, pos)?; // Right associative
            return Ok(Expr::Pow(Box::new(base), Box::new(exp)));
        }

        Ok(base)
    }

    fn parse_unary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        if *pos < tokens.len() && matches!(tokens[*pos], Token::Minus) {
            *pos += 1;
            let expr = self.parse_unary(tokens, pos)?;
            return Ok(Expr::Neg(Box::new(expr)));
        }

        self.parse_primary(tokens, pos)
    }

    fn parse_primary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        if *pos >= tokens.len() {
            return Err(MathError::ParseError("Unexpected end of input".to_string()));
        }

        match &tokens[*pos] {
            Token::Number(n) => {
                *pos += 1;
                Ok(Expr::Const(*n))
            }
            Token::Ident(name) => {
                *pos += 1;

                // Check if it's a function call
                if *pos < tokens.len() && matches!(tokens[*pos], Token::LParen) {
                    *pos += 1; // consume '('
                    let arg = self.parse_expr(tokens, pos)?;

                    if *pos >= tokens.len() || !matches!(tokens[*pos], Token::RParen) {
                        return Err(MathError::ParseError("Expected ')'".to_string()));
                    }
                    *pos += 1; // consume ')'

                    return match name.as_str() {
                        "sin" => Ok(Expr::Sin(Box::new(arg))),
                        "cos" => Ok(Expr::Cos(Box::new(arg))),
                        "tan" => Ok(Expr::Tan(Box::new(arg))),
                        "ln" => Ok(Expr::Ln(Box::new(arg))),
                        "exp" => Ok(Expr::Exp(Box::new(arg))),
                        "sqrt" => Ok(Expr::Sqrt(Box::new(arg))),
                        "abs" => Ok(Expr::Abs(Box::new(arg))),
                        _ => Err(MathError::ParseError(format!("Unknown function: {}", name))),
                    };
                }

                // It's a variable
                let symbol = self.symbols.intern(name);
                Ok(Expr::Var(symbol))
            }
            Token::LParen => {
                *pos += 1;
                let expr = self.parse_expr(tokens, pos)?;

                if *pos >= tokens.len() || !matches!(tokens[*pos], Token::RParen) {
                    return Err(MathError::ParseError("Expected ')'".to_string()));
                }
                *pos += 1;

                Ok(expr)
            }
            _ => Err(MathError::ParseError(format!(
                "Unexpected token: {:?}",
                tokens[*pos]
            ))),
        }
    }
}

// ============================================================================
// Tokenizer
// ============================================================================

#[derive(Debug, Clone)]
enum Token {
    Number(Rational),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, MathError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Skip whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Single character tokens
        match c {
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
                continue;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
                continue;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
                continue;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
                continue;
            }
            '^' => {
                tokens.push(Token::Caret);
                i += 1;
                continue;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
                continue;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
                continue;
            }
            _ => {}
        }

        // Numbers
        if c.is_ascii_digit() || c == '.' {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }

            let num_str: String = chars[start..i].iter().collect();

            // Parse as integer or decimal
            if num_str.contains('.') {
                // Parse as decimal, convert to rational
                let val: f64 = num_str
                    .parse()
                    .map_err(|_| MathError::ParseError(format!("Invalid number: {}", num_str)))?;

                // Approximate as rational (simple approach)
                let scale = 1_000_000i64;
                let numer = (val * scale as f64).round() as i64;
                tokens.push(Token::Number(Rational::new(numer, scale)));
            } else {
                let val: i64 = num_str
                    .parse()
                    .map_err(|_| MathError::ParseError(format!("Invalid integer: {}", num_str)))?;
                tokens.push(Token::Number(Rational::from_integer(val)));
            }
            continue;
        }

        // Identifiers
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }

            let ident: String = chars[start..i].iter().collect();
            tokens.push(Token::Ident(ident));
            continue;
        }

        return Err(MathError::ParseError(format!("Unknown character: {}", c)));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("42").unwrap();
        assert_eq!(expr, Expr::int(42));
    }

    #[test]
    fn test_parse_variable() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("x").unwrap();
        let x = symbols.get("x").unwrap();
        assert_eq!(expr, Expr::Var(x));
    }

    #[test]
    fn test_parse_addition() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("1 + 2").unwrap();
        assert_eq!(
            expr,
            Expr::Add(Box::new(Expr::int(1)), Box::new(Expr::int(2)))
        );
    }

    #[test]
    fn test_parse_complex() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("x^2 + 2*x + 1").unwrap();
        let x = symbols.get("x").unwrap();

        // Just check it parsed without error
        assert!(matches!(expr, Expr::Add(_, _)));
    }

    #[test]
    fn test_parse_function() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("sin(x)").unwrap();
        let x = symbols.get("x").unwrap();

        assert!(matches!(expr, Expr::Sin(_)));
    }
}
