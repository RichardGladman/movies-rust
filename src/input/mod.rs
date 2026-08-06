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

pub fn get_char(prompt: String, valid_values: Vec<char>, to_upper: bool) -> char {
    let mut ch: char;
    let mut inp: String;
    let mut valid = false;
    
    loop {
        let hint = prompt.clone() + ": ";
        inp = String::new();
        
        output::plain_text(hint, false);
        io::stdin().read_line(&mut inp).expect("Error reading input");

         if to_upper {
            inp = inp.to_uppercase();
        }

        ch = inp.chars().next().unwrap();

        if valid_values.is_empty() {
            valid = true;
        } else {
            for value in &valid_values {
                if ch == *value {
                    valid = true;
                    break;
                }
            }
            if !valid {
                output::error_message("Invalid option. Please try again".to_string());
            }
        }
        
        if valid {
            break;
        }
    }

    return ch;
}
