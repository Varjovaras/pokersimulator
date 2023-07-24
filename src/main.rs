mod deck;
mod hand_calculator;
mod poker;
use poker::Poker;

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
            panic!("decks are equa");
        }
        i += 1;
    }

    let _card = poker.deck._top_card();
    let _card = poker.deck._top_card();
    poker.shuffle_deck();

    println!("{:#?}", poker.deck.cards[0]);
    poker.shuffle_deck();
}
