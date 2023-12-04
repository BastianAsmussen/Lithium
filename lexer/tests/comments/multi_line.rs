use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_multi_line() {
    let input = r"
        /*
         This is a multi-line comment,
         it can span multiple lines!
        */

        let a = 10; /* This is another multi-line comment, it doesn't span multiple lines. */
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::Integer(10),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
