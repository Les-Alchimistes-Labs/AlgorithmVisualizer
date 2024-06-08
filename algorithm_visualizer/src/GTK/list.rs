use std::cell::RefCell;
use cairo::{ImageSurface, Format};
use gtk::prelude::*;
use gtk::{ Grid, Orientation, Paned, Button, Label, Entry, 
	ComboBoxText, Image, Notebook,};
	
use crate::lists::insertion_sort::insertion_sort;
use crate::lists::merge_sort::merge_sort;
use crate::lists::counting_sort::counting_sort;
use crate::lists::reverse::reverse;
use crate::lists::shuffle::shuffle;

use crate::lists::stalin_sort::stalin_sort;
use crate::lists::quick_sort::quick_sort;
use crate::lists::bogo_sort::bogo_sort;

use crate::GTK::utilities::*;
use crate::CURRENT_LIST;
use crate::BOGO_WARNED;

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
	combo.append_text("Shuffle");
	combo.append_text("Reverse");
    combo.append_text("Insertion sort");
    combo.append_text("Merge sort");
    combo.append_text("Counting sort");
    combo.append_text("Quick sort");
    combo.append_text("Stalin sort");
    combo.append_text("Bogo sort");
    
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
	    if text.is_empty() 
	    {
			message("no entry", "nothing typed");
			return        
	    }
	    let number = parser(&text);
	    entry.set_text("");
		if number == i32::MAX
		{
			return
		}
		CURRENT_LIST.push(number as i64);
		paint_list(notebook,String::from("Add"),CURRENT_LIST.len()-1,CURRENT_LIST.len());
	}
}

fn remove_number(notebook :&mut Notebook,entry :&Entry)
{
	unsafe
	{
		let text = entry.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no entry", "nothing typed");
			return        
	    }
		let number = parser(&text);
		entry.set_text("");
		if number == i32::MAX
		{
			return
		}
		for i in 0..CURRENT_LIST.len()
		{
			if number as i64==CURRENT_LIST[i]
			{
				CURRENT_LIST.remove(i);
				paint_list(notebook,String::from("Remove"),CURRENT_LIST.len(),CURRENT_LIST.len());
				return
			}
		}
		message("not found", "not found in the list");
		return
	}	
}

pub fn paint_list(notebook :&mut Notebook,op : String, pos :usize , old_pos : usize)
{
	unsafe
	{
	    let height = 797.0;
		let width = 1160.0;
		let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).expect("Failed to create surface");
		let cr = cairo::Context::new(&surface).unwrap();
		cr.set_source_rgb(0.0,0.0,0.0);
		let _ = cr.paint();
		cr.clone().move_to(185.0,100.0);
		cr.set_source_rgb(1.0,1.0,1.0);
		cr.set_font_size(36.0);
		cr.move_to(10.0,35.0);
		let _ = cr.show_text(&op);
		
		let string = &get_string();
		let mut txtw = cr.text_extents(string).unwrap();
		let mut font_size = 36.0;
		while txtw.width >= width 
		{
			font_size-=0.1;
			cr.set_font_size(font_size);
			txtw = cr.text_extents(string).unwrap();
		}
		cr.move_to(width/2.0 -txtw.width/2.0,100.0);
		
		let _ =cr.show_text(string);
		
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
				if i == pos as i32 
				{
					cr.set_source_rgb(0.0,1.0,0.0);
				}
				else if i == old_pos as i32
				{	
					cr.set_source_rgb(1.0,0.0,0.0);
				}	
				else
				{
					cr.set_source_rgb(1.0,1.0,1.0);
				}
				let begin_height = (height - (((CURRENT_LIST[i as usize] + min*-1 +1)  as f64/ (max_value +min*-1 +1) as f64) ) *  max_height as f64) as i32;
				cr.rectangle(
				        (min_width + ((bar_width+1) * i)) as f64,
				        (begin_height)as f64,
				        bar_width as f64,
				        height - begin_height as f64,
				    );
				let _ = cr.fill();
			}
		}
		let image = Image::from_surface(Some(&surface)); 
		let boxe = Grid::new();

		boxe.attach(&image,0,0,1,1);	
		notebook.append_page(&boxe,Some(&Label::new(Some(&op))));	
		notebook.show_all();				
		notebook.set_current_page(Some(notebook.n_pages()-1));
		
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

