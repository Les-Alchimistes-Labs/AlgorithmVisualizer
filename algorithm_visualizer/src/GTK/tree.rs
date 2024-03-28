use gtk::prelude::*;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label};




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

pub fn create_tree_tab() -> gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	
	panel.pack1(&grid,false,false);
	panel.pack2(&notebook,true,true);
	
	//boutton : reset /add /remove/ search
	//3 entry : add/ remove/ search 
	// combo box : 
	let space = Label::new(Some("                               "));	
	let choose = Label::new(Some("----|| searching algorithm ||---"));
	let space_1 = Label::new(Some("                               "));
	
	let combo = ComboBoxText::new();
	combo.append_text("depth-first search");
    combo.append_text("breadth-first search");
    
    let edit_label_1 =  Label::new(Some("                       "));
    let edit_label =  Label::new(Some("----|| edit tree ||----"));
    let edit_label_2=  Label::new(Some("                       "));
    
    let add_button = Button::with_label("add");
    let remove_button = Button::with_label("remove");
    let reset_button =  Button::with_label("reset");
    let search_button =  Button::with_label("search");
    let sort_1=  Label::new(Some("                       "));
    let sort_2=  Label::new(Some("                       "));
    
    let add_entry = Entry::new();
    add_entry.set_placeholder_text(Some("add a node"));
    let remove_entry = Entry::new();
    remove_entry.set_placeholder_text(Some("remove a node"));
    let refresh1=  Label::new(Some("                       "));
	let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("search a node"));
    let refresh_button= Button::with_label("refresh");
    
    
    
     
    grid.attach(&space_1,0,0,2,1);
    grid.attach(&choose,0,1,2,1);
    grid.attach(&space,0,2,2,1);
    grid.attach(&combo,0,3,2,1);
    grid.attach(&edit_label_1,0,4,2,1);
    grid.attach(&edit_label,0,5,2,1);
    grid.attach(&edit_label_2,0,6,2,1);
    grid.attach(&add_entry,0,7,1,1);
    grid.attach(&add_button,1,7,1,1);
    grid.attach(&remove_entry,0,8,1,1);
    grid.attach(&remove_button,1,8,1,1);
    grid.attach(&reset_button,0,9,2,2);
    grid.attach(&sort_1,0,10,2,1);
    grid.attach(&sort_2,0,11,2,1);
    grid.attach(&search_button,1,13,1,1);
    grid.attach(&search_entry,0,13,1,1);
    grid.attach(&refresh1,0,15,2,1);
    grid.attach(&refresh_button,0,16,2,1);
    
    
    
    
    
    
    panel
	
} 

//pub fn tree2dot(arbre :struct Btree) ->String 
//{
	
//} 
