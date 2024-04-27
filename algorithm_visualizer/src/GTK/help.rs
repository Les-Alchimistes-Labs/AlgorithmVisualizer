use gtk::prelude::*;
use gtk::{Window, MessageDialog, DialogFlags, MessageType, ButtonsType};
	
pub fn list_info()
{
	let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "In the left part :
											 \n\tCombo box: choose a sorting algorithm
											 \n\n\tAdd: add the written number in the list
											 \n\n\tRemove: remove the written number in the list
											 \n\n\tReset: remove every number in the list
											 \n\n\tSort the list: sort the list using the selected algorithm
											 \n\n\tRefresh: remove every tabs and show the current state of the list
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your \n\tkeyboard");
	dialog.set_title("List tab information");
	dialog.run();
	dialog.close();
	return     
}

pub fn tree_info()
{
	let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "In the left part :
											 \n\tCombo box: choose a searching algorithm
											 \n\n\tAdd: add the written number in the tree
											 \n\n\tRemove: remove the written number in the tree
											 \n\n\tReset: remove every node in the tree
											 \n\n\tSearch: search in the tree using the selected algorithm
											 \n\n\tRefresh: remove every tabs and show the current state of the tree
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your \n\tkeyboard");
	dialog.set_title("Tree tab information");
	dialog.run();
	dialog.close();
	return     
}

pub fn graph_info()
{
	
	let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "nothing for now");
	dialog.set_title("Graph tab information");
	dialog.run();
	dialog.close();
	return 
}


pub fn credits()
{
	 let dialog = MessageDialog::new(None::<&Window>,
											 DialogFlags::MODAL,
											 MessageType::Info,
											 ButtonsType::Close,
											 "This application was realised in the context of a project for the S4 at EPITA.
											 \nrealised by: 
											 \n\t- Victor Tang
											 \n\t- Jayson Vanmarcke
											 \n\t- Olivier Bensemhoun
											 \n\t- Sacha Layani");
	dialog.set_title("credits");
	dialog.run();
	dialog.close();
	return	
}
