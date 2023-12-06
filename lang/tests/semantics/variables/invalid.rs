use lang::lexer::Lexer;
use lang::parser::Parser;
use lang::semantics::errors::Error;
use lang::semantics::Semantics;

#[test]
#[allow(clippy::unwrap_used)]
fn test_use_undeclared_variable() {
    let input = r"
        fn main() {
            let x = 0;
            let y = x + z;
        }
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse().unwrap();

    let mut semantics = Semantics::new(&ast);
    let result = semantics.analyze();

    assert_eq!(
        result,
        Err(Error::VariableNotDefined {
            name: "z".to_string()
        })
    );
}
