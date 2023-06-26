#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Token { kind }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Ident(String),
    String(String),
    /// fn
    Fn,
    /// "("
    Lparen,
    /// ")"
    Rparen,
    /// "["
    Lbracket,
    /// "]"
    Rbracket,
    /// "{"
    Lbrace,
    /// "}"
    Rbrace,
    /// ";"
    Semicolon,
}
