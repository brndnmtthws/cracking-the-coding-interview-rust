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

    fn get_bst_sequences_inner(&self, node: &NodeRef<T>) -> Vec<Vec<T>> {
        let mut final_seqs: Vec<Vec<T>> = vec![];
        if let Some(left) = node.borrow().left.as_ref() {
            if let Some(right) = node.borrow().right.as_ref() {
                let left_seqs = self.get_bst_sequences_inner(left);
                let right_seqs = self.get_bst_sequences_inner(right);
                for seq_l in left_seqs.iter() {
                    for seq_r in right_seqs.iter() {
                        let mut l_s: Vec<T> = vec![];
                        l_s.push(node.borrow().data);
                        l_s.append(&mut seq_l.clone());
                        l_s.append(&mut seq_r.clone());
                        final_seqs.push(l_s);
                        let mut r_s: Vec<T> = vec![];
                        r_s.push(node.borrow().data);
                        r_s.append(&mut seq_r.clone());
                        r_s.append(&mut seq_l.clone());
                        final_seqs.push(r_s);
                    }
                }
            } else {
                let left_seqs = self.get_bst_sequences_inner(left);
                for seq_l in left_seqs.iter() {
                    let mut l_s: Vec<T> = vec![];
                    l_s.push(node.borrow().data);
                    l_s.append(&mut seq_l.clone());
                    final_seqs.push(l_s);
                }
            }
        } else if let Some(right) = node.borrow().right.as_ref() {
            let right_seqs = self.get_bst_sequences_inner(right);
            for seq_r in right_seqs.iter() {
                let mut r_s: Vec<T> = vec![];
                r_s.push(node.borrow().data);
                r_s.append(&mut seq_r.clone());
                final_seqs.push(r_s);
            }
        } else {
            let mut s: Vec<T> = vec![];
            s.push(node.borrow().data);
            final_seqs.push(s)
        }
        final_seqs
    }

    fn get_bst_sequences(&self) -> Vec<Vec<T>> {
        if let Some(head) = self.head.as_ref() {
            self.get_bst_sequences_inner(head)
        } else {
            vec![]
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
    fn test_get_bst_sequences() {
        let mut binary_tree = BinaryTree::<i32>::new();
        binary_tree.insert(2);
        binary_tree.insert(1);
        binary_tree.insert(3);

        let sequences = binary_tree.get_bst_sequences();
        assert_eq!(sequences, vec![vec![2, 1, 3], vec![2, 3, 1],]);
    }
}

fn main() {
    let mut binary_tree = BinaryTree::<i32>::new();
    binary_tree.insert(2);
    binary_tree.insert(1);
    binary_tree.insert(3);

    let sequences = binary_tree.get_bst_sequences();
    assert_eq!(sequences, vec![vec![2, 1, 3], vec![2, 3, 1],]);
}
