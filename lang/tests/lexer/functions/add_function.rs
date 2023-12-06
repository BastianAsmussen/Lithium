use lang::lexer::{tokens::TokenKind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_function() {
    let input = r"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Function,
        TokenKind::Identifier("add".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("a".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::Comma,
        TokenKind::Identifier("b".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::RightParenthesis,
        TokenKind::Arrow,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("a".into()),
        TokenKind::Plus,
        TokenKind::Identifier("b".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
