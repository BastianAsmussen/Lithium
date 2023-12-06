use lang::lexer::tokens::TokenKind;
use lang::lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_continue() {
    let input = r"
        continue;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Continue,
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
