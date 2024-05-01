use crate::uGraph;
use crate::ucGraph;
use crate::GTK::graphs::graph::paint_ugraph;
use crate::UGRAPH;
use	crate::UCGRAPH;
use gtk::Notebook;

pub fn dfs_ugraph(x:i32,M:&mut Vec<bool>, mark :bool,notebook : &mut Notebook)
{
	unsafe 
	{   
		let g = UGRAPH.clone().unwrap();         
		M[x as usize]= mark;
		for i in 0..(g.adjlists[x as usize].len())
		{
			if ! M[g.adjlists[x as usize][i] as usize]
			{
				paint_ugraph("dfs",notebook,g.adjlists[x as usize][i],-1);
				dfs_ugraph(g.adjlists[x as usize][i],M,mark,notebook);
			}
		}
	}
}
