use crate::deck::Deck;

pub struct Poker {
    pub players: i32,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: Deck,
}

impl Poker {
    //initialization for custom game of hold em
    pub fn _new(players: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        let deck = Deck::new();
        Poker {
            players,
            hand_size,
            cards_on_table,
            deck,
        }
    }

    pub fn new_texas_hold_em(players: i32) -> Poker {
        let deck = Deck::new();
        Poker {
            players,
            hand_size: 2,
            cards_on_table: 5,
            deck,
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle_cards();
    }
}
