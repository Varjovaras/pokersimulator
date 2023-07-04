mod poker;
use poker::Deck_in_game;

fn main() {
    let mut deck = Deck_in_game::new();
    // deck.shuffle_cards();

    // let card = deck.top_card();

    let mut deck2 = Deck_in_game::new();

    // let mut i: u128 = 0;
    // while i < u128::MAX {
    //     deck2.shuffle_cards();
    //     if i % 10000 == 0 {
    //         println!("{}", i);
    //     }
    //     if deck.cards == deck2.cards {
    //         panic!("decks are equa");
    //     }
    //     i += 1;
    // }

    let card = deck.top_card();
    let card = deck.top_card();
    // let card = deck.top_card();
    // let card = deck.top_card();
    // deck.shuffle_cards();

    // println!("{:#?}", card);
    println!("{:#?}", deck.cards);
    deck.shuffle_cards();
    println!("pasa");
    println!("{:#?}", deck.cards_on_table);
    // println!("{:#?}", card);
}
