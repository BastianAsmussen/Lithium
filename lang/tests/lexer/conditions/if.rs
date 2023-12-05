use lang::lexer::{token::Kind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_if() {
    let input = r"
        if true { /* ... */ }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::If,
        Kind::True,
        Kind::LeftCurlyBrace,
        Kind::RightCurlyBrace,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
