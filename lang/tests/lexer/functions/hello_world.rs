use lang::lexer::{tokens::TokenKind, Lexer};

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

    let actual_token_kinds: Vec<_> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Function,
        TokenKind::Identifier("greet".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("name".into()),
        TokenKind::Colon,
        TokenKind::Identifier("str".into()),
        TokenKind::RightParenthesis,
        TokenKind::Arrow,
        TokenKind::Identifier("str".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::String("Hello, ".into()),
        TokenKind::Plus,
        TokenKind::Identifier("name".into()),
        TokenKind::Plus,
        TokenKind::String("!".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Identifier("print".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("greet".into()),
        TokenKind::LeftParenthesis,
        TokenKind::String("World".into()),
        TokenKind::RightParenthesis,
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
