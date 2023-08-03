use crate::{
    deck::Card,
    hand_value_calculator::{self, HandValues},
};

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
