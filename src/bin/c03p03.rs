#[derive(Debug)]
struct StackSet<T> {
    arr: Vec<Vec<T>>,
    capacity: usize,
}

impl<T> StackSet<T>
where
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
    T: std::clone::Clone,
{
    fn new(capacity: usize) -> Self {
        StackSet {
            capacity,
            arr: vec![Vec::<T>::with_capacity(capacity)],
        }
    }

    fn push(&mut self, value: T) {
        if self.arr.last().as_ref().unwrap().len() >= self.capacity {
            self.arr.push(Vec::<T>::with_capacity(self.capacity));
        }
        if let Some(a) = self.arr.last_mut() {
            a.push(value.clone());
        }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(a) = self.arr.last_mut() {
            let ret = a.pop();
            if self.arr.len() > 1 && self.arr.last().as_ref().unwrap().is_empty() {
                self.arr.pop();
            }
            ret
        } else {
            None
        }
    }

    fn pop_at(&mut self, index: usize) -> Option<T> {
        if index < self.arr.len() {
            if let Some(a) = self.arr.get_mut(index) {
                let ret = a.pop();
                if self.arr[index].is_empty() {
                    self.arr.remove(index);
                }
                ret
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_of_stacks() {
        let mut set_of_stacks: StackSet<i32> = StackSet::new(10);
        for i in 0..100 {
            set_of_stacks.push(i);
        }
        for i in (0..100).rev() {
            let res = set_of_stacks.pop().unwrap();
            assert_eq!(i, res);
        }
        for i in 0..20 {
            set_of_stacks.push(i);
        }
        let pop_at_res = set_of_stacks.pop_at(0).unwrap();
        assert_eq!(pop_at_res, 9);
        for i in (0..20).rev() {
            if i == 9 {
                continue;
            }
            let res = set_of_stacks.pop().unwrap();
            assert_eq!(i, res);
        }
    }
}

fn main() {
    let mut set_of_stacks: StackSet<i32> = StackSet::new(10);
    set_of_stacks.push(1);
    set_of_stacks.push(1);
    set_of_stacks.push(1);
    set_of_stacks.pop_at(0);
    set_of_stacks.push(1);
    set_of_stacks.pop();
}
