mod deck;
mod hand_value_calculator;
mod player;
mod poker;

use crate::hand_value_calculator::HandValues;

use crate::poker::Poker;

fn main() {
    let _poker = Poker::new_texas_hold_em(4);
    // let poker2 = Poker::new_texas_hold_em(4);

    // poker.shuffle_deck();
    // let mut i: u128 = 0;
    // while i < u128::MAX {
    //     // poker2.shuffle_deck();
    //     if i % 1000000000 == 0 {
    //         println!("{}", i);
    //     }
    //     if i == 1000000000000000 {
    //         println!("{}", i);

    //         break;
    //     }
    //     if poker.deck.cards == poker2.deck.cards {
    //         panic!("decks are equal");
    //     }
    //     i += 1;
    // }

    // let _card = poker.deck._top_card();
    // let _card = poker.deck._top_card();

    // poker.shuffle_deck();

    // println!("{:#?}", poker.deck.cards[0]);
    // poker.shuffle_deck();

    let mut poker = Poker::new(1, 2, 5);

    let mut i: u128 = 0;
    let mut royal_flushes: u128 = 0;
    let mut straight_flushes: u128 = 0;

    while i < u128::MAX {
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
    // let mut i: u128 = 0;
    // while i < u128::MAX {
    //     poker.shuffle_deck();
    //     let mut cards: Vec<Card> = Vec::new();
    //     for j in 0..poker.total_cards as usize {
    //         cards.push(poker.deck.cards[j]);
    //     }

    //     let hand = Hand::new(cards);

    //     hand.hand_value();

    //     let value = hand.value;
    //     if value == HandValues::RoyalFlush {
    //         println!("{:?}", i);
    //         println!("{:?}", hand.cards);
    //         println!("{:?}", hand.value);
    //         // break;
    //     }

    //     // if i % 10000 == 0 {
    //     //     println!("{:?}", i);
    //     // }
    //     i += 1;
    // }

    // let mut cards: Vec<Card> = Vec::new();
    // for i in 0..poker.total_cards as usize {
    //     cards.push(poker.deck.cards[i]);
    // }

    // let hand = Hand::new(cards);
    // println!("{:?}", hand);

    // hand.hand_value();
    // println!("{:?}", hand.value);
    // assert_eq!(hand.value, HandValues::Straight);
}
