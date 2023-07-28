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
    _RoyalFlush,
}

impl HandValues {}

pub fn hand_value(hand: &Vec<Card>) -> HandValues {
    if hand.len() < 5 {
        panic!("Hand size too small");
    }
    let values = card_helper(&hand);

    let _is_flush: bool = is_flush(&hand);
    let is_straight: bool = _is_straight(values);
    if is_straight && _is_flush {
        // if self._is_royal_flush() {
        //     self.value = HandValues::RoyalFlush;
        // } else {
        //     self.value = HandValues::StraightFlush;
        // }
        return HandValues::StraightFlush;
    }

    if is_four_of_kind(values) {
        return HandValues::FourOfKind;
    }

    if is_full_house(values) {
        return HandValues::FullHouse;
    }

    return HandValues::HighCard;
}

pub fn card_helper(cards: &Vec<Card>) -> [u8; 14] {
    let mut values: [u8; 14] = [0; 14];
    for card in cards.iter() {
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

pub fn _is_royal_flush() -> bool {
    todo!("Implement later");
}

pub fn _is_straight(values: [u8; 14]) -> bool {
    let mut cards_in_a_row: u8 = 0;
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

pub fn is_flush(cards: &Vec<Card>) -> bool {
    let mut hearts: u8 = 0;
    let mut diamonds: u8 = 0;
    let mut clubs: u8 = 0;
    let mut spades: u8 = 0;
    for card in cards.iter() {
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

pub fn is_four_of_kind(values: [u8; 14]) -> bool {
    for i in values {
        if i == 4 {
            return true;
        }
    }
    return false;
}

pub fn is_full_house(values: [u8; 14]) -> bool {
    return values.contains(&3) && values.contains(&2);
}
