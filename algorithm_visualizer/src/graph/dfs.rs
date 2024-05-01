use crate::GTK::graphs::graph::paint_ugraph;
use crate::UGRAPH;
use	crate::UCGRAPH;
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
