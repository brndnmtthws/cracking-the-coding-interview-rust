use std::collections::HashSet;
use std::rc::Rc;

struct LinkedList<T> {
    head: Option<NodeRef<T>>,
}

type NodeRef<T> = Box<Rc<Node<T>>>;

struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

struct IntoIter<T>(LinkedList<T>);

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&mut self, new_value: T) {
        let tail = self.tail();
        let new_node = Rc::new(Node::<T> {
            data: new_value,
            next: None,
        });
        // tail.next = Some(new_node);
    }

    fn tail(&mut self) -> Rc<Node<T>> {
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            cur = node.next.as_ref();
        }
        **cur.unwrap()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &***node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| Rc::get_mut(node).unwrap()),
        }
    }

    fn list_has_duplicates(&self) -> bool {
        false
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            dbg!("a");
            self.next = self.next.as_ref().map(|node| *node);
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            dbg!("x");
            self.next = node.next.as_mut().map(|node| Rc::get_mut(node).unwrap());
            &mut node.data
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_has_duplicates() {
        let mut list = LinkedList::<String>::new();
        list.append(String::from("item1"));
        list.append(String::from("item2"));

        dbg!(list.iter().next.map(|node| &node.data));
        for x in list.iter() {
            dbg!(x);
        }

        assert_eq!(list.list_has_duplicates(), false);
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.append(String::from("item1"));
    list.append(String::from("item2"));

    dbg!(list.iter().next.map(|node| &node.data));
    for x in list.iter() {
        dbg!(x);
    }
}
