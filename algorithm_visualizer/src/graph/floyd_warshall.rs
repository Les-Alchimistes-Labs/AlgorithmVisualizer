use crate::DICGRAPH;
use crate::GTK::graphs::digraph_c::paint_dicgraph;
use gtk::Notebook;

pub fn floyd_warshall(start : usize, notebook : &mut Notebook)
{
	unsafe 
	{
		let g = DICGRAPH.clone().unwrap();
		let n = g.order as usize;
		let mut dist = vec![vec![i32::MAX ;n]; n];
		let mut p = vec![vec![-2 ;n]; n]; 
		for x in 0..n
		{
			for y in 0..n
			{
				let cost = g.costs.get(&(x as i32,y as i32));
				if cost.is_some()
				{
					dist[x][y] = *cost.unwrap();
					p[x][y] = x as i32;
				}
			}
			dist[x][x]= 0 ;
			p[x][x] = -1;
		}
		let mut colors = vec![0;n];
		for i in 0..n
		{
			colors[i] = 2;
			for x in 0..n
			{
				if dist[x][i] != i32::MAX 
				{
					for y in 0..n
					{
						if dist[i][y] != i32::MAX && dist[x][i] + dist[i][y]<dist[x][y]
						{
							dist[x][y] = dist[x][i] + dist[i][y];
							p[x][y] =i as i32;
							if x == start
							{
								paint_dicgraph("floyd warshall",notebook,colors.clone(),vec![(i as i32,y as i32)],vec![("dist",dist[x].clone()),("p",p[x].clone())]);
							}
						}
					}
				}                                                                       
			}
			colors[i] = 1;
		}
	}	
}
