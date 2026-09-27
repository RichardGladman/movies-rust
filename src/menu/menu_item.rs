use crate::{model::MovieModel, output};

#[derive(Clone)]
pub struct MenuItem {
    option: char,
    text: String,
    action: Option<fn(Vec<MovieModel>)>,
}

impl MenuItem {
    pub fn new(option: char, text: String, action: Option<fn(Vec<MovieModel>)>) -> MenuItem {
        return MenuItem {
            option, text, action
        };
    }

    pub fn get_option(&self) -> char {
        return self.option;
    }

    pub fn get_action(&self) -> Option<fn(Vec<MovieModel>)> {
        return self.action;
    }

    pub fn render(&self, new_line: bool) {
        let mut message = String::new();

        message.push(self.option);
        message.push_str(". ");
        message.push_str(&self.text);

        output::plain_text(&message, new_line);
    }
}