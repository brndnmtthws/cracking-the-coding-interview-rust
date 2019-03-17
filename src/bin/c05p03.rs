fn flip_bit_to_win(n: i32) -> i32 {
    let mut ret = 0;
    for i in 0..32 {
        let mut bits = n >> i;
        let mut flipped_zero = false;
        let mut ones = 0;
        while bits != 0 {
            if 1 & bits == 0 {
                if flipped_zero {
                    break;
                } else {
                    flipped_zero = true;
                }
            }
            ones += 1;
            bits >>= 1;
        }
        ret = std::cmp::max(ret, ones);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_bit_to_win() {
        assert_eq!(flip_bit_to_win(0b1_1011), 5);
        assert_eq!(flip_bit_to_win(0b11_0001), 3);
    }
}

fn main() {
    flip_bit_to_win(0b11_0001);
}
