pub mod dfs{
    use std::usize;
    use std::collections::HashSet;
    use std::collections::VecDeque;
    
    //structure of a graph with list of adjacence

    pub struct Graph
    {
        pub edges: Vec<Vec<usize>>,
        pub vertices: usize,
    }

    //method for creating new graph

    impl Graph
    {
        pub fn new(vertices: usize) -> Self
        {
            Graph
            {
                edges: vec![vec![]; vertices],
                vertices,
            }
        }

        pub fn add_edge(&mut self, x: usize, y: usize)
        {
            self.edges[x].push(y);
            self.edges[y].push(x);
        }
        
        pub fn bfs(&self, begin: usize) ->Vec<usize>
        {
            let mut v = HashSet::new();
            let mut q = VecDeque::new();
            let mut vec = vec![];

            v.insert(begin);
            q.push_back(begin);

            while let Some(x)  = q.pop_front()
            {
                vec.push(x);
                for &i in &self.edges[x]
                {
                    if !v.contains(&i)
                    {
                        v.insert(i);
                        q.push_back(i);
                    }
                }
            }
            vec
        }
    }
}

pub fn main(){
    use crate::dfs::Graph;

    let mut gra = Graph::new(5);
    gra.add_edge(0,1);
    gra.add_edge(0,2);
    gra.add_edge(1,3);
    gra.add_edge(2,4);
    println!("0 -> 1");
    println!("0 -> 2");
    println!("1 -> 3");
    println!("2 -> 4");
    let ar = gra.bfs(0);
    println!("{:?}",ar);
}
