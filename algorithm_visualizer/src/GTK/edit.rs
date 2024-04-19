use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;
use std::process::Command;


use crate::CURRENT_LIST;
use crate::BTREE;



use crate::tree::dfs::parcours_profondeur;


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
	}
}
pub fn save_list_img() 
{
	//unsafe
	//{
		//let location = "algorithm_visualizer/src/save/image/list.png";
		//let mut path = get_absolute("algorithm_visualizer");
		//path.push_str(location);
		//let output = PathBuf::from(path);
	
		//let mut file = File::create(output).expect("failed to create file");
	    //file.write_all(&data).expect("failed to write to file");
	//}	
}

pub fn save_tree_dot() 
{
	let content = dot();
	let location = "algorithm_visualizer/src/save/dot/tree.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	let output = PathBuf::from(path);
	
	let mut file = File::create(output).expect("failed to create file");
    file.write_all(content.as_bytes()).expect("failed to write to file");
}


pub fn save_tree_png()
{
	let location = "algorithm_visualizer/src/save/tmp/tree.dot";
	let mut path = get_absolute("algorithm_visualizer");
	path.push_str(location);
	
	
	let output =  "/algorithm_visualizer/src/save/image/tree.png";
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
pub fn dot() -> String 
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
				result.push_str(&get_strin(s.parse().unwrap()));
				result.push_str(&format!(" [label=\"{}\"",s));
				result.push_str("]\n");	
			}
			let mut queue = vec![];
			queue.push(BTREE.as_mut().unwrap());
			while queue.len() != 0
			{
				let node = queue.remove(0);
				if node.left != None 
				{
					let nb = node.left.as_mut().unwrap().key;
					result.push_str(&format!("n{}->n{}\n",get_strin(node.key), get_strin(nb)));
					queue.push(node.left.as_mut().unwrap());
				}
				if node.right != None 
				{
					let nb = node.right.as_mut().unwrap().key;
					result.push_str(&format!("n{}->n{}\n",get_strin(node.key), get_strin(nb)));
					queue.push(node.right.as_mut().unwrap());
				}
			}
		}
	}	
	result.push('}');
	println!("{}",&result);
	result 
}

pub fn get_strin(nb :i32) -> String
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