fn sort_the_list(notebook :&mut Notebook,combo : &mut ComboBoxText)
{
	unsafe
	{
		if CURRENT_LIST.len() < 2
		{
			message("already sorted", "nothing to sort");
			return
		}
		let raw = (*combo).active_text();
		let text2 = match raw 
		{
			Some(string) => string.to_string(),
			_ => String::new(), 
		};
		if text2==""
		{
			message("no algorithm", "no algorithm selected");
			return
		}
		else if text2=="Insertion sort"
		{
			clear(notebook);
			insertion_sort(notebook);
		}
		else if text2=="Counting sort"
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
					message("negative numbers", "counting sort doesn't work with negative numbers !");
	                return
	            }
	        }
			clear(notebook);
	        counting_sort(notebook,max);
		}
		else if text2 =="Merge sort"
		{
			clear(notebook);
			merge_sort(notebook);
		}
		else if text2 =="Reverse"
		{
			clear(notebook);
			reverse(notebook);
		}
		else if text2 =="Shuffle"
		{
			clear(notebook);
			shuffle(notebook);
		}
		else if text2 =="Stalin sort"
		{
			clear(notebook);
			stalin_sort(notebook);
		}
		else if text2 =="Quick sort"
		{
			clear(notebook);
			quick_sort(notebook,0,CURRENT_LIST.len() as i32-1);
		}
		else if text2 =="Bogo sort"
		{
			if BOGO_WARNED
			{
				clear(notebook);
				bogo_sort(notebook);
				BOGO_WARNED = false;				
			}
			else
			{
				message("Warning","Bogo sort might crash the application because it sorts randomly.\n be sure to have a low number of element in the listbefore proceeding \n\n click again to proceed");
				BOGO_WARNED = true;
			}
		}
	}		
}

fn information(combo : &mut ComboBoxText)
{
	let raw = (*combo).active_text();
	let text2 = match raw 
	{
		Some(string) => string.to_string(),
		_ => String::new(), 
	};
	let to_show;
	let title;									 

	match text2.as_str() 
	{
		"Insertion sort"=> 
		{
			title ="Insertion sort";
			to_show = "Insertion sort is a very simple algorithm where it takes in ascending order every number and swap then with the previous one until it is sorted." ;
		},
		"Merge sort"    => 
		{
			title ="Merge sort";
			to_show = "Merge sort is a algorithm that split the list in half recursively until there's 2 or less number and then merge then in sorted order.";
		},
		"Counting sort" => 
		{
			title= "Counting sort";
			to_show = "Counting sort is a algorithm that uses a list to count the occurence of every number and then use it to evaluate the starting index for every number and0 update the list.";
		},
		"Reverse"       => 
		{
			title = "Reverse";
			to_show = "reverses the values in the list, so if itis sorted in ascending order,it puts it in descendiong order.";
		},	
		"Shuffle"       => 
		{
			title = "Shuffle";
			to_show = "shuffles all the elements in the list.";
		},		
		"Quick sort"    => 
		{
			title = "Quick sort";
			to_show = "Quick sort is an recursive algorithm that uses a pivot to swap values wether they're greater or smaller.";
		},
		"Stalin sort" => 
		{
			title = "Stalin sort";
			to_show = "Stalin sort is a very effective sorting algorithm but at the cost of losing information, it removes every values that make the list unsorted.";
		},	
		"Bogo sort" => 
		{
			title = "Bogo sort";
			to_show = "bogo sort is one of the less effective sorting algorithm that shuffles the list until it is sorted.";
		},
		_ => 
		{
			title = "error";
			to_show = "no sorting algorithm selected !";
		},
	}
	message(title,to_show);
	return
}




