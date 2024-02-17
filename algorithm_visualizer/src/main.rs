use gtk::prelude::*;
use gtk::{Box, Notebook, Orientation, Window, WindowType,Label};

#[allow(non_snake_case)]
pub mod GTK;


use crate::GTK::menu::create_menu_bar;
use crate::GTK::list::create_list_tab;

fn main() {
	// Initialiser l'application GTK
    gtk::init().expect("Failed to initialize GTK.");
    
    
    
    
    
    
    
    
    
    //=============gtk=============//
    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Algorithm Visualizer");
    window.set_default_size(1000, 600);
    
    let verti_box = Box::new(Orientation::Vertical, 0);
    window.add(&verti_box);
    let menu_bar = create_menu_bar();
    verti_box.pack_start(&menu_bar, false, false, 0);
    
    let notebook = Notebook::new();
    let lists = create_list_tab();
    //let trees = create_list_tab();
    //let graphs = create_list_tab();
    
    notebook.append_page(&lists,Some(&Label::new(Some("List"))));
    
    
    verti_box.pack_end(&notebook, true, true, 0);
    
    window.show_all();
    gtk::main();
}
