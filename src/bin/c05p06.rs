fn count_ones(n: i32) -> i32 {
    let mut count = 0;
    let mut tmp = n;
    while tmp != 0 {
        count += tmp & 1;
        tmp >>= 1;
    }
    count
}

fn conversion_count(a: i32, b: i32) -> i32 {
    let xor = a ^ b;
    count_ones(xor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(conversion_count(0b1_1101, 0b0_1111), 2);
    }
}

fn main() {
    conversion_count(0b1_1101, 0b0_1111);
}
