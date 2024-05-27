use crate::GTK::utilities::*;

	
pub fn list_info()
{
	message("List tab information","In the left part :
											 \n\tCombo box: choose a sorting algorithm
											 \n\n\tInformation : gives information on the choosen algorithm
											 \n\n\tAdd: add the written number in the list
											 \n\n\tRemove: remove the written number in the list
											 \n\n\tReset: remove every number in the list
											 \n\n\tSort the list: sort the list using the selected algorithm
											 \n\n\tRefresh: remove every tabs and show the current state of the list
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your \n\tkeyboard");
	return     
}

pub fn tree_info()
{
	message("Tree tab information","In the left part :
											 \n\tCombo box: choose a searching algorithm
											 \n\n\tInformation : gives information on the choosen algorithm
											 \n\n\tAdd: add the written number in the tree
											 \n\n\tRemove: remove the written number in the tree
											 \n\n\tReset: remove every node in the tree
											 \n\n\tSearch: search in the tree using the selected algorithm
											 \n\n\tRefresh: remove every tabs and show the current state of the tree
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your \n\tkeyboard");
	return     
}

pub fn graph_info()
{
	message("Graph tab information","In the left part:
											 \n\tCombo box: choose a searching algorithm
											 \n\n\tInformation : gives information on the choosen algorithm
											 \n\nEdges:
											 \n\n\tAdd: add an edge using the input given by the user 
											 \n\n\tRemove: remove an edge using the input given by the user 
											 \n\nVertices:
											 \n\n\tAdd: add a vertice in the graph
											 \n\n\tRemove: remove a vertice in the graph with all related edges 
											 \n\n\tReset: remove every vertices in the graph
											 \n\n\tSearch: search in the graph using the selected algorithm
											 \n\n\tRefresh: remove every tabs and show the current state of the graph
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your \n\tkeyboard" );
	return 
}


pub fn credits()
{
	message("Credits","This application was realised in the context of a project for the S4 at EPITA.
											 \nRealised by: 
											 \n\t- Victor Tang
											 \n\t- Jayson Vanmarcke
											 \n\t- Olivier Bensemhoun
											 \n\t- Sacha Layani");
	return	
}
