fn all_chars_unique_part_a(s: &str) -> bool {
    use std::collections::HashSet;
    let mut characters: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if characters.contains(&c) {
            return false;
        }
        characters.insert(c);
    }
    true
}

fn all_chars_unique_part_b(s: &str) -> bool {
    let mut bitfield: i64 = 0;
    let a_int_char: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char: i16 = c as i16;
        int_char -= a_int_char;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit
        bitfield |= 1 << int_char;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefga")), false);
    }
}

fn main() {
    all_chars_unique_part_a(&String::from("helloworld"));
    all_chars_unique_part_b(&String::from("helloworld"));
}
