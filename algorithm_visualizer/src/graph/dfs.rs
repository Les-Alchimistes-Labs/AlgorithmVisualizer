use crate::GTK::graphs::graph::paint_ugraph;
use crate::GTK::graphs::digraph::paint_digraph;
use crate::UGRAPH;
use	crate::UCGRAPH;
use crate::DIGRAPH;
use	crate::DICGRAPH;
use gtk::Notebook;

pub fn dfs_ugraph(x:i32,m:&mut Vec<bool>, mark :bool,notebook : &mut Notebook)
{
	unsafe 
	{   
		let g = UGRAPH.clone().unwrap();         
		m[x as usize]= mark;
		for i in 0..(g.adjlists[x as usize].len())
		{
			if ! m[g.adjlists[x as usize][i] as usize]
			{
				paint_ugraph("dfs",notebook,g.adjlists[x as usize][i],-1);
				dfs_ugraph(g.adjlists[x as usize][i],m,mark,notebook);
			}
		}
	}
}
																							
pub fn dfs_digraph(x:i32,m:&mut Vec<bool>, mark :bool,notebook : &mut Notebook)
{
	unsafe 
	{   
		let g = DIGRAPH.clone().unwrap();         
		m[x as usize]= mark;
		for i in 0..(g.adjlists[x as usize].len())
		{
			if ! m[g.adjlists[x as usize][i] as usize]
			{
				paint_digraph("dfs",notebook,g.adjlists[x as usize][i],-1);
				dfs_digraph(g.adjlists[x as usize][i],m,mark,notebook);
			}
		}
	}
}
