use gtk::prelude::*;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	, Image  };

use std::cell::RefCell;

use gdk_pixbuf::Pixbuf;

use crate::UGRAPH;
use crate::uGraph;
use crate::graph::dfs::dfs_ugraph;
use crate::graph::bfs::bfs_ugraph;
use crate::GTK::utilities::*;

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
		if UGRAPH ==None 
		{
			UGRAPH = Some(uGraph::new(1));
			paint_ugraph("Add vertice",notebook,vec![2],vec![]);
		}
		else
		{
			let mut g = UGRAPH.clone().unwrap();
			g.order+=1;
			g.adjlists.push(vec![]);
			let mut colors = vec![0;g.order as usize];
			colors[g.order as usize -1] = 2;
			UGRAPH = Some(g);
			paint_ugraph("Add vertice",notebook,colors,vec![]);
		}
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
		paint_ugraph("Reset",notebook, vec![],vec![]);
	}
}

fn remove_vertice(notebook :&mut Notebook)
{
	 unsafe
	 {
		if UGRAPH == None
		{
			message("not initialized","empty graph");
			return 
		}
		let mut g = UGRAPH.clone().unwrap();
		if g.order == 0
		{
			message("no vertice","no vertices left");
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
		paint_ugraph("remove vertice",notebook, vec![],vec![]);
	 }
}

fn add_edge(start: &Entry, end: &Entry,notebook :&mut Notebook)
{
	unsafe
	{
		if UGRAPH == None
		{
			message("not initialized","empty graph");
			start.set_text("");
			end.set_text("");
			return 
		}
		
		
		let text = start.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			end.set_text("");
			return        
	    }
	    let number1 = parser(&text);
	    start.set_text("");
	    if number1 == i32::MAX
		{
			end.set_text("");
			return
		}
	    

		let text = end.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			return        
	    }
	    let number = parser(&text);
		end.set_text("");
		if number == i32::MAX
		{
			return
		}
		
		let mut g = UGRAPH.clone().unwrap();
		g.push(number1,number);
		if number < g.order && number1 <g.order
		{
			let mut colors = vec![0;g.order as usize];
			colors[number1 as usize] =2; 
			colors[number as usize] =2; 
			UGRAPH = Some(g);	
			paint_ugraph("add edge",notebook,colors,vec![(number1,number)]);
		}	
		
	}
}
fn remove_edge(start : &Entry,end : &Entry,notebook :&mut Notebook )
{
	unsafe 
	{
		if UGRAPH == None
		{
			message("not initialized","empty graph");
			start.set_text("");
			end.set_text("");
			return 
		}
		
		
		let text = start.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no input", "nothing typed");
			end.set_text("");
			return        
	    }
	    let number1 = parser(&text);
	    start.set_text("");
	    if number1 == i32::MAX 
		{
			end.set_text("");
			return
		}
		
		
		let text = end.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no input", "nothing typed");
			return        
	    }
		let number = parser(&text);
		end.set_text("");
		if number == i32::MAX 
		{
			return
		}
		
		if number == number1 
		{
			message("error","same number");
			return
		}

		let mut g =UGRAPH.clone().unwrap();
		if number1 >= g.order || number >= g.order || number1 < 0 || number < 0 
		{
			message("not found", "not a vertex");
			return
		}
		for i in 0..(g.adjlists[number1 as usize].len())
		{
			if g.adjlists[number1 as usize][i] == number
			{
				g.adjlists[number1 as usize].remove(i);
				break
			}
		}
		for i in 0..(g.adjlists[number as usize].len())
		{
			if g.adjlists[number as usize][i] == number1
			{
				g.adjlists[number as usize].remove(i);
				break
			}
		}
		let order = g.order ;
		UGRAPH = Some(g);
		paint_ugraph("remove edge",notebook,vec![0; order as usize],vec![]);
	}
}
fn dot(colors: Vec<i32>, edges :Vec<(i32,i32)>) -> String
{
	let mut result = String::from("graph ugraph {\n");
	unsafe
	{
		if UGRAPH != None
		{
			dbg!(&colors);
			let mut g = UGRAPH.clone().unwrap();
			let order = g.order;
			for i in 0..order
			{
				result.push('n');
				result.push_str(&i.to_string());
				result.push_str(&format!(" [label=\"{}\"",&i.to_string()));
				if colors[i as usize] == 2
				{
					result.push_str(", style = filled , color = green ]\n");
				}
				else if colors[i as usize] == 1
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
					result.push_str("--");
					result.push_str("n");
					result.push_str(&tmp.to_string());
					for k in 0..edges.len()
					{
						if edges[k] ==(i,tmp)||edges[k]==(tmp,i)
						{
							result.push_str(" [color = red]");
						}
					}
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


pub fn paint_ugraph(op :&str,notebook :&mut Notebook, colors: Vec<i32>, edges :Vec<(i32,i32)>)  
{
	let content = dot(colors,edges);
	save_dot_tmp(content,"ugraph");
	save_png_tmp("ugraph");
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
		if UGRAPH ==None 
		{
			message("not initialized","empty graph");
			entry.set_text("");
			return
		}
		let raw =  (*algo).active_text();
		let text = Some(raw);
		let text2 = match text 
		{
			Some(Some(string)) => string.to_string(),
			_ => String::new(), 
		};
		let text = entry.text().to_string(); 
	    if text.is_empty() {
			message("no input","nothing typed");
			return        
	    }
	    let number1 = parser(&text);
	    entry.set_text("");
	    if number1 == i32::MAX
	    {
			return
		}
		if text2 ==""
		{
			message("no algorithm","no seaching algorithm selected");
			return
		}
		if text2 == "depth-first search"
		{
			let g = UGRAPH.clone().unwrap();
			let mut m = vec![false ; g.order as usize];
			if number1>=g.order || number1<0
			{
				message("not found", "not a vertex");
				return
			}
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let mut colors = vec![0;g.order as usize] ;
			colors[number1 as usize]= 2;
			paint_ugraph("dfs",notebook,colors,vec![]);
			dfs_ugraph(number1,&mut m,true,notebook);
			
		}
		if text2 == "breadth-first search"
		{
			let g = UGRAPH.clone().unwrap();
			if number1>=g.order || number1<0
			{
				message("not found", "not a vertex");
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
			let order = UGRAPH.clone().unwrap().order as usize;
			paint_ugraph("refresh",notebook,vec![0; order],vec![]);
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
	message(title,to_show);
}
