use lang::lexer::{token::Kind, Lexer};

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

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("a".into()),
        Kind::Assign,
        Kind::Integer(10),
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
