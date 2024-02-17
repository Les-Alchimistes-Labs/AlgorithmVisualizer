use gtk::prelude::*;
use gtk::{ Grid, Notebook, Orientation, Paned, Button, Label, Entry,ComboBoxText};

pub fn create_list_tab()->gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	let space   = Label::new(Some("                               "));	
	let choose = Label::new(Some("----|| sorting algorithm ||----"));
	let space_1 = Label::new(Some("                               "));
	let combo =ComboBoxText::new();
    combo.append_text("Insertion sort");
    combo.append_text("Merge sort");
    combo.append_text("Counting sort");
    
    let edit_label_1 =  Label::new(Some("                       "));
    let edit_label =  Label::new(Some("----|| edit list ||----"));
    let edit_label_2=  Label::new(Some("                       "));
    let add_button = Button::with_label("add");
    let remove_button = Button::with_label("remove");
    let sort_button =  Button::with_label("sort the list");
    let sort_1=  Label::new(Some("                       "));
    let sort_2=  Label::new(Some("                       "));
    
    let add_entry = Entry::new();
    add_entry.set_placeholder_text(Some("add a number"));
    let remove_entry = Entry::new();
    remove_entry.set_placeholder_text(Some("remove a number"));
    
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
    grid.attach(&sort_1,0,9,2,2);
    grid.attach(&sort_2,0,10,2,2);
    grid.attach(&sort_button,0,11,2,2);
    
    
    panel.pack1(&grid, true, true);
    panel.pack2(&notebook,true,true);
    panel
    
}
