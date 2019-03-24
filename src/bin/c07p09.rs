struct CircularArray<T> {
    arr: Vec<T>,
    offset: usize,
}

struct Iter<'a, T: 'a> {
    circular_array: &'a CircularArray<T>,
    next: usize,
}

impl<'a, T> CircularArray<T>
where
    T: std::clone::Clone,
{
    fn new(arr: &[T]) -> Self {
        Self {
            arr: arr.to_vec(),
            offset: 0,
        }
    }

    fn rotate(&mut self, offset: usize) {
        self.offset += offset;
    }

    fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            circular_array: self,
            next: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.circular_array.arr.len() {
            self.next += 1;
            self.circular_array
                .arr
                .get(((self.next - 1) + self.circular_array.offset) % self.circular_array.arr.len())
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_array() {
        let mut arr = CircularArray::<char>::new(&['a', 'b', 'c']);
        let res1: Vec<char> = arr.iter().cloned().collect();
        assert_eq!(res1, vec!['a', 'b', 'c']);
        arr.rotate(1);
        let res2: Vec<char> = arr.iter().cloned().collect();
        assert_eq!(res2, vec!['b', 'c', 'a']);
    }
}

fn main() {
    let mut arr = CircularArray::<char>::new(&['a', 'b', 'c']);
    let _res1: Vec<char> = arr.iter().cloned().collect();
    arr.rotate(1);
}
