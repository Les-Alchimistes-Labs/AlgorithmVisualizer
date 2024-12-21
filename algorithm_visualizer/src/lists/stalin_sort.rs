use crate::GTK::list::paint_list;
use crate::CURRENT_LIST;
use gtk::Notebook;


pub fn stalin_sort(notebook : &mut Notebook)
{
    unsafe
    {
        let mut index = 1;
        while index != CURRENT_LIST.len() 
        {
            if CURRENT_LIST[index-1]>CURRENT_LIST[index]
            {
                CURRENT_LIST.remove(index);
                paint_list(notebook,String::from("Stalin sort"),index,CURRENT_LIST.len());
            }
            else
            {
                index+= 1;
                paint_list(notebook,String::from("Stalin sort"),index,index-1);
            }
            
        }
    }
}
