use gtk::prelude::*;
use gtk::{ Notebook, Orientation, Window, WindowType, Label};
#[allow(non_snake_case)]

pub mod GTK;
pub mod lists;
pub mod tree;


use crate::GTK::menu::create_menu_bar;
use crate::GTK::list::create_list_tab;
use crate::GTK::tree::create_tree_tab;

#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Btree {
    key: i32,
    left: Option<Box<Btree>>,
    right: Option<Box<Btree>>,
}

impl Btree {
    fn new(key: i32, left: Option<Box<Btree>>, right: Option<Box<Btree>>) -> Self {
        Btree { key, left, right }
    }
}

static mut CURRENT_LIST :Vec<i64> = vec![];
static mut BTREE :Option<Box<Btree>> = None;








fn main() {
	// Initialiser l'application GTK
    gtk::init().expect("Failed to initialize GTK.");  
    
    
    
    
    
    
    
    
    
    //=============gtk=============//
    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Algorithm Visualizer");
    window.set_default_size(1000, 600);
    
     //if let Some(settings) = window.get_settings() {
        //settings.set_property_gtk_application_prefer_dark_theme(true);
    //}
    
    let verti_box = gtk::Box::new(Orientation::Vertical, 0);
    window.add(&verti_box);
    let menu_bar = create_menu_bar();
    verti_box.pack_start(&menu_bar, false, false, 0);
    let notebook = Notebook::new();
    let list_tab = create_list_tab();
    let trees = create_tree_tab();
    //let graphs = create_list_tab();
    
    
    notebook.append_page(&list_tab,Some(&Label::new(Some("List"))));
    notebook.append_page(&trees,Some(&Label::new(Some("Tree"))));
    
    
    verti_box.pack_end(&notebook, true, true, 0);
    
    window.show_all();
    gtk::main();
    
    window.connect_destroy(|_| {
        gtk::main_quit();
    });
}
