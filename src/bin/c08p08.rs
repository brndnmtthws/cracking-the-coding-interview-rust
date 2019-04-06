fn permutations_with_dups(s: &str) -> Vec<String> {
    if s.is_empty() {
        vec![]
    } else if s.len() == 1 {
        vec![s.to_string()]
    } else if s.len() == 2 {
        vec![s.to_string(), s.chars().rev().collect()]
    } else {
        let subperms = permutations_with_dups(&s[1..]);
        let item = s.chars().next().unwrap();
        subperms
            .iter()
            .flat_map(|substr| {
                let mut new_subperms: Vec<String> = vec![];
                for (idx, _) in substr.chars().enumerate() {
                    let mut s = substr.clone();
                    s.insert(idx, item);
                    new_subperms.push(s);
                }
                let mut s = substr.clone();
                s.push(item);
                new_subperms.push(s);
                new_subperms
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_with_dups() {
        let s1 = "abb";
        let permutations = vec!["abb", "bab", "bba", "abb", "bab", "bba"];
        assert_eq!(permutations_with_dups(&s1), permutations);
    }
}

fn main() {
    let s1 = "abc";
    permutations_with_dups(&s1);
}
