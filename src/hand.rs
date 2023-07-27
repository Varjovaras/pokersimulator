use crate::{
    deck::Card,
    hand_calculator::{self, HandValues, _is_straight, is_flush},
};

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub value: HandValues,
}

impl Hand {
    pub fn _new(hand: Vec<Card>) -> Hand {
        return Hand {
            cards: hand,
            value: HandValues::HighCard,
        };
    }

    pub fn _add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn _new_empty_hand() -> Hand {
        return Hand {
            cards: Vec::new(),
            value: HandValues::HighCard,
        };
    }

    pub fn _empty_hand(&mut self) {
        self.cards = Vec::new();
    }

    pub fn _highest_card(&self) -> Card {
        let mut highest = self.cards[0];
        for i in self.cards.iter() {
            if i.value > highest.value {
                highest = *i;
            }
        }
        return highest;
    }

    pub fn hand_value(&mut self) {
        if self.cards.len() < 5 {
            panic!("Hand size too small");
        }
        let mut values = hand_calculator::card_helper(self);

        let _is_flush: bool = is_flush(self);
        let is_straight: bool = _is_straight(values);
        if is_straight && _is_flush {
            // if self._is_royal_flush() {
            //     self.value = HandValues::RoyalFlush;
            // } else {
            //     self.value = HandValues::StraightFlush;
            // }
            self.value = HandValues::StraightFlush;
            return;
        }
    }
}
