use lexer::token::Kind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_ints() {
    let input = r"
        let positive_whole = 10;
        let negative_whole = -10;

        let positive_hexadecimal = 0x10;
        let negative_hexadecimal = -0x10;

        let positive_octal = 0o10;
        let negative_octal = -0o10;

        let positive_binary = 0b10;
        let negative_binary = -0b10;
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("positive_whole".into()),
        Kind::Assign,
        Kind::Integer(10),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("negative_whole".into()),
        Kind::Assign,
        Kind::Integer(-10),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("positive_hexadecimal".into()),
        Kind::Assign,
        Kind::Integer(16),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("negative_hexadecimal".into()),
        Kind::Assign,
        Kind::Integer(-16),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("positive_octal".into()),
        Kind::Assign,
        Kind::Integer(8),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("negative_octal".into()),
        Kind::Assign,
        Kind::Integer(-8),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("positive_binary".into()),
        Kind::Assign,
        Kind::Integer(2),
        Kind::Semicolon,
        Kind::Variable,
        Kind::Identifier("negative_binary".into()),
        Kind::Assign,
        Kind::Integer(-2),
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
