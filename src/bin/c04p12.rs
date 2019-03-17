extern crate num;

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct BinaryTree {
    head: Option<NodeRef>,
}

#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl BinaryTree {
    fn new() -> Self {
        Self { head: None }
    }

    fn insert(&mut self, value: i32) -> NodeRef {
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

    fn insert_at(&mut self, parent_node: &mut NodeRef, new_node: NodeRef) -> NodeRef {
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

    fn visit_from<F>(&self, parent_node: &NodeRef, f: &mut F)
    where
        F: FnMut(&NodeRef),
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
        F: FnMut(&NodeRef),
    {
        if self.head.is_some() {
            self.visit_from(self.head.as_ref().unwrap(), &mut f)
        }
    }

    fn paths_with_sum_inner(&self, node: &NodeRef, sum: i32, accumulation: i32) -> i32 {
        let mut paths = 0;
        let new_acc = accumulation + node.borrow().data;
        if new_acc == sum {
            paths += 1;
        }
        if let Some(left) = node.borrow().left.as_ref() {
            paths += self.paths_with_sum_inner(left, sum, new_acc);
            paths += self.paths_with_sum_inner(left, sum, 0);
        }
        if let Some(right) = node.borrow().right.as_ref() {
            paths += self.paths_with_sum_inner(right, sum, new_acc);
            paths += self.paths_with_sum_inner(right, sum, 0);
        }
        paths
    }

    fn paths_with_sum(&self, sum: i32) -> i32 {
        if let Some(head) = self.head.as_ref() {
            self.paths_with_sum_inner(head, sum, 0)
        } else {
            0
        }
    }
}

impl Display for BinaryTree {
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
    fn test_paths_with_sum() {
        let mut t1 = BinaryTree::new();
        t1.insert(2);
        t1.insert(1);
        t1.insert(3);
        t1.insert(-1);
        t1.insert(1);
        t1.insert(1);
        t1.insert(1);

        assert_eq!(t1.paths_with_sum(1), 15);
        assert_eq!(t1.paths_with_sum(2), 9);
        assert_eq!(t1.paths_with_sum(5), 2);
    }
}

fn main() {
    let mut t1 = BinaryTree::new();
    t1.insert(2);
    t1.insert(1);
    t1.insert(3);
    t1.paths_with_sum(1);
}
