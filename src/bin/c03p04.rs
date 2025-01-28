#[derive(Debug)]
struct MyQueue<T> {
    arr: Vec<T>,
}

impl<T> MyQueue<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
{
    fn new() -> Self {
        MyQueue {
            arr: Vec::<T>::new(),
        }
    }

    fn add(&mut self, value: T) {
        self.arr.push(value);
    }

    fn remove(&mut self) -> Option<T> {
        let mut rev = Vec::<T>::new();
        while let Some(element) = self.arr.pop() {
            rev.push(element);
        }
        let ret = rev.pop();
        while let Some(element) = rev.pop() {
            self.arr.push(element);
        }
        ret
    }
}

fn main() {
    let mut my_queue: MyQueue<i32> = MyQueue::new();
    my_queue.add(1);
    my_queue.remove();
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
