use cards::*;

use std::collections::HashMap;
use std::io::{stdout, stdin, Read, Write};
use std::convert::TryFrom;
use std::fmt;

fn main() {
    play_game();
}

#[derive(Debug)]
struct NoWinner;

impl fmt::Display for NoWinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NoWinner {}

fn play_game() -> Result<Player, Box<dyn std::error::Error>> {
    let mut game = FishGame::new(2)?;

    let mut player_index = 0;
    while !game.has_empty_hand() {
        game.turn(player_index);
        dbg!(&game);
        player_index = (player_index + 1) % game.players.len();
    }

    match game.players.into_iter().max_by(|a,b| a.paired.len().cmp(&b.paired.len())) {
        Some(player) => Ok(player),
        None => Err(Box::new(NoWinner))
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Hand,
    paired: Hand,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.into(),
            hand: Hand::new(),
            paired: Hand::new(),
        }
    }

    fn discard_pairs(&mut self) -> DealResult<usize> {
        let mut pairs: HashMap<Value, usize> = HashMap::new();
        
        for card in self.hand.cards() {
            *pairs.entry(card.value).or_insert(0) += 1;
        }

        for (value, count) in pairs.iter().filter(|(_value, count)| **count > 1_usize) {
            let len = count - (count % 2);
            for _i in 0..len {
                let index = self.hand.cards().position(|card| card.value == *value).expect("The card should be here!");
                self.hand.deal(index, &mut self.paired)?;
            }
        }

        Ok(pairs.len())
    }
}

#[derive(Debug)]
struct FishGame {
    river: Deck,
    players: Vec<Player>,
}

impl FishGame {
    fn new(n_players: usize) -> DealResult<Self> {
        let mut players = Vec::new();
        for n in 0..n_players {
            players.push(Player::new(&format!("Player{}", n)));
        }

        Ok(
            FishGame {
                river: Deck::new(),
                players,
            }.first_deal()?
        )
    }

    // Deal 5 cards to each player
    fn first_deal(mut self) -> DealResult<Self> {
        for player in self.players.iter_mut() {
            for _i in 0..5 {
                self.river.deal(0, player)?;
            }
            player.discard_pairs()?;
        }

        Ok(self)
    }

    fn has_empty_hand(&self) -> bool {
        self.players.iter().any(|p| p.hand.is_empty())
    }

    fn turn(&mut self, player_index: usize) {
        let value = user_ask_value(&mut stdout(), &self.players[player_index]);
        dbg!(value);
    }
}

fn user_ask_value<W: Write>(writer: &mut W, player: &Player) -> Value {
    write!(writer, "{}: Ask for a card value: ", &player.name).expect("write");
    writer.flush().expect("write");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("stdin");
        
    let value = match input.chars().nth(0) {
        Some(c) => Value::try_from(c),
        None => { 
            eprintln!("Must enter a character!");
            return user_ask_value(writer, player);
        }
    };

    match value {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid card value!");
            return user_ask_value(writer, player);
        },
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
