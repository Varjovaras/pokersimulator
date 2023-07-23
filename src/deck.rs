use rand::{seq::SliceRandom, thread_rng};
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn _self_value(&self) -> &str {
        match *self {
            Suit::Hearts => "HEARTS",
            Suit::Diamonds => "DIAMONDS",
            Suit::Clubs => "CLUBS",
            Suit::Spades => "SPADES",
        }
    }
}

pub const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]

pub enum Value {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub const VALUES: [Value; 13] = [
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
];

impl Value {
    fn _self_value(&self) -> i32 {
        match *self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }

    fn _from_char(ch: char) -> Result<Value, &'static str> {
        let rank = match ch {
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' | '1' => Value::Ace,
            _ => return Err("Invalid rank"),
        };
        Ok(rank)
    }

    pub fn _diff(self, other: Self) -> u8 {
        let min = cmp::min(self as u8, other as u8);
        let max = cmp::max(self as u8, other as u8);
        max - min
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { value, suit }
    }
}

#[derive(PartialEq)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in SUITS {
            for value in VALUES {
                let card = Card::new(suit, value);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    pub fn _new_empty_deck() -> Deck {
        let cards = Vec::new();
        Deck { cards }
    }

    pub fn shuffle_cards(&mut self) -> () {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn top_card(&mut self) -> Card {
        self.cards[0]
    }

    pub fn _contains_card(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }

    pub fn _add_card(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn _len(&self) -> usize {
        self.cards.len()
    }
}
