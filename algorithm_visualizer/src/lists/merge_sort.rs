use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;




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
        p += 1;
    }
    if v[p] != va
    {
        v.push(va);
        p = *n - 1;
        while va < v[p]
        {
            let t: usize  = v[p];
            v[p] = v[p+1];
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
        p += 1;
    }
    if v[p] == va
    {
        while p + 1 < *n
        {
            let t: usize  = v[p];
            v[p] = v[p+1];
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
}

pub fn merge_sort(notebook :&mut Notebook)
{
	unsafe
	{
		
        let mut l = List{
            l : CURRENT_LIST.clone(),
            n_p : 2,
            p_s : vec![]
        };
        merge_sort_set(&mut l, notebook);

        while l.p_s.len() > 2
        {
			
            merge_sort_join(&mut l,notebook);
            CURRENT_LIST = l.l.clone();
            paint_list(notebook,String::from("Merge sort"),0,0);
        }
	}
}

fn merge_sort_set(l: &mut List, notebook :&mut Notebook)
{

    l.n_p = 2;
    l.p_s.push(0);
    l.p_s.push(l.l.len());
    while merge_sort_cut(&mut l.n_p,&mut l.p_s)
    {
		
    }
	merge_sort_sort(&mut l.l, &mut l.n_p,&mut l.p_s,notebook);
}

fn merge_sort_cut( n_p: &mut usize, p_s :&mut Vec<usize>) -> bool
{
    let mut p_s2 = p_s.clone();
    let mut found = false;
    let mut p = 0;
    let so = *n_p;
    while p + 1< so
    {
        let m = p_s[p+1] - p_s[p];
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


fn merge_sort_sort(v :&mut Vec<i64>,n_p: &mut usize, p_s :&mut Vec<usize>, notebook : &mut Notebook)
{
	unsafe
	{
			
	    let mut p_v = 1;
	    while p_v < *n_p
	    {
	        let diff = p_s[p_v] - p_s[p_v - 1];
	        if diff >= 2 && v[p_s[p_v - 1 ]] > v[p_s[p_v - 1] + 1]
	        {
				CURRENT_LIST = v.clone();
	            paint_list(notebook,String::from("Merge sort"),p_s[p_v - 1 ],p_s[p_v - 1 ]);
	            paint_list(notebook,String::from("Merge sort"),p_s[p_v - 1] + 1,p_s[p_v - 1] + 1);
	            (v[p_s[p_v - 1 ]],v[p_s[p_v - 1] + 1]) = 
	                (v[p_s[p_v - 1] + 1],v[p_s[p_v - 1]]);
	            CURRENT_LIST = v.clone();
	                
	        }
	        p_v += 1;
	    }
	}
}

fn merge_sort_join(li :&mut List,notebook :&mut Notebook)
{
    let v = &li.l;
    let mut n_p = &mut li.n_p;
    let mut p_s = &mut li.p_s;
    let mut v2 :  Vec<i64> = Vec::new();
    let mut p_v = 1;
    while p_v + 1 < *n_p 
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
                p1 += 1;
            }
            else
            {
                v2.push(v[p2]);
                p2 += 1;
            }
        }
        while p1 < s1 
        {
            v2.push(v[p1]);
            p1 += 1;
        }
        while p2 < s2 
        {
            v2.push(v[p2]);
            p2 += 1;
        }
        remove_m(p_s[p_v],&mut p_s,&mut n_p);
        p_v += 1;
   }
    let mut p = 0;
    while p_v < *n_p && p_s[p_v - 1] + p < p_s[*n_p - 1]
    {
        v2.push(v[p_s[p_v - 1] + p ]);
        p += 1;
    }
    
   
   unsafe
   {
	                                                    
	    for i in 0..li.l.len()
	    {
			li.l[i] = v2[i];
			CURRENT_LIST = li.l.clone();
		    paint_list(notebook,String::from("Merge sort"),i,i);
		}
   }
}



