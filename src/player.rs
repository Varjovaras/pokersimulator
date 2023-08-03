use crate::{deck::Card, hand::Hand};

#[derive(Debug)]
pub struct Player {
    pub cards: Vec<Card>,
    pub hand: Hand,
    pub chips: i32,
    pub bet: i32,
    pub folded: bool,
}

impl Player {
    pub fn new(chips: i32) -> Player {
        Player {
            cards: Vec::new(),
            hand: Hand::new_empty_hand(),
            chips,
            bet: 0,
            folded: false,
        }
    }

    pub fn deal_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn empty_hand(&mut self) {
        self.hand = Hand::new_empty_hand();
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
    use crate::{hand_value_calculator::HandValues, poker::Poker};

    use super::*;

    #[test]
    fn straight_flush_works() {
        let poker = Poker::_new_texas_hold_em(4);

        let mut cards: Vec<Card> = Vec::new();
        for i in 0..poker.total_cards as usize {
            cards.push(poker.deck.cards[i]);
        }

        let hand = Hand::new(cards);
        hand._hand_value();
        assert_eq!(hand.value, HandValues::StraightFlush);
    }
}
