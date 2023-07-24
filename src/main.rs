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
        // poker2.shuffle_deck();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equa");
        }
        i += 1;
    }

    let _card = poker.deck.top_card();
    let _card = poker.deck.top_card();
    poker.shuffle_deck();

    println!("{:#?}", poker.deck.cards);
    poker.deck.shuffle_cards();

    // let mut poker = Poker::new_texas_hold_em(8);
    // poker._new_player(1000);
    // poker._deal_cards();
    // assert_eq!(poker.players[0].hand.len(), 2);
    // assert_eq!(poker.players[1].hand.len(), 2);
    // assert_ne!(poker.players[0].hand, poker.players[1].hand);
    // assert_eq!(poker.players[0].hand[0].suit, Suit::Spades);
    // assert_eq!(poker.players[0].hand[0].value, Value::King);

    // println!("{:#?}", poker.players[1].hand);

    // println!("{:#?}", card);
}
