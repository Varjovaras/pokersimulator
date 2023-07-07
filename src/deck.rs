use rand::{seq::SliceRandom, thread_rng};

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

pub struct DeckInGame {
    pub cards: Vec<Card>,
    pub cards_dealt: Vec<Card>,
}

impl DeckInGame {
    pub fn new() -> DeckInGame {
        let mut deck = Vec::new();
        for suit in SUITS {
            for value in VALUES {
                let card = Card::new(suit, value);
                deck.push(card);
            }
        }
        DeckInGame {
            cards: deck,
            cards_dealt: Vec::new(),
        }
    }

    pub fn shuffle_cards(&mut self) -> () {
        let mut rng = thread_rng();
        for _i in 0..self.cards_dealt.len() {
            match self.cards_dealt.pop() {
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
                self.cards_dealt.push(card.clone());
                return card;
            }
            None => todo!(""),
        };
    }
}
