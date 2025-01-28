fn is_rotated_string(s1: &str, s2: &str) -> bool {
    let triplet = format!("{}{}{}", s1, s1, s1);

    triplet.contains(s2)
}

fn main() {
    is_rotated_string("waterbottle", "erbottlewat");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_rotation() {
        assert!(is_rotated_string("waterbottle", "erbottlewat"));
        assert!(!is_rotated_string("herpderp", "lolbob"));
    }
}
