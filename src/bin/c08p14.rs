fn count_eval(s: &str, result: bool) -> u64 {
    if s.is_empty() {
        0
    } else if s.len() == 1 {
        if (s == "1") == result {
            1
        } else {
            0
        }
    } else {
        let mut ways: u64 = 0;
        for (i, c) in s.char_indices().skip(1).step_by(2) {
            let left = &s[..i];
            let right = &s[(i + 1)..];

            let left_true = count_eval(left, true);
            let left_false = count_eval(left, false);
            let right_true = count_eval(right, true);
            let right_false = count_eval(right, false);

            let total = (left_true + left_false) * (right_true + right_false);

            let total_true = if c == '^' {
                left_true * right_false + left_false * right_true
            } else if c == '&' {
                left_true * right_true
            } else if c == '|' {
                left_true * right_true + left_false * right_true + left_true * right_false
            } else {
                0
            };

            ways += if result {
                total_true
            } else {
                total - total_true
            }
        }
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_evaluation() {
        assert_eq!(count_eval("1^0|0|1", false), 2);
        assert_eq!(count_eval("0&0&0&1^1|0", true), 10);
    }
}

fn main() {
    count_eval("1^0|0|1", false);
}
