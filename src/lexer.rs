#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lexer<'src>(&'src str);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Token {
    MovLeft,
    MovRight,
    Incr,
    Decr,
    RightBrack,
    LeftBrack,
    Output,
    Replace,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LexError {
    UnexpectedChar(char),
}

impl<'src> Lexer<'src> {
    pub const fn new(input: &'src str) -> Self {
        Self(input)
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = Vec::with_capacity(self.0.len());
        while let Some(c) = self.peek() {
            match c {
                '+' => tokens.push(Token::Incr),
                '-' => tokens.push(Token::Decr),
                '[' => tokens.push(Token::LeftBrack),
                ']' => tokens.push(Token::RightBrack),
                ',' => tokens.push(Token::Replace),
                '.' => tokens.push(Token::Output),
                '<' => tokens.push(Token::MovLeft),
                '>' => tokens.push(Token::MovRight),
                _ if c.is_whitespace() => {}
                v => return Err(LexError::UnexpectedChar(v)), // maybe switch it to just ignore stuff
            }
            self.advance();
        }
        Ok(tokens)
    }

    fn advance(&mut self) {
        self.0 = &self.0[1..];
    }

    fn peek(&self) -> Option<char> {
        self.0.chars().nth(0)
    }
}
