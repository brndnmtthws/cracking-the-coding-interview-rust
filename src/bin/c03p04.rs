#[derive(Debug)]
struct MyQueue<T> {
    arr: Vec<T>,
}

impl<T> MyQueue<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::clone::Clone,
{
    fn new() -> Self {
        MyQueue {
            arr: Vec::<T>::new(),
        }
    }

    fn add(&mut self, value: T) {
        self.arr.push(value.clone());
    }

    fn remove(&mut self) -> Option<T> {
        let mut rev = Vec::<T>::new();
        while !self.arr.is_empty() {
            rev.push(self.arr.pop().unwrap());
        }
        let ret = rev.pop();
        while !rev.is_empty() {
            self.arr.push(rev.pop().unwrap());
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut my_queue: MyQueue<i32> = MyQueue::new();
        for i in 0..10 {
            my_queue.add(i);
        }
        for i in 0..10 {
            assert_eq!(my_queue.remove().unwrap(), i);
        }
    }
}

fn main() {
    let mut my_queue: MyQueue<i32> = MyQueue::new();
    my_queue.add(1);
    my_queue.remove();
}
