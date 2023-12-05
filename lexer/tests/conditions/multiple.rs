use lexer::token::Kind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_multiple() {
    let input = r"
        let a = 10;
        if a == 10 {
            // ...
        } else if a == 20 {
            // ...
        } else {
            // ...
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("a".into()),
        Kind::Assign,
        Kind::Integer(10),
        Kind::Semicolon,
        Kind::If,
        Kind::Identifier("a".into()),
        Kind::Equality,
        Kind::Integer(10),
        Kind::LeftCurlyBrace,
        Kind::RightCurlyBrace,
        Kind::Else,
        Kind::If,
        Kind::Identifier("a".into()),
        Kind::Equality,
        Kind::Integer(20),
        Kind::LeftCurlyBrace,
        Kind::RightCurlyBrace,
        Kind::Else,
        Kind::LeftCurlyBrace,
        Kind::RightCurlyBrace,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
