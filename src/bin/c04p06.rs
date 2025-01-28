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
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::marker::Copy,
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
            Self::insert_at(&mut head, ret)
        }
    }

    fn insert_at(parent_node: &mut NodeRef<T>, new_node: NodeRef<T>) -> NodeRef<T> {
        if new_node.borrow().data < parent_node.borrow().data {
            if parent_node.borrow().left.is_some() {
                let mut new_parent = parent_node.borrow_mut().left.as_mut().unwrap().clone();
                Self::insert_at(&mut new_parent, new_node)
            } else {
                parent_node.borrow_mut().left = Some(new_node.clone());
                new_node
            }
        } else if parent_node.borrow().right.is_some() {
            let mut new_parent = parent_node.borrow_mut().right.as_mut().unwrap().clone();
            Self::insert_at(&mut new_parent, new_node)
        } else {
            parent_node.borrow_mut().right = Some(new_node.clone());
            new_node
        }
    }

    fn visit_from<F>(parent_node: &NodeRef<T>, f: &mut F)
    where
        F: FnMut(&NodeRef<T>),
    {
        f(parent_node);
        if let Some(left) = parent_node.borrow().left.as_ref() {
            Self::visit_from(left, f);
        }
        if let Some(right) = parent_node.borrow().right.as_ref() {
            Self::visit_from(right, f);
        }
    }

    fn visit_all<F>(&self, mut f: F)
    where
        F: FnMut(&NodeRef<T>),
    {
        if self.head.is_some() {
            Self::visit_from(self.head.as_ref().unwrap(), &mut f)
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

    fn dfs_from<F>(f: &mut F, node: NodeRef<T>)
    where
        F: FnMut(&NodeRef<T>),
    {
        if let Some(left) = node.borrow().left.as_ref() {
            Self::dfs_from(f, left.clone());
        }
        f(&node);
        if let Some(right) = node.borrow().right.as_ref() {
            Self::dfs_from(f, right.clone());
        }
    }

    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(&NodeRef<T>),
    {
        if let Some(head) = &self.head {
            Self::dfs_from(&mut f, head.clone());
        }
    }
}

impl<T: Display> Display for BinaryTree<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::marker::Copy,
{
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        self.visit_all(|v| {
            write!(w, "{}, ", v.borrow().data).unwrap();
        });
        write!(w, "]")
    }
}

fn main() {
    let mut binary_tree = BinaryTree::<i32>::new();
    let arr: Vec<i32> = (0..10).collect();
    binary_tree.add_vector(&arr);
    binary_tree.dfs(|node| println!("{}", node.borrow().data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_successor() {
        let mut balanced_binary_tree = BinaryTree::<i32>::new();
        let arr: Vec<i32> = (0..10).collect();
        balanced_binary_tree.add_vector(&arr);
        let mut iter = arr.iter();
        balanced_binary_tree.dfs(|node| {
            let value = iter.next();
            assert_eq!(&node.borrow().data, value.unwrap());
        });
    }
}
