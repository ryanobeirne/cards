use super::*;

pub trait Deal<I, T>
where Self: Give<Item=I>, T: Take<Item=I> {
    fn deal(&mut self, index: usize, taker: &mut T) {
        taker.take(self.give(index))
    }
}

impl<G, I, T> Deal<I, T> for G
where G: Give<Item=I>, T: Take<Item=I> {}

pub trait Give {
    type Item;
    fn give(&mut self, index: usize) -> Self::Item;
}

pub trait Take {
    type Item;
    fn take(&mut self, item: Self::Item);
}

impl Give for Deck {
    type Item = Card;
    fn give(&mut self, index: usize) -> Self::Item {
        self.cards.remove(index)
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
    fn give(&mut self, index: usize) -> Self::Item {
        self.cards.remove(index)
    }
}

#[test]
fn deal_deck_to_hand() {
    let mut deck = Deck::default();
    let mut hand = Hand::new();

    deck.deal(0, &mut hand);
    assert_eq!(hand.cards[0], Card { value: Value::Ace, suit: Suit::Spades });
    assert_eq!(deck.len(), 51);
    assert_eq!(hand.len(), 1);
    assert_eq!(deck.cards().next(), Some(&Card { value: Value::Two, suit: Suit::Spades }));
}
