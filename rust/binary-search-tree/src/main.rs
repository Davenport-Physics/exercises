use std::cmp::Ord;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Debug> Node<T> {

    pub fn new(value: T) -> Node<T> {

        Node {
            value: value,
            left: None,
            right: None
        }

    }

    pub fn from(array: &[T]) -> Node<T> {

        let value: T = array[array.len() / 2];

        if array.len() == 1 {

            return Node::new(value);

        }

        let left: &[T]  = &array[..array.len()/2];
        let right: &[T] = &array[array.len()/2+1..];

        Node {
            value: value,
            left:  if left.len() == 0 { None } else { Some(Box::new(Node::from(left))) },
            right: if right.len() == 0 { None } else { Some(Box::new(Node::from(right))) }
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

    pub fn insert(&mut self, value: T) {

        if self.value == value {
            return;
        }

        if self.value > value {

            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }

        } else {
                    
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }

        }

    }

    pub fn delete(&mut self, value: T) {

        if self.value > value {

            if let Some(left) = &mut self.left {

                if value != left.value {
                    left.delete(value);
                    return;
                }

            }

            if let Some(left) = self.left.take() {
                self.left = self.delete_node(left);
            }

        } else if self.value < value {

            if let Some(right) = &mut self.right {

                if value != right.value {
                    right.delete(value);
                    return;
                }

            }

            if let Some(right) = self.right.take() {
                self.right = self.delete_node(right);
            }

        }

    }

    fn delete_node(&mut self, mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {

        let num_children = node.num_children();
        // previous take has already left a None in it's spot. Nothing else to be done.
        if num_children == 0 {

            return None;

        } else if num_children == 1 {

            if node.left.is_some() {
                return node.left;
            } else {
                return node.right;
            }

        }

        let mut root = node.left.as_mut().unwrap();
        while root.right.is_some() {
            root = root.right.as_mut().unwrap();
        }

        node.value = root.value;
        if let Some(left) = root.left.take() {
            *root = left;
        }

        Some(node)

    }

    pub fn traversal(&self) {

        if let Some(left) = &self.left {
            left.traversal();
        }

        println!("{:?}", self.value);

        if let Some(right) = &self.right {
            right.traversal();
        }

    }

    pub fn traversal_vec(&self) -> Vec<T> {
            
        let mut vec = Vec::new();

        if let Some(left) = &self.left {
            vec.extend(left.traversal_vec());
        }

        vec.push(self.value);

        if let Some(right) = &self.right {
            vec.extend(right.traversal_vec());
        }

        vec

    }

    pub fn traversal_vec_all_children(&self) -> Vec<T> {
            
        let mut vec = Vec::new();

        if let Some(left) = &self.left {
            vec.extend(left.traversal_vec());
        }

        if let Some(right) = &self.right {
            vec.extend(right.traversal_vec());
        }

        vec

    }

    pub fn num_children(&self) -> usize {

        let mut num_children = 0;

        if let Some(_) = &self.left {
            num_children += 1;
        }

        if let Some(_) = &self.right {
            num_children += 1;
        }

        num_children

    }


}

struct BST<T> {
    pub root: Option<Box<Node<T>>>
}

impl<T: Ord + Copy + Debug> BST<T> {

    pub fn new() -> BST<T> {

        BST {
            root: None
        }

    }

    pub fn from(mut array: Vec<T>) -> BST<T> {

        if array.len() == 0 {
            return BST::new();
        }

        array.sort();
        remove_duplicates(&mut array);

        BST {
            root: Some(Box::new(Node::from(&array)))
        }

    }

    pub fn exists(&self, value: T) -> bool {

        if let Some(root) = &self.root {
            return root.exists(value);
        }
        false

    }

    pub fn delete(&mut self, value: T) {

        if let Some(root) = &mut self.root {
            if root.value == value {

                let traversal_vec = root.traversal_vec_all_children();
                self.root = BST::from(traversal_vec).root;

            } else {

                root.delete(value);

            }
        }

    }

    pub fn insert(&mut self, value: T) {

        if let Some(root) = &mut self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(Node::from(&[value])));
        }

    }

    pub fn traversal(&self) {

        if let Some(root) = &self.root {
            root.traversal();
        }

    }

}

fn main() {

    let mut bst = BST::from(vec![1, 1, 2, 4, 4, 5, 6, 7, 8, 9, 10, 7, 8]);
    bst.insert(3);
    bst.delete(5);
    bst.delete(6);
    bst.traversal();

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



