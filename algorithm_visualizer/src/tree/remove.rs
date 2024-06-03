use crate::Btree;
use gtk::Notebook;
use crate::GTK::tree::paint_tree;
use crate::BTREE;

fn delete_bis(btree: &mut Box<Btree>,notebook: &mut Notebook, key: i32) -> (Option<Box<Btree>>,bool) {
    if key > btree.key {
		if btree.right == None {
			paint_tree("Remove",notebook,btree.key,btree.key);
			return (None,false);
		}else {
			if btree.right.as_mut().unwrap().right == None && btree.right.as_mut().unwrap().left ==None && btree.right.as_mut().unwrap().key == key {
				paint_tree("Remove",notebook,btree.right.as_mut().unwrap().key,btree.key);
				btree.right = None;
				paint_tree("Remove",notebook,btree.key,btree.key);
				return (Some(btree.clone()),true)
			}else {
				paint_tree("Remove",notebook,btree.right.as_mut().unwrap().key,btree.key);
				let (t,b) = delete_bis(btree.right.as_mut().unwrap(),notebook, key);
				paint_tree("Remove",notebook,btree.key,btree.right.as_mut().unwrap().key);
				if !b{
					paint_tree("Remove",notebook,btree.key,btree.key);
					return (None,false);
				} else {
					btree.right = t;
				}
			} 
		}
    } 
    else if key < btree.key {
		if btree.left == None {
			paint_tree("Remove",notebook,btree.key,btree.key);
			return (None,false);
		} else {
			if btree.left.as_mut().unwrap().right == None && btree.left.as_mut().unwrap().left ==None && btree.left.as_mut().unwrap().key == key {
				paint_tree("Remove",notebook,btree.left.as_mut().unwrap().key,btree.key);
				btree.left = None;
				paint_tree("Remove",notebook,btree.key,btree.key);
				return (Some(btree.clone()),true)
			} else {
				paint_tree("Remove",notebook,btree.left.as_mut().unwrap().key,btree.key);
				let (t,b) = delete_bis(btree.left.as_mut().unwrap(),notebook, key);
				paint_tree("Remove",notebook,btree.key,btree.left.as_mut().unwrap().key);
				if !b {
					paint_tree("Remove",notebook,btree.key,btree.key);
					return (None,false);
				} else {
					btree.left =t;
				}
			} 
		}
    } 
    else {
        if btree.left.is_none() && btree.right.is_none() {
			paint_tree("Remove",notebook,btree.key,btree.key);
            return(None,false) ;
        } else if btree.left.is_none() {
            if let Some(left_child) = btree.right.clone() {
			    dbg!(&btree.right);
                            paint_tree("Remove",notebook,btree.right.as_mut().unwrap().key,btree.key);
			    //*btree = left_child;
			    //paint_tree("Remove",notebook,btree.right.as_mut().unwrap().key,btree.key);
			    return (btree.right.clone(),true);
			} else {
				paint_tree("Remove",notebook,btree.key,btree.key);
			    return (None,false);
			}
        } else if btree.right.is_none() {
            if let Some(left_child) = btree.left.clone() {
				paint_tree("Remove",notebook,btree.left.as_mut().unwrap().key,btree.key);
			        //*btree = left_child;
			        //paint_tree("Remove",notebook,btree.left.as_mut().unwrap().key,btree.key);
			    return (btree.left.clone(),true);
			} else {
				paint_tree("Remove",notebook,btree.key,btree.key);
			    return (None,false);
			}

        } else {
            let y = max(btree.left.clone());
            paint_tree("Remove",notebook,btree.left.as_mut().unwrap().key,btree.key);
            btree.key = y;
            let (t,_b) =delete_bis(btree.left.as_mut().unwrap(),notebook, y);
            btree.left = t;
        }
    }
    (Some(btree.clone()),true)
}

pub fn delete(notebook :&mut Notebook, key: i32) -> (Option<Box<Btree>>,bool) {
    unsafe {
	    if BTREE ==None {
	        (None,false)
	    } else {
			if BTREE.clone().unwrap().key == key && BTREE.clone().unwrap().left == None &&
			 BTREE.clone().unwrap().right == None {
				BTREE = None;
				(None,true)
			} else {			
				delete_bis(BTREE.as_mut().unwrap(),notebook, key)
			}
	    }
	}
}

fn max(btree: Option<Box<Btree>>) -> i32 {
    let result = btree.clone().unwrap().key;
    if btree.clone().unwrap().right == None {
        return result;
    }
    max(btree.clone().unwrap().right)
}



