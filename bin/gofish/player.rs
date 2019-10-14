use super::*;
use std::io;

pub use PlayerType::*;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub paired: Hand,
    pub player_type: PlayerType,
}

impl Player {
    pub fn new(name: &str, player_type: PlayerType) -> Self {
        Player {
            name: name.into(),
            hand: Hand::new(),
            paired: Hand::new(),
            player_type,
        }
    }

    pub fn discard_pairs(&mut self) -> DealResult<usize> {
        let mut pairs: HashMap<Value, usize> = HashMap::new();

        for card in self.hand.cards() {
            *pairs.entry(card.value).or_insert(0) += 1;
        }

        for (value, count) in pairs.iter().filter(|(_value, count)| **count > 1_usize) {
            let len = count - (count % 2);
            for _i in 0..len {
                let index = self
                    .hand
                    .cards()
                    .position(|card| card.value == *value)
                    .expect("The card should be here!");
                self.hand.deal(index, &mut self.paired)?;
            }
        }

        Ok(pairs.len())
    }

    pub fn has_value<V: Into<Value> + Copy>(&self, value: V) -> bool {
        self.hand.cards().any(|c| c.value == value.into())
    }

    pub fn match_cards_from_value<V: Into<Value> + Copy>(&self, value: V) -> MatchIndex<Card> {
        self.hand
            .cards()
            .enumerate()
            .filter(|(_, card)| card.value == value.into())
            .map(|(i, card)| (i, card.clone()))
            .collect()
    }

    pub fn say<W: Write, D: fmt::Display>(&self, w: &mut W, d: D) -> io::Result<()> {
        write!(w, "{}: \"{}\"", self.name, d)
    }
}

impl Take for Player {
    type Item = Card;
    fn take(&mut self, card: Card) {
        self.hand.take(card);
    }
}

impl Give for Player {
    type Item = Card;
    fn give(&mut self, index: usize) -> DealResult<Self::Item> {
        self.hand.give(index)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlayerType {
    Human,
    Computer,
}

pub struct PlayerIndex {
    pub current: usize,
    pub next: usize,
    pub count: usize,
}

impl PlayerIndex {
    pub fn new(count: usize) -> Self {
        PlayerIndex {
            current: 0,
            next: 1,
            count,
        }
    }
}

impl PlayerIndex {
    pub fn increment(&mut self) {
        self.current = self.next;
        self.next = (self.next + 1) % self.count;
    }
}
