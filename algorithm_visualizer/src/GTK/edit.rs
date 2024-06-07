use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::fs::read_to_string;

use crate::{CURRENT_LIST, BTREE, UGRAPH, UCGRAPH,DICGRAPH,DIGRAPH};
use crate::GTK::utilities::*;

pub fn save_list_text()
{
	unsafe
	{
		let mut content = String::new();
		for i in 0..CURRENT_LIST.len()
		{
			content.push_str(&CURRENT_LIST[i].to_string());
			content.push_str("\n");
		}
		let location = "algorithm_visualizer/src/save/dot/list.txt";
		let mut path = get_absolute("algorithm_visualizer");
		path.push_str(location);
		let output = PathBuf::from(path);
		
		let mut file = File::create(output).expect("failed to create file");
	    file.write_all(content.as_bytes()).expect("failed to write to file");
	    message("save success", "successfully saved the list as text");
	}
}

pub fn save_dot(t : &str) 
{
	unsafe
	{
		match t 
		{
			"tree"     => { if BTREE == None 
				{
					message("not intialized","nothing to save");
					return
				}},
				
			"ugraph"   =>{ if UGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"ucgraph"  => { if UCGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"digraph"  => { if DIGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"dicgraph" =>{ if DICGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			_          =>panic!("error in save"),
		}
		
		let mut from = String::from("algorithm_visualizer/src/save/tmp/");
		from.push_str(t);
		from.push_str(".dot");
		let mut path_from = get_absolute("algorithm_visualizer");
		path_from.push_str(&from);
		let content = read_to_string(path_from).unwrap();
		let mut location = String::from("algorithm_visualizer/src/save/dot/");
		location.push_str(t);
		location.push_str(".dot");
		
		let mut path = get_absolute("algorithm_visualizer");
		path.push_str(&location);
		let output = PathBuf::from(path);
		
		let mut file = File::create(output).expect("failed to create file");
	    file.write_all(content.as_bytes()).expect("failed to write to file");
	    match t 
		{
			"tree"     => message("save success","successfully saved the tree as dot"),
				
			"ugraph"   => message("save success","successfully saved the undirected graph without cost as dot"),

			"ucgraph"  => message("save success","successfully saved the undirected graph with cost as dot"),

			"digraph"  => message("save success","successfully saved the directed graph without cost as dot"),
			
			"dicgraph" => message("save success","successfully saved the directed graph with cost as dot"),
			
			_          =>panic!("error in save"),
		}
	}
}


pub fn save_png(t: &str)
{
	unsafe 
	{
		match t 
		{
			"tree"     => { if BTREE == None 
				{
					message("not intialized","nothing to save");
					return
				}},
				
			"ugraph"   =>{ if UGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"ucgraph"  => { if UCGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"digraph"  => { if DIGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			"dicgraph" =>{ if DICGRAPH == None 
				{
					message("not intialized","nothing to save");
					return
				}},
			
			_          =>panic!("error in save"),
		}
		let mut location = String::from("algorithm_visualizer/src/save/tmp/");
		location.push_str(t);
		location.push_str(".dot");
		let mut path = get_absolute("algorithm_visualizer");
		path.push_str(&location);
	
		let mut output =  String::from("/algorithm_visualizer/src/save/image/");
		output.push_str(t);
		output.push_str(".png");
		let mut path_out = get_absolute("algorithm_visualizer");
		path_out.push_str(&output);
		
		let _com = Command::new("dot")
	                        .arg("-Tpng")
	                        .arg(path)
	                        .arg("-o")
	                        .arg(path_out.clone())
	                        .output()
	                        .expect("failed to execute process");                
	    match t 
		{
			"tree"     => message("save success","successfully saved the tree as image"),
				
			"ugraph"   => message("save success","successfully saved the undirected graph without cost as image"),

			"ucgraph"  => message("save success","successfully saved the undirected graph with cost as image"),

			"digraph"  => message("save success","successfully saved the directed graph without cost as image"),
			
			"dicgraph" => message("save success","successfully saved the directed graph with cost as image"),
			
			_          =>panic!("error in save"),
		}
	}
}
