//use std::error::Error;
//use std::io::{Error as IoError, ErrorKind};
use rand::thread_rng;
use rand::seq::SliceRandom;

mod display;

//type BoxErr = Box<dyn Error>;
//type Result<T> = std::result::Result<T, BoxErr>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            value: Value::Ace,
            suit: Suit::Spades,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            _ => panic!("You can't use '{}' for a card!", u)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn into_iter() -> AllSuits {
        AllSuits {
            suits: [
                Suit::Spades,
                Suit::Clubs,
                Suit::Hearts,
                Suit::Diamonds,
            ],
            index: 0,
        }
    }
}

struct AllSuits {
    suits: [Suit; 4],
    index: usize,
}

impl Iterator for AllSuits {
    type Item = Suit;
    fn next(&mut self) -> Option<Self::Item> {
        let suit = self.suits.get(self.index);
        self.index += 1;

        match suit {
            Some(s) => Some(*s),
            None => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Deck {
    cards: [Card; 52],
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = [Card::default(); 52];
        let mut index = 0;

        for suit in Suit::into_iter() {
            for value in 1..=13 {
                let card = Card {
                    value: Value::from(value),
                    suit,
                };

                cards[index] = card;
                index += 1;
            }
        }

        Deck { cards }
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter()
            .zip(other.cards.iter())
            .all(|(s, o)| s == o)
    }
}

impl Eq for Deck {}

impl Deck {
    pub fn new() -> Self {
        Deck::default()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn shuffled(mut self) -> Self{
        self.cards.shuffle(&mut thread_rng());
        self
    }
}

#[test]
fn default_deck() {
    let deck0 = Deck::default();
    println!("{:?}", &deck0);

    let mut deck1 = Deck::default();
    assert_eq!(deck0, deck1);

    deck1.shuffle();

    println!("{:?}", &deck1);

    // There is a 1/8.06e+67 chance that this will panic!
    assert_ne!(deck0, deck1);
}
