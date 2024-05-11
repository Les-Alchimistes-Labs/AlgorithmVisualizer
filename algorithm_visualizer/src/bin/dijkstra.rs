pub struct graph_path {
    pub nodes: Vec<Vec<(usize,usize)>>,
}

impl graph_path {
    pub fn choose_min(&self, dist: Vec<i32>, bo: Vec<bool>) -> i32
    {
        let mut min = i32::MAX;
        let mut min2 = 0;
        for i in 0..dist.len()
        {
            if dist[i] < min && bo[i] == false
            {
                min = dist[i];
                min2 = i as i32;
            }
        }
        return min2;
    }

    pub fn dijkstra(&self, src: i32) -> (Vec<i32>, Vec<i32>) {
        let mut dist = vec![i32::MAX; self.nodes.len()];
        dist[src as usize] = 0;
        let mut pp = vec![-2; self.nodes.len()];
        pp[src as usize] = -1;
        let mut bo = vec![true; self.nodes.len()];
        let mut x = src;
        let mut n = 1;
        while bo.iter().filter(|x| **x).count() != 0 && n < self.nodes.len()
        {
            x = self.choose_min(dist.clone(), bo.clone());
            bo[x as usize] = false;
            for i in 0..self.nodes[x as usize].len()
            {
                for j in self.nodes[i as usize].iter()
                {
                    if dist[j.0 as usize] > dist[x as usize] + j.1 as i32
                    {
                        dist[j.0 as usize] = dist[x as usize] + j.1 as i32;
                        pp[j.0 as usize] = x;
                    }
                }
            }
            n += 1;
        }
        return (dist, pp);
    }
}
fn main()
{
    let graph = graph_path {
        nodes: vec![
            vec![(1, 5), (2, 3)],
            vec![(0, 5), (2, 2), (3, 6)],
            vec![(0, 3), (1, 2), (3, 7)],
            vec![(1, 6), (2, 7)],
        ],
    };

    let source = 0;

    let (dist, _pp) = graph.dijkstra(source);
    
    println!("{:?}", dist);
    println!("Le résultat a la main est: [0, 5, 2, 6]");
}
