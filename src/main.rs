mod input;
mod output;
mod menu;
mod util;
mod model;

use {crate::util::filehandler, crate::menu::{Menu, MenuItem}};
use crate::model::MovieModel;

fn main() {
    let movies: Vec<MovieModel> = filehandler::load();
    let menu = create_menu();
    let mut choice: char;

    loop {
        menu.render(true);
        choice = menu.selection();

        if choice == 'Q' {
            break;
        }

        menu.invoke(choice, &movies);
    }

    filehandler::save(movies);
}

fn create_menu() -> Menu {
    let mut menu = Menu::new("Main Menu".to_string(), "Make your selection: ".to_string());
    menu.add_item(MenuItem::new('A', "Add Movie".to_string(), Some(do_add)));
    menu.add_item(MenuItem::new('E',  "Edit Movie".to_string(), None));
    menu.add_item(MenuItem::new('D', "Delete Movie".to_string(), None));
    menu.add_item(MenuItem::new('V', "View Movie".to_string(), None));
    menu.add_item(MenuItem::new('S', "Search Movies".to_string(), None));
    menu.add_item(MenuItem::new('Q', "Quit".to_string(), None));

    return menu;
}

fn do_add(movies: &Vec<MovieModel>) {
    println!("Doing nothing");
}
