use super::*;

#[derive(Debug)]
pub struct FishGame {
    pub river: Deck,
    pub players: Vec<Player>,
}

impl FishGame {
    pub fn new(n_players: usize) -> DealResult<Self> {
        let mut players = Vec::new();
        for n in 0..n_players {
            players.push(Player::new(&format!("Player{}", n)));
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
            }
            None => {
                let card = self.go_fish();
                println!("Caught one! [{}]", card);
                self.players[index.current].take(card);
            }
        }

        self.players[index.current].discard_pairs()?;

        Ok(())
    }

    fn go_fish(&mut self) -> Card {
        let index = ask_user_index(&mut stdout(), self.river.len());
        match self.river.give(index) {
            Ok(card) => card,
            Err(_) => {
                eprintln!("Invalid selection!");
                self.go_fish()
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

fn user_ask_value<W: Write>(w: &mut W, player: &Player, next: &Player) -> Value {
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
                user_ask_value(w, player, next)
            }
        }
        Err(_) => {
            writeln!(w, "Invalid card value!").expect("write");
            user_ask_value(w, player, next)
        }
    }
}

