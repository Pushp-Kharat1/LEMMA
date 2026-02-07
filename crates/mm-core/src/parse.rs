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
//! - Operators: `+`, `-`, `*`, `/`, `^`, `%` (mod), `!` (factorial), `=` (equation)
//! - Parentheses: `(`, `)`
//! - Functions:
//!   - Trig: `sin`, `cos`, `tan`
//!   - Exp/Log: `ln`, `exp`
//!   - Misc: `sqrt`, `abs`, `floor`, `ceil`
//!   - Number Theory: `gcd(a,b)`, `lcm(a,b)`, `binomial(n,k)`
//!   - Calculus: `diff(expr, var)`, `int(expr, var)`
//!   - Big Ops: `sum(var, from, to, body)`, `prod(var, from, to, body)`
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

use crate::{Expr, MathError, Rational, SymbolTable};

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
        let expr = self.parse_equation(&tokens, &mut pos)?;

        if pos < tokens.len() {
            return Err(MathError::ParseError(format!(
                "Unexpected token at end of input: {:?}",
                tokens[pos]
            )));
        }

        Ok(expr)
    }

    // Level 1: Equations (=)
    fn parse_equation(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        let lhs = self.parse_additive(tokens, pos)?;

        if *pos < tokens.len() {
            if let Token::Eq = tokens[*pos] {
                *pos += 1;
                let rhs = self.parse_additive(tokens, pos)?;
                return Ok(Expr::Equation {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                });
            }
        }

        Ok(lhs)
    }

    // Level 2: Additive (+, -)
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

    // Level 3: Multiplicative (*, /, %)
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
                Token::Percent => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    left = Expr::Mod(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    // Level 4: Power (^) - Right associative
    fn parse_power(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        let base = self.parse_unary(tokens, pos)?;

        if *pos < tokens.len() && matches!(tokens[*pos], Token::Caret) {
            *pos += 1;
            let exp = self.parse_power(tokens, pos)?; // Recursion for right associativity
            return Ok(Expr::Pow(Box::new(base), Box::new(exp)));
        }

        Ok(base)
    }

    // Level 5: Unary (-)
    fn parse_unary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        if *pos < tokens.len() && matches!(tokens[*pos], Token::Minus) {
            *pos += 1;
            let expr = self.parse_unary(tokens, pos)?;
            return Ok(Expr::Neg(Box::new(expr)));
        }

        self.parse_postfix(tokens, pos)
    }

    // Level 6: Postfix (!)
    fn parse_postfix(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Expr, MathError> {
        let mut expr = self.parse_primary(tokens, pos)?;

        while *pos < tokens.len() && matches!(tokens[*pos], Token::Bang) {
            *pos += 1;
            expr = Expr::Factorial(Box::new(expr));
        }

        Ok(expr)
    }

    // Level 7: Primary (Number, Var, Paren, Function)
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
                    let args = self.parse_args(tokens, pos)?;

                    if *pos >= tokens.len() || !matches!(tokens[*pos], Token::RParen) {
                        return Err(MathError::ParseError("Expected ')'".to_string()));
                    }
                    *pos += 1; // consume ')'

                    return self.construct_function_call(name, args);
                }

                // It's a variable or constant
                match name.as_str() {
                    "pi" | "Pi" | "PI" | "π" | "Π" => Ok(Expr::Pi),
                    "e" | "E" => Ok(Expr::E),
                    _ => {
                        let symbol = self.symbols.intern(name);
                        Ok(Expr::Var(symbol))
                    }
                }
            }
            Token::LParen => {
                *pos += 1;
                let expr = self.parse_equation(tokens, pos)?; // Reset precedence for inside parens

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

    fn parse_args(&mut self, tokens: &[Token], pos: &mut usize) -> Result<Vec<Expr>, MathError> {
        let mut args = Vec::new();
        if *pos < tokens.len() && !matches!(tokens[*pos], Token::RParen) {
            args.push(self.parse_equation(tokens, pos)?);
            while *pos < tokens.len() && matches!(tokens[*pos], Token::Comma) {
                *pos += 1;
                args.push(self.parse_equation(tokens, pos)?);
            }
        }
        Ok(args)
    }

    fn construct_function_call(&mut self, name: &str, args: Vec<Expr>) -> Result<Expr, MathError> {
        match (name, args.len()) {
            // Unary functions
            ("sin", 1) => Ok(Expr::Sin(Box::new(args[0].clone()))),
            ("cos", 1) => Ok(Expr::Cos(Box::new(args[0].clone()))),
            ("tan", 1) => Ok(Expr::Tan(Box::new(args[0].clone()))),
            ("ln", 1) => Ok(Expr::Ln(Box::new(args[0].clone()))),
            ("exp", 1) => Ok(Expr::Exp(Box::new(args[0].clone()))),
            ("sqrt", 1) => Ok(Expr::Sqrt(Box::new(args[0].clone()))),
            ("abs", 1) => Ok(Expr::Abs(Box::new(args[0].clone()))),
            ("floor", 1) => Ok(Expr::Floor(Box::new(args[0].clone()))),
            ("ceil", 1) => Ok(Expr::Ceiling(Box::new(args[0].clone()))),
            ("factorial", 1) => Ok(Expr::Factorial(Box::new(args[0].clone()))),

            // Binary functions
            ("gcd", 2) => Ok(Expr::GCD(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            )),
            ("lcm", 2) => Ok(Expr::LCM(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            )),
            ("mod", 2) => Ok(Expr::Mod(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            )),
            ("binomial", 2) => Ok(Expr::Binomial(
                Box::new(args[0].clone()),
                Box::new(args[1].clone()),
            )),

            // Calculus
            ("diff", 2) | ("derivative", 2) => {
                if let Expr::Var(v) = args[1] {
                    Ok(Expr::Derivative {
                        expr: Box::new(args[0].clone()),
                        var: v,
                    })
                } else {
                    Err(MathError::ParseError(
                        "Second argument to diff must be a variable".to_string(),
                    ))
                }
            }
            ("int", 2) | ("integral", 2) => {
                if let Expr::Var(v) = args[1] {
                    Ok(Expr::Integral {
                        expr: Box::new(args[0].clone()),
                        var: v,
                    })
                } else {
                    Err(MathError::ParseError(
                        "Second argument to int must be a variable".to_string(),
                    ))
                }
            }

            // Big Operators (Sum/Prod)
            // sum(var, from, to, body)
            ("sum", 4) => {
                if let Expr::Var(v) = args[0] {
                    Ok(Expr::Summation {
                        var: v,
                        from: Box::new(args[1].clone()),
                        to: Box::new(args[2].clone()),
                        body: Box::new(args[3].clone()),
                    })
                } else {
                    Err(MathError::ParseError(
                        "First argument to sum must be a variable".to_string(),
                    ))
                }
            }
            ("prod", 4) => {
                if let Expr::Var(v) = args[0] {
                    Ok(Expr::BigProduct {
                        var: v,
                        from: Box::new(args[1].clone()),
                        to: Box::new(args[2].clone()),
                        body: Box::new(args[3].clone()),
                    })
                } else {
                    Err(MathError::ParseError(
                        "First argument to prod must be a variable".to_string(),
                    ))
                }
            }

            _ => Err(MathError::ParseError(format!(
                "Unknown function or wrong number of arguments: {}({} args)",
                name,
                args.len()
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
    Percent,
    Bang,
    Eq,
    LParen,
    RParen,
    Comma,
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
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
                continue;
            }
            '!' => {
                tokens.push(Token::Bang);
                i += 1;
                continue;
            }
            '=' => {
                tokens.push(Token::Eq);
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
            ',' => {
                tokens.push(Token::Comma);
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
        // Just check it parsed without error
        assert!(matches!(expr, Expr::Add(_, _)));
    }

    #[test]
    fn test_parse_function() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);

        let expr = parser.parse("sin(x)").unwrap();
        assert!(matches!(expr, Expr::Sin(_)));
    }

    #[test]
    fn test_parse_equation() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);
        let expr = parser.parse("x + 1 = 2").unwrap();
        assert!(matches!(expr, Expr::Equation { .. }));
    }

    #[test]
    fn test_parse_factorial() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);
        let expr = parser.parse("x!").unwrap();
        assert!(matches!(expr, Expr::Factorial(_)));
    }

    #[test]
    fn test_parse_calculus() {
        let mut symbols = SymbolTable::new();
        let mut parser = Parser::new(&mut symbols);
        let expr = parser.parse("diff(x^2, x)").unwrap();
        assert!(matches!(expr, Expr::Derivative { .. }));
    }
}
