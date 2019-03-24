#[derive(Clone, Debug, PartialEq, Eq)]
struct User {
    name: String,
}

#[derive(Clone, Debug)]
struct Client {
    user: User,
    ip_address: String,
}

#[derive(Clone, Debug)]
struct Message {
    user: User,
    text: String,
}

#[derive(Clone, Debug)]
struct ChatServer {
    clients: Vec<Client>,
}

impl Message {
    fn new(from: &str, text: &str) -> Self {
        Message {
            user: User {
                name: from.to_string(),
            },
            text: text.to_string(),
        }
    }
}

impl Client {
    fn new(name: &str, ip_address: &str) -> Self {
        Client {
            user: User {
                name: name.to_string(),
            },
            ip_address: ip_address.to_string(),
        }
    }

    fn recv(&self, message: &Message) {
        println!("{}: {}", message.user.name, message.text);
    }
}

impl ChatServer {
    fn new() -> Self {
        ChatServer { clients: vec![] }
    }

    fn add_client(&mut self, client: Client) -> &Client {
        self.clients.push(client);
        self.clients.last().unwrap()
    }

    fn send(&self, message: &Message) {
        self.clients
            .iter()
            .filter(|c| c.user != message.user)
            .for_each(|c| c.recv(message));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_server() {
        let mut chat_server = ChatServer::new();

        chat_server.add_client(Client::new("Alice", "10.1.0.1"));
        chat_server.add_client(Client::new("Bob", "10.1.0.2"));

        chat_server.send(&Message::new("Alice", "Hello Bob"));
        chat_server.send(&Message::new("Bob", "Hello Alice"));
    }
}

fn main() {
    let mut chat_server = ChatServer::new();

    chat_server.add_client(Client::new("Alice", "10.1.0.1"));
    chat_server.add_client(Client::new("Bob", "10.1.0.2"));

    chat_server.send(&Message::new("Alice", "Hello Bob"));
    chat_server.send(&Message::new("Bob", "Hello Alice"));
}
