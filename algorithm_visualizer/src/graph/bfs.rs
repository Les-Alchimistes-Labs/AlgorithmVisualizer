use crate::GTK::graphs::graph::paint_ugraph;
use crate::UGRAPH;
use	crate::UCGRAPH;
use gtk::Notebook;


pub fn  bfs_ugraph(start : i32,notebook :&mut Notebook)
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
			paint_ugraph("bfs",notebook,tmp,-1);
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

