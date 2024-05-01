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

use crate::UGRAPH;
use crate::uGraph;
use crate::graph::dfs::dfs_ugraph;
use crate::graph::bfs::bfs_ugraph;

pub fn get_paned() -> gtk::Paned
{
		let paned = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	paned.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);
	let choose = Label::new(Some("----|| searchig algorithm ||----"));
	let info = Button::with_label("information");
	
	
	let combo =ComboBoxText::new();
	combo.append_text("depth-first search");
    combo.append_text("breadth-first search");
    
    let edit_label =  Label::new(Some("----|| edit graph ||----"));       
    
    let edges= Label::new(Some("--|Edges|--"));
    let add_edge_label= Label::new(Some("Add:"));
    let add_start_entry = Entry::new();
    add_start_entry.set_placeholder_text(Some("start"));
    
    let add_end_entry = Entry::new();
    add_end_entry.set_placeholder_text(Some("end"));
    add_end_entry.set_max_width_chars(5);
															   
    let add_button = Button::with_label("add");
    
    let remove_edge_label= Label::new(Some("Remove:"));
    
    let remove_start_entry = Entry::new();
    remove_start_entry.set_placeholder_text(Some("edge starting point"));
    
    let remove_end_entry = Entry::new();
    remove_end_entry.set_placeholder_text(Some(" edge ending point"));
    let remove_button = Button::with_label("remove");
    
    let vertices =Label::new(Some("--|Vertices|--"));

    let add_v_button =Button::with_label("add");
    let remove_v_button = Button::with_label("remove");
    
    let reset_button =  Button::with_label("reset");
    let sort_button =  Button::with_label("search");
    let refresh_button= Button::with_label("refresh");
    let starting_point = Entry::new();
	starting_point.set_placeholder_text(Some("starting vertice"));
    
    let space_0  = Label::new(Some("                       "));
    let space_1  = Label::new(Some("                       "));
    let space_2  = Label::new(Some("                       "));	
    let space_3  = Label::new(Some("                       ")); 
    let space_4  = Label::new(Some("                       "));
    let space_5  = Label::new(Some("                       "));
    let space_6  = Label::new(Some("                       "));
    let space_7  = Label::new(Some("                       "));
    let space_8  = Label::new(Some("                       "));
    let space_9  = Label::new(Some("                       "));
    let space_10 = Label::new(Some("                       "));
    let space_11 = Label::new(Some("                       "));
    let space_12 = Label::new(Some("                       "));
    
    grid.attach(&space_0           ,0,0,2,1);
    grid.attach(&choose            ,0,1,2,1);
    grid.attach(&space_1           ,0,2,2,1);
    grid.attach(&combo             ,0,3,2,1);
    
    grid.attach(&info              ,0,4,2,1);
    
    grid.attach(&space_2           ,0,5,2,1);
    grid.attach(&edit_label        ,0,6,2,1);
    grid.attach(&space_3           ,0,7,2,1);
    
    grid.attach(&edges             ,0,8,2,1);
    grid.attach(&space_4           ,0,9,2,1);
    grid.attach(&add_edge_label    ,0,10,2,1);
    grid.attach(&space_5           ,0,11,2,1);
    
    grid.attach(&add_start_entry   ,0,12,1,1);
    grid.attach(&add_end_entry     ,1,12,1,1);
    grid.attach(&add_button        ,0,13,2,1);

    grid.attach(&space_6           ,0,14,2,1);
	grid.attach(&remove_edge_label ,0,15,2,1);
	grid.attach(&space_7           ,0,16,2,1);
    
    grid.attach(&remove_start_entry,0,17,1,1);
    grid.attach(&remove_end_entry  ,1,17,1,1);
    grid.attach(&remove_button     ,0,18,2,1);
    
    grid.attach(&space_8           ,0,19,2,1);
    grid.attach(&vertices          ,0,20,2,1);
    grid.attach(&space_9           ,0,21,2,1);
    grid.attach(&add_v_button      ,0,24,1,1);
    grid.attach(&remove_v_button   ,1,24,1,1); 
    
    grid.attach(&space_10          ,0,29,2,1);
    grid.attach(&reset_button      ,0,30,2,1);
    
    grid.attach(&space_11          ,0,31,2,1);
    grid.attach(&space_12          ,0,32,2,1);
    
    grid.attach(&refresh_button    ,0,33,2,1);
    grid.attach(&starting_point    ,0,34,1,1);
    grid.attach(&sort_button       ,1,34,1,1);

    grid.set_size_request(200, -1);
    
    
    let combo_ref = RefCell::new(combo);
    
    //a connecter
    
    {
        let notebook_ref_clone = notebook_ref.clone();
        add_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            add_edge(&add_start_entry,&add_end_entry,&mut notebook_mut);
        });
    } 
    {
        let notebook_ref_clone = notebook_ref.clone();
        remove_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            remove_edge(&remove_start_entry,&remove_end_entry,&mut notebook_mut);
        });
    } 
    
    
    {
        let notebook_ref_clone = notebook_ref.clone();
        add_v_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            add_vertice(&mut notebook_mut);
        });
    } 
    {
        let notebook_ref_clone = notebook_ref.clone();
        remove_v_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            remove_vertice(&mut notebook_mut);
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
        let combo_ref_clone = combo_ref.clone();
        sort_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            let mut combo_mut = combo_ref_clone.borrow_mut();
            search(&mut combo_mut,&mut notebook_mut,&starting_point);
        });
    }
    {
        let notebook_ref_clone = notebook_ref.clone();
        refresh_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            refresh(&mut notebook_mut);
        });
    }
    {
        let combo_ref_clone = combo_ref.clone();
        info.connect_clicked(move |_| {
            let mut combo_mut = combo_ref_clone.borrow_mut();
            information(&mut combo_mut);
        });
    }
    
    
	
	paned.pack1(&grid,false,false);
	paned
	
}

