use std::sync::{Arc, Mutex};
use cairo::Context;
use gtk::prelude::*;
use gtk::{ Grid, Notebook, Orientation, Paned, Button, Label, Entry, 
	ComboBoxText, Window, MessageDialog, DialogFlags, MessageType,
	ButtonsType, DrawingArea, Widget };	
	
use crate::lists::insertion_sort::insertion_sort;
use crate::CURRENT_LIST;





pub fn create_list_tab(notebook :&Arc<Mutex<Notebook>>)->gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();

		 
	let space = Label::new(Some("                               "));	
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
    let reset_button =  Button::with_label("reset");
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
    grid.attach(&reset_button,0,9,2,2);
    grid.attach(&sort_1,0,10,2,2);
    grid.attach(&sort_2,0,11,2,2);
    grid.attach(&sort_button,0,12,2,2);
    grid.set_size_request(200, -1);
       
     
	add_button.connect_clicked(move |_| {add_number(&add_entry);});
	
	reset_button.connect_clicked(move |_| {reset();});
	
	remove_button.connect_clicked(move |_| {remove_number(&remove_entry);});
	
	sort_button.connect_clicked(move |_| {sort_the_list(&combo);});
	
    panel.pack1(&grid, true, true);
    
	let cloned_notebook = Arc::clone(notebook);
	let arc_notebook  = &*cloned_notebook;
	let borrowed_notebook = arc_notebook.lock().unwrap();
	panel.pack2(&*borrowed_notebook,true,true);

    panel
}

fn reset()
{
	unsafe
	{
		CURRENT_LIST= vec![];
		dbg!(&CURRENT_LIST);
	}
}

fn add_number(entry :&Entry)
{
	unsafe
	{
		let text = entry.text().to_string(); 
	    if text.is_empty() {
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "nothing typed !");
			dialog.run();
			dialog.close();
			return        
	    }
	    let mut is_negative = false;
	    let mut copy = text.clone();
	    entry.set_text("");
	    if copy.remove(0) =='-'
	    {
			is_negative =true;
		}
		let mut to_parse = if is_negative{copy} else {text};
		let mut number :i64 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number = number*10+(c as u8 - 48) as i64;
			}
			else
			{
				wrong= true;
			}
		}
		if wrong
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "not a number !");
			dialog.run();
			dialog.close();
			return
		}
		if is_negative
		{
			number*=-1;
		}
		CURRENT_LIST.push(number);
		dbg!(&CURRENT_LIST);
	}
}


fn remove_number(entry :&Entry)
{
	unsafe
	{
		
		let text = entry.text().to_string(); 
	    if text.is_empty() {
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "nothing typed !");
			dialog.run();
			dialog.close();
			return        
	    }
	    let mut is_negative = false;
	    let mut copy = text.clone();
	    entry.set_text("");
	    if copy.remove(0) =='-'
	    {
			is_negative =true;
		}
		let mut to_parse = if is_negative{copy} else {text};
		let mut number :i64 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number = number*10+(c as u8 - 48) as i64;
			}
			else
			{
				wrong= true;
			}
		}
		if wrong
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "not a number !");
			dialog.run();
			dialog.close();
			return
		}
		if is_negative
		{
			number*=-1;
		}
		for i in 0..CURRENT_LIST.len()
		{
			if number==CURRENT_LIST[i]
			{
				CURRENT_LIST.remove(i);
				paint_list(CURRENT_LIST.len(),CURRENT_LIST.len());
				dbg!(&CURRENT_LIST);
				return
			}
		}
		let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "not found in the list !");
		dialog.run();
		dialog.close();
		return
	}	
}

fn sort_the_list(combo : &ComboBoxText)
{
	unsafe
	{
		let raw = (*combo).active_text();
		let text = Some(raw);
		let text2 = match text 
		{
			Some(Some(string)) => string.to_string(),
			_ => String::new(), 
		};
		if text2==""
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "no sorting algorithm selected !");
			dialog.run();
			dialog.close();
			return
		}
		if text2=="Insertion sort"
		{
			insertion_sort();
			dbg!(&CURRENT_LIST);
		}
	}		
}

pub fn paint_list(pos :usize , old_pos : usize)
{
	unsafe
	{
		
		let drawing = DrawingArea::new();

		

		//(*borrowed_notebook).append_page(&drawing, Some(&Label::new(Some("steps"))));

      

	
		drawing.set_size_request(600, 600);
		let width= 600.0;
		let height = 600.0;
		
		drawing.connect_size_allocate(move |widget, _| {
	    widget.queue_draw();
		});
		
		let max_height = (height *0.7) as i32;
		let min_width  = (width *0.1) as i32;
		let max_width  = (width *0.9) as i32;
		let mut max_value  = CURRENT_LIST[0];
		for i in 1..CURRENT_LIST.len()
		{
			 if max_value<CURRENT_LIST[i]
			 {
				 max_value=CURRENT_LIST[i];
			 }
		} 
		let nb_to_draw = CURRENT_LIST.len() as i32;
		let bar_width  = (max_width - min_width - nb_to_draw +1)/nb_to_draw;
		
		for i in 0..nb_to_draw
		{
			let clone_pos = Arc::new(pos);
			let clone_old_pos = Arc::new(old_pos);
			let clone_height = Arc::new(height);
			let clone_max_height = Arc::new(max_height);
			let clone_min_width = Arc::new(min_width);
			let clone_max_value = Arc::new(max_value);
			let clone_bar_width = Arc::new(bar_width);
			drawing.connect_draw(move|widget,cr|{
			if i == *clone_pos as i32 
			{
				cr.set_source_rgb(0.0,1.0,0.0);
			}
			else if i == *clone_old_pos as i32
			{	
				cr.set_source_rgb(1.0,0.0,0.0);
			}	
			else
			{
				cr.set_source_rgb(1.0,1.0,1.0);
			}
			let begin_height = (*clone_height - ((CURRENT_LIST[i as usize]*100/ (*clone_max_value)) as f64) *  *clone_max_height as f64) as i32;
			cr.rectangle((*clone_min_width+ *clone_bar_width*i +1)as f64,begin_height as f64,*clone_bar_width as f64,*clone_height -begin_height as f64);
			cr.fill();
			widget.draw(cr);
			
			Inhibit(false)	
			});
		}
	}
}

