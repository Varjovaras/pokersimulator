use crate::deck::{Card, Suit, Value};

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

pub struct Hand {
    hand: Vec<Card>,
    value: HandValues,
}

impl Hand {
    pub fn hand_value(&self) -> HandValues {
        if self.hand.len() < 5 {
            panic!("Hand size too small");
        }
        let is_flush: bool = self.is_flush();
        let is_straight: bool = self.is_straight();
        return HandValues::RoyalFlush;
    }

    fn is_flush(&self) -> bool {
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

    fn is_straight(&self) -> bool {
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

        todo!()
    }
}
