mod deck;
use deck::Card;
use deck::DeckInGame;

#[warn(dead_code)]
enum _PokerHand {
    HighCard,
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

impl _PokerHand {
    fn _hand_value(&self) -> i32 {
        match *self {
            _PokerHand::HighCard => 0,
            _PokerHand::OnePair => 1,
            _PokerHand::TwoPair => 2,
            _PokerHand::ThreeOfKind => 3,
            _PokerHand::Straight => 4,
            _PokerHand::Flush => 5,
            _PokerHand::FullHouse => 6,
            _PokerHand::FourOfKind => 7,
            _PokerHand::StraightFlush => 8,
            _PokerHand::RoyalFlush => 9,
        }
    }
}

pub struct Poker {
    pub players: i32,
    pub hand_size: i32,
    pub cards_on_table: i32,
    pub deck: DeckInGame,
}

impl Poker {
    //initialization for custom game of hold em
    fn _new(players: i32, hand_size: i32, cards_on_table: i32) -> Poker {
        let deck = DeckInGame::new();

        Poker {
            players,
            hand_size,
            cards_on_table,
            deck,
        }
    }

    fn new_texas_hold_em(players: i32) -> Poker {
        let deck = DeckInGame::new();
        Poker {
            players,
            hand_size: 2,
            cards_on_table: 5,
            deck,
        }
    }
}

struct _PlayerHand {
    hand: Vec<Card>,
}

fn main() {
    let mut poker = Poker::new_texas_hold_em(4);
    let mut poker2 = Poker::new_texas_hold_em(4);

    poker.deck.shuffle_cards();
    let mut i: u128 = 0;
    while i < u128::MAX {
        poker2.deck.shuffle_cards();
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if poker.deck.cards == poker2.deck.cards {
            panic!("decks are equa");
        }
        i += 1;
    }

    let _card = poker.deck.top_card();
    let card = poker2.deck.top_card();
    // let card = deck.top_card();
    // let card = deck.top_card();
    // deck.shuffle_cards();

    println!("{:#?}", card);
    println!("{:#?}", poker.deck.cards);
    poker.deck.shuffle_cards();
    println!("pasa");
    println!("{:#?}", poker.deck.cards_dealt);
    // println!("{:#?}", card);
}
