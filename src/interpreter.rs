use crate::parser::Expr;

pub const STACK_SIZE: usize = 1024 * 1024; // give the user 1MB of stack

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Interpreter {
    expr: Vec<Expr>,
    stack: [u8; STACK_SIZE],
    pointer: usize,
}

impl Interpreter {
    pub const fn new(expr: Vec<Expr>) -> Self {
        Self {
            expr,
            stack: [0u8; STACK_SIZE],
            pointer: 0usize,
        }
    }

    pub fn interpret(&mut self) {
        execute(&self.expr, &mut self.pointer, &mut self.stack);
    }
}

pub fn execute(expr: &Vec<Expr>, pointer: &mut usize, stack: &mut [u8; STACK_SIZE]) {
    for expr in expr.iter() {
        match expr {
            Expr::MovLeft => {
                if *pointer == 0 {
                    *pointer = STACK_SIZE - 1;
                } else {
                    *pointer -= 1;
                }
            }
            Expr::MovRight => {
                if *pointer == STACK_SIZE - 1 {
                    *pointer = 0;
                } else {
                    *pointer += 1;
                }
            }
            Expr::Incr => stack[*pointer] = stack[*pointer].wrapping_add(1),
            Expr::Decr => stack[*pointer] = stack[*pointer].wrapping_sub(1),
            Expr::Output => {
                let ascii_char = stack[*pointer] as char;
                print!("{ascii_char}",);
            }
            Expr::Replace => todo!(
                "Did not yet add replace function as I was too lazy to read up on what exactly it's supposed to do"
            ),
            Expr::Loop(exprs) => {
                while stack[*pointer] != 0 {
                    execute(exprs, pointer, stack);
                }
            }
        }
    }
}
