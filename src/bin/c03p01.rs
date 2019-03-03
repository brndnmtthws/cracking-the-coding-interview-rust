#[derive(Debug)]
struct Pointer {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct ThreeInOne<T> {
    arr: Vec<T>,
    pointers: Vec<Pointer>,
}

impl<T> ThreeInOne<T> {
    fn new() -> Self {
        ThreeInOne {
            arr: Vec::<T>::new(),
            pointers: vec![],
        }
    }

    fn push(&mut self, stack: usize, value: T) {
        let last_end = if self.pointers.is_empty() {
            0
        } else {
            self.pointers.last().unwrap().end
        };
        if self.pointers.len() < stack + 1 {
            self.pointers.resize_with(stack + 1, || Pointer {
                start: last_end,
                end: last_end,
            });
        }

        self.arr.insert(self.pointers[stack].start, value);
        self.pointers[stack].end += 1;
        for pointer in self.pointers.iter_mut().skip(stack + 1) {
            pointer.start += 1;
            pointer.end += 1;
        }
    }

    fn pop(&mut self, stack: usize) -> Option<T> {
        if self.pointers.len() < stack + 1 || self.pointers[stack].start == self.pointers[stack].end
        {
            return None;
        }
        let result = self.arr.remove(self.pointers[stack].start);
        self.pointers[stack].end -= 1;
        for pointer in self.pointers.iter_mut().skip(stack + 1) {
            pointer.start -= 1;
            pointer.end -= 1;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threeinone() {
        let mut three_in_one: ThreeInOne<i32> = ThreeInOne::new();
        three_in_one.push(0, 1);
        three_in_one.push(1, 2);
        three_in_one.push(2, 3);
        assert_eq!(three_in_one.pop(2).unwrap(), 3);
        assert_eq!(three_in_one.pop(1).unwrap(), 2);
        assert_eq!(three_in_one.pop(0).unwrap(), 1);

        three_in_one.push(0, 4);
        three_in_one.push(1, 5);
        three_in_one.push(2, 6);
        three_in_one.push(0, 7);
        three_in_one.push(1, 8);
        three_in_one.push(2, 9);
        assert_eq!(three_in_one.pop(2).unwrap(), 9);
        assert_eq!(three_in_one.pop(1).unwrap(), 8);
        assert_eq!(three_in_one.pop(0).unwrap(), 7);
    }
}

fn main() {
    let mut three_in_one: ThreeInOne<i32> = ThreeInOne::new();
    three_in_one.push(0, 1);
    three_in_one.pop(0);
}
