use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct BinaryTree<T> {
    head: Option<NodeRef<T>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<NodeRef<T>>,
    right: Option<NodeRef<T>>,
}

impl<T> BinaryTree<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::marker::Copy,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn insert(&mut self, value: T) -> NodeRef<T> {
        let ret = Rc::new(RefCell::new(Node {
            data: value,
            left: None,
            right: None,
        }));
        if self.head.is_none() {
            self.head = Some(ret.clone());
            ret
        } else {
            let mut head = self.head.as_mut().unwrap().clone();
            self.insert_at(&mut head, ret.clone())
        }
    }

    fn insert_at(&mut self, parent_node: &mut NodeRef<T>, new_node: NodeRef<T>) -> NodeRef<T> {
        if new_node.borrow().data < parent_node.borrow().data {
            if parent_node.borrow().left.is_some() {
                let mut new_parent = parent_node.borrow_mut().left.as_mut().unwrap().clone();
                self.insert_at(&mut new_parent, new_node)
            } else {
                parent_node.borrow_mut().left = Some(new_node.clone());
                new_node
            }
        } else if parent_node.borrow().right.is_some() {
            let mut new_parent = parent_node.borrow_mut().right.as_mut().unwrap().clone();
            self.insert_at(&mut new_parent, new_node)
        } else {
            parent_node.borrow_mut().right = Some(new_node.clone());
            new_node
        }
    }

    fn visit_from<F>(&self, parent_node: &NodeRef<T>, f: &mut F)
    where
        F: FnMut(&NodeRef<T>),
    {
        f(parent_node);
        if let Some(left) = parent_node.borrow().left.as_ref() {
            self.visit_from(left, f);
        }
        if let Some(right) = parent_node.borrow().right.as_ref() {
            self.visit_from(right, f);
        }
    }

    fn visit_all<F>(&self, mut f: F)
    where
        F: FnMut(&NodeRef<T>),
    {
        if self.head.is_some() {
            self.visit_from(self.head.as_ref().unwrap(), &mut f)
        }
    }

    fn add_vector(&mut self, arr: &[T]) {
        if arr.len() > 2 {
            let middle = arr.len() / 2;
            self.insert(arr[middle]);
            self.add_vector(&arr[0..middle]);
            self.add_vector(&arr[(middle + 1)..arr.len()]);
        } else {
            for i in arr {
                self.insert(*i);
            }
        }
    }

    fn height_inner(&self, node: &NodeRef<T>, height: usize) -> usize {
        let mut max_height = height;
        if let Some(left) = node.borrow().left.as_ref() {
            max_height = std::cmp::max(self.height_inner(left, height + 1), max_height);
        }
        if let Some(right) = node.borrow().right.as_ref() {
            max_height = std::cmp::max(self.height_inner(right, height + 1), max_height);
        }
        max_height
    }

    fn height(&self) -> usize {
        if let Some(head) = self.head.as_ref() {
            self.height_inner(head, 1)
        } else {
            0
        }
    }

    fn check_balanced_inner(&self, node: &NodeRef<T>) -> bool {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(left) = node.borrow().left.as_ref() {
            left_height = self.height_inner(left, 1);
        }
        if let Some(right) = node.borrow().right.as_ref() {
            right_height = self.height_inner(right, 1);
        }
        (left_height as i64 - right_height as i64).abs() <= 1
    }

    fn check_balanced(&self) -> bool {
        if let Some(head) = self.head.as_ref() {
            self.check_balanced_inner(head)
        } else {
            true
        }
    }
}

impl<T: Display> Display for BinaryTree<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::marker::Copy,
{
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        self.visit_all(|v| {
            write!(w, "{}, ", v.borrow().data).unwrap();
        });
        write!(w, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_balanced() {
        let mut balanced_binary_tree = BinaryTree::<i32>::new();
        let arr: Vec<i32> = (0..10).collect();
        balanced_binary_tree.add_vector(&arr);
        assert_eq!(balanced_binary_tree.height(), 4);
        assert_eq!(balanced_binary_tree.check_balanced(), true);

        let mut imbalanced_binary_tree = BinaryTree::<i32>::new();
        for i in arr {
            imbalanced_binary_tree.insert(i);
        }
        assert_eq!(imbalanced_binary_tree.height(), 10);
        assert_eq!(imbalanced_binary_tree.check_balanced(), false);
    }
}

fn main() {
    let mut binary_tree = BinaryTree::<i32>::new();
    let arr: Vec<i32> = (0..10).collect();
    binary_tree.add_vector(&arr);
    binary_tree.height();
    binary_tree.check_balanced();
}
