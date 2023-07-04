use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn self_value(&self) -> &str {
        match *self {
            Suit::Hearts => "HEARTS",
            Suit::Diamonds => "DIAMONDS",
            Suit::Clubs => "CLUBS",
            Suit::Spades => "SPADES",
        }
    }
}

const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Two,
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

const VALUES: [Value; 13] = [
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
    fn self_value(&self) -> i32 {
        match *self {
            Value::Two => 0,
            Value::Three => 1,
            Value::Four => 2,
            Value::Five => 3,
            Value::Six => 4,
            Value::Seven => 5,
            Value::Eight => 6,
            Value::Nine => 7,
            Value::Ten => 8,
            Value::Jack => 9,
            Value::Queen => 10,
            Value::King => 11,
            Value::Ace => 12,
        }
    }

    fn from_char(ch: char) -> Result<Value, &'static str> {
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    /// Creates a card with the given suit and rank
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { value, suit }
    }
}

#[derive(Debug)]

pub struct Deck_in_game {
    pub cards: Vec<Card>,
    pub cards_on_table: Vec<Card>,
}

impl Deck_in_game {
    pub fn new() -> Deck_in_game {
        let mut deck = Vec::new();
        for suit in SUITS {
            for value in VALUES {
                let card = Card::new(suit, value);
                deck.push(card);
            }
        }
        Deck_in_game {
            cards: deck,
            cards_on_table: Vec::new(),
        }
    }

    pub fn shuffle_cards(&mut self) -> () {
        let mut rng = thread_rng();
        for _i in 0..self.cards_on_table.len() {
            match self.cards_on_table.pop() {
                Some(card) => self.cards.push(card),
                None => panic!("No card while trying to push from cards_to_table to cards"),
            }
        }
        self.cards.shuffle(&mut rng);
    }

    pub fn top_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            Some(card) => {
                self.cards_on_table.push(card.clone());
                return card;
            }
            None => todo!(""),
        };
    }
}
