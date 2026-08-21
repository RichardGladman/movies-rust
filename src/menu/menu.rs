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

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn render(&self, new_line: bool) {
        let mut header = String::from("\n");
        header.push_str(&self.header);
        header.push_str("\n\n");
        output::plain_text(&header, new_line);

        for item in &self.items {
            (*item).render(new_line);
        }
    }

    pub fn selection(&self) -> char {
        let mut valid_values = Vec::new();

        for item in self.items.clone() {
            valid_values.push(item.get_option());
        }

        return input::get_char(&self.prompt, valid_values, true);
    }

    pub fn invoke(&self, choice: char) {
        for item in self.items.clone() {
            let option = item.clone().get_action();
            if item.clone().get_option() == choice {
                if let Some(func) = option {
                    func();
                }
                break;
            }
        }
    }
}