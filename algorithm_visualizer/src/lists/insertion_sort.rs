use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;
pub fn insertion_sort(notebook : &mut Notebook )
{
	unsafe
	{
		let len = CURRENT_LIST.len();
	    
	    for i in 1..len {
	        let mut j = i;
	        while j > 0 && CURRENT_LIST[j - 1] > CURRENT_LIST[j] 
	        {
				paint_list(notebook,String::from("Insertion Sort"), i , j);
	            CURRENT_LIST.swap(j, j - 1);
	            paint_list(notebook,String::from("Insertion Sort"), i , j);
	            j -= 1;
	        }
	    } 
	    
		//for i in 1..CURRENT_LIST.len()
		//{
			//for j in (1..=i).rev()
			//{
				//let tmp = CURRENT_LIST[j];
				//if tmp<CURRENT_LIST[j-1]
				//{
					//paint_list(notebook,String::from("Insertion Sort"), i , j);
					//CURRENT_LIST[j] = CURRENT_LIST[j-1];
					//CURRENT_LIST[j-1] = tmp;
					//paint_list(notebook,String::from("Insertion Sort"), i , j);
				//}
				//paint_list(notebook,String::from("Insertion Sort"), i , j);
			//}
		//}
	    
	    
	}
	
	
}
