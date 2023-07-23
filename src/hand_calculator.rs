use crate::deck::{Card, Suit, Value};
#[derive(Debug, PartialEq)]
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
    pub fn new(hand: Vec<Card>) -> Hand {
        return Hand {
            hand,
            value: HandValues::HighCard,
        };
    }

    pub fn highest_card(&self) -> Card {
        let mut highest = self.hand[0];
        for i in self.hand.iter() {
            if i.value > highest.value {
                highest = *i;
            }
        }
        return highest.clone();
    }

    pub fn hand_value(&mut self) {
        if self.hand.len() < 5 {
            panic!("Hand size too small");
        }
        let is_flush: bool = self.is_flush();
        let is_straight: bool = self.is_straight();
        if is_straight {
            self.value = HandValues::Straight;
            return;
        }
        self.value = HandValues::RoyalFlush;
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

        let mut straight_helper = 0;
        for i in values {
            println!("{}", straight_helper);
            if i == 0 {
                straight_helper = 0;
                continue;
            }
            straight_helper += 1;
            if straight_helper == 5 {
                return true;
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        deck::Card,
        hand_calculator::{self, HandValues},
        Game,
    };

    #[test]
    fn test_is_straight() {
        let mut poker = Game::new_texas_hold_em(4);

        let mut cards: Vec<Card> = Vec::new();
        for i in 0..7 {
            cards.push(poker.deck.cards[i]);
        }

        let mut hand = hand_calculator::Hand::new(cards);
        hand.hand_value();

        println!("{:#?}", hand.value);
        println!("{:#?}", hand.hand);

        assert_eq!(hand.value, HandValues::Straight);

        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
    }
}
