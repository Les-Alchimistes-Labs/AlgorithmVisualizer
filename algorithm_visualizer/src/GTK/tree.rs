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

use crate::BTREE;
use crate::tree::insert::insert;
use crate::tree::remove::delete;
use crate::tree::dfs::*;

use crate::tree::bfs::parcours_largeur;


pub fn create_tree_tab() -> gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	
	panel.pack1(&grid,false,false);
	panel.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);
	
	//boutton : reset /add /remove/ search
	//3 entry : add/ remove/ search 
	// combo box : 
	let space = Label::new(Some("                               "));	
	let choose = Label::new(Some("----|| searching algorithm ||---"));
	let space_1 = Label::new(Some("                               "));
	
	let combo = ComboBoxText::new();
	combo.append_text("depth-first search (prefix)");
	combo.append_text("depth-first search (infix)");
	combo.append_text("depth-first search (suffix)");
    combo.append_text("breadth-first search");
    
    let edit_label_1 =  Label::new(Some("                       "));
    let edit_label =  Label::new(Some("----|| edit tree ||----"));
    let edit_label_2=  Label::new(Some("                       "));
    
    let add_button = Button::with_label("add");
    let remove_button = Button::with_label("remove");
    let reset_button =  Button::with_label("reset");
    let search_button =  Button::with_label("search");
    let refresh_button= Button::with_label("refresh");
    
    let sort_1=  Label::new(Some("                       "));
    let sort_2=  Label::new(Some("                       "));
    
    let add_entry = Entry::new();
    add_entry.set_placeholder_text(Some("add a node"));
    let remove_entry = Entry::new();
    remove_entry.set_placeholder_text(Some("remove a node"));
    let refresh1=  Label::new(Some("                       "));
    
    
    
     
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
    grid.attach(&search_button,0,13,2,1);
    grid.attach(&refresh1,0,15,2,1);
    grid.attach(&refresh_button,0,16,2,1);
    
    grid.set_size_request(200, -1);
       
							
     
	{
        let notebook_ref_clone = notebook_ref.clone();
        add_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            add_node(&mut notebook_mut,&add_entry);
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
            remove_node(&mut notebook_mut,&remove_entry);
        });
    }
	
	
	{
        let notebook_ref_clone = notebook_ref.clone();
        search_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            search(&mut notebook_mut,&combo);
        });
    }
    {
        let notebook_ref_clone = notebook_ref.clone();
        refresh_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            refresh(&mut notebook_mut);
        });
    }
	
    
    panel
	
} 



pub fn dot(current : i32 ,old :i32 ) -> String 
{
	let mut result = String::from("digraph tree {\n");
	unsafe
	{
		if BTREE !=None
		{
			let mut tmp = String::new();
			tmp = parcours_profondeur(&mut BTREE,tmp);
			result.push_str(&format!("//{}\n",&tmp));
			for s in tmp.split_whitespace()
			{
				result.push('n');
				result.push_str(&get_string(s.parse().unwrap()));
				result.push_str(&format!(" [label=\"{}\"",s));
				if current.to_string() == s 
				{
					result.push_str(", style = filled , color = green]\n");
				}
				else if old.to_string() == s 
				{
					result.push_str(", style = filled , color = red]\n");
				}
				else
				{
					result.push_str("]\n");
				}
			}
			let mut queue = vec![];
			queue.push(BTREE.as_mut().unwrap());
			while queue.len() != 0
			{
				let node = queue.remove(0);
				if node.left != None 
				{
					let nb = node.left.as_mut().unwrap().key;
					result.push_str(&format!("n{}->n{}\n",get_string(node.key), get_string(nb)));
					queue.push(node.left.as_mut().unwrap());
				}
				if node.right != None 
				{
					let nb = node.right.as_mut().unwrap().key;
					result.push_str(&format!("n{}->n{}\n",get_string(node.key), get_string(nb)));
					queue.push(node.right.as_mut().unwrap());
				}
			}
		}
	}	
	result.push('}');
	println!("{}",&result);
	result 
}
pub fn get_string(nb :i32) -> String
{
	let mut result = String::new();
	if nb < 0 
	{
		result.push('_');
		let tmp = nb *-1;
		result.push_str(&tmp.to_string())
	}
	else
	{
		result.push_str(&nb.to_string());
	}
	result
}


