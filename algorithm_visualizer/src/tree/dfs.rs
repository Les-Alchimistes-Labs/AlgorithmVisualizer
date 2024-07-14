use crate::Btree;
use gtk::Notebook;
use crate::GTK::tree::paint_tree;

pub fn parcours_profondeur(btree: &mut Option<Box<Btree>>, mut result: String) -> String {
    if btree.is_none() {
        return result;
    }
    if !btree.is_none() {
        result.push_str(&btree.as_ref().unwrap().key.to_string());
        result.push_str(" ");
        result = parcours_profondeur(&mut btree.as_mut().unwrap().left, result);
        result = parcours_profondeur(&mut btree.as_mut().unwrap().right, result);
    }
    result.clone()
}

pub fn dfs_pre(btree: &mut Option<Box<Btree>>, mut result: String, notebook :&mut Notebook) -> String {
    if btree.is_none() {
        return result;
    }
    if !btree.is_none() {
        result.push_str(&btree.as_ref().unwrap().key.to_string());
        result.push_str(" ");
		paint_tree("prefix",notebook,btree.as_mut().unwrap().key,btree.as_mut().unwrap().key);
        result = dfs_pre(&mut btree.clone().as_mut().unwrap().left, result,notebook);
        result = dfs_pre(&mut btree.clone().as_mut().unwrap().right, result,notebook);
    }
    result.clone()
    
}

pub fn dfs_in(btree: &mut Option<Box<Btree>>, mut result: String, notebook :&mut Notebook) -> String {
    if btree.is_none() {
        return result;
    }
    if !btree.is_none() {
        result.push_str(&btree.as_ref().unwrap().key.to_string());
        result.push_str(" ");
        result = dfs_in(&mut btree.clone().as_mut().unwrap().left, result,notebook);
		paint_tree("infix",notebook,btree.as_mut().unwrap().key,btree.as_mut().unwrap().key);
        result = dfs_in(&mut btree.clone().as_mut().unwrap().right, result,notebook);
    }
    result.clone()
}

pub fn dfs_suf(btree: &mut Option<Box<Btree>>, mut result: String, notebook :&mut Notebook) -> String {
    if btree.is_none() {
        return result;
    }
    if !btree.is_none() {
        result.push_str(&btree.as_ref().unwrap().key.to_string());
        result.push_str(" ");
        result = dfs_suf(&mut btree.as_mut().unwrap().left, result,notebook);
        result = dfs_suf(&mut btree.as_mut().unwrap().right, result,notebook);
        paint_tree("suffix",notebook,btree.as_mut().unwrap().key,btree.as_mut().unwrap().key);

    }
    result.clone()
}
