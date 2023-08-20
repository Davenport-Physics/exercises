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
    root: Node<T>
}

impl<T: Ord + Copy + Debug> BST<T> {

    pub fn new(mut array: Vec<T>) -> BST<T> {

        array.sort();
        remove_duplicates(&mut array);

        BST {
            root: Node::new(&array)
        }

    }

    pub fn exists(&self, value: T) -> bool {

        self.root.exists(value)

    }

}

fn main() {

    let bst = BST::new(vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8]);
    println!("{}", bst.exists(1));

}

pub fn remove_duplicates<T: Ord>(array: &mut Vec<T>) {

    for i in 0..array.len() {

        for j in (i+1..array.len()).rev() {

            if array[i] == array[j] {
                array.remove(j);
            }

        }

    }

}

#[test]
pub fn test_remove_duplicates() {

    let mut array = vec![1, 1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8];

    array.sort();
    remove_duplicates(&mut array);
    assert!(array == vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

}

