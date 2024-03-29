use crate::Btree;

pub fn parcours_largeur(btree: &mut Option<Box<Btree>>, mut result: String) -> String{
    let mut queue = vec![];
    queue.push(btree.clone());
    while !queue.is_empty(){
        let mut current = queue.remove(0);
        if current.is_none(){
            continue;
        }else{
            result.push_str(&current.as_ref().unwrap().key.to_string());
            result.push_str(" ");
            queue.push(current.as_mut().unwrap().left.clone());
            queue.push(current.as_mut().unwrap().right.clone());
        }

    }
    result
}
