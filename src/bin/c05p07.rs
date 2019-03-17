fn bitwise_swap(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_swap() {
        let mut a = 0b110;
        let mut b = 0b001;
        bitwise_swap(&mut a, &mut b);
        assert_eq!(a, 0b001);
        assert_eq!(b, 0b110);
    }
}

fn main() {
    let mut a = 0b110;
    let mut b = 0b001;
    bitwise_swap(&mut a, &mut b);
}
