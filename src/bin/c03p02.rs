#[derive(Debug)]
struct Stack<T> {
    arr: Vec<T>,
    min: Vec<T>,
}

impl<T> Stack<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::clone::Clone,
{
    fn new() -> Self {
        Stack {
            arr: Vec::<T>::new(),
            min: Vec::<T>::new(),
        }
    }

    fn min(&self) -> Option<&T> {
        if self.min.is_empty() {
            None
        } else {
            self.min.last()
        }
    }

    fn push(&mut self, value: T) {
        if self.min.is_empty() || (!self.min.is_empty() && value <= *self.min.last().unwrap()) {
            self.min.push(value.clone());
        }
        self.arr.push(value.clone());
    }

    fn pop(&mut self) -> Option<T> {
        let result = self.arr.pop();
        if self.arr.is_empty() {
            self.min.clear();
        } else if result.is_some() && self.min.last().unwrap() == result.as_ref().unwrap() {
            self.min.pop();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_with_min() {
        let mut stack_with_min: Stack<i32> = Stack::new();
        stack_with_min.push(3);
        assert_eq!(*stack_with_min.min().unwrap(), 3);
        stack_with_min.push(2);
        assert_eq!(*stack_with_min.min().unwrap(), 2);
        stack_with_min.push(1);
        assert_eq!(*stack_with_min.min().unwrap(), 1);
        assert_eq!(stack_with_min.pop().unwrap(), 1);
        assert_eq!(*stack_with_min.min().unwrap(), 2);
        assert_eq!(stack_with_min.pop().unwrap(), 2);
        assert_eq!(*stack_with_min.min().unwrap(), 3);
        assert_eq!(stack_with_min.pop().unwrap(), 3);
        assert_eq!(stack_with_min.min().is_none(), true);
    }
}

fn main() {
    let mut stack_with_min: Stack<i32> = Stack::new();
    stack_with_min.push(1);
    stack_with_min.min();
    stack_with_min.pop();
}
