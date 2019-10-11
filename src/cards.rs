//! An iterator over a collection of cards

use super::*;

/// The iterator over a collectoin of cards
pub trait Cards<'a> {
    fn cards(&'a self) -> CardIter<'a>;

    fn card_count(&'a self) -> usize {
        self.cards().count()
    }

    /// Test if all cards are unique
    /// Returns true if cards are empty
    fn are_unique(&'a self) -> bool {
        let first = self.cards().nth(0);
        match first {
            Some(card) => self.cards().skip(1).all(|c| c != card),
            None => true,
        }
    }
}

pub trait CardsMut<'a> {
    fn cards_mut(&'a mut self) -> CardIterMut<'a>;
}

/// The iterator over `Cards`: by calling `.cards()`
pub struct CardIter<'a> {
    cards: Vec<&'a Card>,
    index: usize,
}

impl<'a> Iterator for CardIter<'a> {
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

impl<'a> Cards<'a> for Deck {
    fn cards(&'a self) -> CardIter<'a> {
        CardIter {
            cards: self.cards.iter().collect(),
            index: 0,
        }
    }
}

impl<'a> Cards<'a> for Hand {
    fn cards(&'a self) -> CardIter<'a> {
        CardIter {
            cards: self.cards.iter().collect(),
            index: 0,
        }
    }

}

pub struct CardIterMut<'a> {
    cards: Vec<&'a mut Card>,
}

impl<'a> Iterator for CardIterMut<'a> {
    type Item = &'a mut Card;
    fn next(&mut self) -> Option<Self::Item> {
        self.cards.reverse();
        let card = self.cards.pop();
        self.cards.reverse();
        card
    }
}

impl<'a> CardsMut<'a> for Deck {
    fn cards_mut(&'a mut self) -> CardIterMut<'a> {
        CardIterMut {
            cards: self.cards.iter_mut().collect(),
        }
    }
}

#[test]
fn cards_mut() {
    let mut deck = Deck::default();
    dbg!(&deck);
    
    // Make all the cards in the deck the same
    for card in deck.cards_mut() {
        *card = Card::default(); // Ace of Spades
    }

    dbg!(&deck);

    assert!(deck.cards().all(|c| *c == deck.cards[0]));
}
