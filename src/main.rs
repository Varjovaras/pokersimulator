use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    cards_on_table: Vec<Card>,
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
        Deck {
            cards: deck,
            cards_on_table: Vec::new(),
        }
    }

    fn shuffle_cards(&mut self) -> () {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn top_card(&mut self) -> Card {
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

fn main() {
    let mut deck = Deck::new();
    deck.shuffle_cards();

    let card = deck.top_card();

    let mut deck2 = Deck::new();

    for i in 1..1000000 {
        deck2.shuffle_cards();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if deck.cards == deck2.cards {
            panic!("decks are equa");
        }
    }

    println!("{:#?}", card);
    println!("{:#?}", deck.cards);
    println!("{:#?}", deck.cards_on_table);
    println!("{:#?}", card);
}
