use gtk::prelude::*;
use std::cell::RefCell;
use gdk_pixbuf::Pixbuf;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	,Image };

use crate::GTK::utilities::*;

use crate::OS;
use crate::BTREE;
use crate::tree::insert::insert;
use crate::tree::remove::delete;
use crate::tree::dfs::*;
use crate::tree::bfs::*;

pub fn create_tree_tab() -> gtk::Paned
{
	let panel = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	
	panel.pack1(&grid,false,false);
	panel.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);

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
    grid.attach(&sort_1,0,12,2,1);
    grid.attach(&sort_2,0,13,2,1);
    grid.attach(&search_button,0,14,2,1);
    grid.attach(&refresh1,0,15,2,1);
    grid.attach(&refresh_button,0,16,2,1);
    
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
        let combo_ref_clone = combo_ref.clone();
        search_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            let mut combo_mut = combo_ref_clone.borrow_mut();
            search(&mut notebook_mut,&mut combo_mut);
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
		"depth-first search (prefix)"=> 
		{
			title ="DFS prefix";
			to_show = "depth-first search prefix is a recusive searching algorithm that go into the root then the the left child and finishes with the right child";
		
		
		},
		"depth-first search (infix)"    => 
		{
			title ="DFS infix";
			to_show = "depth-first search infix is a recusive searching algorithm that go into the left child then the root and finishes with the right child";
		

		},
		"depth-first search (suffix)" => 
		{
			title= "DFS suffix";
			to_show = "depth-first search suffix is a recusive searching algorithm that go into the left child then the right child and finishes with the root ";
		},
		"breadth-first search" => 
		{
			title= "BFS";
			to_show = "breadth-first search is a iterative algorithm that go through every node in the hierarchical order";
		},
		_ => 
		{
			title = "error";
			to_show = "no seraching algorithm selected !";
		},
	}
	message(title,to_show);
	return
}

pub fn dot(current : i32 ,old :i32 ) -> String 
{
	let mut result = String::from("digraph tree {\n");
	unsafe
	{
		if BTREE !=None
		{
			let mut tmp = String::new();
			tmp = parcours_profondeur(&mut BTREE.clone(),tmp);
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
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			return        
	    }
		let number = parser(&text);
		entry.set_text("");
		if number == i32::MAX
		{
			return
		}
		let mut tmp = String::new();
		tmp = parcours_profondeur(&mut BTREE.clone(), tmp);
		for n in tmp.split_whitespace()
		{
			if n == &number.to_string()
			{
				message("error","already in the tree");
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
		let text = entry.text().to_string(); 
		if text.is_empty() 
		{
			message("no input","nothing typed");
			return        
		}
		let number = parser(&text);
		entry.set_text("");
		if number == i32::MAX
		{
			return
		}
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		let (_t,b) = delete(notebook,number);
		if !b
		{
			message("not found","not found in the tree");
			return
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
			message("no algorithm", "no searching algorithm selected");
			return
		}
		if text2 == "depth-first search (prefix)" 
		{
			clear(notebook);
			let mut _tmp = String::new();
			_tmp = dfs_pre(&mut BTREE.clone(),_tmp,notebook);
		}
		if text2 == "depth-first search (infix)" 
		{
			clear(notebook);
			let mut _tmp = String::new();
			_tmp = dfs_in(&mut BTREE.clone(),_tmp,notebook);
		}
		if text2 == "depth-first search (suffix)" 
		{
			clear(notebook);
			let mut _tmp = String::new();
			_tmp = dfs_suf(&mut BTREE.clone(),_tmp,notebook);
		}
		if text2 == "breadth-first search"
		{
			clear(notebook);
			let mut _tmp = String::new();
			_tmp = parcours_largeur(&mut BTREE.clone(),_tmp,notebook);
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

pub fn paint_tree(op :&str,notebook :&mut Notebook, current :i32 , old : i32)  
{
	let content = dot(current,old);
	save_dot_tmp(content,"tree");
	save_png_tmp("tree");
	
	let output;
	match OS
	{
	    "windows" 	=>  output = "\\algorithm_visualizer\\src\\save\\tmp\\tree.png",
	    _ 			=>  output = "/algorithm_visualizer/src/save/tmp/tree.png",
	}	
	let mut path_out = get_absolute("algorithm_visualizer");
	path_out.push_str(output);
	
	let pixbuf = Pixbuf::from_file(path_out);
	
	let image = Image::from_pixbuf(Some(&pixbuf.unwrap())); 
		
	let boxe = Grid::new();

	boxe.attach(&image,0,0,1,1);
	notebook.append_page(&boxe,Some(&Label::new(Some(op))));
	notebook.show_all();
	notebook.set_current_page(Some(notebook.n_pages()-1));

	
}


