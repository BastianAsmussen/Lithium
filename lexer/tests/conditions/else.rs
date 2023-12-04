use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_else() {
    let input = r"
        else { /* ... */ }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Else,
        TokenKind::LeftCurlyBrace,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
