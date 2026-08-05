use ansi_term::Colour;

pub fn error_message(message: &String) {
    println!("{}", Colour::Red.paint(message));
}

pub fn success_message(message: &String) {
    println!("{}", Colour::Green.paint(message));
}

pub fn plain_text(message: &String, new_line: bool) {
    println!("{}", message);
    if new_line {
        println!("\n");
    }
}