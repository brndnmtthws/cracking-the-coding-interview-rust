fn urlify(url: &'static str) -> String {
    let placeholder = "%20";

    url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s)
        } else {
            acc + placeholder + s
        }
    })
}

// Alternative solution with shorter syntax
fn urlify_2(url: &str) -> String {
    url.trim().replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&"Mr John Smith    "), "Mr%20John%20Smith");
    }
    #[test]
    fn test_urlify_2() {
        assert_eq!(urlify_2(&"Mr John Smith    "), "Mr%20John%20Smith");
    }
}

fn main() {
    urlify(&"Mr John Smith    ");
    urlify_2(&"Mr John Smith    ");
}
