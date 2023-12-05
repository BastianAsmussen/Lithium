use lexer::token::Kind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_break() {
    let input = r"
        break;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [Kind::Break, Kind::Semicolon, Kind::EndOfFile];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
