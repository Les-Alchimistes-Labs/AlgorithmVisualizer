use gtk::prelude::*;
use std::fs::read_to_string;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter, ResponseType, Window};

use crate::{CURRENT_LIST,BTREE,DICGRAPH};
use crate::tree::insert::*;
use crate::dicGraph;
use crate::GTK::utilities::*;

pub fn open_list() {
    let file_chooser = FileChooserDialog::new(
        Some("Open List"),
        None::<&Window>,      
        FileChooserAction::Open,
    );
    file_chooser.add_buttons(&[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Ok),
        ]);

    let filter = FileFilter::new();
    filter.add_pattern("*.txt");
    filter.set_name(Some("Text files"));
    file_chooser.add_filter(&filter);

    file_chooser.connect_response(move |dialog, response| 
    {
        match response 
        {
            ResponseType::Ok => 
            {
                if let Some(file) = dialog.file() 
                {
					let path = file.path();
                    match path
                    {
						Some(path_buf) => 
						{
							let path_string: String = path_buf.to_string_lossy().into_owned();
							opened_list(path_string);   
				        }
				        None => 
				        {
				            message("no path","no path provided");
				        }
					}
				}
			}
			_ => {}
		}
			dialog.close();
		}
	);
    file_chooser.run();
}
fn opened_list(a :String)
{
	let tmp = read_to_string(a);
	unsafe
    {
		CURRENT_LIST=vec![];		
	}
    
    let  strings =  tmp.unwrap();
    for line in strings.lines()
    {
		match line.parse::<i64>() 
		{
			Ok(parsed_i64) => 
			{
				unsafe
				{
					CURRENT_LIST.push(parsed_i64);
				}
				
			}
			Err(_e) => 
			{
				unsafe
				{
					CURRENT_LIST=vec![];
					message("incorrect list","not a correct list");
					return
				}
			}
		}
	}
	unsafe
	{		
		dbg!(&CURRENT_LIST);
	}
}

pub fn open_dot(op : i32) 
{
    let file_chooser = FileChooserDialog::new(
        Some("Open tree"),
        None::<&Window>,      
        FileChooserAction::Open,
    );
    file_chooser.add_buttons(&[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Ok),
        ]);

    let filter = FileFilter::new();
    filter.add_pattern("*.dot");
    filter.set_name(Some("DOT files"));
    file_chooser.add_filter(&filter);

    file_chooser.connect_response(move |dialog, response| 
    {
        match response 
        {
            ResponseType::Ok => 
            {
                if let Some(file) = dialog.file() 
                {
					let path = file.path();
                    match path
                    {
						Some(path_buf) => 
						{
							let path_string: String = path_buf.to_string_lossy().into_owned();
							match op 
							{
								0 => open_tree(path_string),
								1 => open_dicgraph(path_string),
								_ => message("error","error encountered"),
							}
				            
				        }
				        None => 
				        {
				            message("no path","no path provided");
				        }
					}
				}
			}
			_ => {}
		}
			dialog.close();
		}
	);
    file_chooser.run();
}

fn open_tree(a :String)
{
	unsafe
	{
		let tmp = read_to_string(a);
		BTREE =None;	
	    let  strings =  tmp.unwrap();
	    for line in strings.lines()
	    {
			let mut tmp = line.clone().to_string();
			let c =tmp.remove(0);
			if c =='/'
			{
				if tmp.remove(0) == '/'
				{
					for number in tmp.split_whitespace()
					{
						match number.parse::<i32>()
						{
							Ok(parsed) => {  create(parsed) },
							
							Err(_e) => {
							BTREE=None;
							message("incorrect tree","not a correct binary tree");
							return }
						}
					}
					dbg!(&BTREE);
					return;
				}
			}
		}
		message("no prefix comment","the tree doesn't contain a prefix search comment");
		return	
	}
}
fn open_dicgraph(a :String)
{
	unsafe 
	{
		let file = read_to_string(a).unwrap();
		DICGRAPH = None;
		let mut it = file.lines();
		let mut line = it.next();
		if !line.is_some()
		{
			message("incorrect graph","not a correct directed graph with cost");
			return 
		}
		let mut words = line.unwrap().split_whitespace();
		let mut word = words.next();
		while word != Some("//")
		{
			if !word.is_some()
			{
				message("incorrect graph","not a correct directed graph with cost");
				return 
			}
			word = words.next();
		}
		word = words.next();
		if !word.is_some()
		{
			message("incorrect graph","not a correct directed graph with cost");
			return 
		}
		let order = parser(word.unwrap(),"DICGRAPH");
		let mut start;
		let mut end;
		let mut cost;
		let mut g = dicGraph::new(order);
		line = it.next();
		dbg!(&line);
		if !line.is_some()
		{
			message("incorrect graph","not a correct directed graph with cost");
			return 
		}
		while line.unwrap().split_whitespace().next() != Some("n0")
		{
			words = line.unwrap().split_whitespace();
			word = words.next();
			while word != Some("//")
			{
				word = words.next();
			}
			word = words.next();
			if !word.is_some()
			{
				message("incorrect graph","not a correct directed graph with cost");
				return 
			}
			start = parser(word.unwrap(),"DICGRAPH");
			word = words.next();
			if !word.is_some()
			{
				message("incorrect graph","not a correct directed graph with cost");
				return 
			}
			end = parser(word.unwrap(),"DICGRAPH");
			word = words.next();
			if !word.is_some()
			{
				message("incorrect graph","not a correct directed graph with cost");
				return 
			}
			cost = parser(word.unwrap(),"DICGRAPH");
			g.adjlists[start as usize].push(end);
			g.costs.insert((start,end),cost);
			line = it.next();
			if !line.is_some()
			{
				message("incorrect graph","not a correct directed graph with cost");
				return 
			}
			println!("ah");
		} 
		DICGRAPH = Some(g);
		dbg!(&DICGRAPH);	
	}
}
fn parser(to_parse :&str, T : &str) -> i32
{
	match to_parse.parse::<i32>()
	{
		Ok(parsed) => {return parsed},
		Err(_e) => { 	let title;
						let content;
						match  T 
						{
							"BTREE" => {title = "incorrect tree";
										content = "the tree doesn't contain a prefix search comment"},
							"DICGRAPH" => {title = "incorrect graph";
											content = "not a correct directed graph with cost"},
							_          => {title = "error";
											content = "error encountered";},
						}
						
						message(title,content);
						return i32::MAX },
	}
}
