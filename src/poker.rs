use crate::deck::{Card, Deck};

pub struct Poker {
    pub player_amount: i32,
    pub players: Vec<Player>,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: Deck,
}

impl Poker {
    //initialization for custom game of hold em
    pub fn _new(player_amount: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        Poker {
            player_amount,
            players: Vec::new(),
            hand_size,
            cards_on_table,
            deck: Deck::new(),
        }
    }

    pub fn new_texas_hold_em(player_amount: i32) -> Poker {
        let mut players: Vec<Player> = Vec::new();
        for _ in 0..player_amount {
            players.push(Player::_new(1000));
        }
        Poker {
            player_amount,
            players: players,
            hand_size: 2,
            cards_on_table: 5,
            deck: Deck::new(),
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.deck.shuffle_cards();
    }

    pub fn _deal_cards(&mut self) {
        for _ in 0..self.hand_size {
            for player in &mut self.players {
                player._deal_card(self.deck._top_card());
            }
        }
    }

    pub fn _new_player(&mut self, chips: i32) {
        self.player_amount += 1;
        self.players.push(Player::_new(chips));
    }
}

#[derive(Debug)]
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

    pub fn _deal_card(&mut self, card: Card) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{Suit, Value};

    #[test]
    fn dealing_cards_work() {
        let mut poker = Poker::new_texas_hold_em(8);
        poker._new_player(1000);
        poker._deal_cards();
        assert_eq!(poker.players[0].hand.len(), 2);
        assert_eq!(poker.players[1].hand.len(), 2);
        assert_ne!(poker.players[0].hand, poker.players[1].hand);
        assert_ne!(poker.players[0].hand, poker.players[1].hand);
        assert_eq!(poker.players[0].hand[0].suit, Suit::Spades);
        assert_eq!(poker.players[0].hand[0].value, Value::King);
    }
}
