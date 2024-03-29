use crate::Btree;
use gtk::Notebook;
use crate::GTK::tree::paint_tree;
use crate::BTREE;



fn delete_bis(btree: &mut Box<Btree>,notebook: &mut Notebook, key: i32) -> Option<Box<Btree>> {
    if key > btree.key 
    {
		if btree.right == None 
		{
			return None;
		}
		else
		{
			if btree.right.as_mut().unwrap().right == None && btree.right.as_mut().unwrap().left ==None && btree.right.as_mut().unwrap().key == key 
			{
				btree.right = None;
				return Some(Box::new(Btree::new(1,None,None)))
			}
			else
			{
				let tmp = delete_bis(btree.right.as_mut().unwrap(),notebook, key);
				if tmp == None 
				{
					return None;
				}
				else
				{
					btree.right = tmp;
				}
			} 
		}
    } 
    else if key < btree.key 
    {
		if btree.left == None
		{
			return None;
		} 
        else
        {
			let tmp = delete_bis(btree.left.as_mut().unwrap(),notebook, key);
			if tmp == None 
			{
				return None;
			}
			else
			{
				btree.left =tmp;
			} 
		}
    } 
    else 
    {
        if btree.left.is_none() && btree.right.is_none() 
        {
            return None;
        } 
        else if btree.left.is_none() 
        {
            if let Some(left_child) = btree.right.take() {
			    *btree = left_child;
			    return Some(btree.clone());
			} else {
			    return None;
			}
        } 
        else if btree.right.is_none()
        {
            if let Some(left_child) = btree.left.take() {
			    *btree = left_child;
			    return Some(btree.clone());
			} else {
			    return None;
			}

        } 
        else 
        {
            let y = max(btree.left.clone());
            btree.key = y;
            btree.left = delete_bis(btree.left.as_mut().unwrap(),notebook, y);
        }
    }
    Some(btree.clone())
}

pub fn delete(notebook :&mut Notebook, key: i32) -> Option<Box<Btree>> {
    unsafe
    {
		
	    if BTREE ==None 
	    {
	        None
	    } 
	    else 
	    {
			if BTREE.clone().unwrap().key == key && BTREE.clone().unwrap().left == None &&
			 BTREE.clone().unwrap().right == None
			{
				BTREE = None;
				return Some(Box::new(Btree::new(1,None,None)));
			}
			else
			{			
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



