use crate::{
    deck::Card,
    hand_value_calculator::{self, HandValues},
};

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

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub value: HandValues,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        let value = hand_value_calculator::hand_value(&cards);
        Hand { cards, value }
    }

    pub fn new_empty_hand() -> Hand {
        Hand {
            cards: Vec::new(),
            value: HandValues::HighCard,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn _highest_card(&self) -> Card {
        let mut highest = self.cards[0];
        for i in self.cards.iter() {
            if i.value > highest.value {
                highest = *i;
            }
        }
        highest
    }

    pub fn _hand_value(&self) -> HandValues {
        hand_value_calculator::hand_value(&self.cards)
    }
}

#[cfg(test)]
mod tests {
    use crate::poker::Poker;

    use super::*;

    #[test]
    fn straight_flush_works() {
        let poker = Poker::new_texas_hold_em(4);

        let mut cards: Vec<Card> = Vec::new();
        for i in 0..poker.total_cards as usize {
            cards.push(poker.deck.cards[i]);
        }

        let hand = Hand::new(cards);
        hand._hand_value();
        assert_eq!(hand.value, HandValues::StraightFlush);
    }
}
