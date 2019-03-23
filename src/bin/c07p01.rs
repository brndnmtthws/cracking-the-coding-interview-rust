#[derive(Copy, Clone, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Clone, Debug)]
struct DeckOfCards {
    cards: Vec<Card>,
}

fn make_cards(suit: Suit) -> Vec<Card> {
    vec![
        Card {
            suit,
            rank: Rank::Two,
        },
        Card {
            suit,
            rank: Rank::Three,
        },
        Card {
            suit,
            rank: Rank::Four,
        },
        Card {
            suit,
            rank: Rank::Five,
        },
        Card {
            suit,
            rank: Rank::Six,
        },
        Card {
            suit,
            rank: Rank::Seven,
        },
        Card {
            suit,
            rank: Rank::Eight,
        },
        Card {
            suit,
            rank: Rank::Nine,
        },
        Card {
            suit,
            rank: Rank::Ten,
        },
        Card {
            suit,
            rank: Rank::Jack,
        },
        Card {
            suit,
            rank: Rank::Queen,
        },
        Card {
            suit,
            rank: Rank::King,
        },
        Card {
            suit,
            rank: Rank::Ace,
        },
    ]
}

impl DeckOfCards {
    fn new() -> DeckOfCards {
        let mut cards: Vec<Card> = vec![];
        cards.append(&mut make_cards(Suit::Clubs));
        cards.append(&mut make_cards(Suit::Diamonds));
        cards.append(&mut make_cards(Suit::Hearts));
        cards.append(&mut make_cards(Suit::Spades));
        DeckOfCards { cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_of_cards() {
        let deck_of_cards = DeckOfCards::new();
        assert_eq!(deck_of_cards.cards.len(), 52);
    }
}

fn main() {
    let _deck_of_cards = DeckOfCards::new();
}
