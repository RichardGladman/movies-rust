pub struct MenuItem {
    option: char,
    text: &str,
    action: fn(),
}

impl MenuItem {
    pub fn new(option: char, text: &str, action: fn()) -> MenuItem {
        return MenuItem {
            option, text, action
        };
    }

    pub fn get_option(self) -> char {
        return self.option;
    }

    pub fn get_action(self) -> fn() {
        return self.action;
    }

    pub fn render(self, new_line: bool) {
        let mut message = String::new();

        message.push(self.option);
        message.push_str(". ");
        message.push_str(self.text);

        output::plain_text(message, new_line);
    }
}