use std::sync::{Arc, Mutex};
use gtk::prelude::*;
use gtk::{ Grid, Notebook, Orientation, Paned, Button, Label, Entry, 
	ComboBoxText, Window, MessageDialog, DialogFlags, MessageType,
	ButtonsType};
	
use crate::lists::insertion_sort::insertion_sort;
use crate::lists::counting_sort::counting_sort;



pub fn create_list_tab(mut current_list :&Arc<Mutex<Vec<i64>>>)->gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
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
    
    
    let cloned_list = Arc::clone(current_list);      
	add_button.connect_clicked(move |_| {
    let arc_ref = &*cloned_list; 
    let mut borrowed_list = arc_ref.lock().unwrap(); 
    add_number(&mut borrowed_list, &add_entry);
	});
	
	let cloned_list = Arc::clone(current_list);	
	reset_button.connect_clicked(move |_| {
    let arc_ref = &*cloned_list; 
    let mut borrowed_list = arc_ref.lock().unwrap(); 
    reset(&mut borrowed_list);
	});
	
	let cloned_list = Arc::clone(current_list);	
	remove_button.connect_clicked(move |_| {
    let arc_ref = &*cloned_list; 
    let mut borrowed_list = arc_ref.lock().unwrap(); 
    remove_number(&mut borrowed_list,&remove_entry);
	});
	
	let cloned_list = Arc::clone(current_list);	
	sort_button.connect_clicked(move |_| {
    let arc_ref = &*cloned_list; 
    let mut borrowed_list = arc_ref.lock().unwrap(); 
    sort_the_list(&mut borrowed_list,&combo);
	});
	

    
    
    
    
    panel.pack1(&grid, true, true);
    panel.pack2(&notebook,true,true);
    panel
    
}

fn reset(current_list : &mut Vec<i64>)
{
	*current_list= vec![];
	dbg!(current_list);
	
}

fn add_number(current_list : &mut Vec<i64>,entry :&Entry)
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
	let mut copy = (*current_list).clone();
	copy.push(number);
	*current_list = copy;
	dbg!(current_list);
}


fn remove_number(current_list :&mut Vec<i64>,entry :&Entry)
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
	for i in 0..current_list.len()
	{
		if number==current_list[i]
		{
			current_list.remove(i);
			dbg!(current_list);
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

fn sort_the_list(current_list :&mut Vec<i64>,combo : &ComboBoxText)
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
		insertion_sort(current_list);
		dbg!(&current_list);
	}

    if text2=="Counting sort"
    {
        let mut max: usize = 0;
        for i in 0..current_list.len() 
        {
            if current_list[i] as usize > max
            {
                max = current_list[i] as usize;
            }
        }
        for &i in current_list.iter()
        {
            if i < 0
            {
                let dialog = MessageDialog::new(None::<&Window>,
                                         DialogFlags::MODAL,
                                         MessageType::Info,
                                         ButtonsType::Close,
                                         "can't work with a negative number !");
                dialog.run();
                dialog.close();
                return
            }
        }
        counting_sort(current_list, max);
        dbg!(current_list);
    }
		
}
