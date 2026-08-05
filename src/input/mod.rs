use crate::output;

use std::io;

pub fn get_text(prompt: String, min_len: usize, default_value: String) -> String {
    let mut inp: String = String::from("");

    loop {
        let hint = prompt.clone() + ": ";
        output::plain_text(hint, false);
        io::stdin().read_line(&mut inp).expect("Error reading input");
        inp = inp.trim().to_string();

        if min_len > 0 && inp.len() < min_len {
            output::error_message(format!("Error: input must be at least {} characters long", min_len));
            inp = String::from("");
        }

        if min_len == 0 || inp.len() >= min_len {
            break;
        }
    }

    return if inp.len() > 0 { inp } else { default_value };
}