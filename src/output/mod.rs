use ansi_term::Colour;
use std::io::{self, Write};

pub fn error_message(message: &str) {
    println!("{}", Colour::Red.paint(message));
}

pub fn success_message(message: &str) {
    println!("{}", Colour::Green.paint(message));
}

pub fn plain_text(message: &str, new_line: bool) {
    print!("{}", message);
    if new_line {
        print!("\n");
    }
    io::stdout().flush().unwrap(); 
}