use gtk::prelude::*;
use gtk::{Box, Notebook, Orientation, Window, WindowType, Label, Paned };
#[allow(non_snake_case)]
pub mod GTK;
pub mod lists;


use crate::GTK::menu::create_menu_bar;
use crate::GTK::list::create_list_tab;
static mut CURRENT_LIST :Vec<i64> = vec![];
static mut NOTEBOOK: Option<Notebook>= None;
static mut PANED: Option<Paned>= None;


fn main() {
	// Initialiser l'application GTK
    gtk::init().expect("Failed to initialize GTK.");  
    unsafe
    {
		NOTEBOOK = Some(Notebook::new());
	}
    
    
    
    
    
    
    
    
    
    //=============gtk=============//
    
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Algorithm Visualizer");
    window.set_default_size(1000, 600);
    
     //if let Some(settings) = window.get_settings() {
        //settings.set_property_gtk_application_prefer_dark_theme(true);
    //}
    
    let verti_box = Box::new(Orientation::Vertical, 0);
    window.add(&verti_box);
    let menu_bar = create_menu_bar();
    verti_box.pack_start(&menu_bar, false, false, 0);
    let notebook = Notebook::new();
    let list_tab = create_list_tab();
    //let trees = create_list_tab();
    //let graphs = create_list_tab();
    
    
    notebook.append_page(&list_tab,Some(&Label::new(Some("List"))));
    
    
    verti_box.pack_end(&notebook, true, true, 0);
    
    window.show_all();
    gtk::main();
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
}
