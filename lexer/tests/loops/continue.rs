use lexer::token::Kind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_continue() {
    let input = r"
        continue;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [Kind::Continue, Kind::Semicolon, Kind::EndOfFile];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
