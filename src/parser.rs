use crate::lexer::Token;

pub struct Parser(Vec<Token>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    MovLeft,
    MovRight,
    Incr,
    Decr,
    Output,
    Replace,
    Loop(Vec<Expr>),
}

impl Parser {
    pub const fn new(tokens: Vec<Token>) -> Self {
        Self(tokens)
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expr = Vec::new();
        while let Some(token) = self.0.first().copied() {
            self.0.remove(0);
            match token {
                Token::MovLeft => expr.push(Expr::MovLeft),
                Token::MovRight => expr.push(Expr::MovRight),
                Token::Incr => expr.push(Expr::Incr),
                Token::Decr => expr.push(Expr::Decr),
                Token::Output => expr.push(Expr::Output),
                Token::Replace => expr.push(Expr::Replace),
                Token::LeftBrack => {
                    let inner = self.parse();
                    expr.push(Expr::Loop(inner));
                }
                Token::RightBrack => break,
            }
        }
        expr
    }
}
