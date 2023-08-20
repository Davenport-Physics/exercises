use std::cmp::Ord;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Debug> Node<T> {

    pub fn new(array: &[T]) -> Node<T> {

        let value: T = array[array.len() / 2];

        if array.len() == 1 {

            return Node {
                value: value,
                left: None,
                right: None
            }

        }

        let left: &[T]  = &array[..array.len()/2];
        let right: &[T] = &array[array.len()/2+1..];

        Node {
            value: value,
            left:  if left.len() == 0 { None } else { Some(Box::new(Node::new(&array[..array.len()/2]))) },
            right: if right.len() == 0 { None } else { Some(Box::new(Node::new(&array[array.len()/2..]))) }
        }

    }

    pub fn exists(&self, value: T) -> bool {

        if self.value == value {

            return true;

        } else if self.value > value {

            if let Some(left) = &self.left {
                return left.exists(value);
            } else {
                return false;
            }

        } else if self.value < value {

            if let Some(right) = &self.right {
                return right.exists(value);
            } else {
                return false;
            }

        }

        return false;

    }

}

struct BST<T> {
    root: Option<Node<T>>
}

impl<T: Ord + Copy + Debug> BST<T> {

    pub fn new() -> BST<T> {

        BST {
            root: None
        }

    }

    pub fn from(mut array: Vec<T>) -> BST<T> {

        if array.len() == 0 {
            panic!("Array must have at least one element. Use BST::new() instead.");
        }

        array.sort();
        remove_duplicates(&mut array);

        BST {
            root: Some(Node::new(&array))
        }

    }
    pub fn exists(&self, value: T) -> bool {

        if let Some(root) = &self.root {
            return root.exists(value);
        }
        false

    }

    pub fn delete(&mut self, value: T) {

        todo!();

    }

    pub fn insert(&mut self, value: T) {

        todo!();

    }

}

fn main() {

    let bst = BST::from(vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8]);
    println!("{}", bst.exists(1));

}

pub fn remove_duplicates<T: Ord>(array: &mut Vec<T>) {

    let mut idx = 0;
    loop {

        if array.len() == idx + 1 {
            break;
        }

        if array[idx] == array[idx + 1] {
            array.remove(idx + 1);
        } else {
            idx += 1;
        }

    }

}

#[cfg(test)]
mod tests {

    use super::BST;

    #[test]
    pub fn test_remove_duplicates() {

        let mut array = vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8];

        array.sort();
        super::remove_duplicates(&mut array);
        assert!(array == vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    }

    #[test]
    pub fn test_bst_bad_val() {

        let bst = BST::from(vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8]);
        assert!(!bst.exists(11));

    }

    #[test]
    pub fn test_bst_good_val() {

        let bst = BST::from(vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8]);
        assert!(bst.exists(1));

    }

}



