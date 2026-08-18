mod input;
mod output;
mod menu;

use {menu::Menu, menu::MenuItem};

fn main() {
    let menu = create_menu();

    menu.render(true);
}

fn create_menu() -> Menu {
    let mut menu = Menu::new("Main Menu".to_string(), "Make your selection: ".to_string());

    menu.add_item(MenuItem::new('A', "Add Movie".to_string(), dummy));
    menu.add_item(MenuItem::new('E',  "Edit Movie".to_string(), dummy));
    menu.add_item(MenuItem::new('D', "Delete Movie".to_string(), dummy));
    menu.add_item(MenuItem::new('V', "View Movie".to_string(), dummy));
    menu.add_item(MenuItem::new('S', "Search Movies".to_string(), dummy));
    menu.add_item(MenuItem::new('Q', "Quit".to_string(), dummy));

    return menu;
}

fn dummy() {}
