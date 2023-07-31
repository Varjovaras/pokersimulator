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
        return Hand { cards, value };
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

    pub fn hand_value(&self) -> HandValues {
        return hand_value_calculator::hand_value(&self.cards);
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
        hand.hand_value();
        assert_eq!(hand.value, HandValues::StraightFlush);
    }
}