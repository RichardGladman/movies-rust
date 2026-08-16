use crate::menu::MenuItem;

use crate::input;
use crate::output;

pub struct Menu {
    header: String, prompt: String, items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(header: String, prompt: String) -> Menu {
        return Menu {header: header, prompt: prompt, items: Vec::<MenuItem>::new()};
    }

    pub fn get_header(self) -> String {
        return self.header;
    }

    pub fn get_prompt(self) -> String {
        return self.prompt;
    }

    pub fn get_items(self) -> Vec<MenuItem> {
        return self.items;
    }

    pub fn add_item(mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn render(self, new_line: bool) {
        let mut header = String::from("\n");
        header.push_str(&self.header);
        header.push_str("\n\n");
        output::plain_text(header, new_line);

        for item in self.items {
            item.render(new_line);
        }
    }

    pub fn selection(self) -> char {
        let mut valid_values = Vec::new();

        for item in self.items {
            valid_values.push(item.get_option());
        }

        return input::get_char(self.prompt, valid_values, true);
    }

    pub fn invoke(self, choice: char) {
        for item in self.items.clone() {
            let func = item.clone().get_action().clone();
            if item.clone().get_option().clone() == choice {
                func();
                break;
            }
        }
    }
}