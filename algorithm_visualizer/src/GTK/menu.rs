use gtk::prelude::*;
use crate::GTK::file::*;
use crate::GTK::edit::*;


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
    let open = gtk::MenuItem::with_label("Open");
    let sub_open = gtk::Menu::new();
    let list_open = gtk::MenuItem::with_label("list");
    let tree_open = gtk::MenuItem::with_label("tree");
    let graph_open = gtk::MenuItem::with_label("graph");
    let item_quit = gtk::MenuItem::with_label("Quit");

    

    // Créer un sous-menu pour le menu "File"
    let menu_items = gtk::Menu::new();

    // Ajouter les éléments de menu au sous-menu
    sub_open.append(&list_open);
    sub_open.append(&tree_open);
    sub_open.append(&graph_open);
    menu_items.append(&open);
    menu_items.append(&item_quit);
    open.set_submenu(Some(&sub_open));

    // Configurer le menu "File" avec le sous-menu
    result.set_submenu(Some(&menu_items));
    
    
    // CONNECTER LES BOUTONS
    list_open.connect_activate(|_| {
        open_list();
    });
    tree_open.connect_activate(|_| {
        open_tree();
    });
    
    
    item_quit.connect_activate(|_| gtk::main_quit());
    result
}

fn edition_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("Edit");

    // Créer des éléments de menu pour le menu "Edition"
    
    let as_image = gtk::MenuItem::with_label("Save as image");
    let menu_img = gtk::Menu::new();
    let img_list = gtk::MenuItem::with_label("List");
    let img_tree = gtk::MenuItem::with_label("Tree");
    let img_graph = gtk::MenuItem::with_label("Graph");
    
    menu_img.append(&img_list);
    menu_img.append(&img_tree);
    menu_img.append(&img_graph);
    as_image.set_submenu(Some(&menu_img));
    
    
    let as_text = gtk::MenuItem::with_label("Save as text");
    let menu_txt = gtk::Menu::new();
    let txt_list = gtk::MenuItem::with_label("List");
    let txt_tree = gtk::MenuItem::with_label("Tree");
    let txt_graph = gtk::MenuItem::with_label("Graph");
    
    menu_txt.append(&txt_list);
    menu_txt.append(&txt_tree);
    menu_txt.append(&txt_graph);
    as_text.set_submenu(Some(&menu_txt));
    

    // Créer un sous-menu pour le menu "Edition"
    let menu_items = gtk::Menu::new();

    // Ajouter l'élément de menu au sous-menu
    menu_items.append(&as_image);
    menu_items.append(&as_text);

    // Configurer le menu "Edition" avec le sous-menu
    result.set_submenu(Some(&menu_items));


	// CONNECTER LES BOUTONS
	
	txt_list.connect_activate(|_| {
       save_list_text();
    });
    img_list.connect_activate(|_| {
       save_list_img();
    });
    
    txt_tree.connect_activate(|_| {
       save_tree_dot();
    });
    
    img_tree.connect_activate(|_| {
       save_tree_png();
    });

	
	
	
    result
}

fn help_menu_create() -> gtk::MenuItem {
    let result = gtk::MenuItem::with_label("Help");

    // Créer des éléments de menu pour le menu "Help"
    let item_info = gtk::MenuItem::with_label("Informations");
    let sub_info = gtk::Menu::new();
    let Hlist = gtk::MenuItem::with_label("List");
    let Htree = gtk::MenuItem::with_label("Tree");
    let Hgraph = gtk::MenuItem::with_label("Graph");
    sub_info.append(&Hlist);
    sub_info.append(&Htree);
    sub_info.append(&Hgraph);
    item_info.set_submenu(Some(&sub_info));
    
    
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