pub fn add_node(notebook :&mut Notebook, entry : &Entry)
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
		let mut number :i32 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number = number*10+(c as u8 - 48) as i32;
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
		let mut tmp = String::new();
		tmp = parcours_profondeur(&mut BTREE, tmp);
		for n in tmp.split_whitespace()
		{
			if n == &number.to_string()
			{
				let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "already in the tree !");
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
		insert(notebook,number);
		paint_tree("Add",notebook,number,number)
	}
	
}
pub fn remove_node(notebook :&mut Notebook, entry : &Entry)
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
		let mut number :i32 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number = number*10+(c as u8 - 48) as i32;
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
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		let (_t,b) = delete(notebook,number);
		dbg!(&BTREE);
		if !b
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "not found in the tree !");
			dialog.run();
			dialog.close();
			return
		}
	}
}


pub fn reset(notebook :&mut Notebook)
{
	unsafe 
	{
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		BTREE = None;
		dbg!(&BTREE);
		paint_tree("reset",notebook,1,1);
	}
	
}
pub fn search(notebook :&mut Notebook, combo : &ComboBoxText)
{
	 unsafe 
	 {
		let raw =  (*combo).active_text();
		let text = Some(raw);
		let text2 = match text 
		{
			Some(Some(string)) => string.to_string(),
			_ => String::new(), 
		};
		if text2 ==""
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
		if text2 == "depth-first search (prefix)" 
		{
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			_tmp = dfs_pre(&mut BTREE,_tmp,notebook);
		}
		if text2 == "depth-first search (infix)" 
		{
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			_tmp = dfs_in(&mut BTREE,_tmp,notebook);
		}
		if text2 == "depth-first search (suffix)" 
		{
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			_tmp = dfs_suf(&mut BTREE,_tmp,notebook);
		}
		
		
		if text2 == "breadth-first search"
		{
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut _tmp = String::new();
			_tmp = parcours_largeur(&mut BTREE,_tmp,notebook);
		}
	 }
}

pub fn refresh(notebook :&mut Notebook)
{
	unsafe
	{
		if BTREE != None 
		{	
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			paint_tree("refresh",notebook,BTREE.as_mut().unwrap().key,BTREE.as_mut().unwrap().key);
		}
	}
}

pub fn save_dot_tmp(current : i32 ,old :i32 ) 
{
	let content = dot(current,old);
	let location = "algorithm_visualizer/src/save/tmp/tree.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	let output = PathBuf::from(path);
	
	let mut file = File::create(output).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");
}


pub fn save_png_tmp()
{
	let location = "algorithm_visualizer/src/save/tmp/tree.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	
	
	let output =  "/algorithm_visualizer/src/save/tmp/tree.png";
	let mut path_out = get_absolute("algorithm_visualizer");
	path_out.push_str(output);
	
	
	let _com = Command::new("dot")
                        .arg("-Tpng")
                        .arg(path)
                        .arg("-o")
                        .arg(path_out.clone())
                        .output()
                        .expect("failed to execute process");
}

pub fn paint_tree(op :&str,notebook :&mut Notebook, current :i32 , old : i32)  
{
	save_dot_tmp(current,old);
	save_png_tmp();
	let output =  "/algorithm_visualizer/src/save/tmp/tree.png";
	let mut path_out = get_absolute("algorithm_visualizer");
	path_out.push_str(output);
	
	let pixbuf = Pixbuf::from_file(path_out);
	
	let image = Image::from_pixbuf(Some(&pixbuf.unwrap())); 
		
	let boxe = Grid::new();

	boxe.attach(&image,0,0,1,1);
	notebook.append_page(&boxe,Some(&Label::new(Some(op))));
	notebook.show_all();
	notebook.set_current_page(Some(notebook.n_pages()-1));
	drop(boxe);
	notebook.queue_draw();
	
	gtk::main_iteration();
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

