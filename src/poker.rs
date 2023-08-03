use crate::{
    deck::{Card, Deck},
    hand::Hand,
    player::Player,
};

pub struct Poker {
    pub player_amount: i32,
    pub players: Vec<Player>,
    pub hand_size: i32,
    pub amount_of_cards_on_table: i32,
    pub cards_on_table: Vec<Card>,
    pub total_cards: i32,
    pub deck: Deck,
}

impl Poker {
    //initialization for custom game of hold em
    pub fn new(player_amount: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        if cards_on_table + hand_size * player_amount > 52 {
            panic!("Too many cards on table or on players hands");
        }

        let players: Vec<Player> = players_for_new_game(player_amount);
        Poker {
            player_amount,
            players,
            hand_size,
            amount_of_cards_on_table: cards_on_table,
            cards_on_table: Vec::new(),
            total_cards: hand_size + cards_on_table,
            deck: Deck::new(),
        }
    }

    pub fn _new_texas_hold_em(player_amount: i32) -> Poker {
        let players: Vec<Player> = players_for_new_game(player_amount);

        Poker {
            player_amount,
            players,
            hand_size: 2,
            amount_of_cards_on_table: 5,
            cards_on_table: Vec::new(),
            total_cards: 7,
            deck: Deck::new(),
        }
    }

    pub fn shuffle_deck(&mut self) {
        self.empty_player_hands();
        self.deck = Deck::new();
        self.cards_on_table = Vec::<Card>::new();
        self.deck.shuffle_cards();
    }

    fn empty_player_hands(&mut self) {
        for player in &mut self.players {
            player.empty_hand();
        }
    }

    fn deal_cards_to_players(&mut self) {
        for _ in 0..self.hand_size {
            for player in &mut self.players {
                player.deal_card(self.deck.top_card());
            }
        }
    }

    fn deal_cards_to_table(&mut self) {
        if self.amount_of_cards_on_table == 0 {
            return;
        }
        for _ in 0..self.amount_of_cards_on_table {
            self.cards_on_table.push(self.deck.top_card());
        }
    }

    fn _new_player(&mut self, chips: i32) {
        self.player_amount += 1;
        self.players.push(Player::new(chips));
    }

    pub fn play_round(&mut self) {
        self.shuffle_deck();
        self.deal_cards_to_players();
        self.deal_cards_to_table();
        self.calculate_hand_values();
    }

    fn calculate_hand_values(&mut self) {
        let cards_on_table = &self.cards_on_table;
        for player in &mut self.players {
            let mut hand = player.hand.cards.clone();

            for card in cards_on_table {
                hand.push(*card)
            }
            player.hand = Hand::new(hand);
        }
    }
}

fn players_for_new_game(player_amount: i32) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for _ in 0..player_amount {
        players.push(Player::new(1000));
    }
    players
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{Suit, Value};

    #[test]
    fn dealing_cards_work() {
        let mut poker = Poker::_new_texas_hold_em(8);
        poker._new_player(1000);
        poker.deal_cards_to_players();
        assert_eq!(poker.players[0].hand.cards.len(), 2);
        assert_eq!(poker.players[1].hand.cards.len(), 2);
        assert_ne!(poker.players[0].hand.cards, poker.players[1].hand.cards);
        assert_eq!(poker.players[0].hand.cards[0].suit, Suit::Spades);
        assert_eq!(poker.players[0].hand.cards[0].value, Value::King);
    }
}
