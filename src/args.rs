use clap::Parser;

/// Brainfuck interpreter
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Args {
    /// Path to brainfuck file to execute
    #[arg(short, long)]
    pub path: Option<String>,

    /// Raw brainfuck code
    #[arg(short, long)]
    pub code: Option<String>,
}
