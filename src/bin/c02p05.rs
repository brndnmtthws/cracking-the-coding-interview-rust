use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

struct Iter<T> {
    next: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    fn tail(node: &NodeRef<T>) -> Option<NodeRef<T>> {
        if let Some(cur) = node.borrow().next.as_ref().cloned() {
            return Node::tail(&cur.clone());
        }
        Some(node.clone())
    }
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq,
    T: std::hash::Hash,
    T: std::clone::Clone,
    T: std::cmp::PartialOrd,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&mut self, new_value: T) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: new_value,
                next: None,
            })));
        }
    }

    fn tail(&self) -> Option<NodeRef<T>> {
        if let Some(cur) = self.head.as_ref().cloned() {
            if cur.borrow().next.is_none() {
                return Some(cur.clone());
            } else {
                return Node::tail(&cur.clone());
            }
        }
        None
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = cur.borrow().next.clone();
            return Some(cur.clone());
        }
        None
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")
    }
}

fn sum_backward(left: &LinkedList<i32>, right: &LinkedList<i32>) -> LinkedList<i32> {
    let mut left_sum = 0;
    for (n, node) in left.iter().enumerate() {
        left_sum += node.borrow().data * 10_i32.pow(n as u32);
    }

    let mut right_sum = 0;
    for (n, node) in right.iter().enumerate() {
        right_sum += node.borrow().data * 10_i32.pow(n as u32);
    }
    let sum = left_sum + right_sum;

    let mut new_list = LinkedList::<i32>::new();

    let mut digits = 0;
    while 10_i32.pow(digits) <= sum {
        digits += 1;
    }

    for n in 0..digits {
        let mut digit_value = sum / 10_i32.pow(n);
        digit_value %= 10;
        new_list.append(digit_value);
    }

    new_list
}

fn sum_forward(left: &LinkedList<i32>, right: &LinkedList<i32>) -> LinkedList<i32> {
    let mut left_sum = 0;
    let left_vec: Vec<i32> = left.iter().map(|node| node.borrow().data).collect();
    for (n, value) in left_vec.iter().rev().enumerate() {
        left_sum += value * 10_i32.pow(n as u32);
    }

    let mut right_sum = 0;
    let right_vec: Vec<i32> = right.iter().map(|node| node.borrow().data).collect();
    for (n, value) in right_vec.iter().rev().enumerate() {
        right_sum += value * 10_i32.pow(n as u32);
    }
    let sum = left_sum + right_sum;

    let mut new_list = LinkedList::<i32>::new();

    let mut digits = 0;
    while 10_i32.pow(digits) <= sum {
        digits += 1;
    }

    for n in (0..digits).rev() {
        let mut digit_value = sum / 10_i32.pow(n);
        digit_value %= 10;
        new_list.append(digit_value);
    }

    new_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_backward() {
        let mut left = LinkedList::<i32>::new();
        left.append(7);
        left.append(1);
        left.append(6);

        let mut right = LinkedList::<i32>::new();
        right.append(5);
        right.append(9);
        right.append(2);

        let result = sum_backward(&left, &right);
        let mut iter = result.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 2);
        assert_eq!(iter.next().unwrap().borrow().data, 1);
        assert_eq!(iter.next().unwrap().borrow().data, 9);
    }

    #[test]
    fn test_sum_forward() {
        let mut left = LinkedList::<i32>::new();
        left.append(6);
        left.append(1);
        left.append(7);

        let mut right = LinkedList::<i32>::new();
        right.append(2);
        right.append(9);
        right.append(5);

        let result = sum_forward(&left, &right);
        let mut iter = result.iter();
        assert_eq!(iter.next().unwrap().borrow().data, 9);
        assert_eq!(iter.next().unwrap().borrow().data, 1);
        assert_eq!(iter.next().unwrap().borrow().data, 2);
    }
}

fn main() {
    let mut left = LinkedList::<i32>::new();
    left.append(6);

    let mut right = LinkedList::<i32>::new();
    right.append(2);
    sum_backward(&left, &right);
    sum_forward(&left, &right);
}
