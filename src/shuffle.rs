use rand::seq::SliceRandom;
use rand::thread_rng;

use super::*;

pub trait Shuffle where Self: Sized {
    fn shuffle(&mut self);

    fn shuffled(mut self) -> Self {
        self.shuffle();
        self
    }
}

impl Shuffle for Deck {
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Shuffle for Hand {
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
