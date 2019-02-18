fn one_char_changed(s1: &str, s2: &str) -> bool {
    let mut had_edit = false;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 && !had_edit {
            had_edit = true;
        } else if c1 != c2 {
            return false;
        }
    }
    true
}

fn one_char_inserted(s1: &str, s2: &str) -> bool {
    assert!(s1.len() == s2.len() + 1);
    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < s1.len() && idx2 < s2.len() {
        let char1 = s1.get(idx1..=idx1).unwrap();
        let char2 = s2.get(idx2..=idx2).unwrap();
        if char1 == char2 {
            idx2 += 1;
        }
        idx1 += 1;
    }
    idx1 >= s1.len() - 1 && idx2 == s2.len()
}

fn is_one_edit_away(s1: &str, s2: &str) -> bool {
    if s1 == s2 {
        return true;
    }
    if ((s1.len() as i32) - (s2.len() as i32)).abs() > 1 {
        return false;
    }
    if s1.len() == s2.len() {
        return one_char_changed(s1, s2);
    }
    if s2.len() > s1.len() {
        return one_char_inserted(s2, s1);
    }
    one_char_inserted(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_edit_away() {
        assert_eq!(is_one_edit_away("pale", "pale"), true);
        assert_eq!(is_one_edit_away("pale", "ple"), true);
        assert_eq!(is_one_edit_away("pales", "pale"), true);
        assert_eq!(is_one_edit_away("pale", "bale"), true);
        assert_eq!(is_one_edit_away("pale", "bake"), false);

        assert_eq!(is_one_edit_away("pale", "8e9auh"), false);
        assert_eq!(is_one_edit_away("pale", ""), false);
        assert_eq!(is_one_edit_away("pale", "pale "), true);
    }
}

fn main() {
    is_one_edit_away("pale", "ple");
}
