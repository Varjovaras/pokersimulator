use crate::deck::{Card, Suit};

pub enum HandValues {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

impl HandValues {}

pub struct Hand {
    hand: Vec<Card>,
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

    pub fn is_flush(&self) -> bool {
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
        todo!()
    }
}
