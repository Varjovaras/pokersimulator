use crate::deck::{Card, Suit, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
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

pub fn hand_value(hand: &Vec<Card>) -> HandValues {
    if hand.len() < 5 {
        println!("{:?}", hand);
        return HandValues::HighCard;
        // panic!("Hand size too small");
    }
    let values = card_helper(&hand);

    let is_flush: bool = is_flush(&hand);
    let is_straight: bool = is_straight(values);
    if is_straight && is_flush {
        return is_straight_flush(hand);
    }

    if is_four_of_kind(values) {
        return HandValues::FourOfKind;
    }

    if is_full_house(values) {
        return HandValues::FullHouse;
    }

    if is_flush {
        return HandValues::Flush;
    }

    if is_straight {
        return HandValues::Straight;
    }

    if is_three_of_kind(values) {
        return HandValues::ThreeOfKind;
    }

    // println!("{:?}", hand);

    match how_many_pairs(values) {
        0 => return HandValues::HighCard,
        1 => return HandValues::OnePair,
        2 => return HandValues::TwoPair,
        3 => return HandValues::TwoPair, //possible with ace cause checking both 1 and 14
        _ => return HandValues::HighCard,
    }
}

fn is_straight(values: [u8; 14]) -> bool {
    let mut cards_in_a_row: u8 = 0;
    for i in values {
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

fn is_flush(cards: &Vec<Card>) -> bool {
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
/*
* 1. Returns HandValues::RoyalFlush if hand is a royal flush
* 2. Returns HandValues::StraightFlush if hand is a straight flush
* 3. Returns HandValues::Flush if hand is a flush
*/

fn is_straight_flush(hand: &Vec<Card>) -> HandValues {
    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let values = [
        Value::Ace,
        Value::Two,
        Value::Three,
        Value::Four,
        Value::Five,
        Value::Six,
        Value::Seven,
        Value::Eight,
        Value::Nine,
        Value::Ten,
        Value::Jack,
        Value::Queen,
        Value::King,
        Value::Ace,
    ];

    let mut straight_in_row: u8;
    for suit in suits {
        straight_in_row = 0;
        for value in values {
            if !hand.contains(&Card { suit, value }) {
                straight_in_row = 0;
                continue;
            }
            straight_in_row += 1;
            if straight_in_row == 5 && value == Value::Ace {
                return HandValues::RoyalFlush;
            }
            if straight_in_row == 5 {
                return HandValues::StraightFlush;
            }
        }
    }
    return HandValues::Flush;
}

fn is_four_of_kind(values: [u8; 14]) -> bool {
    for i in values {
        if i == 4 {
            return true;
        }
    }
    return false;
}

fn is_full_house(values: [u8; 14]) -> bool {
    return values.contains(&3) && values.contains(&2);
}

fn is_three_of_kind(values: [u8; 14]) -> bool {
    return values.contains(&3) && !values.contains(&2) && !values.contains(&4);
}

fn how_many_pairs(values: [u8; 14]) -> u8 {
    let mut pairs: u8 = 0;
    for i in values {
        if i == 2 {
            pairs += 1;
        }
    }
    return pairs;
}

fn card_helper(cards: &Vec<Card>) -> [u8; 14] {
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
