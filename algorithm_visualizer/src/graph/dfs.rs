use crate::GTK::graphs::graph::paint_ugraph;
use crate::GTK::graphs::digraph::paint_digraph;
use crate::UGRAPH;
use crate::DIGRAPH;
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
				let mut colors = vec![0;g.order as usize];
				for j in 0..m.len()
				{
					if m[j] 
					{
						colors[j]=1;
					}
				}
				colors[g.adjlists[x as usize][i] as usize] = 2;
				paint_ugraph("dfs",notebook,colors,vec![(g.adjlists[x as usize][i],x)]);
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
				let mut colors = vec![0;g.order as usize];
				for j in 0..m.len()
				{
					if m[j] 
					{
						colors[j]=1;	
					}
				}
				colors[g.adjlists[x as usize][i] as usize] = 2;
				paint_digraph("dfs",notebook,colors,vec![(g.adjlists[x as usize][i],x)]);
				dfs_digraph(g.adjlists[x as usize][i],m,mark,notebook);
			}
		}
	}
}
