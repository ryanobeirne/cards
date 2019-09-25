use std::fmt;
use super::*;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl fmt::Display  for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Value::Two   => "2",
            Value::Three => "3",
            Value::Four  => "4",
            Value::Five  => "5",
            Value::Six   => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine  => "9",
            Value::Ten   => "10",
            Value::Jack  => "J",
            Value::Queen => "Q",
            Value::King  => "K",
            Value::Ace   => "A",
        })
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Clubs    => '♣',
            Suit::Diamonds => '♦',
            Suit::Hearts   => '♥',
            Suit::Spades   => '♠',
        })
    }
}

impl fmt::Debug for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cards_iter = self.cards.iter().peekable();
        let mut cards = String::new();

        while let Some(card) = cards_iter.next() {
            cards.push_str(&card.to_string());
            if let Some(_) = cards_iter.peek() {
                cards.push_str(", ");
            }
        }

        write!(f, "[{}]", cards)
    }
}
