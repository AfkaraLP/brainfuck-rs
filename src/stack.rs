use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stack<const N: usize> {
    pointer: usize,
    stack: [u8; N],
}

impl<const N: usize> Stack<N> {
    pub const fn new() -> Self {
        Self {
            pointer: 0,
            stack: [0u8; N],
        }
    }

    pub const fn incr(&mut self) {
        self.stack[self.pointer] = self.stack[self.pointer].wrapping_add(1);
    }

    pub const fn decr(&mut self) {
        self.stack[self.pointer] = self.stack[self.pointer].wrapping_sub(1);
    }

    pub const fn shift_left(&mut self) {
        if self.pointer == 0 {
            self.pointer = N - 1;
        } else {
            self.pointer -= 1;
        }
    }
    pub const fn shift_right(&mut self) {
        if self.pointer == N - 1 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
    }

    pub const fn current_value(&self) -> u8 {
        self.stack[self.pointer]
    }

    pub const fn replace(&mut self, val: u8) {
        self.stack[self.pointer] = val;
    }

    pub fn print(&self) {
        let ascii_char = self.stack[self.pointer] as char;
        print!("{ascii_char}");
    }

    pub fn get_user_input(&mut self) {
        println!("awaiting input...");
        let mut stdout = std::io::stdout();
        _ = stdout.flush();

        let mut buf: String = String::new();
        let stdin = std::io::stdin();
        if let Err(err_code) = stdin.read_line(&mut buf) {
            return println!("Input could not be read, please try again! [{err_code}]");
        }
        let Some(input) = buf.chars().next() else {
            return println!("Could not convert input to valid 8bit ascii character");
        };

        self.replace(input as u8);
    }
}
