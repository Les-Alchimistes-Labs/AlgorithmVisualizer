use crate::DICGRAPH;
use crate::GTK::graphs::digraph_c::paint_dicgraph;
use gtk::Notebook;


fn choose_min(dist :&Vec<i32>, m :&Vec<bool>) -> i32
{
	let mut result = i32::MAX;
	let mut index = -2;
	for i in 0..dist.len()
	{
		if result> dist[i] && m[i]
		{
			result = dist[i];
			index= i as i32;
		}
	}
	index
}



pub fn dijkstra(start: usize, notebook : &mut Notebook)
{
	unsafe
	{
		let g = DICGRAPH.clone().unwrap();
		let mut dist = vec![i32::MAX ; g.order as usize];
		let mut p = vec![-2; g.order as usize];
		let mut m = vec![true ;g.order as usize];
		let mut x = start as i32;
		let mut n = 1;
		dist[start] = 0;
		p[start] = -1;
		let mut colors = vec![0;g.order as usize];
		while x != (-2) && n <g.order
		{
			m[x as usize]= false;
			colors[x as usize] = 2;
			for i in 0..g.adjlists[x as usize].len() 
			{
				let y = g.adjlists[x as usize][i];
				if dist[x as usize] + g.costs.get(&(x,y)).unwrap()< dist[y as usize]
				{
					dist[y as usize]= dist[x as usize] + g.costs.get(&(x,y)).unwrap();
					p[y as usize]=x
				}
				paint_dicgraph("dijkstra",notebook,colors.clone(),vec![(x,y)],vec![("dist",dist.clone()),("p",p.clone())]);
			}
			colors[x as usize] = 1;
			x = choose_min(&dist,&m);
			n+=1;
		}
		dbg!(dist);
		dbg!(p);
	}
}
