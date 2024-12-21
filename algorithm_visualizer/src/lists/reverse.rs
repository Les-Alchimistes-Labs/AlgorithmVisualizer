use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;


pub fn reverse(notebook : &mut Notebook)
{
    unsafe
    {
        let clone = CURRENT_LIST.clone();
        let mut index = 0; 
        for i in (0..clone.len()).rev()
        {
            CURRENT_LIST[index] = clone[i];
            index+=1;
        }
        paint_list(notebook,String::from("Reverse"),clone.len(),clone.len());
    }
}
