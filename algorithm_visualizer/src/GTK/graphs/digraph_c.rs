use gtk::prelude::*;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	, Window, MessageDialog, DialogFlags, MessageType, ButtonsType,Image  };

use std::cell::RefCell;
use std::process::Command;

use gdk_pixbuf::Pixbuf;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;


pub fn get_d_paned_cost() -> gtk::Paned
{
	let paned = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	paned.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);
	let choose = Label::new(Some("----|| searchig algorithm ||----"));
	let info = Button::with_label("information");
	
	
	let combo =ComboBoxText::new();
    combo.append_text("Dijkstra");
    combo.append_text("Bellman Ford");
    combo.append_text("A*");
    combo.append_text("Floyd Warshall");
    
    let edit_label =  Label::new(Some("----|| edit graph ||----"));       
    
    let edges= Label::new(Some("--|Edges|--"));
    let add_edge_label= Label::new(Some("Add:"));
    let add_start_entry = Entry::new();
    add_start_entry.set_placeholder_text(Some("start"));
    
    let add_end_entry = Entry::new();
    add_end_entry.set_placeholder_text(Some("end"));
    add_end_entry.set_max_width_chars(5);
															   
    let add_cost_entry = Entry::new();
    add_cost_entry.set_placeholder_text(Some("cost"));
    let add_button = Button::with_label("add");
    
    let remove_edge_label= Label::new(Some("Remove:"));
    
    let remove_start_entry = Entry::new();
    remove_start_entry.set_placeholder_text(Some("start"));
    
    let remove_end_entry = Entry::new();
    remove_end_entry.set_placeholder_text(Some("end"));
    let remove_button = Button::with_label("remove");
    
    let vertices =Label::new(Some("--|Vertices|--"));
    let add_v_button =Button::with_label("add");
    let remove_v_button = Button::with_label("remove");
    
    let reset_button =  Button::with_label("reset");
    let sort_button =  Button::with_label("search");
    let refresh_button= Button::with_label("refresh");
    
    let space_0  = Label::new(Some("                       "));
    let space_1  = Label::new(Some("                       "));
    let space_2  = Label::new(Some("                       "));	
    let space_3  = Label::new(Some("                       ")); 
    let space_4  = Label::new(Some("                       "));
    let space_5  = Label::new(Some("                       "));
    let space_6  = Label::new(Some("                       "));
    let space_7  = Label::new(Some("                       "));
    let space_8  = Label::new(Some("                       "));
    let space_9  = Label::new(Some("                       "));
    let space_10 = Label::new(Some("                       "));
    let space_11 = Label::new(Some("                       "));
    let space_12 = Label::new(Some("                       "));
    
    grid.attach(&space_0           ,0,0,2,1);
    grid.attach(&choose            ,0,1,2,1);
    grid.attach(&space_1           ,0,2,2,1);
    grid.attach(&combo             ,0,3,2,1);
    
    grid.attach(&info              ,0,4,2,1);
    
    grid.attach(&space_2           ,0,5,2,1);
    grid.attach(&edit_label        ,0,6,2,1);
    grid.attach(&space_3           ,0,7,2,1);
    
    grid.attach(&edges             ,0,8,2,1);
    grid.attach(&space_4           ,0,9,2,1);
    grid.attach(&add_edge_label    ,0,10,2,1);
    grid.attach(&space_5           ,0,11,2,1);
    
    grid.attach(&add_start_entry   ,0,12,1,1);
    grid.attach(&add_end_entry     ,1,12,1,1);
    grid.attach(&add_cost_entry    ,0,13,1,1);
    grid.attach(&add_button        ,1,13,1,1);

    grid.attach(&space_6           ,0,14,2,1);
	grid.attach(&remove_edge_label ,0,15,2,1);
	grid.attach(&space_7           ,0,16,2,1);
    
    grid.attach(&remove_start_entry,0,17,1,1);
    grid.attach(&remove_end_entry  ,1,17,1,1);
    grid.attach(&remove_button     ,0,18,2,1);
    
    grid.attach(&space_8           ,0,19,2,1);
    grid.attach(&vertices          ,0,20,2,1);
    grid.attach(&space_9           ,0,21,2,1);

    grid.attach(&add_v_button      ,0,24,1,1);
 
    grid.attach(&remove_v_button   ,1,24,1,1); 
    
    grid.attach(&space_10          ,0,29,2,1);
    grid.attach(&reset_button      ,0,30,2,1);
    
    grid.attach(&space_11          ,0,31,2,1);
    grid.attach(&space_12          ,0,32,2,1);
    grid.attach(&sort_button       ,1,33,1,1);
    
    grid.attach(&refresh_button    ,0,33,1,1);
    
    
    
    grid.set_size_request(200, -1);
    
	
	
	paned.pack1(&grid,false,false);
	paned
}
