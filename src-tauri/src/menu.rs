use tauri::{CustomMenuItem, Menu, Submenu};

pub fn get_menu() -> Menu {
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let save = CustomMenuItem::new("save".to_string(), "Save");
    let saveas = CustomMenuItem::new("saveas".to_string(), "Save As");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let close = CustomMenuItem::new("close".to_string(), "Close");
    // let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(save).add_item(saveas).add_item(quit));
    
    Menu::new()
        .add_submenu(submenu)
    //   .add_native_item(MenuItem::Copy)
    //   .add_item(CustomMenuItem::new("hide", "Hide"))

}
