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
    T: std::cmp::PartialEq,
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

    fn has_cycle(&self) -> Option<NodeRef<T>> {
        let mut tortoise_iter = self.iter();
        let mut hare_iter = self.iter();
        let mut tortoise = tortoise_iter.next();
        hare_iter.next(); // start 1 iteration ahead of tortoise
        let mut hare = hare_iter.next();
        let mut prev_tortoise = tortoise.as_ref().unwrap().clone();
        let mut prev_hare = hare.as_ref().unwrap().clone();
        while hare.is_some() && tortoise.is_some() {
            if Rc::ptr_eq(hare.as_ref().unwrap(), tortoise.as_ref().unwrap()) {
                if Rc::ptr_eq(&prev_tortoise, &prev_hare) {
                    return Some(prev_tortoise.clone());
                } else {
                    return Some(hare.as_ref().unwrap().clone());
                }
            }
            hare = hare_iter.next();
            prev_hare = hare.as_ref().unwrap().clone();
            if hare.is_some() {
                hare = hare_iter.next();
            }
            prev_tortoise = tortoise.as_ref().unwrap().clone();
            tortoise = tortoise_iter.next();
        }
        None
    }
}

impl<'a, T> Iterator for Iter<T> {
    type Item = NodeRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.as_ref()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let datavec = vec!['A', 'B', 'C', 'D', 'E'];

        let mut cycle_list = LinkedList::<char>::new();
        for value in datavec.iter() {
            cycle_list.append(*value);
        }

        let mut list_iter = cycle_list.iter();
        list_iter.next();
        list_iter.next();
        let third_node = list_iter.next();
        cycle_list.tail().unwrap().borrow_mut().next = Some(third_node.unwrap().clone());

        let cycle_result = cycle_list.has_cycle();
        assert_eq!(cycle_result.is_some(), true);
        assert_eq!(cycle_result.as_ref().unwrap().borrow().data, 'C');

        let mut nocycle_list = LinkedList::<char>::new();
        for value in datavec.iter() {
            nocycle_list.append(*value);
        }

        let nocycle_result = nocycle_list.has_cycle();
        assert_eq!(nocycle_result.is_none(), true);

        // Second case
        let datavec2 = vec!['A', 'B', 'C', 'D', 'E', 'F'];

        let mut cycle_list2 = LinkedList::<char>::new();
        for value in datavec2.iter() {
            cycle_list2.append(*value);
        }

        let mut list_iter2 = cycle_list2.iter();
        list_iter2.next();
        list_iter2.next();
        let third_node2 = list_iter2.next();
        cycle_list2.tail().unwrap().borrow_mut().next = Some(third_node2.unwrap().clone());

        let cycle_result2 = cycle_list2.has_cycle();
        assert_eq!(cycle_result2.is_some(), true);
        assert_eq!(cycle_result2.as_ref().unwrap().borrow().data, 'C');
    }

}

fn main() {
    let mut left = LinkedList::<i32>::new();
    left.append(6);
    left.has_cycle();
}
