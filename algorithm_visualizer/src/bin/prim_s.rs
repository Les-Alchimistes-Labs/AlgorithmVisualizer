struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
    old_p : Vec<usize>, // old pos sograph_primmet
    histogram : Vec<Option<i64>>,
    index : usize,
}

/*
   let mutut m: Graph = Graph{
   s: Vec::new(),
   l: Vec::new(),
   c: Vec::new(),
   }


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
   while p< graph_prim.old_p.len() && p < graph_e.s.len()
   {
   graph_prim.old_p[p] = 0;
   p +=1;
   }
   while p < graph_e.s.len()
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
   graph_prim.old_p[0] = 1;
   println!("graph  {:?} ",graph_prim.old_p);
   1
   }
   }
   else
   {
   let mut min :Option<(usize,i64)> = None;
   let mut posm = 0;
   let index = graph_prim.index_l;
   let mut p2: usize = graph_prim.index_p;
   let adjlist = &graph_e.l[index];
   while p2 < adjlist.len()
   {
//println!("is {}", graph_prim.old_p[adjlist[p2].0 + 1] == 0);
if graph_prim.old_p[adjlist[p2].0 ] == 0 
&& (min == None || min.unwrap().1 > adjlist[p2].1)
{
//dbg!(graph_prim.old_p[adjlist[p2].0 + 1]);
min = Some(adjlist[p2]);
posm = index;
println!("graph_primin {:?}",min.unwrap());
}
p2+= 1;
}
println!("graph  {:?} ",graph_prim.old_p);
if min == None
{
let mut i = graph_prim.index_l;
println!("{}",graph_prim.old_p.len());
if i< graph_prim.old_p.len()
{
while i < graph_prim.old_p.len() && graph_prim.old_p[i] == 1 
{
i+=1;
}
if i == graph_prim.old_p.len()
{
    0
}
else
{
    graph_prim.old_p[i] = 1; 
    graph_prim.index_l = i;
    1
}
}
else
{
    0
}
}
else
{
    let f = min.unwrap();
    println!("add  {:?} ",graph_e.l[posm]);
    graph_prim.old_p[f.0] = 1;
    graph_prim.l[f.0].push((posm,f.1));
    graph_prim.l[posm].push(f);
    graph_prim.index_l = f.0;
    1
}
}

}
*/


fn prim_set(graph_e :&mut Graph,graph_prim :&mut Graph)
{
    let mut p = 0;
    while p < graph_e.s.len()
    {
        graph_prim.s.push(graph_e.s[p]);
        graph_prim.l.push(vec![]);
        graph_prim.histogram.push(Some(0));
        p += 1;
    }

    graph_prim.old_p.push(0);
    graph_prim.histogram[0] = Some(1);
}




fn prim(ge :&mut Graph, gp :&mut Graph) ->u8
{
    if ge.s.len() == gp.old_p.len()
    {
        0
    }
    else
    {
        let mut min : Option<(usize,usize,i64)> = None;
        for (index,l) in gp.old_p.iter().enumerate()
        {
            for e in ge.l[*l].iter()
            {
                //println!("{}",e.0);
                if gp.histogram[e.0].unwrap() == 0 && 
                    (min == None || min.unwrap().2 > e.1)
                {
                    //dbg!(e);
                    let t =  e;
                    min = Some((index,t.0,t.1));
                }
            }
        }
        if min != None
        {
            let t = min.unwrap();
            gp.histogram[t.1] = Some(1);
            gp.old_p.push(t.1);
            gp.l[t.1].push((t.0,t.2));
            gp.l[t.0].push((t.1,t.2));
            1
        }
        else
        {
            println!("this way");
            let mut p = gp.index;
            while p < gp.histogram.len() && gp.histogram[p].unwrap() == 1
            {
                p += 1;
            }
            if p ==  gp.histogram.len()
            {
                0
            }
            else
            {
                gp.old_p.push(p);
                if ge.l[p].len() != 0 
                {
                    let mut min = ge.l[p][0] ;
                    //let mut fo = false;
                    let mut pos = 0;
                    while pos < ge.l[p].len() 
                        && gp.histogram[ge.l[p][pos].0].unwrap() != 0
                    {
                        //println!("{}",e.0);
                        if min.1 > ge.l[p][pos].1
                        {
                            //dbg!(e);
                            min = ge.l[p][pos];
                        }
                        pos += 1;
                    }
                    //println!("{}",fo);
                    if  pos == ge.l[p].len()
                    {
                    println!("this way");
                    gp.l[p].push((min.0,min.1));
                    gp.l[min.0].push((p,min.1));
                    }
                }
                gp.index= p +1;
                1
            }
        }
    }

}







fn main()
{
    let mut g1 : Graph = Graph {
        s: vec![0,1,2], //sograph_primmet
        l: vec![vec![(2,10),(1,5)],vec![(0,5),(2,3)],vec![(0,10),(1,3)]],  //lien
        old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        index : 0,
    };
    let mut g2 : Graph = Graph {
        s: vec![], //sograph_primmet
        l: Vec::new(),  //lien
        old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        index : 0,
    };
    prim_set(&mut g1, &mut g2);
    println!("{:?}",g2.old_p);
    println!("{:?}",g2.l);
    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.old_p);
    println!("{:?}",g2.l);
    println!("{:?}",g2.histogram); 
    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.old_p);
    println!("{:?}",g2.l);
    println!("{:?}",g2.histogram); 
    println!("{}",prim(&mut g1, &mut g2));
    println!("{:?}",g2.s);
    println!("{:?}",g2.l);
    println!("{:?}",g2.histogram); 

    println!("done \n");

    let mut g1 : Graph = Graph {
        s: vec![0,1,2,3,4,5,6], //sograph_primmet
        l: vec![
            vec![(1,1)],
            vec![(2,1)],
            vec![(3,1)],
            vec![(4,1)],
            vec![(5,1)],
            vec![(3,1)],
            vec![(2,1)],
        ],  //lien
        old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        index : 0,
    };
    let mut g2 : Graph = Graph {
        s: vec![], //sograph_primmet
        l: Vec::new(),  //lien
        old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        index : 0,
    };
    prim_set(&mut g1, &mut g2);
    let mut f = prim(&mut g1, &mut g2);

    while f == 1
    {
        println!("{:?}",g2.s);
        println!("{:?}",g2.l);
        println!("{:?}",g2.histogram); 


        println!("{}",f);
        f = prim(&mut g1, &mut g2);
    }

    println!("{}",f);
    println!("{:?}",g2.s);
    println!("{:?}",g2.l);
    println!("{:?}",g2.histogram);

}




