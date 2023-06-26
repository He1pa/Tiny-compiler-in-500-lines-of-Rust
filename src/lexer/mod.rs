use self::token::Token;

pub mod token;

pub(crate) const EOF_CHAR: char = '\0';

#[derive(Clone, Debug, Default)]
pub struct Lexer {
    /// Input char array.
    input: Vec<char>,
    /// Current position.
    pos: usize,
    /// Next position.
    next_pos: usize,
    /// Current char.
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> anyhow::Result<Self> {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            ..Default::default()
        };
        lexer.init()?;
        Ok(lexer)
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.read_char()?;
        Ok(())
    }

    /// Read a char from the input string.
    fn read_char(&mut self) -> anyhow::Result<()> {
        if self.next_pos >= self.input.len() {
            self.ch = EOF_CHAR;
        } else {
            self.ch = self.input[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos = self.pos + 1;
        Ok(())
    }

    pub fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_whitespace()?;
        let token = match self.ch {
            EOF_CHAR => Token::new(token::TokenKind::Eof),
            ';' => Token::new(token::TokenKind::Semicolon),
            '{' => Token::new(token::TokenKind::Lbrace),
            '[' => Token::new(token::TokenKind::Lbracket),
            '(' => Token::new(token::TokenKind::Lparen),
            '}' => Token::new(token::TokenKind::Rbrace),
            ']' => Token::new(token::TokenKind::Rbracket),
            ')' => Token::new(token::TokenKind::Rparen),
            '"' => {
                let s = self.read_string()?;
                Token::new(token::TokenKind::String(s))
            }
            _ => {
                if Self::is_letter(self.ch) {
                    let ident = self.read_identifier()?;
                    match ident.as_str() {
                        "fn" => return Ok(Token::new(token::TokenKind::Fn)),
                        _ => return Ok(Token::new(token::TokenKind::Ident(ident))),
                    }
                } else {
                    todo!()
                }
            }
        };
        self.read_char()?;
        Ok(token)
    }

    /// isLetter 辅助函数用来判断给定的参数是否为字母
    /// 示例中包含 ch =='_'，这意味着下划线_会被视为字母，允许在标识符和关键字中使用
    fn is_letter(ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' || ch == '!'
    }

    fn skip_whitespace(&mut self) -> anyhow::Result<()> {
        loop {
            if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn read_identifier(&mut self) -> anyhow::Result<String> {
        let position = self.pos;
        while Self::is_letter(self.ch) {
            self.read_char()?;
        }

        let ident = self.input.get(position..self.pos).unwrap().iter().collect();

        Ok(ident)
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        let pos = self.pos + 1;
        loop {
            self.read_char()?;
            if self.ch == '"' {
                break;
            }
        }

        Ok(self.input[pos..self.pos].iter().collect())
    }
}
