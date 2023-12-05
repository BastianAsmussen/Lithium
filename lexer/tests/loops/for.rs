use lexer::token::Kind;
use lexer::Lexer;

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

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::For,
        Kind::Identifier("i".into()),
        Kind::Range,
        Kind::Integer(0),
        Kind::To,
        Kind::Integer(10),
        Kind::LeftCurlyBrace,
        Kind::RightCurlyBrace,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
