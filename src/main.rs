use std::fs::read_to_string;

use clap::Parser as ArgsParser;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

/// Brainfuck interpreter
#[derive(ArgsParser, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    /// Path to brainfuck file to execute
    #[arg(short, long)]
    path: Option<String>,

    /// Raw brainfuck code
    #[arg(short, long)]
    code: Option<String>,
}

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args = Args::parse();
    match (args.code, args.path) {
        (None, None) => {
            println!("please provide a path or some code to execute, run with -h for more options")
        }
        (None, Some(path)) => run_from_file(&path),
        (Some(code), None) => run_brainfuck(&code),

        (Some(code), Some(path)) => {
            run_brainfuck(&code);
            run_from_file(&path);
        }
    }
}

pub fn run_brainfuck(code: &str) {
    let Ok(tokens) = Lexer::new(code).lex() else {
        return println!("Failed to lex file");
    };
    let parsed = Parser::new(tokens).parse();
    Interpreter::new(parsed).interpret();
}

pub fn run_from_file(path: &str) {
    let Ok(code) = read_to_string(path) else {
        return println!("Failed to read file at path: {path}");
    };
    run_brainfuck(&code);
}
