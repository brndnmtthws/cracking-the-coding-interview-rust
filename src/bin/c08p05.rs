fn recursive_multiply(a: i64, b: i64) -> i64 {
    if b == 0 {
        0
    } else if b < 0 {
        a - (a + recursive_multiply(a, b.abs()))
    } else if b > 1 {
        a + recursive_multiply(a, b - 1)
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_multiply() {
        assert_eq!(recursive_multiply(2, 0), 0);
        assert_eq!(recursive_multiply(2, 2), 4);
        assert_eq!(recursive_multiply(2, 3), 6);
        assert_eq!(recursive_multiply(2, 4), 8);
        assert_eq!(recursive_multiply(3, 2), 6);
        assert_eq!(recursive_multiply(3, 3), 9);
        assert_eq!(recursive_multiply(3, 4), 12);
        assert_eq!(recursive_multiply(2, -2), -4);
        assert_eq!(recursive_multiply(-2, -2), 4);
        assert_eq!(recursive_multiply(-2, 2), -4);
        assert_eq!(recursive_multiply(1, -1), -1);
        assert_eq!(recursive_multiply(-1, -1), 1);
        assert_eq!(recursive_multiply(-1, 1), -1);
        assert_eq!(recursive_multiply(1, 0), 0);
        assert_eq!(recursive_multiply(0, 1), 0);
    }
}

fn main() {
    recursive_multiply(2, 0);
}
