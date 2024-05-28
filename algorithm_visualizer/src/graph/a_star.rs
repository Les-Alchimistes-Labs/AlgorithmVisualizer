use crate::DICGRAPH;
use crate::GTK::graphs::digraph_c::paint_dicgraph;
use gtk::Notebook;

fn heuristic_e(start : i32 , end :i32 ) -> i32
{
	(end - start).abs()
}

pub fn a_star(start : usize , end : usize, notebook :&mut Notebook)
{
	unsafe
	{
		let g = DICGRAPH.clone().unwrap();
		let order = g.order as usize;
		let mut close = vec![];
		let mut open = vec![start as i32];
		let mut p = vec![-2; order]; 
		let mut dist = vec![i32::MAX ;order];
		let mut h = vec![i32::MAX ;order];
		let mut f = vec![i32::MAX ;order];
		dist[start] = 0;                                              
		h[start] = heuristic_e(start as i32,end as i32);
		f[start] = h[start];
		p[start] = -1;
		while !open.is_empty()
		{
			let current = *open.iter().min_by_key(|&x| f[*x as usize]).unwrap();
			if current == end as i32
			{
				let mut colors = get_colors(close.clone());
				colors[end] =2;
				paint_dicgraph("A*",notebook,colors,vec![(current,end as i32)],vec![("dist",dist.clone()),("p",p.clone()), ("H",h.clone()),("F",f.clone())]);
				return 
			}
			open.retain(|&x| x != current);
			close.push(current);
			for i in 0..g.adjlists[current as usize].len()
			{
				let y = g.adjlists[current as usize][i] as usize;
				let tmp = dist[current as usize] + g.costs.get(&(current as i32, y as i32)).unwrap();
				if !open.contains(&(y as i32)) || tmp <dist[y]
				{
					p[y] = current;
					dist[y] = tmp;
					h[y] = heuristic_e(y as i32, end as i32);
					f[y] = dist[y] + h[y];
					if !open.contains(&(y as i32))
					{
						open.push(y as i32);
					}
					let mut colors = get_colors(close.clone());
					colors[current as usize] = 2;
					paint_dicgraph("A*",notebook,colors,vec![(current,y as i32)],vec![("dist",dist.clone()),("p",p.clone()), ("H",h.clone()),("F",f.clone())]);
				}
			}
		}
		
	}
}
fn get_colors(close : Vec<i32>) -> Vec<i32>
{
	unsafe
	{
		let g = DICGRAPH.clone().unwrap();
		let mut result = vec![0;g.order as usize];
		for i in 0..close.len()
		{
			result[close[i] as usize] = 1;
		}
		result
	}
}
