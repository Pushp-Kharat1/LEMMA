use mm_macro::expr;

#[test]
fn test_basic_expr() {
    let mut my_symbol_table = mm_core::SymbolTable::new();
    let compiletime = expr!(4 * x - 370 * y ^ 2, my_symbol_table);
    let mut parser = mm_core::parse::Parser::new(&mut my_symbol_table);
    let runtime = parser.parse("4*x - 370*y^2").unwrap();
    assert_eq!(compiletime, runtime);
}
