use lexer::token::TokenKind;
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

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("positive_whole".into()),
        TokenKind::Assign,
        TokenKind::Integer(10),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("negative_whole".into()),
        TokenKind::Assign,
        TokenKind::Integer(-10),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("positive_hexadecimal".into()),
        TokenKind::Assign,
        TokenKind::Integer(16),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("negative_hexadecimal".into()),
        TokenKind::Assign,
        TokenKind::Integer(-16),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("positive_octal".into()),
        TokenKind::Assign,
        TokenKind::Integer(8),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("negative_octal".into()),
        TokenKind::Assign,
        TokenKind::Integer(-8),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("positive_binary".into()),
        TokenKind::Assign,
        TokenKind::Integer(2),
        TokenKind::Semicolon,

        TokenKind::Variable,
        TokenKind::Identifier("negative_binary".into()),
        TokenKind::Assign,
        TokenKind::Integer(-2),
        TokenKind::Semicolon,
        
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
