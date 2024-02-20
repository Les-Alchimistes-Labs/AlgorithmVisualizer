use std::{
    cell::Cell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt,
    hash::{Hash, Hasher},
};

pub fn dijkstra(src: &Vertex<'_>, adjlist: &HashMap<&Vertex<'_>, Vec<(&Vertex<'_>,usize)>>)
{
    src.dist.set(0);
    //Fill the binary heap, verticees with the smallest distance go first
    let mut vst = BinaryHeap::new();
    adjlist.keys().for_each(|t| vst.push(*t));
    //we visit the vertices with the smallest distance first, this is what makes dijstra a greedy
    //algorithmes
    while let Some(t) = vst.pop()
    {
        if let Some(nb) = adjlist.get(t)
        {
            for (i, c) in nb
            {
                let n_dist = t.dist.get() + c;
                if n_dist < i.dist.get()
                {
                    i.dist.set(n_dist);
                }
            }
            //When changing a vertex distance, the binary heap doesn't update the position of the
            //vertex. That's why we create a new heap with the right order
            let mut n_heap = BinaryHeap::new();
            vst.iter().for_each(|j| n_heap.push(*j));
            vst = n_heap;
        }
    }
}

fn main()
{
    let a = Vertex::new("a");
    let b = Vertex::new("b");
    let c = Vertex::new("c");
    let d = Vertex::new("d");
    let e = Vertex::new("e");
    //let f = Vertex::new("f");
    //let g = Vertex::new("g");
    //let h = Vertex::new("h");
    //let i = Vertex::new("i");
    //let j = Vertex::new("j");

    //A map from vertices to their adjacent vertices includingcosts
    let mut adjlist= HashMap::new();
    adjlist.insert(&a, vec![(&b,10),(&d,5)]);
    adjlist.insert(&b, vec![(&d,2),(&c,1)]);
    adjlist.insert(&c, vec![(&e,4)]);
    adjlist.insert(&d, vec![(&b,3),(&c,9),(&e,2)]);    
    adjlist.insert(&e, vec![(&a,7),(&c,6)]);
    //adjlist.insert(&f, vec![(&a,5),(&e,4)]);
    //adjlist.insert(&g, vec![(&f,5),(&d,8)]);
    //adjlist.insert(&h, vec![(&g,3),(&f,10)]);
    //adjlist.insert(&i, vec![(&h,7),(&g,2)]);
    //adjlist.insert(&j, vec![(&i,5)]);

    dijkstra(&a, &adjlist);
    adjlist.keys().for_each(|v| println!("{}", v));
}

#[derive(Eq)]
struct Vertex<'a> {
    nom: &'a str,
    dist: Cell<usize>,
}

impl<'a> Vertex<'a>{
    fn new(nom: &'a str) -> Vertex<'a>{
        Vertex{
            nom,
            dist: Cell::new(usize::max_value()),
        }
    }
}

impl<'a> Hash for Vertex<'a>{
    fn hash<Hashh: Hasher>(&self, s: &mut Hashh){
        self.nom.hash(s);
    }
}

//Since this Vertex will be put in a priority queue wherethe vertices with the smallest distance
//should be processed first, cmp returns GT if self.dist().get < other.distances.get()
impl<'a> Ord for Vertex<'a>{
    fn cmp(&self, oth: &Vertex<'a>) -> Ordering{
        oth.dist.get().cmp(&self.dist.get())
        }
}

impl<'a> PartialOrd for Vertex<'a>{
    fn partial_cmp(&self, oth: &Vertex<'a>) -> Option<Ordering>{
        Some(self.cmp(oth))
        }
}

impl<'a> PartialEq for Vertex<'a>{
    fn eq(&self, oth: &Vertex<'a>) -> bool{
        self.nom == oth.nom
        }
}

impl<'a> fmt::Display for Vertex<'a>{
    fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result{
        write!(fm, "nom: {}, dist: {}",self.nom,self.dist.get())
        }
}

