use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;

pub fn counting_sort(max : usize)
{
	unsafe
	{
		let mut occ: Vec<usize> = vec![0; max + 1];
	    for i in 0..CURRENT_LIST.len()
	    {
	        occ[CURRENT_LIST[i] as usize ] += 1;
	    }
	    let mut index = 0;
	    for (i,&j) in occ.iter().enumerate()
	    {
	        for _k in 0..j
	        {
	            CURRENT_LIST[index] = i as i64;
	            paint_list(String::from("Counting Sort"),CURRENT_LIST[index] as usize,i);
	            index +=1;
	        }
	    }
	}
} 
