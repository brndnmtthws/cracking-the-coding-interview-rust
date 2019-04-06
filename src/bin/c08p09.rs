fn get_parens(n: u32) -> Vec<String> {
    if n == 0 {
        vec![]
    } else if n == 1 {
        vec!["()".to_string()]
    } else if n == 2 {
        vec!["()()".to_string(), "(())".to_string()]
    } else {
        let subparens = get_parens(n - 1);
        subparens
            .iter()
            .flat_map(|substr| {
                let mut new_subparens: Vec<String> = vec![];
                new_subparens.push(format!("(){}", substr));
                // Handle special case of "()().." being in the substr
                let next_substr = format!("{}()", substr);
                if new_subparens.last().unwrap() != &next_substr {
                    new_subparens.push(next_substr);
                }
                new_subparens.push(format!("({})", substr));
                new_subparens
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parens() {
        assert_eq!(get_parens(0).is_empty(), true);
        assert_eq!(get_parens(1), vec!["()".to_string()]);
        assert_eq!(get_parens(2), vec!["()()".to_string(), "(())".to_string(),]);
        assert_eq!(
            get_parens(3),
            vec![
                "()()()".to_string(),
                "(()())".to_string(),
                "()(())".to_string(),
                "(())()".to_string(),
                "((()))".to_string(),
            ]
        );
    }
}

fn main() {
    get_parens(3);
}
