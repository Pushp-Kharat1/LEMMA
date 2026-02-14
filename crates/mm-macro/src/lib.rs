use mm_core::{parse::Parser, Expr, SymbolTable};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    let expr_code = input.to_token_stream();
    expr_code.into()
}

struct MacroInput {
    expr: Expr,
    symbol_table_path: syn::Path,
    temp_symbol_table: SymbolTable,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut expr_tokens = proc_macro2::TokenStream::new();
        let mut symbol_table_path: Option<syn::Path> = None;

        while !input.is_empty() {
            // Check for the comma separator at the top level
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                symbol_table_path = Some(input.parse()?);

                // Ensure we are at the end of the input
                if !input.is_empty() {
                    return Err(input.error("unexpected token after symbol table"));
                }
                break;
            }

            // Consume token for the expression
            expr_tokens.extend(std::iter::once(input.parse::<proc_macro2::TokenTree>()?));
        }

        let path =
            symbol_table_path.ok_or_else(|| input.error("expected comma and symbol table"))?;
        let expr_str = expr_tokens.to_string();

        let mut temp_symbol_table = SymbolTable::new();
        let mut parser = Parser::new(&mut temp_symbol_table);
        let expr = parser.parse(&expr_str).map_err(|e| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Math parse error: {}", e),
            )
        })?;

        Ok(MacroInput {
            expr,
            symbol_table_path: path,
            temp_symbol_table,
        })
    }
}

impl MacroInput {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        expr_to_token_stream(&self.expr, &self.symbol_table_path, &self.temp_symbol_table)
    }
}

