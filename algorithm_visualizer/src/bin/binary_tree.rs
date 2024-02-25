#[derive(PartialEq)]
#[derive(Debug, Clone)]
pub struct Btree {
    key: i32,
    left: Option<Box<Btree>>,
    right: Option<Box<Btree>>,
}

impl Btree {
    fn new(key: i32, left: Option<Box<Btree>>, right: Option<Box<Btree>>) -> Self {
        Btree { key, left, right }
    }
}

fn insert_bis(btree: &mut Box<Btree>, key: i32) {
    if key <= btree.key {
        if !btree.left.is_none() {
            insert_bis(&mut btree.left.as_mut().unwrap(), key);
        } else {
            btree.left = Some(Box::new(Btree::new(key, None, None)));
        }
    } else {
        if !btree.right.is_none() {
            insert_bis(&mut btree.right.as_mut().unwrap(), key);
        } else {
            btree.right = Some(Box::new(Btree::new(key, None, None)));
        }
    }
}

pub fn insert(btree: &mut Option<Box<Btree>>, key: i32) {
    if btree.clone().unwrap().key == -1 {
        *btree = Some(Box::new(Btree::new(key, None, None)));
    } else {
        insert_bis(btree.as_mut().unwrap(), key);
    }
}

fn max(btree: Option<Box<Btree>>) -> i32 {
    let result = btree.clone().unwrap().key;
    if btree.clone().unwrap().right == None {
        return result;
    }
    max(btree.clone().unwrap().right)
}

fn delete_bis(btree: &mut Box<Btree>, key: i32) -> Option<Box<Btree>> {
    if key > btree.key {
        btree.right = delete_bis(btree.right.as_mut().unwrap(), key)
    } else if key < btree.key {
        btree.left = delete_bis(btree.left.as_mut().unwrap(), key)
    } else {
        if btree.left.is_none() && btree.right.is_none() {
            return None;
        } else if btree.left.is_none() {
            return btree.right.take();
        } else if btree.right.is_none() {
            return btree.left.take();
        } else {
            let y = max(btree.left.clone());
            btree.key = y;
            btree.left = delete_bis(btree.left.as_mut().unwrap(), y);
        }
    }
    Some(btree.clone())
}

pub fn delete(btree: &mut Option<Box<Btree>>, key: i32) {
    if btree.clone().unwrap().key == -1 {
        panic!("The tree is empty !");
    } else {
        delete_bis(btree.as_mut().unwrap(), key);
    }
}


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

fn main() {
    let mut btree = Some(Box::new(Btree::new(10, None, None)));

    insert(&mut btree, 5);
    println!("Insert (node : 5) : {}", parcours_profondeur(&mut btree, String::new()));
    insert(&mut btree, 20);
    println!("Insert (node : 20) : {}", parcours_profondeur(&mut btree, String::new()));
    insert(&mut btree, 3);
    println!("Insert (node : 3) : {}", parcours_profondeur(&mut btree, String::new()));
    insert(&mut btree, 7);
    println!("Insert (node : 7) : {}", parcours_profondeur(&mut btree, String::new()));
    insert(&mut btree, 14);
    println!("Insert (node : 14) : {}", parcours_profondeur(&mut btree, String::new()));
    insert(&mut btree, 22);
    println!("Insert (node : 22): {}", parcours_profondeur(&mut btree, String::new()));
    println!("BFS : {}", parcours_largeur(&mut btree, String::new()));

    delete(&mut btree, 7);
    println!("Delete (node : 7) : {}", parcours_profondeur(&mut btree, String::new()));
    delete(&mut btree, 20);
    println!("Delete (node : 20) : {}", parcours_profondeur(&mut btree, String::new()));
    println!("BFS : {}", parcours_largeur(&mut btree, String::new()));
}

#[cfg(test)]
mod insert_tests {
    use crate::{Btree, insert, parcours_profondeur};

    #[test]
    fn test_parcours_profondeur_alone() {
        let mut btree = Some(Box::new(Btree::new(-1, None, None)));
        insert(&mut btree, 10);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "10 ");
    }


    #[test]
    fn test_parcours_profondeur_special() {
        let mut btree = Some(Box::new(Btree::new(-1, None, None)));
        insert(&mut btree, 1);
        insert(&mut btree, 2);
        insert(&mut btree, 3);
        insert(&mut btree, 4);
        insert(&mut btree, 5);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "1 2 3 4 5 ");
    }

    #[test]
    fn test_parcours_profondeur_same_value() {
        let mut btree = Some(Box::new(Btree::new(10, None, None)));
        insert(&mut btree, 5);
        insert(&mut btree, 20);
        insert(&mut btree, 3);
        insert(&mut btree, 7);
        insert(&mut btree, 14);
        insert(&mut btree, 22);
        insert(&mut btree, 22);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "10 5 3 7 20 14 22 22 ");
    }


    #[test]
    fn test_parcours_profondeur() {
        let mut btree = Some(Box::new(Btree::new(10, None, None)));
        insert(&mut btree, 5);
        insert(&mut btree, 20);
        insert(&mut btree, 3);
        insert(&mut btree, 7);
        insert(&mut btree, 14);
        insert(&mut btree, 22);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "10 5 3 7 20 14 22 ");
    }
}

#[cfg(test)]
mod delete_tests {
    use crate::{Btree, insert, delete, parcours_profondeur};

    #[test]
    fn test_parcours_profondeur() {
        let mut btree = Some(Box::new(Btree::new(10, None, None)));
        insert(&mut btree, 5);
        insert(&mut btree, 20);
        insert(&mut btree, 3);
        insert(&mut btree, 7);
        insert(&mut btree, 14);
        insert(&mut btree, 22);
        delete(&mut btree, 20);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "10 5 3 7 14 22 ");
    }

    #[test]
    fn test_parcours_profondeur_special() {
        let mut btree = Some(Box::new(Btree::new(-1, None, None)));
        insert(&mut btree, 1);
        insert(&mut btree, 2);
        insert(&mut btree, 3);
        insert(&mut btree, 4);
        insert(&mut btree, 5);
        delete(&mut btree, 2);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "1 3 4 5 ");
    }

    #[test]
    fn test_parcours_profondeur_delete_root() {
        let mut btree = Some(Box::new(Btree::new(10, None, None)));
        insert(&mut btree, 5);
        insert(&mut btree, 20);
        insert(&mut btree, 3);
        insert(&mut btree, 7);
        insert(&mut btree, 14);
        insert(&mut btree, 22);
        delete(&mut btree, 10);
        assert_eq!(parcours_profondeur(&mut btree, String::new()), "7 5 3 20 14 22 ");
    }
}