use std::sync::{Arc, Mutex};
use cairo::{ImageSurface, Format};
use gtk::prelude::*;
use gtk::{ Grid, Orientation, Paned, Button, Label, Entry, 
	ComboBoxText, Window, MessageDialog, DialogFlags, MessageType,
	ButtonsType,Image, Notebook};
	
use std::env;	
	

use std::cell::RefCell;

use crate::lists::insertion_sort::insertion_sort;
use crate::lists::merge_sort::merge_sort;
use crate::lists::counting_sort::counting_sort;

use crate::CURRENT_LIST;
#[allow(unused_must_use)]




pub fn create_list_tab()->gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	
	let notebook = Notebook::new();
	panel.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);

		 
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
    let refresh_button= Button::with_label("refresh");
    let sort_1=  Label::new(Some("                       "));
    let sort_2=  Label::new(Some("                       "));
    
    let add_entry = Entry::new();
    add_entry.set_placeholder_text(Some("add a number"));
    let remove_entry = Entry::new();
    remove_entry.set_placeholder_text(Some("remove a number"));
    let refresh1=  Label::new(Some("                       "));
    let info = Button::with_label("information");
    
    
    grid.attach(&space_1,0,0,2,1);
    grid.attach(&choose,0,1,2,1);
    grid.attach(&space,0,2,2,1);
    grid.attach(&combo,0,3,2,1);
    grid.attach(&info,0,4,2,1);
    grid.attach(&edit_label_1,0,5,2,1);
    grid.attach(&edit_label,0,6,2,1);
    grid.attach(&edit_label_2,0,7,2,1);
    grid.attach(&add_entry,0,8,1,1);
    grid.attach(&add_button,1,8,1,1);
    grid.attach(&remove_entry,0,9,1,1);
    grid.attach(&remove_button,1,9,1,1);
    grid.attach(&reset_button,0,10,2,2);
    grid.attach(&sort_1,0,11,2,1);
    grid.attach(&sort_2,0,12,2,1);
    grid.attach(&sort_button,0,14,2,2);
    grid.attach(&refresh1,0,16,2,1);
    grid.attach(&refresh_button,0,17,2,1);
    
    
    grid.set_size_request(200, -1);
       
							
	let combo_ref = RefCell::new(combo);
    {
        let combo_ref_clone = combo_ref.clone();
        info.connect_clicked(move |_| {
            let mut combo_mut = combo_ref_clone.borrow_mut();
            information(&mut combo_mut);
        });
    }
	{
        let notebook_ref_clone = notebook_ref.clone();
        add_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            add_number(&mut notebook_mut,&add_entry);
        });
    }
	
	
	{
        let notebook_ref_clone = notebook_ref.clone();
        reset_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            reset(&mut notebook_mut);
        });
    }
	
	
	{
        let notebook_ref_clone = notebook_ref.clone();
        remove_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            remove_number(&mut notebook_mut,&remove_entry);
        });
    }
	
	
	{
        let notebook_ref_clone = notebook_ref.clone();
        let combo_ref_clone = combo_ref.clone();
        sort_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
             let mut combo_mut = combo_ref_clone.borrow_mut();
            sort_the_list(&mut notebook_mut,&mut combo_mut);
        });
    }
    {
        let notebook_ref_clone = notebook_ref.clone();
        refresh_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            refresh(&mut notebook_mut);
        });
    }
	
    panel.pack1(&grid, false, false);
    panel
		
}
fn information(combo : &mut ComboBoxText)
{
	let raw = (*combo).active_text();
	let text = Some(raw);
	let text2 = match text 
	{
		Some(Some(string)) => string.to_string(),
		_ => String::new(), 
	};
	let to_show;
	let title;
											 

	match text2.as_str() 
	{
		"Insertion sort"=> 
		{
			title ="Insertion sort";
			to_show = "Insertion sort is a very simple algorithm where it takes in ascending order every number and swap then with the previous one until it is sorted" ;
		
		
		},
		"Merge sort"    => 
		{
			title ="Merge sort";
			to_show = "Merge sort is a algorithm that split the list in half recursively until there's 2 or less number and then merge then in sorted order";
		

		},
		"Counting sort" => 
		{
			title= "Counting sort";
			to_show = "Counting sort is a algorithm that uses a list to count the occurence of every number and then use it to evaluate the starting index for every number and0 update the list";
		},
		_ => 
		{
			title = "error";
			to_show = "no sorting algorithm selected !";
		},
	}
	let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 to_show);
											 
	dialog.set_title(title);
	
	dialog.run();
	dialog.close();
	return
}

fn reset(notebook :&mut Notebook)
{                  
	unsafe
	{
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		CURRENT_LIST= vec![];		
		paint_list(notebook,String::from("Reset"),0,0)
	}
}

fn add_number(notebook :&mut Notebook,entry :&Entry)
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
		paint_list(notebook,String::from("Add"),CURRENT_LIST.len()-1,CURRENT_LIST.len());
	}
}


fn remove_number(notebook :&mut Notebook,entry :&Entry)
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
				paint_list(notebook,String::from("Remove"),CURRENT_LIST.len(),CURRENT_LIST.len());
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

