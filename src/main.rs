mod input;
mod output;

fn main() {
    let inp = input::get_text("Enter some text".to_string(), 5, "".to_string());
    output::success_message(inp);

    let valid: Vec<char> = vec!['A', 'B', 'C'];
    let ch = input::get_char("Enter a character".to_string(), valid, true);
    output::success_message(ch.to_string());

    let int = input::get_integer("Enter a number".to_string(), 0);
    output::success_message(int.to_string());

    let int = input::get_integer("Enter a number".to_string(), 59);
    output::success_message(int.to_string());

}
