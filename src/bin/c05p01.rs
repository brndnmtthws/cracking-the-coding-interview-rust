fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
    let mut ret = n;
    let mut m_mask = 0;
    for x in i..=j {
        m_mask |= 1 << x;
    }
    ret &= !m_mask;
    ret |= m << i;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_bits() {
        assert_eq!(insert_bits(0b100_0000_0000, 0b10011, 2, 6), 0b100_0100_1100);
    }
}

fn main() {
    insert_bits(0b100_0000_0000, 0b10011, 2, 6);
}
