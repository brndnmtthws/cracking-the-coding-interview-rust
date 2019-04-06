fn permutations_without_dups(s: &str) -> Vec<String> {
    if s.is_empty() {
        vec![]
    } else if s.len() == 1 {
        vec![s.to_string()]
    } else if s.len() == 2 {
        vec![s.to_string(), s.chars().rev().collect()]
    } else {
        let subperms = permutations_without_dups(&s[1..]);
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
    use std::collections::HashSet;
    use std::iter::FromIterator;

    fn hashset(data: &[String]) -> HashSet<String> {
        HashSet::from_iter(data.iter().cloned())
    }

    use super::*;

    #[test]
    fn test_permutations_without_dups() {
        let s1 = "abc";
        let permutations = vec!["abc", "bac", "bca", "acb", "cab", "cba"];
        assert_eq!(permutations_without_dups(&s1), permutations);

        let s2 = "abcdefg";
        let s2_perms = permutations_without_dups(&s2);
        let s2_set = hashset(&s2_perms);
        assert_eq!(s2_perms.len(), 5040);
        assert_eq!(s2_perms.len(), s2_set.len());
    }
}

fn main() {
    let s1 = "abc";
    permutations_without_dups(&s1);
}
