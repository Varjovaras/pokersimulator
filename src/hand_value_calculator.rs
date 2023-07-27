use crate::{
    deck::{Suit, Value},
    hand::Hand,
};

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

pub fn card_helper(hand: &mut Hand) -> [u8; 14] {
    let mut values: [u8; 14] = [0; 14];
    for card in hand.cards.iter() {
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

pub fn is_flush(hand: &mut Hand) -> bool {
    let mut hearts: u8 = 0;
    let mut diamonds: u8 = 0;
    let mut clubs: u8 = 0;
    let mut spades: u8 = 0;
    for card in hand.cards.iter() {
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

pub fn _is_four_of_kind(values: [u8; 14]) -> bool {
    for i in values {
        if i == 4 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{deck::Card, Poker};

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
