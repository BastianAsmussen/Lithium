use lang::lexer::{tokens::Token, tokens::TokenKind, Lexer};
use lang::parser::ast::AST;
use lang::parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_integer() {
    let input = r"
        let x = 1;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = AST::new(vec![Statement::Variable {
        name: Token::new(TokenKind::Identifier("x".into()), 1, 5),
        initializer: Some(Expression::Literal(Literal::Number(1.0))),
    }]);

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

    let mut parser = Parser::new(&tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = AST::new(vec![Statement::Variable {
        name: Token::new(TokenKind::Identifier("x".into()), 1, 5),
        initializer: Some(Expression::Literal(Literal::Number(1.0))),
    }]);

    assert_eq!(actual_ast, expected_ast);
}
