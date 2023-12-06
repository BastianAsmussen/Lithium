use lang::lexer::{tokens::Token, tokens::TokenKind, Lexer};
use lang::parser::ast::AST;
use lang::parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_order_of_operations() {
    let input = r"
        let x = 1 + 2 * 3 - 4 / 5;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = AST::new(vec![Statement::Variable {
        name: Token::new(TokenKind::Identifier("x".into()), 1, 5),
        initializer: Some(Expression::Binary {
            left: Box::from(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(TokenKind::Plus, 1, 10),
                right: Box::from(Expression::Binary {
                    left: Box::from(Expression::Literal(Literal::Number(2.0))),
                    operator: Token::new(TokenKind::Star, 1, 14),
                    right: Box::from(Expression::Literal(Literal::Number(3.0))),
                }),
            }),
            operator: Token::new(TokenKind::Minus, 1, 18),
            right: Box::from(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(4.0))),
                operator: Token::new(TokenKind::Slash, 1, 22),
                right: Box::from(Expression::Literal(Literal::Number(5.0))),
            }),
        }),
    }]);

    assert_eq!(actual_ast, expected_ast);
}
