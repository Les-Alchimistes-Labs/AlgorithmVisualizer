
fn greedy_best_first(tab :&mut Vec<Vec<u8>>, pass: &mut Vec<Vec<u8>>,
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

fn main()
{

}
