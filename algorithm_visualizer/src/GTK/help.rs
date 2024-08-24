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
											 \n\t\tCombo box: choose a searching algorithm
											 \n\n\t\tInformation : gives information on the choosen algorithm
											 \n\n\tEdges:
											 \n\n\t\tAdd: add an edge using the input given by the user 
											 \n\n\t\tRemove: remove an edge using the input given by the user 
											 \n\n\tVertices:
											 \n\n\t\tAdd: add a vertice in the graph
											 \n\n\t\tRemove: remove a vertice in the graph with all related edges 
											 \n\n\t\tReset: remove every vertices in the graph
											 \n\n\t\tSearch: search in the graph using the selected algorithm
											 \n\n\t\tRefresh: remove every tabs and show the current state of the graph
											 \nIn the right part:
											 \n\tto see as an animation : select a tab and press left or right arrow on your keyboard\t " );
	return 
}


pub fn credits()
{
	message("Credits","\tThis application was realised in the context of a project for the S4 at EPITA.\t
											 \n\tRealised by: 
											 \n\t\t- Victor Tang
											 \n\t\t- Jayson Vanmarcke
											 \n\t\t- Olivier Bensemhoun
											 \n\t\t- Sacha Layani");
	return	
}
