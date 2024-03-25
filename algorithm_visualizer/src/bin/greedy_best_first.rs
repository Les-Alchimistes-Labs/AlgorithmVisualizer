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


struct Graph{
    s: Vec<i64>, //sommet
    l: Vec<Vec<(usize,i64)>>,  //lien
    _old_p : Vec<usize>, // old pos sommet
    histogram : Vec<Option<i64>>,
    _index : usize,
}



fn set_greedy(g :&mut Graph)
{
    g.histogram.push(Some(0));// set x
    g.histogram.push(Some(0));// set y
    g.histogram.push(Some(0));// set sizex
    g.histogram.push(Some(0));// set sizex
    
    for _i in 4..(g.s.len()+4)
    {
        let a : Option<i64> = None;
        g.histogram.push(a);
    }
    g.histogram[4] = Some(0); 
}


fn greedy_graph(g :&mut Graph,x :usize,y :usize ) -> bool
{
        let mut end = true;
        while g.histogram[g._index] != None
        {
            let mut cx = x - g.histogram[0] ;
            let mut cy = y - g.histogram[1] ;
            let sx =  g.histogram[2];
            let sy =  g.histogram[3]
            if (cx == 0 && cy == 0)
            {
                return 1;
            }
            else if cx == 0
            {
                if g.histogram[sy*(y-1) + x  +4] == None
                {
                    g.histogram[sy*(y-1) + x  +4] = Some(sy*(y) + x  +4);
                }
 
                else if g.histogram[sy*(y+1) + x  +4] == None
                {
                    g.histogram[sy*(y+1) + x  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }

            }
            else if cy == 0
            {
                if g.histogram[sy*y + (x - 1)  +4] == None
                {
                    g.histogram[sy*y + (x - 1)  +4] = Some(sy*(y) + x  +4);
                }
                else if g.histogram[sy*y + (x +1)  +4] == None
                {
                    g.histogram[sy*y + (x +1)  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }

            }
            else if cx > 0 && cy > 0 
            {
                if g.histogram[sy*(y+1) + x  +4] == None
                {
                     g.histogram[sy*(y+1) + x  +4] = Some(sy*(y) + x  +4);
                }
                else if g.histogram[sy*y + (x +1)  +4] == None
                {
                    g.histogram[sy*y + (x +1)  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }
            }
            else if cx < 0 && cy < 0 
            {
                if g.histogram[sy*(y-1) + x  +4] == None
                {
                     g.histogram[sy*(y-1) + x  +4] = Some(sy*(y) + x  +4);
                }
                else if g.histogram[sy*y + (x -1)  +4] == None
                {
                    g.histogram[sy*y + (x -1)  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }
            }
            else if cx > 0 && cy < 0 
            {
                if g.histogram[sy*(y-1) + x  +4] == None
                {
                     g.histogram[sy*(y-1) + x  +4] = Some(sy*(y) + x  +4);
                }
                else if g.histogram[sy*y + (x +1)  +4] == None
                {
                    g.histogram[sy*y + (x +1)  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }
            }
            else 
            {
                if g.histogram[sy*(y+1) + x  +4] == None
                {
                     g.histogram[sy*(y+1) + x  +4] = Some(sy*(y) + x  +4);
                }
                else if g.histogram[sy*y + (x -1)  +4] == None
                {
                    g.histogram[sy*y + (x -1)  +4] = Some(sy*(y) + x  +4);
                }
                else
                {
                    g._index = g.histogram[sy*y + x  +4].unwrap();
                }
            }
        }
        false
}



fn main()
{
    
}
