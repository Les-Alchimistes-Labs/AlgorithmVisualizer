use std::env;

fn insert_m(va : usize,v : &mut Vec<usize> , n : &mut usize)
{
    let mut p : usize  = 0;
    while p < *n && v[p] < va
    {
        //println!("p: {}", p); 
        p += 1;
    }
    if v[p] != va
    {
        v.push(va);
        p = *n - 1;
        //println!("p: {}", p);
        while va < v[p]
        {
            //println!("p2: {}", p);
            let t: usize  = v[p];
            v[p] = v[p+1];
            //println!("vp: {}", v[p]);
            v[p+1] = t;
            if p > 0
            {
                p -= 1;
            }
        }
    }
    *n = *n + 1;
}


fn remove_m(va : usize,v : &mut Vec<usize> , n : &mut usize)
{
    let mut p : usize  = 0;
    while p < *n && v[p] < va
    {
        //println!("p: {}", p); 
        p += 1;
    }
    if v[p] == va
    {
        //println!("p: {}", p);
        while p + 1 < *n
        {
            //println!("p2: {}", p);
            let t: usize  = v[p];
            v[p] = v[p+1];
            //println!("vp: {}", v[p]);
            v[p+1] = t;
            p += 1;
        }
        v.pop();
    }
    else 
    {
        println!("error");
    }
    *n = *n - 1;
    println!("so vaut : {} {:?}",n, v); 
}




fn merge_sort_cut( n_p: &mut usize, p_s :&mut Vec<usize>) -> bool
{
    let mut p_s2 = p_s.clone();
    let mut found = false;
    let mut p = 0;
    let so = *n_p;
    while p + 1< so
    {
        //println!("pm: {}", p);
        let m = p_s[p+1] - p_s[p];
        //println!("m: {}", m);
        match m
        {
            0 | 1 | 2 => found = false,
            _ => {
                insert_m(p_s[p]+m/2,&mut p_s2,n_p);
                found = true
            },
        }
        p = p + 1 ;
    }
    *p_s = p_s2;
    found

}


fn merge_sort_sort(v :&mut Vec<i64>,n_p: &mut usize, p_s :&mut Vec<usize>)
{
    let mut p_v = 1;
    while p_v < *n_p
    {
        let diff = p_s[p_v] - p_s[p_v - 1];
        println!("diff: {}", diff);
        if diff >= 2 && v[p_s[p_v - 1 ]] > v[p_s[p_v - 1] + 1]
        {
            (v[p_s[p_v - 1 ]],v[p_s[p_v - 1] + 1]) = 
                (v[p_s[p_v - 1] + 1],v[p_s[p_v - 1]]);
        }
        p_v += 1;
    }
}




fn merge_sort_join(v :&mut Vec<i64>,n_p: &mut usize, p_s :&mut Vec<usize>)
{
    let mut v2 :  Vec<i64> = Vec::new();
    let mut p_v = 1;
    while p_v + 1 < *n_p // check if next good
    {
        let mut p1 = p_s[p_v - 1];
        let mut p2 = p_s[p_v ];
        let s1 = p_s[p_v ] ;
        let s2 = p_s[p_v + 1];
        while p1 < s1 && p2 < s2
        {
            if v[ p2 ] >v[ p1]
            {
                v2.push(v[p1]);
                //println!("add p1: {}", v[p1]);
                p1 += 1;
            }
            else
            {
                v2.push(v[p2]);
                p2 += 1;
                //println!("add p2: {}", v[p2]);
            }
        }
        while p1 < s1 
        {
            v2.push(v[p1]);
            //println!("add p1: {}", v[p1]);
            p1 += 1;
        }
        while p2 < s2 
        {
            v2.push(v[p2]);
            //println!("add p2: {}", v[p2]);
            p2 += 1;
        }
        remove_m(p_s[p_v], p_s,n_p);
        p_v += 1;
   }
    let mut p = 0;
    if p_v < *n_p
    {println!("{:?} ", p_s);}
    println!("s: {} {} {:?}",p_v < *n_p && p_s[p_v] + p < p_s[*n_p - 1], p_v,p_s[*n_p-1] );
    while p_v < *n_p && p_s[p_v - 1] + p < p_s[*n_p - 1]
    {
        v2.push(v[p_s[p_v - 1] + p ]);
        p += 1;
    }
    
    println!("v1 vaut : {:?}", v2);

    *v = v2;
}



fn main()
{

    env::set_var("RUST_BACKTRACE", "1"); 
    let mut v1: Vec<i64>= vec![1,2,3,4,5,6,7,8,9,10];
    let mut v: Vec<i64>= vec![1,6,8,4,7,3,2,5,9,10,-1];

    let mut n : usize = 11;
    let mut n_p :usize = 2;
    let  mut p_s: Vec<usize> = vec![0,n];
    while merge_sort_cut(&mut n_p,&mut p_s)
    {
    }
    merge_sort_sort(&mut v, &mut n_p,&mut p_s);
    println!("so vaut : {:?}",v);
    println!("res vaut : {} {:?}",n_p, p_s); 
    merge_sort_join(&mut v,&mut n_p,&mut p_s);
    merge_sort_join(&mut v,&mut n_p,&mut p_s);
    merge_sort_join(&mut v,&mut n_p,&mut p_s);
    println!("sort vaut : {:?}",p_s);





    /* show in detail 
    for i in 1..n_p {
        //println!("i vaut : {} \n", p_s[i]);
        let mut x = p_s[i - 1];
        //println!("diff2: {}",  p_s[i ] -  p_s[i -1]);
        while x < p_s[i]
        {
            println!("v vaut : {}", v[x]);
            x +=1;

        }
        //println!("\n");
    }
    */
}
