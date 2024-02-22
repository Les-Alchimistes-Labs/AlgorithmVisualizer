use std::sync::{Arc, Mutex};
use cairo::{ImageSurface, Format};
use gtk::prelude::*;
use gtk::{ Grid, Notebook, Orientation, Paned, Button, Label, Entry, 
	ComboBoxText, Window, MessageDialog, DialogFlags, MessageType,
	ButtonsType, DrawingArea, Widget ,Image};	
	
use crate::lists::insertion_sort::insertion_sort;
use crate::CURRENT_LIST;
use crate::NOTEBOOK;
use crate::PANED;





pub fn create_list_tab()->gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let clone = panel.clone();
	unsafe
	{
		PANED = Some(clone);
	}
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

    unsafe
    {
		
		let Some(unpack) = &NOTEBOOK else { panic!("not initialized !") };
		panel.pack2(unpack,true,true);

	
		CURRENT_LIST.push(1);
		CURRENT_LIST.push(5);
		CURRENT_LIST.push(35);
		CURRENT_LIST.push(12);
		CURRENT_LIST.push(23);
	
		paint_list(String::from("Insertion Sort"),0,4);
	}	

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

pub fn paint_list(op : String, pos :usize , old_pos : usize)
{
	unsafe
	{
		
		let surface = ImageSurface::create(Format::ARgb32, 740, 500).expect("Failed to create surface");
		let cr = &Arc::new(Mutex::new(cairo::Context::new(&surface)));			
		let cloned_cr = Arc::clone(cr);
		let arc_cr  = &*cloned_cr;
		let borrowed_cr = arc_cr.lock().unwrap();
		borrowed_cr.clone().expect("REASON").set_source_rgb(0.0,0.0,0.0);
		borrowed_cr.clone().expect("REASON").paint();
		borrowed_cr.clone().expect("REASON").set_font_size(36.0);
		borrowed_cr.clone().expect("REASON").move_to(185.0,100.0);
		borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,1.0,1.0);
		borrowed_cr.clone().expect("REASON").show_text(&get_string());
		borrowed_cr.clone().expect("REASON").move_to(10.0,35.0);
		borrowed_cr.clone().expect("REASON").show_text(&op);
		drop(borrowed_cr);

	
		let height = 500.0;
		let width = 740.0;
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
		dbg!(&nb_to_draw);
		let bar_width  = ((max_width-min_width)-(nb_to_draw +1))/nb_to_draw;
		
		for i in 0..nb_to_draw
		{
			let cloned_cr = Arc::clone(cr);
			let arc_cr  = &*cloned_cr;
			let borrowed_cr = arc_cr.lock().unwrap();
			if i == pos as i32 
			{
				borrowed_cr.clone().expect("REASON").set_source_rgb(0.0,1.0,0.0);
			}
			else if i == old_pos as i32
			{	
				borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,0.0,0.0);
			}	
			else
			{
				borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,1.0,1.0);
			}
			drop(borrowed_cr);
			let begin_height = (height - ((CURRENT_LIST[i as usize]as f64/ max_value as f64) ) *  max_height as f64) as i32;
			let cloned_cr = Arc::clone(cr);
			let arc_cr  = &*cloned_cr;
			let mut borrowed_cr = arc_cr.lock().unwrap();
			if let Ok(ref mut ar) = *borrowed_cr {
			    // Unwrap the Result and get a mutable reference to the cairo::Context
			    ar.rectangle(
			        (min_width + ((bar_width+1) * i)) as f64,
			        (begin_height)as f64,
			        bar_width as f64,
			        height - begin_height as f64,
			    );}
			
			drop(borrowed_cr);
			let cloned_cr = Arc::clone(cr);
			let arc_cr  = &*cloned_cr;
			let mut borrowed_cr = arc_cr.lock().unwrap();
			if let Ok(ref mut ar) = *borrowed_cr { ar.fill() ;}
			drop(borrowed_cr);
		}
		let image = Image::from_surface(Some(&surface));
		

		let Some(notebook) = &NOTEBOOK else { panic!("not initialized !") };
		notebook.append_page(&image,Some(&Label::new(Some("List"))));
		notebook.queue_draw();
		let clone = (*notebook).clone();
		let Some(pane) = &PANED else { panic!("not initialized !") };
		pane.pack2(notebook,true,true); 
		
		gtk::main_iteration();
		drop(image);

	}
}
fn get_string()-> String
{
	unsafe
	{
		let mut result = String::new();
		result.push('[');
	
		for i in 0..(CURRENT_LIST.len()-1)
		{
			result.push_str(&CURRENT_LIST[i].to_string());
			result.push(',');
			result.push(' ');
		}
		result.push_str(&CURRENT_LIST[CURRENT_LIST.len()-1].to_string());
		result.push(']');
		result
	}
}

