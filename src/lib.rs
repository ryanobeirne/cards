use rand::seq::SliceRandom;
use rand::thread_rng;

mod display;

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
            _ => panic!("You can't use '{}' for a card!", u),
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

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::new();

        for suit in [Suit::Spades, Suit::Clubs, Suit::Hearts, Suit::Diamonds].iter() {
            for value in 1..=13 {
                let card = Card {
                    value: Value::from(value),
                    suit: *suit,
                };

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
                .zip(other.cards.iter())
                .all(|(s, o)| s == o)
    }
}

impl Eq for Deck {}

impl Deck {
    pub fn new() -> Self {
        Deck::default()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn shuffled(mut self) -> Self {
        self.cards.shuffle(&mut thread_rng());
        self
    }

    pub fn cards<'a>(&'a self) -> Cards<'a> {
        Cards {
            cards: self.cards.iter().collect(),
            index: 0,
        }
    }
}

pub struct Cards<'a> {
    cards: Vec<&'a Card>,
    index: usize,
}

impl<'a> Iterator for Cards<'a> {
    type Item = &'a Card;
    fn next(&mut self) -> Option<Self::Item> {
        let card = self.cards.get(self.index);
        self.index += 1;

        match card {
            Some(c) => Some(*c),
            None => None,
        }
    }
}

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
