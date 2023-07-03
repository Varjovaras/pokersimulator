#[derive(Debug, Clone, Copy)]
enum Suit {
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

#[derive(Debug, Clone, Copy)]
enum Value {
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

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    /// Creates a card with the given suit and rank
    fn new(suit: Suit, value: Value) -> Card {
        Card { value, suit }
    }
}

#[derive(Debug)]

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut deck = Vec::new();
        for suit in SUITS {
            for value in VALUES {
                let card = Card::new(suit, value);
                deck.push(card);
            }
        }
        Deck { cards: deck }
    }
}

fn main() {
    let deck = Deck::new();

    println!("{:#?}", deck.cards);
}
