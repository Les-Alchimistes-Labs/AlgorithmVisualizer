pub mod dfs{
    use std::usize;
    use std::{
        collections::HashSet,
        io::{stdin, stdout, Write},
    };
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
        }

        pub fn dfs(&self, x: usize, v: &mut HashSet<usize>) -> Vec<usize>
        {
            v.insert(x);
            //println!("Noeud visité {}",x)
            let mut vn = vec![x];

            for &v2 in &self.edges[x]
            {
                if !v.contains(&v2)
                {
                    vn.extend(self.dfs(v2, v));
                }
            }
            vn
        }
    }
}

pub fn main(){
    use crate::dfs::Graph;
    use std::collections::HashSet;

    let mut gra = Graph::new(7);
    gra.add_edge(0,1);
    gra.add_edge(0,2);
    gra.add_edge(2,0);
    gra.add_edge(2,3);
    gra.add_edge(1,3);
    gra.add_edge(3,4);
    gra.add_edge(3,5);
    gra.add_edge(5,6);
    println!("2 -> 0");
    println!("0 -> 1");
    println!("2 -> 3");
    println!("2 -> 4");
    println!("4 -> 5");
    println!("2 -> 6");
    let mut v = HashSet::new();
    let ar = gra.dfs(2, &mut v);
    println!("{:?}",ar);
}

