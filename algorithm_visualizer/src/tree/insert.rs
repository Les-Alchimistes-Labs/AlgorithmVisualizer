use crate::Btree;
use gtk::Notebook;
use crate::GTK::tree::paint_tree;
use crate::BTREE;


fn insert_bis(btree: &mut Box<Btree>,notebook : &mut Notebook, key: i32) {

	paint_tree(notebook,btree.key,btree.key);
    if key <= btree.key {
        if !btree.left.is_none() {
            insert_bis(&mut btree.left.as_mut().unwrap(),notebook, key);
        } else {
            btree.left = Some(Box::new(Btree::new(key, None, None)));
        }
    } else {
        if !btree.right.is_none() {
            insert_bis(&mut btree.right.as_mut().unwrap(),notebook, key);
        } else {
            btree.right = Some(Box::new(Btree::new(key, None, None)));
        }
    }
}

pub fn insert(notebook : &mut Notebook, key: i32) {
    unsafe
    {
	    if BTREE ==None 
	    {
	        BTREE = Some(Box::new(Btree::new(key, None, None)));
	    } 
	    else 
	    {
	        insert_bis(BTREE.as_mut().unwrap(),notebook, key);
	    }
	    dbg!(&BTREE);
	}
}
