mod deck;
mod poker;
use deck::{Card, Deck};

pub struct Game {
    pub players: i32,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: Deck,
}

impl Game {
    //initialization for custom game of hold em
    fn _new(players: i32, hand_size: i32, cards_on_table: i32) -> Game {
        let deck = Deck::new();
        Game {
            players,
            hand_size,
            cards_on_table,
            deck,
        }
    }

    fn new_texas_hold_em(players: i32) -> Game {
        let deck = Deck::new();
        Game {
            players,
            hand_size: 2,
            cards_on_table: 5,
            deck,
        }
    }
}

struct _PlayerHand {
    hand: Vec<Card>,
}

fn main() {
    let mut poker = Game::new_texas_hold_em(4);
    let mut poker2 = Game::new_texas_hold_em(4);

    poker.deck.shuffle_cards();
    let mut i: u128 = 0;
    while i < u128::MAX {
        poker2.deck.shuffle_cards();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equa");
        }
        i += 1;
    }

    let _card = poker.deck.top_card();
    let card = poker2.deck.top_card();
    // let card = deck.top_card();
    // let card = deck.top_card();
    // deck.shuffle_cards();

    println!("{:#?}", card);
    println!("{:#?}", poker.deck.cards);
    poker.deck.shuffle_cards();
    // println!("{:#?}", card);
}
