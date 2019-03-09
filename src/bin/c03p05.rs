#[derive(Debug)]
struct SortStack<T> {
    arr: Vec<T>,
}

impl<T> SortStack<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::clone::Clone,
{
    fn new() -> Self {
        SortStack {
            arr: Vec::<T>::new(),
        }
    }

    fn push(&mut self, value: T) {
        if self.arr.is_empty() || value <= *self.arr.last().unwrap() {
            self.arr.push(value.clone());
        } else {
            let mut tmp = Vec::<T>::new();
            while !self.arr.is_empty() && self.arr.last().unwrap() < &value {
                tmp.push(self.arr.pop().unwrap());
            }
            self.arr.push(value.clone());
            while !tmp.is_empty() {
                self.arr.push(tmp.pop().unwrap());
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.arr.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_stack() {
        let mut sort_stack: SortStack<i32> = SortStack::new();
        for i in 0..10 {
            sort_stack.push(i);
        }
        for i in 0..10 {
            assert_eq!(sort_stack.pop().unwrap(), i);
        }
    }
}

fn main() {
    let mut sort_stack: SortStack<i32> = SortStack::new();
    sort_stack.push(1);
    sort_stack.pop();
}
