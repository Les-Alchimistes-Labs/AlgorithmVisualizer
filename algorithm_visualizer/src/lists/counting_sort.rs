use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;

pub fn counting_sort(notebook : &mut Notebook ,max : usize)
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
				paint_list(notebook,String::from("Counting Sort"),index,CURRENT_LIST[index] as usize);
	            CURRENT_LIST[index] = i as i64;
	            paint_list(notebook,String::from("Counting Sort"),index,CURRENT_LIST[index] as usize);
	            index +=1;
	        }
	    }
	}
} 
