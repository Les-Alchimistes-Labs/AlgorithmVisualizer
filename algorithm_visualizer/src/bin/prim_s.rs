struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
    old_p : Vec<usize>, // old pos sograph_primmet
}

/*
   let mutut m: Graph = Graph{
   s: Vec::new(),
   l: Vec::new(),
   c: Vec::new(),
   }
   */

fn prim(graph_e:&mut Graph, graph_prim:&mut Graph) ->i8
{

    if graph_prim.s.len() == 0
    {
        if graph_e.s.len() == 0
        {
            0
        }
        else 
        {
            let mut p = 0;
            while p< graph_prim.old_p.len() && p <= graph_e.s.len()
            {
                graph_prim.old_p[p] = 0;
                p +=1;
            }
            while p <= graph_e.s.len()
            {
                graph_prim.old_p.push(0);
                p += 1;
            }
            p = 0;
            while p < graph_e.s.len()
            {
                graph_prim.s.push(graph_e.s[p]);
                graph_prim.l.push(vec![]);
                p += 1;
            }
            graph_prim.old_p[0] = 0; // last use node
            graph_prim.old_p[1] = 0;
            println!("graph  {:?} ",graph_prim.old_p);
            1
        }
    }
    else
    {
        let mut min :(usize,i64) = (0,0);
        let mut p : usize = graph_prim.old_p[0];
        let mut p2: usize = 1;
        if graph_e.l[0].len() > 0
        {
            min = graph_e.l[graph_prim.old_p[0]][0];
            //graph_primin.1 = graph_e.l[m.old[p+1]][1].1;
        }
        let mut adjlist = &mut graph_e.l[graph_prim.old_p[0]];
        let pos = graph_prim.old_p[0];
        graph_prim.old_p[pos+1] = 1;
        while p2 < adjlist.len()
        {
            if graph_prim.old_p[adjlist[p2].0 + 1] == 0 && min.1 > adjlist[p2].1
            {
                min = adjlist[p2];
                //println!("graph_primin {:?}",min);
            }
            p2+= 1;
        }
        p += 1;
        println!("graph  {:?} ",graph_prim.old_p);

        if min == (0,0) && pos != 0
        {
            0 //check no node left
        }
        else
        {
            //graph_prim.l[min.0].push((graph_prim.old_p[0],min.1));
            graph_prim.l[graph_prim.old_p[0]].push((min.0,min.1));
            graph_prim.old_p[0] = min.0;
            1
        }
    }

}







fn main()
{
    let mut g1 : Graph = Graph {
        s: vec![0,1,2], //sograph_primmet
        l: vec![vec![(2,10),(1,5)],vec![(0,5),(2,3)],vec![(0,10),(1,3)]],  //lien
        old_p : Vec::new(), // old pos sograph_primmet
    };
    let mut g2 : Graph = Graph {
        s: vec![], //sograph_primmet
        l: Vec::new(),  //lien
        old_p : Vec::new(), // old pos sograph_primmet

    };

    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.s);
    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.s);
    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.s);
    println!("{:?}",g2.l);

    println!("done \n");

    let mut g1 : Graph = Graph {
        s: vec![0,1,2,3,4,5], //sograph_primmet
        l: vec![
            vec![(1,1)],
            vec![(2,1)],
            vec![(3,1)],
            vec![(4,1)],
            vec![(5,1)],
            vec![(0,1)]            
        ],  //lien
        old_p : Vec::new(), // old pos sograph_primmet
    };
    let mut g2 : Graph = Graph {
        s: vec![], //sograph_primmet
        l: Vec::new(),  //lien
        old_p : Vec::new(), // old pos sograph_primmet

    };
    let mut f = 0;

    while f <6
    {
        println!("{}",prim(&mut g1, &mut g2));
        //println!("{:?}",g2.s);
        f+=1;
    }
    println!("{:?}",g2.s);
    println!("{:?}",g2.l);




}




