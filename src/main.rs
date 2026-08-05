mod output;

fn main() {
    output::error_message(&"Error Message".to_string());
    output::success_message(&"Succsess Message".to_string());
    output::plain_text(&"Plain Text".to_string(), false);

}
