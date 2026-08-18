use ansi_term::Colour;
use std::io::{self, Write};

pub fn error_message(message: String) {
    println!("{}", Colour::Red.paint(message));
}

pub fn success_message(message: String) {
    println!("{}", Colour::Green.paint(message));
}

pub fn plain_text(message: String, new_line: bool) {
    print!("{}", message);
    if new_line {
        print!("\n");
    }
    io::stdout().flush().unwrap(); 
}