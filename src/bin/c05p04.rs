fn count_ones(n: i32) -> i32 {
    let mut count = 0;
    let mut tmp = n;
    while tmp != 0 {
        count += tmp & 1;
        tmp >>= 1;
    }
    count
}

fn next_number(n: i32) -> (i32, i32) {
    let mut next_smallest = 0;
    let mut next_largest = 0;
    let mut c = n + 1;
    let one_count = count_ones(n);
    while c < std::i32::MAX {
        if count_ones(c) == one_count {
            next_largest = c;
            break;
        }
        c += 1;
    }
    c = n - 1;
    while c > 0 {
        if count_ones(c) == one_count {
            next_smallest = c;
            break;
        }
        c -= 1;
    }
    (next_smallest, next_largest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_number() {
        assert_eq!(next_number(0b100), (0b10, 0b1000));
    }
}

fn main() {
    next_number(0b100);
}
