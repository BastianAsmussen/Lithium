use lang::lexer::tokens::TokenKind;
use lang::lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_for() {
    let input = r"
        for i in 0 to 10 {
            /* ... */
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::For,
        TokenKind::Identifier("i".into()),
        TokenKind::Range,
        TokenKind::Integer(0),
        TokenKind::To,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
