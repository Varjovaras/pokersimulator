use crate::deck::{Card, Suit, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandValues {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
    RoyalFlush,
}

impl HandValues {}

#[derive(Debug)]
pub struct Hand {
    pub hand: Vec<Card>,
    pub value: HandValues,
}

impl Hand {
    pub fn _new(hand: Vec<Card>) -> Hand {
        return Hand {
            hand,
            value: HandValues::HighCard,
        };
    }

    pub fn _add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn _new_empty_hand() -> Hand {
        return Hand {
            hand: Vec::new(),
            value: HandValues::HighCard,
        };
    }

    pub fn _empty_hand(&mut self) {
        self.hand = Vec::new();
    }

    pub fn _highest_card(&self) -> Card {
        let mut highest = self.hand[0];
        for i in self.hand.iter() {
            if i.value > highest.value {
                highest = *i;
            }
        }
        return highest;
    }

    pub fn hand_value(&mut self) {
        if self.hand.len() < 5 {
            panic!("Hand size too small");
        }
        let _is_flush: bool = self._is_flush();
        let is_straight: bool = self._is_straight();
        if is_straight && _is_flush {
            // if self._is_royal_flush() {
            //     self.value = HandValues::RoyalFlush;
            // } else {
            //     self.value = HandValues::StraightFlush;
            // }
            self.value = HandValues::StraightFlush;
            return;
        }

        // if self._is_four_of_kind() {
        //     self.value = HandValues::FourOfKind;
        //     return;
        // }
    }

    pub fn _is_royal_flush(&self) -> bool {
        todo!("Implement later");
    }

    pub fn _is_flush(&self) -> bool {
        let mut hearts: u8 = 0;
        let mut diamonds: u8 = 0;
        let mut clubs: u8 = 0;
        let mut spades: u8 = 0;
        for card in self.hand.iter() {
            match card.suit {
                Suit::Hearts => hearts += 1,
                Suit::Diamonds => diamonds += 1,
                Suit::Clubs => clubs += 1,
                Suit::Spades => spades += 1,
            }
        }
        if hearts >= 5 || diamonds >= 5 || clubs >= 5 || spades >= 5 {
            return true;
        }
        return false;
    }

    fn _is_straight(&self) -> bool {
        let mut values = self.card_helper();
        let mut cards_in_a_row = 0;
        for i in values {
            if i == 10 && cards_in_a_row == 0 {
                break;
            }
            println!("{}", cards_in_a_row);
            if i == 0 {
                cards_in_a_row = 0;
                continue;
            }
            cards_in_a_row += 1;
            if cards_in_a_row == 5 {
                return true;
            }
        }

        return false;
    }

    fn card_helper(&self) -> [u8; 14] {
        let mut values: [u8; 14] = [0; 14];
        for card in self.hand.iter() {
            match card.value {
                Value::Two => {
                    values[1] += 1;
                }
                Value::Three => {
                    values[2] += 1;
                }
                Value::Four => {
                    values[3] += 1;
                }
                Value::Five => {
                    values[4] += 1;
                }
                Value::Six => {
                    values[5] += 1;
                }
                Value::Seven => {
                    values[6] += 1;
                }
                Value::Eight => {
                    values[7] += 1;
                }
                Value::Nine => {
                    values[8] += 1;
                }
                Value::Ten => {
                    values[9] += 1;
                }
                Value::Jack => {
                    values[10] += 1;
                }
                Value::Queen => {
                    values[11] += 1;
                }
                Value::King => {
                    values[12] += 1;
                }
                Value::Ace => {
                    values[0] += 1;
                    values[13] += 1;
                }
            }
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Poker;

    #[test]
    fn straight_flush_works() {
        let poker = Poker::new_texas_hold_em(4);

        let mut cards: Vec<Card> = Vec::new();
        for i in 0..poker.total_cards as usize {
            cards.push(poker.deck.cards[i]);
        }

        let mut hand = Hand::_new(cards);
        hand.hand_value();
        assert_eq!(hand.value, HandValues::StraightFlush);
    }
}
