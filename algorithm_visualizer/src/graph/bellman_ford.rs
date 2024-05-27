use crate::DICGRAPH;
use crate::GTK::graphs::digraph_c::paint_dicgraph;
use gtk::Notebook;


pub fn bellman_ford(start : usize , notebook : &mut Notebook)
{
	unsafe 
	{
		let g = DICGRAPH.clone().unwrap(); 
		let mut dist = vec![i32::MAX ; g.order as usize];
		let mut p = vec![-2 ; g.order as usize];
		let mut colors = vec![0; g.order as usize];
		let mut n = g.order;
		let mut changes = true;
		dist[start] = 0;
		p[start] = -1;
		while n>0 && changes
		{
			changes = false;
			for x in 0..g.order
			{
				if dist[x as usize] != i32::MAX
				{
					colors[x as usize] = 2;
					for i in 0..g.adjlists[x as usize].len()
					{
						let y = g.adjlists[x as usize][i];
						if dist[x as usize] + g.costs.get(&(x,y)).unwrap() < dist[y as usize]
						{
							dist[y as usize] = dist[x as usize ] + g.costs.get(&(x,y)).unwrap();
							p[y as usize] = x ;
							changes = true;
						}
						paint_dicgraph("bellman ford",notebook,colors.clone(),vec![(x,y)],vec![("dist",dist.clone()),("p",p.clone())]);
					}
					colors[x as usize] = 1;
				}
			}
			n-=1;
		}
	}
}
