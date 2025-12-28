use crate::{parser::Expr, stack::Stack};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Interpreter<const N: usize> {
    expr: Vec<Expr>,
    stack: Stack<N>,
}

impl<const N: usize> Interpreter<N> {
    pub const fn new(expr: Vec<Expr>) -> Self {
        Self {
            expr,
            stack: Stack::new(),
        }
    }

    pub fn interpret(&mut self) {
        execute(&self.expr, &mut self.stack);
    }
}

pub fn execute<const N: usize>(exprs: &Vec<Expr>, stack: &mut Stack<N>) {
    for expr in exprs {
        match expr {
            Expr::MovLeft => stack.shift_left(),
            Expr::MovRight => stack.shift_right(),
            Expr::Incr => stack.incr(),
            Expr::Decr => stack.decr(),
            Expr::Output => stack.print(),
            Expr::Replace => todo!(
                "Did not yet add replace function as I was too lazy to read up on what exactly it's supposed to do"
            ),
            Expr::Loop(exprs) => {
                while stack.current_value() != 0 {
                    execute(exprs, stack);
                }
            }
        }
    }
}
