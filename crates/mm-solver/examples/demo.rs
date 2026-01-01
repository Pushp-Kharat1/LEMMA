// LEMMA Interactive Demo
// A simple REPL for mathematical simplification

use mm_core::{parse::Parser, Expr, SymbolTable};
use mm_rules::rule::standard_rules;
use mm_search::{MCTSConfig, NeuralMCTS};
use mm_verifier::Verifier;
use std::io::{self, Write};

fn main() {
    println!("============================================================");
    println!("                    LEMMA Interactive Demo                   ");
    println!("       Neural-Guided Symbolic Mathematics in Rust            ");
    println!("============================================================");
    println!();
    println!("Commands:");
    println!("  simplify <expr>     - Simplify an expression");
    println!("  deriv <expr>        - Compute derivative with respect to x");
    println!("  solve <equation>    - Solve an equation for x");
    println!("  help                - Show this help");
    println!("  quit                - Exit");
    println!();
    println!("Examples:");
    println!("  simplify 2 + 3 * 4");
    println!("  simplify x^2 * x^3");
    println!("  deriv sin(x^2)");
    println!("  deriv x^3 + 2*x");
    println!("  solve 2*x + 3 = 7");
    println!();

    let mut symbols = SymbolTable::new();
    let rules = standard_rules();
    let verifier = Verifier::new();
    let config = MCTSConfig {
        simulations: 200,
        exploration_weight: 1.41,
        max_depth: 30,
        temperature: 1.0,
    };
    let mcts = NeuralMCTS::with_config(rules, verifier, config);

    loop {
        print!("lemma> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "help" | "h" | "?" => {
                println!("Commands: simplify, deriv, solve, help, quit");
            }
            "simplify" | "s" => {
                if parts.len() < 2 {
                    println!("Usage: simplify <expression>");
                    continue;
                }
                let mut parser = Parser::new(&mut symbols);
                match parser.parse(parts[1]) {
                    Ok(expr) => {
                        let result = mcts.simplify(expr);
                        println!("Result: {}", format_expr(&result.result, &symbols));
                        if !result.steps.is_empty() {
                            println!("Steps ({}):", result.steps.len());
                            for step in &result.steps {
                                println!(
                                    "  {} -> {}",
                                    step.rule_name,
                                    format_expr(&step.after, &symbols)
                                );
                            }
                        }
                    }
                    Err(e) => println!("Parse error: {}", e),
                }
            }
            "deriv" | "d" | "derivative" => {
                if parts.len() < 2 {
                    println!("Usage: deriv <expression>");
                    continue;
                }
                let x = symbols.intern("x");
                let mut parser = Parser::new(&mut symbols);
                match parser.parse(parts[1]) {
                    Ok(expr) => {
                        let deriv = Expr::Derivative {
                            expr: Box::new(expr),
                            var: x,
                        };
                        let result = mcts.simplify(deriv);
                        println!(
                            "d/dx({}) = {}",
                            parts[1],
                            format_expr(&result.result, &symbols)
                        );
                        if !result.steps.is_empty() {
                            println!("Steps: {}", result.steps.len());
                        }
                    }
                    Err(e) => println!("Parse error: {}", e),
                }
            }
            "solve" => {
                if parts.len() < 2 {
                    println!("Usage: solve <equation>");
                    continue;
                }
                let eq_parts: Vec<&str> = parts[1].split('=').collect();
                if eq_parts.len() != 2 {
                    println!("Equation must contain exactly one '='");
                    continue;
                }
                let mut parser = Parser::new(&mut symbols);
                let lhs_result = parser.parse(eq_parts[0].trim());
                let mut parser2 = Parser::new(&mut symbols);
                let rhs_result = parser2.parse(eq_parts[1].trim());
                match (lhs_result, rhs_result) {
                    (Ok(lhs), Ok(rhs)) => {
                        let equation = Expr::Equation {
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        };
                        let result = mcts.simplify(equation);
                        println!("Solution: {}", format_expr(&result.result, &symbols));
                        if !result.steps.is_empty() {
                            println!("Steps: {}", result.steps.len());
                        }
                    }
                    (Err(e), _) | (_, Err(e)) => println!("Parse error: {}", e),
                }
            }
            _ => {
                println!(
                    "Unknown command: {}. Type 'help' for available commands.",
                    command
                );
            }
        }
        println!();
    }
}

fn format_expr(expr: &Expr, symbols: &SymbolTable) -> String {
    match expr {
        Expr::Const(r) => {
            if r.is_integer() {
                format!("{}", r.numer())
            } else {
                format!("{}/{}", r.numer(), r.denom())
            }
        }
        Expr::Var(s) => symbols.resolve(*s).unwrap_or("?").to_string(),
        Expr::Add(a, b) => format!(
            "({} + {})",
            format_expr(a, symbols),
            format_expr(b, symbols)
        ),
        Expr::Sub(a, b) => format!(
            "({} - {})",
            format_expr(a, symbols),
            format_expr(b, symbols)
        ),
        Expr::Mul(a, b) => format!("{} * {}", format_expr(a, symbols), format_expr(b, symbols)),
        Expr::Div(a, b) => format!("{} / {}", format_expr(a, symbols), format_expr(b, symbols)),
        Expr::Pow(a, b) => format!("{}^{}", format_expr(a, symbols), format_expr(b, symbols)),
        Expr::Neg(a) => format!("-{}", format_expr(a, symbols)),
        Expr::Sin(a) => format!("sin({})", format_expr(a, symbols)),
        Expr::Cos(a) => format!("cos({})", format_expr(a, symbols)),
        Expr::Tan(a) => format!("tan({})", format_expr(a, symbols)),
        Expr::Exp(a) => format!("e^{}", format_expr(a, symbols)),
        Expr::Ln(a) => format!("ln({})", format_expr(a, symbols)),
        Expr::Sqrt(a) => format!("sqrt({})", format_expr(a, symbols)),
        Expr::Equation { lhs, rhs } => format!(
            "{} = {}",
            format_expr(lhs, symbols),
            format_expr(rhs, symbols)
        ),
        Expr::Derivative { expr, var } => format!(
            "d/d{}({})",
            symbols.resolve(*var).unwrap_or("?"),
            format_expr(expr, symbols)
        ),
        _ => format!("{:?}", expr),
    }
}
