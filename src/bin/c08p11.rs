fn make_change(amount: i64, denoms: &[i64], index: usize) -> i64 {
    if index >= denoms.len() - 1 {
        1
    } else {
        let denom_amount = denoms[index];
        let mut i = 0;
        let mut ways = 0;
        while i * denom_amount <= amount {
            ways += make_change(amount - i * denom_amount, denoms, index + 1);
            i += 1;
        }
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coins() {
        let denoms: Vec<i64> = vec![25, 10, 5, 1];
        assert_eq!(make_change(10, &denoms, 0), 4);
        assert_eq!(make_change(11, &denoms, 0), 4);
        assert_eq!(make_change(15, &denoms, 0), 6);
    }
}

fn main() {
    let denoms: Vec<i64> = vec![25, 10, 5, 1];
    make_change(10, &denoms, 0);
}
