use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct User {
    name: String,
    password_hash: String,
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
}

#[derive(Debug, Clone)]
struct Library {
    books: Vec<Book>,
}

#[derive(Debug, Clone)]
struct OnlineReader {
    libraries: HashMap<User, Library>,
}

impl OnlineReader {
    fn new() -> Self {
        OnlineReader {
            libraries: HashMap::new(),
        }
    }

    fn add_user(&mut self, name: &str, password_hash: &str) -> User {
        let user = User {
            name: name.to_string(),
            password_hash: password_hash.to_string(),
        };
        if !self.libraries.contains_key(&user) {
            self.libraries
                .insert(user.clone(), Library { books: vec![] });
        }
        user
    }

    fn add_book(&mut self, user: &User, title: &str, author: &str) -> Option<&Book> {
        let book = Book {
            title: title.to_string(),
            author: author.to_string(),
        };
        if let Some(library) = self.libraries.get_mut(user) {
            library.books.push(book);
            return library.books.last();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online_book_reader() {
        let mut online_reader = OnlineReader::new();
        let user = online_reader.add_user("Bob", "catscatscats");
        let book = online_reader
            .add_book(&user, "Green Eggs and Ham", "Dr. Suess")
            .unwrap();
        assert_eq!(book.title, "Green Eggs and Ham");
        assert_eq!(book.author, "Dr. Suess");
        assert_eq!(
            online_reader.libraries[&user].books.first().unwrap().title,
            "Green Eggs and Ham"
        );
        assert_eq!(online_reader.libraries[&user].books.len(), 1);
        assert_eq!(online_reader.libraries.len(), 1);
    }
}

fn main() {
    let mut online_reader = OnlineReader::new();
    let user = online_reader.add_user("Bob", "catscatscats");
    let _book = online_reader
        .add_book(&user, "Green Eggs and Ham", "Dr. Suess")
        .unwrap();
}
