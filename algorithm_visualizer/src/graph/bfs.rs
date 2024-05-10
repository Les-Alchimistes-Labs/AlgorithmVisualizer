use crate::GTK::graphs::graph::paint_ugraph;
use crate::GTK::graphs::digraph::paint_digraph;
use crate::UGRAPH;
use crate::DIGRAPH;
use gtk::Notebook;


pub fn bfs_ugraph(start : i32,notebook :&mut Notebook)
{
	unsafe
	{
		let g = UGRAPH.clone().unwrap();		
		let mut q = vec![];                                                               
		let mut m = vec![false ;g.order as usize];
		q.push(start);
		while q.len() !=  0 
		{
			let tmp = q.remove(0);
			m[tmp as usize] = true;
			let mut colors = vec![0;g.order as usize];
			for j in 0..m.len()
			{
				if m[j] 
				{
					colors[j]=1;	
				}
			}
			colors[tmp as usize] = 2;
			paint_ugraph("bfs",notebook,colors,vec![]);
			for i in 0..(g.adjlists[tmp as usize].len())
			{
				if ! m[g.adjlists[tmp as usize][i] as usize]
				{
					q.push(g.adjlists[tmp as usize][i])
				}
			}
		}
	}
	
}

pub fn bfs_digraph(start : i32,notebook :&mut Notebook)
{
	unsafe
	{
		let g = DIGRAPH.clone().unwrap();
		
		let mut q = vec![];
		let mut m = vec![false ;g.order as usize];
		q.push(start);
		while q.len() !=  0 
		{
			let tmp = q.remove(0);
			m[tmp as usize] = true;
			let mut colors = vec![0;g.order as usize];
			for j in 0..m.len()
			{
				if m[j] 
				{
					colors[j]=1;	
				}
			}
			colors[tmp as usize] = 2;
			paint_digraph("bfs",notebook,colors,vec![]);
			for i in 0..(g.adjlists[tmp as usize].len())
			{
				if ! m[g.adjlists[tmp as usize][i] as usize]
				{
					q.push(g.adjlists[tmp as usize][i])
				}
			}
		}
	}
	
}

