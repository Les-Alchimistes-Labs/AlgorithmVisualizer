struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
    old_p : Vec<usize>, // old pos sommet
}

/*
   let mut m: Graph = Graph{
   s: Vec::new(),
   l: Vec::new(),
   c: Vec::new(),
   }
   */

fn prim(v:&mut Graph, m:&mut Graph) ->i8
{

   if m.s.len() == 0
    {
        if v.s.len() == 0
        {
            0
        }
        else 
        {
            m.s.push(v.s[0]);
            m.old_p.push(0);
            m.l.push(vec![]);
            1
        }
    }
    else
    {
        let mut min :(usize,usize,i64) = (0,0,0);
        let mut p : usize = 0;
        while p < m.s.len()
        {
            let mut p2: usize = 0;
            while p2 < v.l[m.old_p[p]].len()
            {
                let old = v.l[m.old_p[p]][p2];
                if !m.s.contains(&v.s[old.0])
                {
                    println!("find {}",v.s[old.0]);
                    if min.0 == min.1
                    {
                        min = (p,old.0,old.1);
                    }
                    else if min.2 > old.1
                    {
                        min = (p,old.0,old.1);
                    }
                    println!("min {:?}",min);
                }
                p2+= 1;
            }
            p += 1;
        }
        if min.0 == min.1
        {
            2
        }
        else
        {
            m.s.push(v.s[min.1]);
            m.l.push(vec![(min.0,min.2)]);
            m.l[min.0].push((min.1,min.2));
            m.old_p.push(min.1);
            1
        }
    }

}







fn main()
{
let mut g1 : Graph = Graph {
    s: vec![0,1,2], //sommet
    l: vec![vec![(2,10),(1,5)],vec![(0,5),(2,3)],vec![(0,10),(1,3)]],  //lien
    old_p : Vec::new(), // old pos sommet
};
let mut g2 : Graph = Graph {
    s: vec![], //sommet
    l: Vec::new(),  //lien
    old_p : Vec::new(), // old pos sommet

};

println!("{}",prim(&mut g1, &mut g2));
println!("{:?}",g2.s);
println!("{}",prim(&mut g1, &mut g2));
println!("{:?}",g2.s);
println!("{}",prim(&mut g1, &mut g2));
println!("{:?}",g2.s);
println!("{:?}",g2.l);

println!("\n");

let mut g1 : Graph = Graph {
    s: vec![0,1,2,4,5], //sommet
    l: vec![
        vec![(2,10),(1,5)],
        vec![(0,5),(2,3)],
        vec![(0,10),(1,3)],
        vec![(1,1),(1,9)],
        vec![(2,9)]],  //lien
    old_p : Vec::new(), // old pos sommet
};
let mut g2 : Graph = Graph {
    s: vec![], //sommet
    l: Vec::new(),  //lien
    old_p : Vec::new(), // old pos sommet

};
let mut f = 0;

while f <5
{
println!("{}",prim(&mut g1, &mut g2));
println!("{:?}",g2.s);
f+=1;
}
println!("{:?}",g2.s);
println!("{:?}",g2.l);




}




