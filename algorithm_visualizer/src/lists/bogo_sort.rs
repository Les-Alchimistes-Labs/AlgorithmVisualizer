use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;
use rand::Rng;


pub fn bogo_sort(notebook :&mut Notebook)
{   
	unsafe
	{   
		let mut rng = rand::thread_rng();
		while !is_sorted()
		{
			let mut clone = CURRENT_LIST.clone();
			for i in 0..CURRENT_LIST.len()
			{
				let index = rng.gen_range(0..clone.len());
				CURRENT_LIST[i] = clone[index].clone();
				clone.remove(index);
			}
			paint_list(notebook,String::from("Bogo sort"),CURRENT_LIST.len(),CURRENT_LIST.len());
		}
	}
}

fn is_sorted() -> bool
{
	unsafe
	{
		for i in 1..CURRENT_LIST.len()
		{
			if CURRENT_LIST[i]<CURRENT_LIST[i-1]
			{
				return false;
			} 
		}
		return true;
	}
}

