/*:
fn _greedy_best_first(tab :&mut Vec<Vec<u8>>, pass: &mut Vec<Vec<u8>>,
    now :&mut Vec<(usize,usize)>,targetx: usize, targety: usize) -> i8
{
    let mut v2 : Vec<(usize,usize)> = Vec::new();
    for v in  &mut *now
    {
        if *v == (targetx,targety)
        {
            return 1;
        }
        else if pass[v.0][v.1] != 0 && tab[v.0][v.1] == 0
        {
            pass[v.0][v.1] = 1;
            let mut max = (v.0,v.1) ;
            let mut c1 = targetx - v.1 + 1;
            let mut c2 = targety - v.0 + 0;
            let mut c = c1*c1 + c2*c2;
            let mut _r = c;
            
            c1 = targetx - v.1 - 1;
            c2 = targety - v.0 + 0;
            c = c1*c1 + c2*c2;
            if c < _r
            {
                max = (v.0 - 1,v.1 + 0 );
                _r = c; 
            }
            c1 = targetx - v.1 + 0;
            c2 = targety - v.0 + 1;
            c = c1*c1 + c2*c2;
            if c < _r
            {
                max = (v.0,v.1 + 1 );
                _r = c; 
            }
            c1 = targetx - v.1 + 0;
            c2 = targety - v.0 - 1;
            c = c1*c1 + c2*c2;
            if  c < _r
            {
                max = (v.0,v.1 - 1 );
                _r = c; 
            }
            v2.push((max.0 as usize,max.1 as usize));

        }
    }
    *now = v2;

if now.len() == 0
{
    return -1;
}
0
}

*/

fn set_greedy(g :&mut Graph)
{
    for _i in 0..g.s.len()
    {
        let a : Option<i64> = None;
        g.histogram.push(a);
    }
    g.histogram[0] = Some(0); 
}


fn greedy(g :&mut Graph ) -> bool
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
        _old_p : Vec::new(), // old pos sograph_primmet
        histogram : Vec::new(),
        _index : 0,
    };
    set_greedy(&mut g1);

    while !greedy(&mut g1)
    {
    }
        println!("histogram is {:?}",g1.histogram);
}

