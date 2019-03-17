fn float_bits_to_str(n: f64) -> String {
    let bits = n.to_bits();
    let mut ret = String::new();
    for i in 0..64 {
        if (1 << (63 - i)) & bits != 0 {
            ret.push('1');
        } else {
            ret.push('0');
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_to_string() {
        assert_eq!(
            float_bits_to_str(0.72),
            "0011111111100111000010100011110101110000101000111101011100001010"
        );
    }
}

fn main() {
    float_bits_to_str(0.72);
}
