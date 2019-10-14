use super::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct FishGame {
    pub river: Deck,
    pub players: Vec<Player>,
}

impl FishGame {
    pub fn new(n_players: usize) -> DealResult<Self> {
        let mut players = Vec::new();
        players.push(Player::new("Human0", Human));
        for n in 1..n_players {
            players.push(Player::new(&format!("Computer{}", n), Computer));
        }

        Ok(FishGame {
            river: Deck::new(),
            players,
        }
        .first_deal()?)
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

    pub fn has_empty_hand(&self) -> bool {
        self.players.iter().any(|p| p.hand.is_empty())
    }

    pub fn turn(&mut self, index: &PlayerIndex) -> Result<()> {
        let player_type = self.players[index.current].player_type;
        let value = match player_type {
            Human => human_ask_value(
                    &mut stdout(),
                    &self.players[index.current],
                    &self.players[index.next],
                ),
            Computer => computer_ask_value(&self.players[index.current]),
        };

        let matches = self.players[index.next].match_cards_from_value(value);

        match matches.first() {
            Some((i, card)) => {
                println!("Here you go! [{}]", card);
                let card = self.players[index.next].give(*i)?;
                self.players[index.current].take(card);
            }
            None => {
                let card = self.go_fish(&player_type);
                println!("Caught one! [{}]", card);
                self.players[index.current].take(card);
            }
        }

        self.players[index.current].discard_pairs()?;

        Ok(())
    }

    fn go_fish(&mut self, player_type: &PlayerType) -> Card {
        let index = match player_type {
            Human => ask_user_index(&mut stdout(), self.river.len()),
            Computer => computer_ask_index(self.river.len()),
        };

        match self.river.give(index) {
            Ok(card) => card,
            Err(_) => {
                eprintln!("Invalid selection!");
                self.go_fish(player_type)
            }
        }
    }
}

fn ask_user_index<W: Write>(w: &mut W, limit: usize) -> usize {
    write!(w, "Go fish! [0-{}]: ", limit - 1).expect("write");
    w.flush().expect("write");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("stdin");

    match input.trim().parse::<usize>() {
        Ok(u) => {
            if u < limit {
                u
            } else {
                eprintln!("Index out of bounds!");
                ask_user_index(w, limit)
            }
        }
        Err(_) => ask_user_index(w, limit),
    }
}

fn human_ask_value<W: Write>(w: &mut W, player: &Player, next: &Player) -> Value {
    write!(w, "{}: Ask {} for a card value: ", player.name, next.name).expect("write");
    w.flush().expect("write");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("stdin");

    match Value::try_from(input.trim()) {
        Ok(v) => {
            if player.has_value(v) {
                v
            } else {
                writeln!(w, "You don't have that card!").expect("write");
                human_ask_value(w, player, next)
            }
        }
        Err(_) => {
            writeln!(w, "Invalid card value!").expect("write");
            human_ask_value(w, player, next)
        }
    }
}

fn computer_ask_value(player: &Player) -> Value {
    match player
        .hand
        .cards()
        .collect::<Vec<_>>()
        .choose(&mut rand::thread_rng())
    {
        Some(card) => {
            println!("Do you have a {:?}?", card.value);
            sleep(2);
            card.value
        },
        None => panic!("Empty hand!"),
    }
}

fn computer_ask_index(limit: usize) -> usize {
    println!("Going fishing!");
    sleep(2);
    rand::thread_rng().gen_range(0, limit)
}

impl fmt::Display for FishGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "River: [0-{}]\n", self.river.len())?;

        for player in self.players.iter() {
            match player.player_type {
                Human => {
                    writeln!(f, "{}: {:?}", player.name, player.hand)?;
                    writeln!(f, "{}: {:?}", "Paired", player.paired)?;
                },
                Computer => {
                    writeln!(f, "{}: [{}]", player.name, player.hand.len())?;
                    writeln!(f, "{}: [{}]", "Paired", player.paired.len())?;
                },
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs))
}
