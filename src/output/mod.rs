use ansi_term::Colour;

fn error_message(&message: String) {
    println!("{}", Colour::Red.paint(message));
}

fn success_message(&message: String) {
    println!("{}", Colour::Green.paint(message));
}

fn plain_text(&message: String, new_line: bool) {
    println!("{}", message);
    if new_line {
        println("\n");
    }
}