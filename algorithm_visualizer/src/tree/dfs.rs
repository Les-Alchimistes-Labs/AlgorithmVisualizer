use crate::Btree;


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
