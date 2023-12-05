use std::fmt::{Display, Formatter};

/// A token kind, which is a type of token.
///
/// # Variants
///
/// * `Identifier(String)` - An identifier, such as `foo` or `bar`.
/// * `Integer(i64)` - An integer, such as `123` or `456`.
/// * `Float(f64)` - A float, such as `1.23` or `4.56`.
/// * `String(String)` - A string, such as `"foo"` or `"bar"`.
/// * `Plus` - A plus sign, `+`.
/// * `Increment` - An increment operator, `++`.
/// * `AddAssign` - An add-assign operator, `+=`.
/// * `Minus` - A minus sign, `-`.
/// * `Decrement` - A decrement operator, `--`.
/// * `SubtractAssign` - A subtract-assign operator, `-=`.
/// * `Star` - An asterisk, `*`.
/// * `Power` - A power operator, `**`.
/// * `MultiplyAssign` - A multiply-assign operator, `*=`.
/// * `Slash` - A slash, `/`.
/// * `DivisionAssign` - A division-assign operator, `/=`.
/// * `Percent` - A percent sign, `%`.
/// * `ModuloAssign` - A modulo-assign operator, `%=`.
/// * `Caret` - A caret, `^`.
/// * `BitwiseXorAssign` - A bitwise-xor-assign operator, `^=`.
/// * `LogicalNot` - A logical-not operator, `!`.
/// * `NotEqual` - A not-equal operator, `!=`.
/// * `Assign` - An equal sign, `=`.
/// * `Equality` - An equality operator, `==`.
/// * `LessThan` - A less-than operator, `<`.
/// * `BitwiseLeftShift` - A bitwise-left-shift operator, `<<`.
/// * `BitwiseLeftShiftAssign` - A bitwise-left-shift-assign operator, `<<=`.
/// * `LessThanOrEqual` - A less-than-or-equal operator, `<=`.
/// * `GreaterThan` - A greater-than operator, `>`.
/// * `BitwiseRightShift` - A bitwise-right-shift operator, `>>`.
/// * `BitwiseRightShiftAssign` - A bitwise-right-shift-assign operator, `>>=`.
/// * `GreaterThanOrEqual` - A greater-than-or-equal operator, `>=`.
/// * `BitwiseAnd` - A bitwise-and operator, `&`.
/// * `LogicalAnd` - A logical-and operator, `&&`.
/// * `BitwiseAndAssign` - A bitwise-and-assign operator, `&=`.
/// * `BitwiseOr` - A bitwise-or operator, `|`.
/// * `BitwiseOrAssign` - A bitwise-or-assign operator, `|=`.
/// * `LogicalOr` - A logical-or operator, `||`.
/// * `LeftParenthesis` - A left parenthesis, `(`.
/// * `RightParenthesis` - A right parenthesis, `)`.
/// * `LeftCurlyBrace` - A left curly brace, `{`.
/// * `RightCurlyBrace` - A right curly brace, `}`.
/// * `LeftBracket` - A left bracket, `[`.
/// * `RightBracket` - A right bracket, `]`.
/// * `Comma` - A comma, `,`.
/// * `Dot` - A dot, `.`.
/// * `Colon` - A colon, `:`.
/// * `Semicolon` - A semicolon, `;`.
/// * `Arrow` - An arrow, `->`.
/// * `True` - A true keyword, `true`.
/// * `False` - A false keyword, `false`.
/// * `If` - An if keyword, `if`.
/// * `Else` - An else keyword, `else`.
/// * `While` - A while keyword, `while`.
/// * `For` - A for keyword, `for`.
/// * `Range` - A range keyword, `in`.
/// * `To` - A to keyword, `to`.
/// * `Break` - A break keyword, `break`.
/// * `Continue` - A continue keyword, `continue`.
/// * `Return` - A return keyword, `return`.
/// * `Function` - A function keyword, `function`.
/// * `Variable` - A variable keyword, `variable`.
/// * `Comment` - A comment, such as `// ...` or `/* ... */`.
/// * `EndOfFile` - An end-of-file token.
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Plus,
    Increment,
    AddAssign,
    Minus,
    Decrement,
    SubtractAssign,
    Star,
    Power,
    MultiplyAssign,
    Slash,
    DivisionAssign,
    Percent,
    ModuloAssign,
    BitwiseXor,
    BitwiseXorAssign,
    LogicalNot,
    NotEqual,
    Assign,
    Equality,
    LessThan,
    BitwiseLeftShift,
    BitwiseLeftShiftAssign,
    LessThanOrEqual,
    GreaterThan,
    BitwiseRightShift,
    BitwiseRightShiftAssign,
    GreaterThanOrEqual,
    BitwiseAnd,
    LogicalAnd,
    BitwiseAndAssign,
    BitwiseOr,
    BitwiseOrAssign,
    LogicalOr,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    True,
    False,
    If,
    Else,
    While,
    For,
    Range,
    To,
    Break,
    Continue,
    Return,
    Function,
    Variable,
    Comment,
    EndOfFile,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Integer(integer) => write!(f, "{integer}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::String(string) => write!(f, "{string}"),
            Self::Plus => write!(f, "+"),
            Self::Increment => write!(f, "++"),
            Self::AddAssign => write!(f, "+="),
            Self::Minus => write!(f, "-"),
            Self::Decrement => write!(f, "--"),
            Self::SubtractAssign => write!(f, "-="),
            Self::Star => write!(f, "*"),
            Self::Power => write!(f, "**"),
            Self::MultiplyAssign => write!(f, "*="),
            Self::Slash => write!(f, "/"),
            Self::DivisionAssign => write!(f, "/="),
            Self::Percent => write!(f, "%"),
            Self::ModuloAssign => write!(f, "%="),
            Self::BitwiseXor => write!(f, "^"),
            Self::BitwiseXorAssign => write!(f, "^="),
            Self::LogicalNot => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Assign => write!(f, "="),
            Self::Equality => write!(f, "=="),
            Self::LessThan => write!(f, "<"),
            Self::BitwiseLeftShift => write!(f, "<<"),
            Self::BitwiseLeftShiftAssign => write!(f, "<<="),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::GreaterThan => write!(f, ">"),
            Self::BitwiseRightShift => write!(f, ">>"),
            Self::BitwiseRightShiftAssign => write!(f, ">>="),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::BitwiseAnd => write!(f, "&"),
            Self::LogicalAnd => write!(f, "&&"),
            Self::BitwiseAndAssign => write!(f, "&="),
            Self::BitwiseOr => write!(f, "|"),
            Self::BitwiseOrAssign => write!(f, "|="),
            Self::LogicalOr => write!(f, "||"),
            Self::LeftParenthesis => write!(f, "("),
            Self::RightParenthesis => write!(f, ")"),
            Self::LeftCurlyBrace => write!(f, "{{"),
            Self::RightCurlyBrace => write!(f, "}}"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Arrow => write!(f, "->"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::While => write!(f, "while"),
            Self::For => write!(f, "for"),
            Self::Range => write!(f, "in"),
            Self::To => write!(f, "to"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::Return => write!(f, "return"),
            Self::Function => write!(f, "function"),
            Self::Variable => write!(f, "variable"),
            Self::Comment | Self::EndOfFile => Ok(()),
        }
    }
}

/// A token, which is a single unit of a program.
///
/// # Fields
///
/// * `kind` - The kind of token.
/// * `line` - The line number of the token.
/// * `column` - The column number of the token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of token.
    /// * `line` - The line number of the token.
    /// * `column` - The column number of the token.
    ///
    /// # Returns
    ///
    /// * `Token` - The new token.
    #[must_use]
    pub const fn new(kind: Kind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}
