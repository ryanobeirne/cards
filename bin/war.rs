use cards::*;

#[cfg(test)]
mod test;

fn main() {
    play_game();
}

fn play_game() -> (usize, usize, usize, usize) {
    let mut game = Game::new(2, 26);
    let mut rounds = 0;
    let mut war_count = 0;

    while !game.has_empty_hand() {
        round(&mut game, &mut war_count);
        rounds += 1;
        if rounds == std::usize::MAX  { 
            eprint!("ENDLESS GAME!");
            dbg!(&game);
            break;
        }
    }

    let (player, win_hand) = game.hands.iter().enumerate()
        .max_by(|(_a, a), (_b, b)| a.len().cmp(&b.len()))
        .expect("Empty hands!");

    let win_len = win_hand.len();

    println!("Player {} wins with {} wars in {} rounds!", player, war_count, rounds);

    // Test that all cards are unique
    assert!(game.hands[0].are_unique());
    assert!(game.hands[1].are_unique());

    (player, win_len, rounds, war_count)
}

fn round(game: &mut Game, war_count: &mut usize) {
    let card0 = game.hands[0].give(0);
    let card1 = game.hands[1].give(0);

    if card0.cmp_value() == card1.cmp_value() {
        *war_count += 1;
        war(game, &mut vec![card0, card1]);
    } else if card0 > card1 {
        game.hands[0].take(card0);
        game.hands[0].take(card1);
    } else {
        game.hands[1].take(card1);
        game.hands[1].take(card0);
    }
}

fn war(game: &mut Game, war_cards: &mut Vec<Card>) {
    let len0 = game.hands[0].len();
    let len1 = game.hands[1].len();

    if len0 < 4 {
        take_all(&mut game.hands[1], war_cards);
        return;
    } else if len1 < 4 {
        take_all(&mut game.hands[0], war_cards);
        return;
    }
    
    for card in &[
        game.hands[0].give(0),
        game.hands[0].give(0),
        game.hands[0].give(0),
        game.hands[0].give(0),
        game.hands[1].give(0),
        game.hands[1].give(0),
        game.hands[1].give(0),
        game.hands[1].give(0),
    ]
    {
        war_cards.push(*card);
    }

    if war_cards[5].cmp_value() == war_cards[9].cmp_value() {
        war(game, war_cards);
    } else if war_cards[5] > war_cards[9] {
        take_all(&mut game.hands[0], war_cards); 
    } else {
        take_all(&mut game.hands[1], war_cards); 
    }
        
}

fn take_all(hand: &mut Hand, cards: &mut Vec<Card>) {
    for card in cards.into_iter() {
        hand.take(*card);
    }
}
