use cards::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::{stdin, stdout, Write};

mod game;
mod player;

use game::*;
use player::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    match play_game(2) {
        Ok(player) => println!("Winner: {:?}", player),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

fn play_game(n_players: usize) -> Result<Player> {
    let mut game = FishGame::new(n_players)?;

    let mut player_index = PlayerIndex::new(game.players.len());
    while !game.has_empty_hand() {
        println!("{}", &game);
        game.turn(&player_index)?;
        player_index.increment();
    }

    println!("{}", &game);

    match game
        .players
        .into_iter()
        .max_by(|a, b| a.paired.len().cmp(&b.paired.len()))
    {
        Some(player) => Ok(player),
        None => Err(Box::new(NoWinner)),
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
