#[derive(Debug)]
struct Stack<T> {
    arr: Vec<T>,
    min: Option<T>,
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
            min: None,
        }
    }

    fn min(&self) -> Option<&T> {
        self.min.as_ref()
    }

    fn push(&mut self, value: T) {
        if self.min.is_none() || value < *self.min.as_ref().unwrap() {
            self.min = Some(value.clone());
        }
        self.arr.push(value.clone());
    }

    fn pop(&mut self) -> Option<T> {
        let result = self.arr.pop();
        if self.arr.is_empty() {
            self.min = None
        } else {
            self.min = Some(self.arr.first().unwrap().clone());
            for value in self.arr.iter().skip(1) {
                if value < self.min.as_ref().unwrap() {
                    self.min = Some(value.clone());
                }
            }
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
