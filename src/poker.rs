use crate::deck::{Card, Suit};

pub enum PokerHands {
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

impl PokerHands {}

pub struct PokerHand {
    hand: Vec<Card>,
}

impl PokerHand {
    pub fn hand_value(&self) -> PokerHands {
        if self.hand.len() < 5 {
            panic!("Hand size too small");
        }
        return PokerHands::RoyalFlush;
    }

    pub fn is_flush(&self) -> Option<Suit> {
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

        if hearts >= 5 {
            return Some(Suit::Hearts);
        } else if diamonds >= 5 {
            return Some(Suit::Diamonds);
        } else if clubs >= 5 {
            return Some(Suit::Clubs);
        } else if spades > 5 {
            return Some(Suit::Spades);
        } else {
            return None;
        };
    }
}
