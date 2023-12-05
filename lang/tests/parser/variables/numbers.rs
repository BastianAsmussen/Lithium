use lang::lexer::{token::Kind, token::Token, Lexer};
use lang::parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_integer() {
    let input = r"
        let x = 1;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Variable {
        name: Token::new(Kind::Identifier("x".into()), 1, 5),
        initializer: Some(Expression::Literal(Literal::Number(1.0))),
    }];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_float() {
    let input = r"
        let x = 1.0;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Variable {
        name: Token::new(Kind::Identifier("x".into()), 1, 5),
        initializer: Some(Expression::Literal(Literal::Number(1.0))),
    }];

    assert_eq!(actual_ast, expected_ast);
}
