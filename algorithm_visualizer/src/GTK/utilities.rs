use gtk::prelude::*;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use gtk::{ Dialog, HeaderBar, Label ,Button,Image, Notebook};
use crate::OS;

pub fn get_absolute(root: &str) ->String
{
	 let separator;
 	 match OS
	 {
	    "windows"	=> separator = '\\',
	    _			=> separator = '/',
	 }

	 let path = env::current_dir().unwrap().to_string_lossy().to_string();
	 if root == ""
	 {
		return path;
	 }
	 let mut words = vec![];
	 let mut result = String::new();
	 let mut word = String::new();
	 for c in path.chars()
	 {
		 if c ==separator
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
		result.push(separator.clone());
	}
	 result 
}

pub fn parser(to_parse :&str) -> i32
{
	match to_parse.parse::<i32>()
	{
		Ok(parsed) => {parsed},
		Err(_e) => { message("incorrect input", "not a number");
					i32::MAX },
	}
}

pub fn message(title : &str, content : &str)
{
	let dialog = Dialog::new();
	if title != ""
	{
		dialog.set_title(title);
	}
	let close_image = Image::from_icon_name(Some("window-close"), gtk::IconSize::Button.into());
	let header_bar = HeaderBar::new();
	let label = Label::new(Some(title));
	header_bar.pack_start(&label);
    header_bar.set_show_close_button(false); 
    let close_button = Button::new();
    close_button.set_image(Some(&close_image));
    header_bar.pack_end(&close_button);
    
    let mut result = String::from("\n\n");
    result.push_str(content);
    result.push_str("\n\n");
    let label2 = Label::new(Some(&result));
    dialog.set_titlebar(Some(&header_bar));
    
    dialog.content_area().add(&label2);
    
	dialog.show_all();
	close_button.connect_clicked(move |_| { dialog.close(); });
	return	
}

pub fn save_dot_tmp(content : String, t : &str) 
{
	let mut location = get_path("tmp",t);
	location.push_str(".dot");
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(&location);
	let output = PathBuf::from(path);
	let mut file = File::create(output).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");
}

pub fn save_png_tmp(t :&str)
{
	let mut location = get_path("tmp",t);
	let mut output = get_path("tmp",t);	
	location.push_str(".dot");
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(&location);
	
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

pub fn clear(notebook : &mut Notebook)
{
	let n_pages = notebook.n_pages();
	for _i in 0..n_pages
	{
		notebook.remove_page(Some(0));
	}
}

pub fn get_path(typ : &str, filename : &str) -> String
{
	let separator;
 	match OS
	{
	   "windows"	=> separator = '\\',
	   _			=> separator = '/',
	}
	let mut result = String::from("algorithm_visualizer");
	result.push(separator.clone());
	if cfg!(debug_assertions)
	{
		result.push_str("src");
		result.push(separator.clone());
	}
	result.push_str("save");
	result.push(separator.clone());
	result.push_str(typ);
	result.push(separator.clone());
	result.push_str(filename);
	result	
}