fn add_vertice(notebook :&mut Notebook)
{
	unsafe 
	{
		let order;
		if UGRAPH ==None 
		{
			UGRAPH = Some(uGraph::new(1));
			order = 1;
		}
		else
		{
			let mut g = UGRAPH.clone().unwrap();
			g.order+=1;
			order = g.order;
			g.adjlists.push(vec![]);
			UGRAPH = Some(g);
		}
		paint_ugraph("Add vertice",notebook,order-1,-1);
	}
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
		UGRAPH = None;
		paint_ugraph("Reset",notebook, -1,-1);
	}
}

fn remove_vertice(notebook :&mut Notebook)
{
	 unsafe
	 {
		let mut g = UGRAPH.clone().unwrap();
		if g.order == 0
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "no vertices left");
			dialog.run();
			dialog.close();
			return
		} 
		g.order-=1;
		g.adjlists.remove(g.order as usize);
		for i in 0..(g.order as usize)
		{
			for j in 0..(g.adjlists[i].len())
			{
				if g.adjlists[i][j]==g.order
				{
					g.adjlists[i].remove(j);
				}
			}
		}  
		UGRAPH = Some(g);                 
		paint_ugraph("remove vertice",notebook, -1,-1);
	 }
}

fn add_edge(start: &Entry, end: &Entry,notebook :&mut Notebook)
{
	unsafe
	{
		let text = start.text().to_string(); 
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
	    start.set_text("");
	    if copy.remove(0) =='-'
	    {
			is_negative =true;
		}
		let mut to_parse = if is_negative{copy} else {text};
		let mut number1 :i32 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number1 = number1*10+(c as u8 - 48) as i32;
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
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "why a negative number ?");
			dialog.run();
			dialog.close();
			return
		}
		let text = end.text().to_string(); 
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
		end.set_text("");
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
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "why a negative number ?");
			dialog.run();
			dialog.close();
			return
		}
		if number == number1 
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "same number !");
			dialog.run();
			dialog.close();
			return
		}
		let mut g = UGRAPH.clone().unwrap();
		if number>=g.order ||number1>= g.order
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "vertice doesn't exist");
			dialog.run();
			dialog.close();
			return
		}
		for i in 0..(g.adjlists[number1 as usize].len())
		{
			if g.adjlists[number1 as usize][i] == number
			{
				let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "already in the graph ");
			dialog.run();
			dialog.close();
			return
			}
		}
		g.adjlists[number1 as usize ].push(number);
		g.adjlists[number as usize ].push(number1);
		g.adjlists[number as usize].sort();
		g.adjlists[number1 as usize].sort();
		UGRAPH = Some(g);
		paint_ugraph("add edge",notebook,-1,-1);	
		
	}
}
fn remove_edge(start : &Entry,end : &Entry,notebook :&mut Notebook )
{
	unsafe 
	{
		if UGRAPH == None
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "no edges !");
			dialog.run();
			dialog.close();
			return 
		}
		let text = start.text().to_string(); 
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
	    start.set_text("");
	    if copy.remove(0) =='-'
	    {
			is_negative =true;
		}
		let mut to_parse = if is_negative{copy} else {text};
		let mut number1 :i32 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number1 = number1*10+(c as u8 - 48) as i32;
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
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "why a negative number ?");
			dialog.run();
			dialog.close();
			return
		}
		let text = end.text().to_string(); 
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
		end.set_text("");
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
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "why a negative number ?");
			dialog.run();
			dialog.close();
			return
		}
		if number == number1 
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "same number !");
			dialog.run();
			dialog.close();
			return
		}
		let mut g =UGRAPH.clone().unwrap();
		if number>= g.order ||number1>= g.order
		{
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "not a vertex");
			dialog.run();
			dialog.close();
			return
		}
		for i in 0..(g.adjlists[number1 as usize].len())
		{
			if g.adjlists[number1 as usize][i] == number
			{
				g.adjlists[number1 as usize].remove(i);
			}
		}
		for i in 0..(g.adjlists[number as usize].len())
		{
			if g.adjlists[number as usize][i] == number1
			{
				g.adjlists[number as usize].remove(i);
			}
		}
		UGRAPH = Some(g);
		paint_ugraph("remove edge",notebook,-1,-1);
	}
}
fn dot(current :i32 , old : i32) -> String
{
	let mut result = String::from("graph ugraph {\n");
	unsafe
	{
		if UGRAPH != None
		{

			let mut g = UGRAPH.clone().unwrap();
			let order = g.order;
			for i in 0..order
			{
				result.push('n');
				result.push_str(&i.to_string());
				result.push_str(&format!(" [label=\"{}\"",&i.to_string()));
				if i == current
				{
					result.push_str(", style = filled , color = green ]\n");
				}
				else if i == old
				{
					result.push_str(", style = filled , color = red ]\n");
				}
				else
				{
					result.push_str("]\n");
				}
			}
			
			for i in 0..(order)
			{
				for j in 0..(g.adjlists[i as usize].len())
				{
					let tmp = g.adjlists[i as usize ][j];
					result.push('n');
					result.push_str(&i.to_string());
					result.push_str(" -- ");
					result.push_str("n");
					result.push_str(&tmp.to_string());
					result.push('\n');
					for k in 0..(g.adjlists[tmp as usize].len())
					{
						if g.adjlists[tmp as usize][k] == i
						{
							g.adjlists[tmp as usize].remove(k);
							break;
						} 
					}
				}
			}
		}		
	}
	result.push_str("}");
	println!("{}",result);
	result
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
pub fn save_dot_tmp(current : i32 ,old :i32 ) 
{
	let content = dot(current,old);
	let location = "algorithm_visualizer/src/save/tmp/ugraph.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	let output = PathBuf::from(path);
	
	let mut file = File::create(output).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");
}

pub fn save_png_tmp()
{
	let location = "algorithm_visualizer/src/save/tmp/ugraph.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	
	
	let output =  "/algorithm_visualizer/src/save/tmp/ugraph.png";
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

pub fn paint_ugraph(op :&str,notebook :&mut Notebook, current :i32 , old : i32)  
{
	save_dot_tmp(current,old);
	save_png_tmp();
	let output =  "/algorithm_visualizer/src/save/tmp/ugraph.png";
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
pub fn search(algo :&mut ComboBoxText , notebook : &mut Notebook, entry :&Entry)
{
	unsafe
	{
		let raw =  (*algo).active_text();
		let text = Some(raw);
		let text2 = match text 
		{
			Some(Some(string)) => string.to_string(),
			_ => String::new(), 
		};
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
		let mut number1 :i32 =0;
		let mut wrong = false;
		while to_parse.chars().count()!=0 && !wrong  
		{
			let c = to_parse.remove(0);
			if c as u8>=48 && c as u8<=57 
			{
				number1 = number1*10+(c as u8 - 48) as i32;
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
			let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "why a negative number ?");
			dialog.run();
			dialog.close();
			return
		}
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
		if text2 == "depth-first search"
		{
			if UGRAPH ==None 
			{
				let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "empty graph !");
				dialog.run();
				dialog.close();
				return
			}
			else
			{
				let g = UGRAPH.clone().unwrap();
				let mut m = vec![false ; g.order as usize];
				if number1>=g.order
				{
					let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "doesn't exist !");
				
					dialog.run();
					dialog.close();
					return
				}
				let n_pages = notebook.n_pages();
				for _i in 0..n_pages
				{
					notebook.remove_page(Some(0));
				}
				paint_ugraph("dfs",notebook,number1,-1);
				dfs_ugraph(number1,&mut m,true,notebook);
			}
		}
		if text2 == "breadth-first search"
		{
			if UGRAPH ==None 
			{
				let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "empty graph !");
				dialog.run();
				dialog.close();
				return
			}
			else
			{
				let g = UGRAPH.clone().unwrap();
				if number1>=g.order
				{
					let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "doesn't exist !");
				
					dialog.run();
					dialog.close();
					return
				}
				let n_pages = notebook.n_pages();
				for _i in 0..n_pages
				{
					notebook.remove_page(Some(0));
				}
				bfs_ugraph(number1,notebook);
			}
		}
	}
}

pub fn refresh(notebook :&mut Notebook)
{
	unsafe
	{
		if UGRAPH != None 
		{	
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			paint_ugraph("refresh",notebook,-1,-1);
		}
	}
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
		"depth-first search"=> 
		{
			title ="DFS";
			to_show = "depth-first search is a recusive searching algorithm that go into the root then in his smallest neighbor until every node in the component is reached once";
		
		
		},
		"breadth-first search" => 
		{
			title= "BFS";
			to_show = "breadth-first search is a iterative algorithm that go through every vertices from the source and propagate in every direction";
		},
		_ => 
		{
			title = "error";
			to_show = "no seraching algorithm selected !";
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
