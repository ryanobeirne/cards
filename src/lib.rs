use std::convert::TryFrom;
use std::io;

mod game;
mod deal;
mod display;
mod cards;
mod shuffle;

pub use crate::cards::*;
pub use deal::*;
pub use game::*;
pub use shuffle::*;

/// The playing card
#[derive(Debug, Copy, Clone, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
        Card { value, suit, }
    }

    /// The value for comparing two cards
    pub fn cmp_value(&self) -> u8 {
        match self.value {
            Value::Two   => 2,
            Value::Three => 3,
            Value::Four  => 4,
            Value::Five  => 5,
            Value::Six   => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine  => 9,
            Value::Ten   => 10,
            Value::Jack  => 11,
            Value::Queen => 12,
            Value::King  => 13,
            Value::Ace   => 14,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp_value().cmp(&other.cmp_value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            value: Value::Ace,
            suit: Suit::Spades,
        }
    }
}

/// The face value of a card
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Value {
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

impl From<u8> for Value {
    fn from(u: u8) -> Value {
        match u {
            1 => Value::Ace,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            _ => panic!("You can't use '{}' for a card!", u),
        }
    }
}

impl TryFrom<char> for Value {
    type Error = io::Error;
    fn try_from(c: char) -> Result<Value, Self::Error> {
        let value = match c {
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            '0' => Value::Ten,
            'J'|'j' => Value::Jack,
            'Q'|'q' => Value::Queen,
            'K'|'k' => Value::King,
            'A'|'a'|'1' => Value::Ace,
            _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
        };

        Ok(value)
    }
}

/// The suit of a card
/// (♦Diamonds, ♣Clubs, ♥Hearts, ♠Spades)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    ///♦
    Diamonds,
    ///♣
    Clubs,
    ///♥
    Hearts,
    ///♠
    Spades,
}

#[test]
fn suit_ord() {
    use Suit::*;
    let mut cards = vec![Clubs, Spades, Diamonds, Hearts];
    cards.sort();
    assert_eq!(cards, vec![Diamonds, Clubs, Hearts, Spades]);
}

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();

        for suit in [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds].iter() {
            for value in 1..=13 {
                let card = Card::new(Value::from(value), *suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self.cards()
                .zip(other.cards())
                .all(|(s, o)| s == o)
    }
}

impl Eq for Deck {}

impl Deck {
    pub fn new() -> Self {
        Deck::default().shuffled()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

}

/// A hand of playing cards
#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn cards_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, Card> {
        self.cards.iter_mut()
    }

    pub fn drain(&mut self) -> Hand {
        Hand {
            cards: self.cards.drain(0..self.len()).collect()
        }
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self.cards()
                .zip(other.cards())
                .all(|(s, o)| s == o)
    }
}

impl Eq for Hand {}

/// Check that the default deck doesn't panic
#[test]
fn default_deck() {
    let deck0 = Deck::default();
    //println!("{:?}", &deck0);

    let mut deck1 = Deck::default();
    assert_eq!(deck0, deck1);

    deck1.shuffle();
    //println!("{:?}", &deck1);

    // There is a 1/8.06e+67 chance that this will panic!
    assert_ne!(deck0, deck1);
}

/// Check that there are 52 unique cards in a deck
#[test]
fn unique_52() {
    let deck = Deck::new().shuffled();
    assert_eq!(deck.cards.len(), 52);

    for index0 in 0..52 {
        for index1 in 0..52 {
            if index0 == index1 { continue }
            assert_ne!(deck.cards[index0], deck.cards[index1]);
        }
    }
}
