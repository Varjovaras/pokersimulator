use crate::deck::{Card, Deck};

pub struct Poker {
    pub player_amount: i32,
    pub players: Vec<Player>,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: Deck,
}

pub struct Player {
    pub hand: Vec<Card>,
    pub chips: i32,
    pub bet: i32,
    pub folded: bool,
}

impl Player {
    pub fn _new(chips: i32) -> Player {
        Player {
            hand: Vec::new(),
            chips,
            bet: 0,
            folded: false,
        }
    }

    pub fn _add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn _empty_hand(&mut self) {
        self.hand = Vec::new();
    }

    pub fn _fold(&mut self) {
        self.folded = true;
    }

    pub fn _bet(&mut self, amount: i32) {
        self.bet += amount;
        self.chips -= amount;
    }

    pub fn _win(&mut self, amount: i32) {
        self.chips += amount;
    }
}

impl Poker {
    //initialization for custom game of hold em
    pub fn _new(player_amount: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        let deck = Deck::new();
        Poker {
            player_amount,
            players: Vec::new(),
            hand_size,
            cards_on_table,
            deck,
        }
    }

    pub fn new_texas_hold_em(player_amount: i32) -> Poker {
        let deck = Deck::new();
        Poker {
            player_amount,
            players: Vec::new(),
            hand_size: 2,
            cards_on_table: 5,
            deck,
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle_cards();
    }

    pub fn _deal_cards(&mut self) {}
}
