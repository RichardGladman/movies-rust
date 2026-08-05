mod input;
mod output;

fn main() {
    let inp = input::get_text("Enter some text".to_string(), 5, "".to_string());
    output::success_message(inp);
}
