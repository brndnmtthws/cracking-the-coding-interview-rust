fn urlify(url: &str) -> String {
    url.trim().replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&"Mr John Smith    "), "Mr%20John%20Smith");
    }
}

fn main() {
    urlify(&"Mr John Smith    ");
}
