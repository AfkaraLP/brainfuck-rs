# Brainfuck Interpreter

![Rust](https://img.shields.io/badge/rust-stable-orange)

A simple Brainfuck interpreter written in Rust. It lexes, parses, and interprets Brainfuck code. Supports reading code from a file or directly as raw input via CLI using [clap](https://crates.io/crates/clap).

## Features

- Lex, parse, and interpret Brainfuck code
- Read code from file or raw input
- Simple, minimal implementation for learning and experimentation

## Usage

```bash
# Run a Brainfuck file
cargo run -- --path ./examples/tictactoe.bf

# Run raw Brainfuck code
cargo run -- --code "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
```

## Roadmap

- [x] Add basic interpreter
- [x] Add replace feature
- [ ] fix input handling
- [ ] Write Compiler
