use gtk::prelude::*;
use gtk::{Window, MessageDialog, DialogFlags, MessageType, ButtonsType };

use std::process::Command;


use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;


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

pub fn parser(to_parse :&str) -> i32
{
	match to_parse.parse::<i32>()
	{
		Ok(parsed) => {parsed},
		Err(_e) => { message("incorrect input", "not a number");
					i32::MAX
					},
	}
}

pub fn message(title : &str, content : &str)
{
	let dialog = MessageDialog::new(None::<&Window>,
								DialogFlags::MODAL,
								MessageType::Info,
								ButtonsType::Close,
								content);
	if title != ""
	{
		dialog.set_title(title);
	}
	dialog.run();
	dialog.close();
	return	
}

pub fn save_dot_tmp(content : String, t : &str) 
{
	let mut location = String::from("algorithm_visualizer/src/save/tmp/");
	location.push_str(t);
	location.push_str(".dot");
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(&location);
	let output = PathBuf::from(path);
	
	let mut file = File::create(output).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");
}

pub fn save_png_tmp(t :&str)
{
	let mut location = String::from("algorithm_visualizer/src/save/tmp/");
	location.push_str(t);
	location.push_str(".dot");
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(&location);
	
	let mut output = String::from("/algorithm_visualizer/src/save/tmp/");
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
}
