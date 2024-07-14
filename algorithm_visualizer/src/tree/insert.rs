use crate::Btree;
use gtk::Notebook;
use crate::GTK::tree::paint_tree;
use crate::BTREE;

fn insert_bis(btree: &mut Box<Btree>,notebook : &mut Notebook, key: i32) {

    if key <= btree.key {
        if !btree.left.is_none() {
			paint_tree("Add",notebook,btree.left.as_mut().unwrap().key,btree.key);
            insert_bis(&mut btree.left.as_mut().unwrap(),notebook, key);
        } else {
            btree.left = Some(Box::new(Btree::new(key, None, None)));
            paint_tree("Add",notebook,key,btree.key);
        }
    }
    else {
        if !btree.right.is_none() {
			paint_tree("Add",notebook,btree.right.as_mut().unwrap().key,btree.key);
            insert_bis(&mut btree.right.as_mut().unwrap(),notebook, key);
        } else {
            btree.right = Some(Box::new(Btree::new(key, None, None)));
            paint_tree("Add",notebook,key,btree.key);
        }
    }
}

pub fn insert(notebook : &mut Notebook, key: i32) {
    unsafe{
	    if BTREE ==None{
	        BTREE = Some(Box::new(Btree::new(key, None, None)));
	    } else {
	        insert_bis(BTREE.as_mut().unwrap(),notebook, key);
	    }
	}
}

pub fn create( key: i32) {
    unsafe {
	    if BTREE ==None {
	        BTREE = Some(Box::new(Btree::new(key, None, None)));
	    } else {
	        create_bis(BTREE.as_mut().unwrap(), key);
	    }
	}
}

fn create_bis(btree: &mut Box<Btree>, key: i32) {

    if key <= btree.key {
        if !btree.left.is_none() {
            create_bis(&mut btree.left.as_mut().unwrap(), key);
        } else {
            btree.left = Some(Box::new(Btree::new(key, None, None)));
        }
    } else {
		if !btree.right.is_none() {
            create_bis(&mut btree.right.as_mut().unwrap(),key);
        } else {
            btree.right = Some(Box::new(Btree::new(key, None, None)));
        }
    }
}
