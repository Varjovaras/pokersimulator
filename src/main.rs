mod deck;
mod hand;
mod hand_value_calculator;
mod player;
mod poker;

use crate::hand_value_calculator::HandValues;
use crate::poker::Poker;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    shuffle();
    royal_flush_calculator();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn shuffle() {
    let poker = Poker::new_texas_hold_em(4);
    let mut poker2 = Poker::new_texas_hold_em(4);

    poker2.shuffle_deck();
    let mut i: u128 = 0;
    while i < u128::MAX {
        poker2.shuffle_deck();
        if i % 100000 == 0 {
            println!("{}", i);
        }
        if i == 1000000000000000 {
            println!("{}", i);

            break;
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equal pasalusta");
        }
        i += 1;
    }
}

fn royal_flush_calculator() {
    let mut poker = Poker::new(1, 2, 5);

    let mut i: u128 = 0;
    let mut royal_flushes: u128 = 0;
    let mut straight_flushes: u128 = 0;

    while i < 1000000 {
        poker.play_round();
        i += 1;

        if i % 500000 == 0 {
            println!("{}", i);
            // println!("{}", poker.cards_on_table.len());

            println!("royal flushes: {}", royal_flushes);
            println!("straight flushes: {}", straight_flushes);
        }
        for player in &poker.players {
            if player.hand.value == HandValues::RoyalFlush {
                // println!("{:?}", player.hand);

                royal_flushes += 1;
            }
            if player.hand.value == HandValues::StraightFlush {
                straight_flushes += 1;
            }
        }
    }
}
