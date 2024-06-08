use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;


pub fn quick_sort(notebook :&mut Notebook ,little: i32, big: i32)
{
    if little < big
    {
        let pos = part(notebook,little, big);
        quick_sort(notebook, little, pos - 1);
        quick_sort(notebook, pos + 1, big);
    }
}

fn part(notebook :&mut Notebook,little: i32, big: i32) -> i32
{
	unsafe
	{
	    let piv = CURRENT_LIST[big as usize];
	    let mut pos = little - 1;
	    for i in little..big
	    {
	        if CURRENT_LIST[i as usize] <= piv
	        {
	            pos += 1;
	            (CURRENT_LIST[pos as usize], CURRENT_LIST[i as usize]) = (CURRENT_LIST[i as usize], CURRENT_LIST[pos as usize]);
	            paint_list(notebook,String::from("Quick sort"),i as usize,pos as usize);
	        }
	    }
	    (CURRENT_LIST[pos as usize + 1], CURRENT_LIST[big as usize]) = (CURRENT_LIST[big as usize], CURRENT_LIST[pos as usize + 1]);
	    paint_list(notebook,String::from("Quick sort"),big as usize , pos as usize +1 );
	    return pos + 1;
	}
}

