mod deck;
mod hand_calculator;
mod poker;

use poker::Poker;

use crate::{
    deck::Card,
    hand_calculator::{Hand, HandValues},
};

fn main() {
    let mut poker = Poker::new_texas_hold_em(4);
    let mut poker2 = Poker::new_texas_hold_em(4);

    poker.shuffle_deck();
    let mut i: u128 = 0;
    while i < u128::MAX {
        poker2.shuffle_deck();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equal");
        }
        i += 1;
    }

    let _card = poker.deck._top_card();
    let _card = poker.deck._top_card();

    poker.shuffle_deck();

    println!("{:#?}", poker.deck.cards[0]);
    poker.shuffle_deck();

    let poker = Poker::new_texas_hold_em(4);

    let mut cards: Vec<Card> = Vec::new();
    for i in 0..poker.total_cards as usize {
        cards.push(poker.deck.cards[i]);
    }

    let mut hand = Hand::_new(cards);
    println!("{:?}", hand);

    hand.hand_value();
    println!("{:?}", hand.value);
    // assert_eq!(hand.value, HandValues::Straight);
}
