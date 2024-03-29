use gtk::prelude::*;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	, Window, MessageDialog, DialogFlags, MessageType, ButtonsType  };
use std::collections::VecDeque;

use std::cell::RefCell;

use crate::BTREE;
use	crate::Btree;
use crate::tree::insert::insert;
use crate::tree::remove::delete;
use crate::tree::dfs::parcours_profondeur;
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
	combo.append_text("depth-first search");
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

pub fn dot(btree : Btree) -> String {
        let mut fake = 1;
        let mut dot_string = String::from("graph {\n");
        dot_string.push_str("graph [ordering=\"out\"]\n");

        let mut queue = VecDeque::new();
        if let Some(ref node) = btree.left {
            queue.push_back(node.clone()); // Cloning the left child
            dot_string.push_str(&format!("{:p}[label=\"{}\"]\n", node, node.key));
        }

        if let Some(ref node) = btree.right {
            queue.push_back(node.clone()); // Cloning the right child
            dot_string.push_str(&format!("{:p}[label=\"{}\"]\n", node, node.key));
        }

        while let Some(node) = queue.pop_front() {
            if let Some(ref left_child) = node.left {
                queue.push_back(left_child.clone()); // Cloning the left child
                dot_string.push_str(&format!("{:p}[label=\"{}\"]\n", left_child, left_child.key));
                dot_string.push_str(&format!("   {:p} -- {:p}\n", node, left_child));
            } else {
                dot_string.push_str(&format!("{}[color=\"white\" label=\"\"]\n", fake));
                dot_string.push_str(&format!("   {:p} -- {}[color=\"white\"]\n", node, fake));
                fake += 1;
            }

            if let Some(ref right_child) = node.right {
                queue.push_back(right_child.clone()); // Cloning the right child
                dot_string.push_str(&format!("{:p}[label=\"{}\"]\n", right_child, right_child.key));
                dot_string.push_str(&format!("   {:p} -- {:p}\n", node, right_child));
            } else {
                dot_string.push_str(&format!("{}[color=\"white\" label=\"\"]\n", fake));
                dot_string.push_str(&format!("   {:p} -- {}[color=\"white\"]\n", node, fake));
                fake += 1;
            }
        }

        dot_string.push_str("}");
        dot_string
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
		insert(notebook,number);
		paint_tree(notebook,number,number);
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
		let btree = delete(notebook,number);
		dbg!(&BTREE);
		if btree == None
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
		else
		{
			paint_tree(notebook,number,number);
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
		paint_tree(notebook,1,1);
	}
	
}
pub fn search(notebook :&mut Notebook, combo : &ComboBoxText)
{
	
}
pub fn refresh(notebook :&mut Notebook)
{
	
}
pub fn paint_tree(notebook :&mut Notebook, current :i32 , old : i32)
{
	
}
