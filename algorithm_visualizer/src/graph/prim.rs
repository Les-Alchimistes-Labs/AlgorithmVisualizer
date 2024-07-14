use crate::UCGRAPH;
use crate::GTK::graphs::graph_c::paint_ucgraph;
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

pub fn prim(start : usize, notebook : &mut Notebook)
{
	unsafe 
	{
		let g = UCGRAPH.clone().unwrap();
		let mut dist = vec![i32::MAX;g.order as usize];
		let mut p = vec![-2 ; g.order as usize];
		let mut m = vec![true ; g.order as usize];
		let mut colors = vec![0;g.order as usize];
		let mut n = 1;
		let mut x = start as i32;
		m[x as usize] = true;
		p[x as usize] = -1;
		dist[x as usize] = 0;
		while x != -2 && n<g.order
		{
			m[x as usize] = false;
			colors[x as usize] = 2;
			for i in 0..g.adjlists[x as usize].len()
			{
				let y = g.adjlists[x as usize][i];
				let cost;
				if  x<y 
				{
					cost = g.costs.get(&(x,y)).unwrap();
				}
				else
				{
					cost = g.costs.get(&(y,x)).unwrap();
				}
				if cost < &dist[y as usize] && m[y as usize]
				{
					dist[y as usize] = *cost;
					p[y as usize] = x;
					let edges = get_edges(&p,&dist);
					paint_ucgraph("prim",notebook,colors.clone(),edges,vec![("dist",dist.clone()),("p",p.clone())]);
				}
			}
			colors[x as usize] = 1;
			x = choose_min(&dist,&m);
			n+=1;
		}
	}
}

fn get_edges(p : &Vec<i32> ,dist : &Vec<i32>) -> Vec<(i32,i32)>
{
	let mut result =  vec![];
	for i in 0..p.len()
	{
		if p[i] >=0 && dist[i] != i32::MAX 
		{
			result.push((i as i32,p[i]));
		} 
	}
	result
}
