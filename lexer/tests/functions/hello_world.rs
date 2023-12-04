use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_hello_world() {
    let input = r#"
        // This is a comment.
        fn greet(name: str) -> str {
            return "Hello, " + name + "!";
        }

        print(greet("World"));
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        lexer::token::TokenKind::Function,
        lexer::token::TokenKind::Identifier("greet".into()),
        lexer::token::TokenKind::LeftParenthesis,
        lexer::token::TokenKind::Identifier("name".into()),
        lexer::token::TokenKind::Colon,
        lexer::token::TokenKind::Identifier("str".into()),
        lexer::token::TokenKind::RightParenthesis,
        lexer::token::TokenKind::Arrow,
        lexer::token::TokenKind::Identifier("str".into()),
        lexer::token::TokenKind::LeftCurlyBrace,
        lexer::token::TokenKind::Return,
        lexer::token::TokenKind::String("Hello, ".into()),
        lexer::token::TokenKind::Plus,
        lexer::token::TokenKind::Identifier("name".into()),
        lexer::token::TokenKind::Plus,
        lexer::token::TokenKind::String("!".into()),
        lexer::token::TokenKind::Semicolon,
        lexer::token::TokenKind::RightCurlyBrace,
        lexer::token::TokenKind::Identifier("print".into()),
        lexer::token::TokenKind::LeftParenthesis,
        lexer::token::TokenKind::Identifier("greet".into()),
        lexer::token::TokenKind::LeftParenthesis,
        lexer::token::TokenKind::String("World".into()),
        lexer::token::TokenKind::RightParenthesis,
        lexer::token::TokenKind::RightParenthesis,
        lexer::token::TokenKind::Semicolon,
        lexer::token::TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
