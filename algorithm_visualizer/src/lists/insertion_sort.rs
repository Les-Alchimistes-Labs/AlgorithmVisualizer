use crate::GTK::list::paint_list;
use std::sync::{Arc, Mutex};
use gtk::Notebook;
use crate::CURRENT_LIST;


pub fn insertion_sort()
{
	unsafe
	{
		let len = CURRENT_LIST.len();
	    
	    for i in 1..len {
	        let mut j = i;
	        while j > 0 && CURRENT_LIST[j - 1] > CURRENT_LIST[j] 
	        {
	            CURRENT_LIST.swap(j, j - 1);
	            paint_list( i , j);
	            j -= 1;
	        }
	    }
	}
	
}
