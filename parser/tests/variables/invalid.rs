use lexer::Lexer;
use parser::Parser;

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_semicolon() {
    let input = r"
        let x = 1
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_expression() {
    let input = r"
        let x = ;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_name() {
    let input = r"
        let = 1;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}
