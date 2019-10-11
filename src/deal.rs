use super::*;
use std::fmt;

pub type DealResult<T> = Result<T, DealError>;

pub trait Deal<I, T>
where Self: Give<Item=I>, T: Take<Item=I> {
    fn deal(&mut self, index: usize, taker: &mut T) -> DealResult<()> {
        match self.give(index) {
            Ok(item) => {
                taker.take(item);
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}

impl<G, I, T> Deal<I, T> for G
where G: Give<Item=I>, T: Take<Item=I> {}

pub trait Give {
    type Item;
    fn give(&mut self, index: usize) -> DealResult<Self::Item>;
}

pub trait Take {
    type Item;
    fn take(&mut self, item: Self::Item);
}

impl Give for Deck {
    type Item = Card;
    fn give(&mut self, index: usize) -> DealResult<Self::Item> {
        let len = self.len();

        if len == 0 {
            return Err(DealError::NothingToGive);
        }

        if index >= len {
            Err(DealError::OutOfBounds) 
        } else {
            Ok(self.cards.remove(index))
        }
    }
}

impl Take for Deck {
    type Item = Card;
    fn take(&mut self, item: Self::Item) {
        self.cards.push(item);
    }
}

impl Take for Hand {
    type Item = Card;
    fn take(&mut self, item: Self::Item) {
        self.cards.push(item);
    }
}

impl Give for Hand {
    type Item = Card;
    fn give(&mut self, index: usize) -> DealResult<Self::Item> {
        let len = self.len();

        if len == 0 {
            return Err(DealError::NothingToGive);
        }

        if index >= len {
            Err(DealError::OutOfBounds) 
        } else {
            Ok(self.cards.remove(index))
        }
    }
}

#[derive(Debug)]
pub enum DealError {
    NothingToGive,
    CannotTake,
    OutOfBounds,
}

impl fmt::Display for DealError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DealError {
    fn description(&self) -> &str {
        match self {
            DealError::CannotTake => "Cannot accept this item",
            DealError::NothingToGive => "Out of items",
            DealError::OutOfBounds => "Index is out of bounds",
        }
    }
}

#[test]
fn deal_deck_to_hand() -> DealResult<()> {
    let mut deck = Deck::default();
    let mut hand = Hand::new();

    deck.deal(0, &mut hand)?;
    assert_eq!(hand.cards[0], Card { value: Value::Ace, suit: Suit::Spades });
    assert_eq!(deck.len(), 51);
    assert_eq!(hand.len(), 1);
    assert_eq!(deck.cards().next(), Some(&Card { value: Value::Two, suit: Suit::Spades }));

    Ok(())
}
