use gtk::prelude::*;
use crate::GTK::file::*;


pub fn create_menu_bar() -> gtk::MenuBar 
{
	let menu_bar = gtk::MenuBar::new();
	
	let file_menu = file_menu_create();
    let edition_menu = edition_menu_create();
    let help_menu = help_menu_create();

    // Ajouter les menus à la barre de menu
    menu_bar.append(&file_menu);
    menu_bar.append(&edition_menu);
    menu_bar.append(&help_menu);

    menu_bar
}


fn file_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("File");

    // Créer des éléments de menu pour le menu "File"
    let list_open = gtk::MenuItem::with_label("Open list");
    let tree_open = gtk::MenuItem::with_label("Open graph");
    let graph_open = gtk::MenuItem::with_label("Open tree");
    let item_quit = gtk::MenuItem::with_label("Quit");

    // Créer un sous-menu pour le menu "File"
    let menu_items = gtk::Menu::new();

    // Ajouter les éléments de menu au sous-menu
    menu_items.append(&list_open);
    menu_items.append(&tree_open);
    menu_items.append(&graph_open);
    menu_items.append(&item_quit);

    // Configurer le menu "File" avec le sous-menu
    result.set_submenu(Some(&menu_items));
    
    
    // CONNECTER LES BOUTONS
    list_open.connect_activate(|_| {
        show_file_chooser_dialog();
    });
    
    
    
    
    item_quit.connect_activate(|_| gtk::main_quit());
    result
}

fn edition_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("Edit");

    // Créer des éléments de menu pour le menu "Edition"
    let save = gtk::MenuItem::with_label("Save");

    // Créer un sous-menu pour le menu "Edition"
    let menu_items = gtk::Menu::new();

    // Ajouter l'élément de menu au sous-menu
    menu_items.append(&save);

    // Configurer le menu "Edition" avec le sous-menu
    result.set_submenu(Some(&menu_items));




	// CONNECTER LES BOUTONS
    result
}

fn help_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("Help");

    // Créer des éléments de menu pour le menu "Help"
    let item_info = gtk::MenuItem::with_label("Informations");
    let item_credits = gtk::MenuItem::with_label("Credits");

    // Créer un sous-menu pour le menu "Help"
    let menu_items = gtk::Menu::new();

    // Ajouter les éléments de menu au sous-menu
    menu_items.append(&item_info);
    menu_items.append(&item_credits);

    // Configurer le menu "Help" avec le sous-menu
    result.set_submenu(Some(&menu_items));

	 // CONNECTER LES BOUTONS

    result
}