/// Convert a parsed `Expr` into a `proc_macro2::TokenStream` that, when emitted into generated code,
/// constructs an equivalent `mm_core::Expr` value at runtime.
///
/// This translation recursively converts each `Expr` variant into tokens that build the corresponding
/// `mm_core::Expr`. Symbol names encountered in `expr` are resolved using `temp_symbols` (a macro-time
/// symbol table) and interned into `runtime_symbol_table` (a runtime symbol table path) in the
/// generated tokens. The function will panic if a referenced symbol is not present in `temp_symbols`,
/// or if unsupported constructs such as quantifiers or logical connectives are encountered.
///
/// `runtime_symbol_table` — path to a runtime symbol table used to intern names in the generated code.
/// `temp_symbols` — macro-time `SymbolTable` used to resolve identifier indices to string names.
///
/// # Returns
///
/// A `proc_macro2::TokenStream` which constructs the corresponding `mm_core::Expr` when compiled into
/// the generated code.
///
/// # Examples
///
/// ```
/// use syn::parse_str;
///
/// // Path to the runtime symbol table used by the generated code.
/// let path: syn::Path = parse_str("crate::SYMBOL_TABLE").unwrap();
///
/// // A temp symbol table for parsing; here we use a default/empty table for symbol-free examples.
/// let temp_symbols = mm_parse::SymbolTable::default();
///
/// let expr = mm_parse::Expr::Pi;
/// let tokens = mm_macro::expr_to_token_stream(&expr, &path, &temp_symbols);
/// let s = tokens.to_string();
/// assert!(s.contains("mm_core :: Expr :: Pi"));
/// ```
fn expr_to_token_stream(
    expr: &Expr,
    runtime_symbol_table: &syn::Path,
    temp_symbols: &SymbolTable,
) -> proc_macro2::TokenStream {
    macro_rules! unary {
        ($variant:ident, $operand:expr) => {{
            let inner = expr_to_token_stream($operand, runtime_symbol_table, temp_symbols);
            quote! { mm_core::Expr::$variant(Box::new(#inner)) }
        }};
    }

    macro_rules! binary {
        ($variant:ident, $lhs:expr, $rhs:expr) => {{
            let left = expr_to_token_stream($lhs, runtime_symbol_table, temp_symbols);
            let right = expr_to_token_stream($rhs, runtime_symbol_table, temp_symbols);
            quote! { mm_core::Expr::$variant(Box::new(#left), Box::new(#right)) }
        }};
    }

    match expr {
        Expr::Const(r) => {
            let n = r.numer();
            let d = r.denom();
            quote! { mm_core::Expr::Const(mm_core::Rational::new(#n, #d)) }
        }
        Expr::Var(s) => {
            let name = temp_symbols
                .resolve(*s)
                .expect("Symbol not found in temp table");
            quote! { mm_core::Expr::Var(#runtime_symbol_table.intern(#name)) }
        }
        Expr::Neg(e) => unary!(Neg, e),
        Expr::Add(l, r) => binary!(Add, l, r),
        Expr::Sub(l, r) => binary!(Sub, l, r),
        Expr::Mul(l, r) => binary!(Mul, l, r),
        Expr::Div(l, r) => binary!(Div, l, r),
        Expr::Pow(l, r) => binary!(Pow, l, r),
        Expr::Sin(e) => unary!(Sin, e),
        Expr::Cos(e) => unary!(Cos, e),
        Expr::Tan(e) => unary!(Tan, e),
        Expr::Arcsin(e) => unary!(Arcsin, e),
        Expr::Arccos(e) => unary!(Arccos, e),
        Expr::Arctan(e) => unary!(Arctan, e),
        Expr::Ln(e) => unary!(Ln, e),
        Expr::Exp(e) => unary!(Exp, e),
        Expr::Sqrt(e) => unary!(Sqrt, e),
        Expr::Abs(e) => unary!(Abs, e),
        Expr::Pi => quote! { mm_core::Expr::Pi },
        Expr::E => quote! { mm_core::Expr::E },
        Expr::Sum(terms) => {
            let term_tokens = terms.iter().map(|term| {
                let coeff_n = term.coeff.numer();
                let coeff_d = term.coeff.denom();
                let inner_expr =
                    expr_to_token_stream(&term.expr, runtime_symbol_table, temp_symbols);
                quote! {
                    mm_core::Term {
                        coeff: mm_core::Rational::new(#coeff_n, #coeff_d),
                        expr: #inner_expr,
                    }
                }
            });
            quote! { mm_core::Expr::Sum(vec![#(#term_tokens),*]) }
        }
        Expr::Product(factors) => {
            let factor_tokens = factors.iter().map(|factor| {
                let base = expr_to_token_stream(&factor.base, runtime_symbol_table, temp_symbols);
                let power = expr_to_token_stream(&factor.power, runtime_symbol_table, temp_symbols);
                quote! {
                    mm_core::Factor {
                        base: #base,
                        power: #power,
                    }
                }
            });
            quote! { mm_core::Expr::Product(vec![#(#factor_tokens),*]) }
        }
        Expr::Derivative { expr, var } => {
            let inner = expr_to_token_stream(expr, runtime_symbol_table, temp_symbols);
            let var_name = temp_symbols.resolve(*var).expect("Symbol not found");
            quote! {
                mm_core::Expr::Derivative {
                    expr: Box::new(#inner),
                    var: #runtime_symbol_table.intern(#var_name),
                }
            }
        }
        Expr::Integral { expr, var } => {
            let inner = expr_to_token_stream(expr, runtime_symbol_table, temp_symbols);
            let var_name = temp_symbols.resolve(*var).expect("Symbol not found");
            quote! {
                mm_core::Expr::Integral {
                    expr: Box::new(#inner),
                    var: #runtime_symbol_table.intern(#var_name),
                }
            }
        }
        Expr::Equation { lhs, rhs } => {
            let l = expr_to_token_stream(lhs, runtime_symbol_table, temp_symbols);
            let r = expr_to_token_stream(rhs, runtime_symbol_table, temp_symbols);
            quote! {
                mm_core::Expr::Equation {
                    lhs: Box::new(#l),
                    rhs: Box::new(#r),
                }
            }
        }
        Expr::GCD(l, r) => binary!(GCD, l, r),
        Expr::LCM(l, r) => binary!(LCM, l, r),
        Expr::Mod(l, r) => binary!(Mod, l, r),
        Expr::Floor(e) => unary!(Floor, e),
        Expr::Ceiling(e) => unary!(Ceiling, e),
        Expr::Factorial(e) => unary!(Factorial, e),
        Expr::Binomial(l, r) => binary!(Binomial, l, r),
        Expr::Gte(l, r) => binary!(Gte, l, r),
        Expr::Gt(l, r) => binary!(Gt, l, r),
        Expr::Lte(l, r) => binary!(Lte, l, r),
        Expr::Lt(l, r) => binary!(Lt, l, r),
        Expr::Sinh(e) => unary!(Sinh, e),
        Expr::Cosh(e) => unary!(Cosh, e),
        Expr::Tanh(e) => unary!(Tanh, e),
        Expr::Limit {
            expr,
            var,
            approaching,
        } => {
            let inner = expr_to_token_stream(expr, runtime_symbol_table, temp_symbols);
            let var_name = temp_symbols.resolve(*var).expect("Symbol not found");
            let target = expr_to_token_stream(approaching, runtime_symbol_table, temp_symbols);
            quote! {
                mm_core::Expr::Limit {
                    expr: Box::new(#inner),
                    var: #runtime_symbol_table.intern(#var_name),
                    approaching: Box::new(#target),
                }
            }
        }
        Expr::Vector(items) => {
            let elems = items
                .iter()
                .map(|e| expr_to_token_stream(e, runtime_symbol_table, temp_symbols));
            quote! { mm_core::Expr::Vector(vec![#(#elems),*]) }
        }
        Expr::Summation {
            var,
            from,
            to,
            body,
        } => {
            let var_name = temp_symbols.resolve(*var).expect("Symbol not found");
            let f = expr_to_token_stream(from, runtime_symbol_table, temp_symbols);
            let t = expr_to_token_stream(to, runtime_symbol_table, temp_symbols);
            let b = expr_to_token_stream(body, runtime_symbol_table, temp_symbols);
            quote! {
                mm_core::Expr::Summation {
                    var: #runtime_symbol_table.intern(#var_name),
                    from: Box::new(#f),
                    to: Box::new(#t),
                    body: Box::new(#b),
                }
            }
        }
        Expr::BigProduct {
            var,
            from,
            to,
            body,
        } => {
            let var_name = temp_symbols.resolve(*var).expect("Symbol not found");
            let f = expr_to_token_stream(from, runtime_symbol_table, temp_symbols);
            let t = expr_to_token_stream(to, runtime_symbol_table, temp_symbols);
            let b = expr_to_token_stream(body, runtime_symbol_table, temp_symbols);
            quote! {
                mm_core::Expr::BigProduct {
                    var: #runtime_symbol_table.intern(#var_name),
                    from: Box::new(#f),
                    to: Box::new(#t),
                    body: Box::new(#b),
                }
            }
        }
        // Quantifiers and logical connectives - fallback (not normally parsed from expr macro)
        Expr::ForAll { .. }
        | Expr::Exists { .. }
        | Expr::And(_, _)
        | Expr::Or(_, _)
        | Expr::Not(_)
        | Expr::Implies(_, _) => {
            panic!("Quantifiers and logical connectives cannot be parsed by expr! macro yet")
        }
    }
}