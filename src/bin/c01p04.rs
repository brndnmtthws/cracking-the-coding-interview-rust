use std::collections::HashMap;
fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut characters: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        if characters.contains_key(&c) {
            if let Some(x) = characters.get_mut(&c) {
                *x += 1;
            }
        } else {
            characters.insert(c, 1);
        }
    }
    return characters;
}

fn palidrome_permutations(s: &str) -> bool {
    let normalized_s = s
        .to_lowercase()
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s);
    let character_counts = count_chars(&normalized_s);
    let is_even = normalized_s.len() % 2 == 0;
    let mut has_odd = false;

    for value in character_counts.values() {
        dbg!(format!(
            "is_even={} has_odd={} value={}",
            is_even, has_odd, value
        ));
        if value % 2 != 0 {
            if is_even || has_odd {
                return false;
            } else {
                has_odd = true;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(palidrome_permutations("Tact Coa"), true);
    }
}

fn main() {
    println!("hello");
}
