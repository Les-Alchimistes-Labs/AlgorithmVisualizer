use gtk::prelude::*;
use std::fs::read_to_string;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter, ResponseType, Window
	, MessageDialog, DialogFlags, MessageType, ButtonsType};
use crate::CURRENT_LIST;

pub fn open_list() {
    // Create a file chooser dialog
    let file_chooser = FileChooserDialog::new(
        Some("Open List"),
        None::<&Window>,      
        FileChooserAction::Open,
    );
    file_chooser.add_buttons(&[
            ("Cancel", ResponseType::Cancel),
            ("Open", ResponseType::Ok),
        ]);

    // Add filters to the file chooser dialog
    let filter = FileFilter::new();
    filter.add_pattern("*.txt");
    filter.set_name(Some("Text files"));
    file_chooser.add_filter(&filter);

    // Connect the "response" signal of the file chooser dialog
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
							// Convert PathBuf to String using to_string_lossy()
							let path_string: String = path_buf.to_string_lossy().into_owned();
							opened_list(path_string);
				            
				        }
				        None => 
				        {
				            println!("No path provided");
				        }
					}
				}
			}
			_ => {}
		}
			dialog.close();
		}
	);
    // Run the file chooser dialog
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
					let dialog = MessageDialog::new(None::<&Window>,
												DialogFlags::MODAL,
												MessageType::Info,
												ButtonsType::Close,
											"not a correct list");
					dialog.run();
					dialog.close();
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
