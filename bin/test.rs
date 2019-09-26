use super::*;
use std::collections::HashMap;

#[test]
fn test_war() {
    const ITERATIONS: usize = 100_000;

    // (KEY = THING, VAL = COUNT)
    let mut winner_counter = HashMap::<usize, usize>::new();
    let mut round_len_counter = HashMap::<usize, usize>::new();
    let mut round_sum: usize = 0;
    let mut war_sum: usize = 0;

    // Play the game 10000 times
    for _i in 0..ITERATIONS {
        let (winner, win_len, rounds, war_count) = play_game();
        // Winner must have all 52 cards
        assert_eq!(win_len, 52);
        // Count how many times the players won
        *winner_counter.entry(winner).or_insert(0) += 1;
        // Count how many times a game was won in this number of rounds
        *round_len_counter.entry(rounds).or_insert(0) += 1;
        // Sum up the rounds for all the games played
        round_sum += rounds;
        war_sum += war_count;
    }

    // Average all the games round lengths
    let avg_rounds = round_sum as f64 / ITERATIONS as f64;

    let avg_wars = war_sum as f64 / ITERATIONS as f64;

    // Find the game with the most rounds, and count how many games won with that amount
    let (max_round_len, max_round_count) = round_len_counter.iter()
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .expect("Empty round_len_counter");

    // Find the game with the least rounds, and count how many games won with that amount
    let (min_round_len, min_round_count) = round_len_counter.iter()
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .expect("Empty round_len_counter");

    // Put all the winning round lengths in a vec
    let mut round_lengths: Vec<(&usize, &usize)> = round_len_counter.iter().collect();
    // Order the rounds by count
    round_lengths.sort_by(|(_, a), (_, b)| b.cmp(a));

    let mid = round_lengths.len() / 2;
    let (med_round_len, med_round_count) = round_lengths.get(mid).expect("Empty round_lengths");

    println!("\nPlayed {} games with an average of {} wars per game:", ITERATIONS, avg_wars);
    println!("Max Round Length: {} Rounds ({} games)", max_round_len, max_round_count);
    println!("Med Round Length: {} Rounds ({} games)", med_round_len, med_round_count);
    println!("Avg Round Length: {} Rounds ({} games)", avg_rounds.round(), get_closest(&round_len_counter, avg_rounds.round() as usize));
    println!("Min Round Length: {} Rounds ({} games)", min_round_len, min_round_count);
}

fn get_closest(map: &HashMap<usize, usize>, val: usize) -> usize {
    let mut find_val = val.clone();

    if let Some(value) = map.get(&find_val) {
        *value
    } else {
        find_val += 1;
        get_closest(map, find_val)
    }
}
