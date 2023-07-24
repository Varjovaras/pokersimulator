use crate::deck::{Card, Suit, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandValues {
    _HighCard = 0,
    _OnePair,
    _TwoPair,
    _ThreeOfKind,
    _Straight,
    _Flush,
    _FullHouse,
    _FourOfKind,
    _StraightFlush,
    _RoyalFlush,
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
            value: HandValues::_HighCard,
        };
    }

    pub fn _add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn _new_empty_hand() -> Hand {
        return Hand {
            hand: Vec::new(),
            value: HandValues::_HighCard,
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

    pub fn _hand_value(&mut self) {
        if self.hand.len() < 5 {
            panic!("Hand size too small");
        }
        let _is_flush: bool = self._is_flush();
        let is_straight: bool = self._is_straight();
        if is_straight {
            self.value = HandValues::_Straight;
            return;
        }
        self.value = HandValues::_RoyalFlush;
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
        let mut values = [0; 14];
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

        let mut cards_in_a_row = 0;
        for i in values {
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

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Poker;

    #[test]
    fn test_is_straight() {
        let poker = Poker::new_texas_hold_em(4);

        let mut cards: Vec<Card> = Vec::new();
        for i in 0..7 {
            cards.push(poker.deck.cards[i]);
        }

        let mut hand = Hand::_new(cards);
        hand._hand_value();

        println!("{:#?}", hand.value);
        println!("{:#?}", hand.hand);

        assert_eq!(hand.value, HandValues::_Straight);
    }
}
