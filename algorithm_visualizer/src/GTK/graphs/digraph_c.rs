use std::sync::{Arc, Mutex};
use cairo::{ImageSurface, Format};
use gtk::prelude::*;
use std::cell::RefCell;
use gdk_pixbuf::Pixbuf;
use gtk::{Grid, Paned ,Orientation, ComboBoxText, Button, Notebook, Entry, Label
	,Image  };

use crate::DICGRAPH;
use crate::dicGraph;
use crate::GTK::utilities::*;

use crate::graph::dijkstra::dijkstra;
use crate::graph::bellman_ford::bellman_ford;
use crate::graph::floyd_warshall::floyd_warshall;
use crate::graph::a_star::a_star;

pub fn get_d_paned_cost() -> gtk::Paned
{
	let paned = Paned::new(Orientation::Horizontal);
	let grid = Grid::new();
	let notebook = Notebook::new();
	paned.pack2(&notebook,true,true);
	let notebook_ref = RefCell::new(notebook);
	let choose = Label::new(Some("----|| searchig algorithm ||----"));
	let info = Button::with_label("information");
	
	let combo =ComboBoxText::new();
    combo.append_text("Dijkstra");
    combo.append_text("Bellman Ford");
    combo.append_text("A*");
    combo.append_text("Floyd Warshall");
    
    let edit_label =  Label::new(Some("----|| edit graph ||----"));       
    
    let edges= Label::new(Some("--|Edges|--"));
    let add_edge_label= Label::new(Some("Add:"));
    let add_start_entry = Entry::new();
    add_start_entry.set_placeholder_text(Some("start"));
    
    let add_end_entry = Entry::new();
    add_end_entry.set_placeholder_text(Some("end"));
    add_end_entry.set_max_width_chars(5);
															   
    let add_cost_entry = Entry::new();
    add_cost_entry.set_placeholder_text(Some("cost"));
    let add_button = Button::with_label("add");
    
    let remove_edge_label= Label::new(Some("Remove:"));
    
    let remove_start_entry = Entry::new();
    remove_start_entry.set_placeholder_text(Some("edge starting point"));
    
    let remove_end_entry = Entry::new();
    remove_end_entry.set_placeholder_text(Some("edge ending point"));
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
    grid.attach(&add_cost_entry    ,0,13,1,1);
    grid.attach(&add_button        ,1,13,1,1);

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
    grid.attach(&sort_button       ,1,34,1,1);
    grid.attach(&starting_point    ,0,34,1,1);
    
    grid.set_size_request(200, -1);
    let combo_ref = RefCell::new(combo);
    {
        let notebook_ref_clone = notebook_ref.clone();
        add_button.connect_clicked(move |_| {
            let mut notebook_mut = notebook_ref_clone.borrow_mut();
            add_edge(&add_start_entry,&add_end_entry,&add_cost_entry,&mut notebook_mut);
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
            search(&mut notebook_mut,&mut combo_mut,&starting_point);
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

fn add_vertice(notebook : &mut Notebook)
{
	unsafe 
	{
		if DICGRAPH ==None 
		{
			DICGRAPH = Some(dicGraph::new(1));
			paint_dicgraph("add Vertice",notebook,vec![2],vec![],vec![]);
		}
		else
		{
			let mut g = DICGRAPH.clone().unwrap();
			g.order+=1;
			g.adjlists.push(vec![]);
			let mut colors = vec![0;g.order as usize];
			colors[g.order as usize -1] = 2;
			
			DICGRAPH = Some(g);
			paint_dicgraph("add Vertice",notebook,colors,vec![],vec![]);
		}
	}
}

fn reset(notebook : &mut Notebook)
{
	unsafe 
	{
		let n_pages = notebook.n_pages();
		for _i in 0..n_pages
		{
			notebook.remove_page(Some(0));
		}
		DICGRAPH = None; 
		paint_dicgraph("Reset",notebook,vec![],vec![],vec![]);
	}
}

fn refresh(notebook : &mut Notebook)
{
	unsafe
	{
		if DICGRAPH != None 
		{	
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			let order = DICGRAPH.clone().unwrap().order as usize;
			paint_dicgraph("Refresh",notebook,vec![0; order],vec![],vec![]);
		}
	}
}


fn remove_vertice(notebook :&mut Notebook)
{
	 unsafe
	 {
		if DICGRAPH == None 
		{
			message("no vertice","no vertices left");
			return
		}
		let mut g = DICGRAPH.clone().unwrap();
		if g.order == 0
		{
			message("no vertice","no vertices left");
			return
		} 
		g.order-=1;
		let order = g.order as usize;
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
		for i in 0..g.order
		{
			g.costs.remove(&(g.order,i as i32));
		}
		for i in 0..g.order
		{
			g.costs.remove(&(i as i32,g.order));
		}
		DICGRAPH = Some(g);
		paint_dicgraph("Remove Vertice",notebook,vec![0; order],vec![],vec![]);                 
	 }
}
fn add_edge(start: &Entry, end: &Entry,cost: &Entry,notebook :&mut Notebook)
{
	unsafe
	{
		if DICGRAPH == None	
		{
			message("not initialized","empty graph");
			start.set_text("");
			end.set_text("");
			cost.set_text("");
			return
		}
		
		let text = start.text().to_string();	    
		if text.is_empty() 
	    {
			message("no input","nothing typed");
			end.set_text("");
			cost.set_text("");
			return        
	    }
	    let number1 = parser(&text);
	    start.set_text("");
	    if number1 == i32::MAX
	    {
			end.set_text("");
			cost.set_text("");
			return
		}
	     
		let text = end.text().to_string(); 
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			cost.set_text("");
			return        
	    }
	    let number = parser(&text);
	    end.set_text("");
		if number == i32::MAX
	    {
			cost.set_text("");
			return
		}
		
	    let text = cost.text().to_string();
	    cost.set_text(""); 
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			return        
	    }
	    let costs = parser(&text);
		if number == i32::MAX
	    {
			return
		}
		let mut g = DICGRAPH.clone().unwrap();
		g.push(number1,number,costs);
		if number1 >=0 && number1<= g.order && number >=0 && number< g.order
		{
			let mut colors = vec![0; g.order as usize];
			colors[number as usize] = 2;
			colors[number1 as usize] = 2;
			DICGRAPH = Some(g);
			paint_dicgraph("add edge",notebook,colors,vec![(number1,number)],vec![]);
		}
	}
}

fn remove_edge(start : &Entry,end : &Entry,notebook :&mut Notebook )
{
	unsafe 
	{
		if DICGRAPH == None
		{
			message("not initialized","empty graph");
			start.set_text("");
			end.set_text("");
			return 
		}
		
		let mut text = start.text().to_string(); 
		start.set_text("");
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			end.set_text("");	
			return        
	    }
	    let number1 = parser(&text);
		if number1 == i32::MAX 
		{
			end.set_text("");
			return
		}

		text = end.text().to_string();
		end.set_text(""); 
	    if text.is_empty() 
	    {
			message("no input","nothing typed");
			return        
	    }
	    let number = parser(&text);
	    if number == i32::MAX 
		{
			return
		}
		
		if number == number1 
		{
			message("error","same number");
			return
		}
		let mut g =DICGRAPH.clone().unwrap();
		if number1 >= g.order || number >= g.order ||number1 < 0 || number < 0 
		{
			message("not found", "not a vertex");
			return
		}
		for i in 0..(g.adjlists[number1 as usize].len())
		{
			if g.adjlists[number1 as usize][i] == number
			{
				g.adjlists[number1 as usize].remove(i);
			}
		}
		g.costs.remove(&(number1,number));
		let order = g.order as usize; 
		DICGRAPH = Some(g);
		paint_dicgraph("remove edge",notebook,vec![0;order],vec![],vec![]);
	}
}

fn dot(colors :Vec<i32>, edges : Vec<(i32,i32)> ) -> String
{
	let mut result = String::from("digraph dicgraph {");
	unsafe
	{
		if DICGRAPH != None
		{

			let g = DICGRAPH.clone().unwrap();
			let order = g.order;
			result.push_str(&format!(" // {}\n",order.to_string()));
			for i in 0..(order)
			{
				for j in 0..(g.adjlists[i as usize].len())
				{
					let tmp = g.adjlists[i as usize ][j];
					result.push('n');
					result.push_str(&i.to_string());
					result.push_str("->");
					result.push_str("n");
					result.push_str(&tmp.to_string());
					result.push_str(" [label = ");
					result.push_str(&g.costs.get(&(i,tmp)).unwrap().to_string());
					for k in 0..edges.len()
					{
						if edges[k]==(i,tmp)
						{
							result.push_str(" ,color = red");
						}
					}
					result.push_str(&format!("] // {} {} {}\n",&i.to_string(),&tmp.to_string(),&g.costs.get(&(i,tmp)).unwrap().to_string()));
				}
			}
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
		}		
	}
	result.push_str("}");
	result
}

pub fn paint_dicgraph(op :&str,notebook :&mut Notebook,colors :Vec<i32>, edges : Vec<(i32,i32)> , info : Vec<(&str,Vec<i32>)>)  
{
	let content = dot(colors,edges);
	save_dot_tmp(content,"dicgraph");
	save_png_tmp("dicgraph");
	let output =  "/algorithm_visualizer/src/save/tmp/dicgraph.png";
	let mut path_out = get_absolute("algorithm_visualizer");
	path_out.push_str(output);
	
	let pixbuf = Pixbuf::from_file(path_out);
	
	let image = Image::from_pixbuf(Some(&pixbuf.unwrap())); 
		
	let boxe = Grid::new();
	let mut n =0;

	boxe.attach(&image,n,0,1,1);
	n+=1;
	let height = 44.0;
	let width = 1060.0;
	for i in 0..info.len()
	{
		let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).expect("Failed to create surface");
		let cr = &Arc::new(Mutex::new(cairo::Context::new(&surface)));			
		let cloned_cr = Arc::clone(cr);
		let arc_cr  = &*cloned_cr;
		let borrowed_cr = arc_cr.lock().unwrap();
		borrowed_cr.clone().expect("REASON").set_source_rgb(1.0,1.0,1.0);
		let _ = borrowed_cr.clone().expect("REASON").paint();
		borrowed_cr.clone().expect("REASON").set_source_rgb(0.0,0.0,0.0);
		borrowed_cr.clone().expect("REASON").set_font_size(36.0);
		let string = &get_string(info[i].0,info[i].1.clone());
		let mut txtw =borrowed_cr.clone().expect("REASON").text_extents(string);
		let mut font_size = 36.0;
		while txtw.unwrap().width >= width
		{
			font_size-=0.1;
			borrowed_cr.clone().expect("REASON").set_font_size(font_size);
			txtw =borrowed_cr.clone().expect("REASON").text_extents(string);
		}
		borrowed_cr.clone().expect("REASON").move_to(width/2.0 -txtw.unwrap().width/2.0,34.0);
		let _ = borrowed_cr.clone().expect("REASON").show_text(string);
		let image = Image::from_surface(Some(&surface));
		boxe.attach(&image,0,n,1,1);
		n+=1; 
	}
	notebook.append_page(&boxe,Some(&Label::new(Some(op))));
	notebook.show_all();
	notebook.set_current_page(Some(notebook.n_pages()-1));
	drop(boxe);
	notebook.queue_draw();
	
	gtk::main_iteration();
}


