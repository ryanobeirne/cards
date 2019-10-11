use cards::*;

use std::collections::{BTreeMap, HashMap};
use std::io::{stdout, stdin, Write};
use std::convert::TryFrom;
use std::fmt;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    match play_game(2) {
        Ok(player) => println!("Winner: {:?}", player),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

struct PlayerIndex {
    current: usize,
    next: usize,
    count: usize,
}

impl PlayerIndex {
    fn new(count: usize) -> Self {
        PlayerIndex {
            current: 0,
            next: 1,
            count,
        }
    }
}

impl PlayerIndex {
    fn increment(&mut self) {
        self.current = self.next;
        self.next = (self.next + 1) % self.count;
    }
}

fn play_game(n_players: usize) -> Result<Player> {
    let mut game = FishGame::new(n_players)?;

    let mut player_index = PlayerIndex::new(game.players.len());
    while !game.has_empty_hand() {
        game.turn(&player_index)?;
        dbg!(&game);
        player_index.increment();
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

    //fn has_value<V: Into<Value> + Copy>(&self, value: V) -> bool {
        //self.hand.cards().any(|c| c.value == value.into())
    //}

    fn match_cards_from_value<V: Into<Value> + Copy>(&self, value: V) -> MatchIndex<Card> {
        self.hand.cards().enumerate()
            .filter(|(_, card)| card.value == value.into())
            .map(|(i, card)| (i, card.clone()))
            .collect()
    }
}

#[derive(Debug)]
struct MatchIndex<T> {
    matches: BTreeMap<usize, T>
}

impl<T> MatchIndex<T> {
    fn first(&self) -> Option<(&usize, &T)> {
        match self.matches.iter().nth(0) {
            Some((i, t)) => Some((i, t)),
            None => None,
        }
    }
}

impl<T> std::iter::FromIterator<(usize, T)> for MatchIndex<T> {
    fn from_iter<I: IntoIterator<Item=(usize, T)>>(iter: I) -> Self {
        MatchIndex{
            matches: iter.into_iter().collect()
        }
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

    fn turn(&mut self, index: &PlayerIndex) -> Result<()> {
        let value = user_ask_value(
            &mut stdout(),
            &self.players[index.current],
            &self.players[index.next],
        );

        let matches = self.players[index.next].match_cards_from_value(value);
        
        match matches.first() {
            Some((i, card)) => {
                println!("Here you go! [{}]", card);
                let card = self.players[index.next].give(*i)?;
                self.players[index.current].take(card);
            },
            None => println!("Go fish!"),
        }

        self.players[index.current].discard_pairs()?;

        Ok(())
    }
}

fn user_ask_value<W: Write>(writer: &mut W, player: &Player, next: &Player) -> Value {
    write!(writer, "{}: Ask {} for a card value: ", player.name, next.name).expect("write");
    writer.flush().expect("write");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("stdin");
        
    match Value::try_from(input.trim()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid card value!");
            return user_ask_value(writer, player, next);
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

#[derive(Debug)]
struct NoWinner;

impl fmt::Display for NoWinner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NoWinner {}
