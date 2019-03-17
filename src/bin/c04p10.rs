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

    fn trees_equal(&self, head: &NodeRef<T>, other_head: &NodeRef<T>) -> bool {
        if head.borrow().data != other_head.borrow().data {
            return false;
        }
        if let Some(left) = head.borrow().left.as_ref() {
            if let Some(other_left) = other_head.borrow().left.as_ref() {
                if !self.trees_equal(left, other_left) {
                    return false;
                }
            } else {
                return false;
            }
        } else if other_head.borrow().left.is_some() {
            return false;
        }
        if let Some(right) = head.borrow().right.as_ref() {
            if let Some(other_right) = other_head.borrow().right.as_ref() {
                return self.trees_equal(right, other_right);
            } else {
                return false;
            }
        } else if other_head.borrow().right.is_some() {
            return false;
        }
        true
    }

    fn find_head<F>(&self, head: &NodeRef<T>, node: &NodeRef<T>, f: &F) -> bool
    where
        F: Fn(&NodeRef<T>) -> bool,
    {
        if head.borrow().data == node.borrow().data {
            return f(node);
        }
        if let Some(left) = node.borrow().left.as_ref() {
            if self.find_head(head, left, f) {
                return true;
            }
        }
        if let Some(right) = node.borrow().right.as_ref() {
            return self.find_head(head, right, f);
        }
        false
    }

    fn is_subtree(&self, other: &BinaryTree<T>) -> bool {
        if let Some(head) = self.head.as_ref() {
            if let Some(other_head) = other.head.as_ref() {
                return self.find_head(head, other_head, &|root_node| {
                    self.trees_equal(head, root_node)
                });
            }
        }
        false
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
    fn test_is_subtree() {
        let mut t1 = BinaryTree::<i32>::new();
        t1.insert(2);
        t1.insert(1);
        t1.insert(3);

        let mut t2 = BinaryTree::<i32>::new();
        t2.insert(2);
        t2.insert(1);
        t2.insert(3);

        assert_eq!(t2.is_subtree(&t1), true);

        let mut t3 = BinaryTree::<i32>::new();
        t3.insert(5);
        t3.insert(6);
        t3.insert(8);
        assert_eq!(t3.is_subtree(&t1), false);
    }
}

fn main() {
    let mut t1 = BinaryTree::<i32>::new();
    t1.insert(2);
    t1.insert(1);
    t1.insert(3);

    let mut t2 = BinaryTree::<i32>::new();
    t2.insert(2);
    t2.insert(1);
    t2.insert(3);

    t2.is_subtree(&t1);
}
