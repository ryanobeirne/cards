use super::*;

#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub hands: Vec<Hand>,
}

impl Game {
    pub fn new(n_hands: usize, hand_size: usize) -> Self {
        let mut deck = Deck::new();
        let mut hands = Vec::new();

        for _hand in 0..n_hands {
            let mut hand = Hand::new();

            for _card in 0..hand_size {
                deck.deal(0, &mut hand);
            }

            hands.push(hand);
        }

        Game {
            deck,
            hands,
        }
    }

    pub fn has_empty_hand(&self) -> bool{
        self.hands.iter().any(|h| h.cards.is_empty())
    }
}

#[test]
fn new_game() {
    let game = Game::new(2, 5);
    assert_eq!(game.deck.len(), 42);
    assert_eq!(game.hands[0].len(), 5);
    assert_eq!(game.hands[1].len(), 5);
    assert_ne!(game.hands[0], game.hands[1]);
}
