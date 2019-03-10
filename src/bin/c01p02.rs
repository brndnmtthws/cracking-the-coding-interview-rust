use std::collections::HashMap;
fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut characters: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let x = characters.entry(c).or_insert(0);
        *x += 1;
    }
    characters
}

fn is_permutation(s1: &str, s2: &str) -> bool {
    let characters_1 = count_chars(s1);
    let characters_2 = count_chars(s2);

    for key in characters_1.keys() {
        if !characters_2.contains_key(&key) {
            return false;
        }
        if characters_1.get(&key) != characters_2.get(&key) {
            return false;
        }
    }
    for key in characters_2.keys() {
        if !characters_1.contains_key(&key) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("tac")),
            true
        );
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("dog")),
            false
        );
    }
}

fn main() {
    is_permutation(&String::from("cat"), &String::from("dog"));
}
