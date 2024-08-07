struct List{
    l: Vec<i64>,
    n_p :usize,
    p_s :Vec<usize>,
}

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
    //println!("so vaut : {} {:?}",n, v); 
}

fn merge_sort_set(l: &mut List)
{

    l.n_p = 2;
    l.p_s.push(0);
    l.p_s.push(l.l.len());
    while merge_sort_cut(&mut l.n_p,&mut l.p_s)
    {
    }
merge_sort_sort(&mut l.l, &mut l.n_p,&mut l.p_s);
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
        //println!("diff: {}", diff);
        if diff >= 2 && v[p_s[p_v - 1 ]] > v[p_s[p_v - 1] + 1]
        {
            (v[p_s[p_v - 1 ]],v[p_s[p_v - 1] + 1]) = 
                (v[p_s[p_v - 1] + 1],v[p_s[p_v - 1]]);
        }
        p_v += 1;
    }
}




fn merge_sort_join(li :&mut List) //v :&mut Vec<i64>,n_p: &mut usize, p_s :&mut Vec<usize>)
{
    let v = &li.l;
    let mut n_p = &mut li.n_p;
    let mut p_s = &mut li.p_s;
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
        remove_m(p_s[p_v],&mut p_s,&mut n_p);
        p_v += 1;
   }
    let mut p = 0;
   //println!("s: {} {} {:?}",p_v < *n_p && p_s[p_v] + p < p_s[*n_p - 1], p_v,p_s[*n_p-1] );
    while p_v < *n_p && p_s[p_v - 1] + p < p_s[*n_p - 1]
    {
        v2.push(v[p_s[p_v - 1] + p ]);
        p += 1;
    }
    
    println!("v1 vaut : {:?}", v2);

    li.l = v2;
}


pub fn test_merge()
{
    println!("merge sort:\n");
    //let v1: Vec<i64>= vec![1,2,3,4,5,6,7,8,9,10];
    let mut v: Vec<Vec<i64>>= vec![
        vec![1,6,8,4,7,3,2,5,9,10,-1],
        vec![-1,1,2,3,4,5,6,7,8,9,10],
        vec![10,9,8,7,6,5,4,3,2,1,-1],
        vec![3,2,4,1]
        ];
    


    for e in v.iter_mut()
    {
        let mut l = List{
            l : e.clone(),
            n_p : 2,
            p_s : vec![]
        };
        merge_sort_set(&mut l);
       println!("v1 vaut : {:?}", e);
        while l.p_s.len() > 2
        {
            println!("p_s vaut : {:?}", l.p_s);
            merge_sort_join(&mut l);
        }
        println!("\n\n");
    }
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

fn main()
{
    test_merge();
}