fn get_string(op :&str ,info :Vec<i32>) -> String
{
	let mut result = String::from(op);
	result.push_str(" : [");
	for i in 0..info.len()-1
	{
		let tmp =info[i];
		if tmp == i32::MAX
		{
			result.push_str("inf");
		}
		else
		{
			result.push_str(&tmp.to_string());
		}
		result.push(',');
		result.push(' ');
	}
	let tmp =info[info.len()-1];
	if tmp == i32::MAX
	{
		result.push_str("inf");
	}
	else
	{
		result.push_str(&tmp.to_string());
	}
	result.push(' ');
	result.push(']');
	result 
}

pub fn search(notebook :&mut Notebook, algo: &mut ComboBoxText, entry : &Entry)
{
	unsafe
	{
		if DICGRAPH ==None 
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
	    if text.is_empty() 
	    {
			message("no input", "nothing typed");
			return        
	    }
		if text2 ==""
		{
			message("no algorithm","no sorting algorithm selected");
			return
		}
		if text2 == "Dijkstra"
		{
			let number1= parser(&text);
		    entry.set_text("");
		    if number1 == i32::MAX
			{
				entry.set_text("");
				return
			}
			let g = DICGRAPH.clone().unwrap();
			if number1>=g.order || number1 < 0
			{
				message("not found","not a vertex");
				return
			}
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			for i in 0..g.order 
			{
				for j in 0..g.order
				{
					match g.costs.get(&(i,j))
					{
						Some(value) => {if *value <0 
										{
											message("negative cost","Dijkstra doesm't work with negatives costs");
											return
										}
										},
						None        => continue,
					}
				}
			}
			dijkstra(number1 as usize,notebook);
		}
		if text2 == "Bellman Ford"
		{
			let number1= parser(&text);
		    entry.set_text("");
		    if number1 == i32::MAX
			{
				entry.set_text("");
				return
			}
			let g = DICGRAPH.clone().unwrap();
			if number1>=g.order || number1 < 0
			{
				message("not found","not a vertex");
				return
			}
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			bellman_ford(number1 as usize,notebook);
		}
		if text2 == "Floyd Warshall"
		{
			let number1= parser(&text);
		    entry.set_text("");
		    if number1 == i32::MAX
			{
				entry.set_text("");
				return
			}
			let g = DICGRAPH.clone().unwrap();
			if number1>=g.order || number1 < 0
			{
				message("not found","not a vertex");
				return
			}
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			floyd_warshall(number1 as usize,notebook);
		}
		if text2 == "A*"
		{
			let mut it = text.split_whitespace();
			let start = parser(it.next().unwrap());
			let g = DICGRAPH.clone().unwrap();
		    if start == i32::MAX
			{
				entry.set_text("");
				return
			}
			if start>=g.order || start < 0
			{
				message("not found","not a vertex");
				return
			}
			let tmp = it.next();
			let end;
			if tmp.is_some()
			{
				end = parser(tmp.unwrap());
			    if end == i32::MAX
				{
					entry.set_text("");
					return
				}
				if end>=g.order || end < 0
				{
					message("not found","not a vertex");
					return
				}
			}
			else
			{
				message("using error"," for A* it need the start and the end write the format like: \"1 2\"");
				return
			}
			let n_pages = notebook.n_pages();
			for _i in 0..n_pages
			{
				notebook.remove_page(Some(0));
			}
			a_star(start as usize,end as usize,notebook);
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
		"Dijkstra"=> 
		{
			title ="Dijkstra";
			to_show = "a shortest path algorithm that chooses the nearest node that isn't visited and uses edge relaxation";
		},
		"Bellman Ford"=> 
		{
			title ="Bellman Ford";
			to_show = "a shortest path algorithm that uses edge relaxation n times where n is the number od nodes in the graph";
		},
		"Floyd Warshall"=> 
		{
			title ="Floyd Warshall";
			to_show = "a shortest path algorithm that \"brute force\" to find all shortest path from every nodes (only show from the source)";
		},
		"A*"=> 
		{
			title ="A*";
			to_show = "a path fiding algorithm that uses a heuristic to find the way";
		},
		_ => 
		{
			title = "error";
			to_show = "no searching algorithm selected !";
		},
	}
	message(title,to_show);
}
