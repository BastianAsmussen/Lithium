use lang::lexer::{token::Kind, token::Token, Lexer};
use lang::parser::{Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_name() {
    let input = r"
        fn () {}
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_parenthesis() {
    let input = r"
        fn main {}
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_braces() {
    let input = r"
        fn main()
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_pure_main() {
    let input = r"
        fn main() {}
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Function {
        name: Token::new(Kind::Identifier("main".into()), 1, 7),
        parameters: vec![],
        return_type: None,
        body: Box::from(Statement::Block { statements: vec![] }),
    }];

    assert_eq!(actual_ast, expected_ast);
}
