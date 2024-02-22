struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
    old_p : Vec<usize>, // old pos sommet
    histogram : Vec<Option<i64>>,
    index : usize,
}


/*
fn dfs (g :&mut Graph,g2 :&mut Graph, p : usize, add : i64) -> (i8,i64)
{
    let mut pv = 0;
    let mut gp = 0;
    let mut max = 0;

    while pv < g.l[p].len()
    {
        if add < 0 && g2.l[p][pv].1 == 0
        {
            g2.l[p][pv].1 = g.l[p][pv].1;
            return (1,1)//to brake
        }
        else if g2.l[p][pv].1 + add <= g.l[p][pv].1
        {
            g2.l[p][pv].1 += add;
            return (1,add);//to brake
        }
        else if g.l[p][pv].1 - g2.l[p][pv].1 > max
        {
            gp = pv;
            max = g.l[p][pv].1 - g2.l[p][pv].1;
        }
        pv += 1;
    }
    let res = dfs( g, g2,pv,g.l[p][pv].1 - g2.l[p][pv].1);
    if res.0 == 1
    {
        g2.l[p][pv].1 +=res.1;
        return (1,res.1);
    }

    (0,0)
}


fn maximum_flow(g : &mut Graph,g2 :&mut Graph)
{
    dfs( g, g2,0,-1);

}
*/

fn set_ford_bellman(g :&mut Graph)
{
    for i in 0..g.s.len()
    {
        let a : Option<i64> = None;
        g.histogram.push(a);
    }
    g.histogram[0] = Some(0); 
}


fn ford_bellman(g :&mut Graph ) -> bool
{
        let mut end = true;
        for e in 0..g.s.len()
        {
            dbg!(e);
            if g.histogram[e] != None
            {
                let val =  g.histogram[e].unwrap();
                for i in g.l[e].iter()
                {
                    if  g.histogram[i.0] == None || 
                        g.histogram[i.0].unwrap() > val + i.1
                    {
                        //println!("{:?}",val + i.1);
                        end = false;
                        g.histogram[i.0] = Some(val + i.1);
                        //dbg!(g.histogram[i.0]);
                    }
                }
            }
        }
    end
}


fn main()
{
      let mut g1 : Graph = Graph {
        s: vec![0,1,2,3,4,5], //sograph_primmet
        l: vec![
            vec![(1,10),(5,8)],
            vec![(3,2)],
            vec![(1,1)],
            vec![(2,-2)],
            vec![(3,-1),(1,-4)],
            vec![(4,1)],
        ],  //lien
        old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        index : 0,
    };
    set_ford_bellman(&mut g1);

    while !ford_bellman(&mut g1)
    {
    }
        println!("histogram is {:?}",g1.histogram);
}