fn sort_the_list(notebook :&mut Notebook,combo : &mut ComboBoxText)
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
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			insertion_sort(notebook);
		}
		if text2=="Counting sort"
		{
	        let mut max: usize = 0;
	        for i in 0..CURRENT_LIST.len()
	        {
	            if CURRENT_LIST[i] as usize > max
	            {
	                max = CURRENT_LIST[i] as usize;
	            }
	        }
	        for i in 0..CURRENT_LIST.len()
	        {
	            if CURRENT_LIST[i] < 0
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
	        let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
	        counting_sort(notebook,max);
		}
		if text2 =="Merge sort"
		{
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			merge_sort(notebook);
		}
	}		
}



pub fn paint_list(notebook :&mut Notebook,op : String, pos :usize , old_pos : usize)
{
	unsafe
	{
		let surface = ImageSurface::create(Format::ARgb32, 740, 500).expect("Failed to create surface");
		let cr = &Arc::new(Mutex::new(cairo::Context::new(&surface)));			
		let cloned_cr = Arc::clone(cr);
		let arc_cr  = &*cloned_cr;
		let borrowed_cr = arc_cr.lock().unwrap();
		borrowed_cr.clone().expect("REASON").set_source_rgb(0.0,0.0,0.0);
		let _ = borrowed_cr.clone().expect("REASON").paint();
		borrowed_cr.clone().expect("REASON").move_to(185.0,100.0);
		borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,1.0,1.0);
		borrowed_cr.clone().expect("REASON").set_font_size(36.0);
		borrowed_cr.clone().expect("REASON").move_to(10.0,35.0);
		let _ = borrowed_cr.clone().expect("REASON").show_text(&op);
		let string = &get_string();
		let mut txtw =borrowed_cr.clone().expect("REASON").text_extents(string);
		let mut font_size = 36.0;
		while txtw.unwrap().width >= 740.0 
		{
			font_size-=0.1;
			borrowed_cr.clone().expect("REASON").set_font_size(font_size);
			txtw =borrowed_cr.clone().expect("REASON").text_extents(string);
		}

		borrowed_cr.clone().expect("REASON").move_to(370.0-txtw.unwrap().width/2.0,100.0);
		
		let _ = borrowed_cr.clone().expect("REASON").show_text(string);
		
		drop(borrowed_cr);

	
		let height = 500.0;
		let width = 740.0;
		let max_height = (height *0.7) as i32;
		let min_width  = (width *0.1) as i32;
		let max_width  = (width *0.9) as i32;
		let nb_to_draw = CURRENT_LIST.len() as i32;
		if nb_to_draw != 0
		{
			
			let mut max_value  = CURRENT_LIST[0];
			for i in 1..CURRENT_LIST.len()
			{
				 if max_value<CURRENT_LIST[i]
				 {
					 max_value=CURRENT_LIST[i];
				 }
			}  
			let bar_width  = ((max_width-min_width)-(nb_to_draw +1))/nb_to_draw;
			let mut min = 1;
			for i in 0..CURRENT_LIST.len()
			{
				 if CURRENT_LIST[i]< min 
				 {
					 min = CURRENT_LIST[i];
				 }
			}			
			for i in 0..nb_to_draw
			{
				let cloned_cr = Arc::clone(cr);
				let arc_cr  = &*cloned_cr;
				let borrowed_cr = arc_cr.lock().unwrap();
				if i == pos as i32 
				{
					let _ = borrowed_cr.clone().expect("REASON").set_source_rgb(0.0,1.0,0.0);
				}
				else if i == old_pos as i32
				{	
					let _ = borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,0.0,0.0);
				}	
				else
				{
					let _ = borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,1.0,1.0);
				}
				drop(borrowed_cr);
				let begin_height = (height - (((CURRENT_LIST[i as usize] + min*-1 +1)  as f64/ (max_value +min*-1 +1) as f64) ) *  max_height as f64) as i32;
				let cloned_cr = Arc::clone(cr);
				let arc_cr  = &*cloned_cr;
				let mut borrowed_cr = arc_cr.lock().unwrap();
				if let Ok(ref mut ar) = *borrowed_cr {
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
				if let Ok(ref mut ar) = *borrowed_cr { let _ = ar.fill() ;}
				drop(borrowed_cr);
			}
		}
		let image = Image::from_surface(Some(&surface)); 
		
		
		let boxe = Grid::new();

		boxe.attach(&image,0,0,1,1);	
		notebook.append_page(&boxe,Some(&Label::new(Some(&op))));	
		notebook.show_all();				
		notebook.set_current_page(Some(notebook.n_pages()-1));
		
		drop(boxe);
		notebook.queue_draw();
		
		gtk::main_iteration();
		
	}
}


fn get_string()-> String
{
	unsafe
	{
		if CURRENT_LIST.len()>0 
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
		else
		{
			String::from("[ ]")
		}
	}	
}

pub fn refresh(notebook : &mut Notebook)
{
	unsafe
	{
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		paint_list(notebook,String::from("Refresh"),CURRENT_LIST.len(),CURRENT_LIST.len());
	}
}

pub fn get_absolute(root: &str) ->String
{
	 let path = env::current_dir().unwrap().to_string_lossy().to_string();
	 let mut words = vec![];
	 let mut result = String::new();
	 let mut word = String::new();
	 for c in path.chars()
	 {
		 if c =='/'
		 {
			 if word==root.to_string()
			 {
				break
			 } 
			 words.push(word.clone());
			 word = String::new();
		 }
		 else
		 {
			 word.push(c);
		 }
	 }
	for i in words
	{
		result.push_str(&i);
		result.push('/');
	}

	 result 
}





