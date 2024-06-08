use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;
use rand::Rng;

pub fn shuffle(notebook : &mut Notebook)
{
	unsafe 
	{
		let mut rng = rand::thread_rng();
		let mut clone = CURRENT_LIST.clone();
		for i in 0..CURRENT_LIST.len()
		{
			let index = rng.gen_range(0..clone.len());
			CURRENT_LIST[i] = clone[index].clone();
			clone.remove(index);
		}
		paint_list(notebook,String::from("Shuffle"),CURRENT_LIST.len(),CURRENT_LIST.len());
	}
}
